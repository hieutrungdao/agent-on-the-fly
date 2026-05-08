// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! AOTF storage: SQLite + WAL connection wrapper and append-only audit log.
//!
//! Public surface:
//! - [`Storage::open`] — open or create a SQLite DB at the given path; sets
//!   `journal_mode = WAL` and `synchronous = NORMAL`; runs the v1 migration.
//! - [`Storage::append`] / [`Storage::list`] — append-only audit log API
//!   (defined in [`audit`]).
//!
//! Append-only enforcement is at the Rust API level: there is no `update`,
//! `delete`, or "raw connection" escape hatch on `pub` surface. A v0.0.3
//! hardening pass adds storage-layer enforcement (revoke DELETE via views /
//! triggers).

mod audit;

pub use audit::{AuditDecision, AuditEntry, NewAuditEntry};

use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

/// SQL bootstrap for v1 schema. Idempotent (uses `CREATE TABLE IF NOT EXISTS`).
const MIGRATION_0001: &str = include_str!("migrations/0001_init.sql");

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("audit row corrupted: {0}")]
    CorruptedRow(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// SQLite-backed storage. Holds a single mutex-guarded connection.
///
/// Walking-skeleton single-writer model: one process, one connection. The
/// gatekeeper is the only writer; the CLI opens read-only via [`Storage::open_read_only`].
pub struct Storage {
    pub(crate) conn: Mutex<Connection>,
}

impl Storage {
    /// Open or create a SQLite DB at `path`. Sets WAL and runs migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let path_ref = path.as_ref();
        if let Some(parent) = path_ref.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let conn = Connection::open(path_ref)?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
        conn.execute_batch(MIGRATION_0001)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Open an existing DB read-only. Used by `aotf-cli` to inspect the audit
    /// log without writer contention. Errors if the path does not exist.
    pub fn open_read_only(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let conn = Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Return the active SQLite `journal_mode` (lowercased). Useful for tests
    /// to confirm WAL is in effect.
    pub fn journal_mode(&self) -> Result<String, StorageError> {
        let conn = self.conn.lock().expect("storage mutex poisoned");
        let mode: String =
            conn.query_row("PRAGMA journal_mode", [], |row| row.get::<_, String>(0))?;
        Ok(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_creates_db_and_enables_wal() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("aotf.db");
        let s = Storage::open(&path).unwrap();
        assert_eq!(s.journal_mode().unwrap().to_lowercase(), "wal");
    }

    #[test]
    fn open_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested/dir/aotf.db");
        let _ = Storage::open(&path).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn migration_is_idempotent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("aotf.db");
        let _ = Storage::open(&path).unwrap();
        let _ = Storage::open(&path).unwrap(); // re-open, no-op migration
    }
}
