// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Append-only audit log.
//!
//! The public surface is intentionally narrow: callers can [`Storage::append`]
//! a new entry and [`Storage::list`] entries since a given id. No update, no
//! delete — the type system is the append-only enforcement.

use crate::{Storage, StorageError};
use aotf_core_types::ActionTier;
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// Allow or deny verdict recorded in the audit log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditDecision {
    Allow,
    Deny,
}

impl AuditDecision {
    pub fn as_wire_str(&self) -> &'static str {
        match self {
            Self::Allow => "ALLOW",
            Self::Deny => "DENY",
        }
    }

    fn from_wire_str(s: &str) -> Result<Self, StorageError> {
        match s {
            "ALLOW" => Ok(Self::Allow),
            "DENY" => Ok(Self::Deny),
            other => Err(StorageError::CorruptedRow(format!(
                "unknown decision: {other}"
            ))),
        }
    }
}

/// A new audit entry — no id yet, [`Storage::append`] assigns one.
#[derive(Debug, Clone)]
pub struct NewAuditEntry {
    pub finding_id: String,
    pub tier: ActionTier,
    pub decision: AuditDecision,
    pub reason: String,
    pub created_micros: i64,
    pub payload: serde_json::Value,
}

/// A persisted audit entry, with the autoincrement id assigned by SQLite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntry {
    pub id: u64,
    pub finding_id: String,
    pub tier: ActionTier,
    pub decision: AuditDecision,
    pub reason: String,
    pub created_micros: i64,
    pub payload: serde_json::Value,
}

fn parse_tier(s: &str) -> Result<ActionTier, StorageError> {
    match s {
        "READ" => Ok(ActionTier::Read),
        "DRAFT" => Ok(ActionTier::Draft),
        "PROPOSE" => Ok(ActionTier::Propose),
        "EXECUTE_REVERSIBLE" => Ok(ActionTier::ExecuteReversible),
        "EXECUTE_IRREVERSIBLE" => Ok(ActionTier::ExecuteIrreversible),
        other => Err(StorageError::CorruptedRow(format!("unknown tier: {other}"))),
    }
}

impl Storage {
    /// Append an entry. Returns the assigned id (autoincrement; total order
    /// per single-writer invariant).
    pub fn append(&self, entry: NewAuditEntry) -> Result<u64, StorageError> {
        let conn = self.conn.lock().expect("audit mutex poisoned");
        let payload_json = serde_json::to_string(&entry.payload)?;
        conn.execute(
            "INSERT INTO audit_entry \
             (finding_id, tier, decision, reason, created_micros, payload_json) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                entry.finding_id,
                entry.tier.as_wire_str(),
                entry.decision.as_wire_str(),
                entry.reason,
                entry.created_micros,
                payload_json,
            ],
        )?;
        Ok(conn.last_insert_rowid() as u64)
    }

    /// Return entries with id strictly greater than `since_id` (or all if
    /// `since_id` is `None`), in ascending id order.
    pub fn list(&self, since_id: Option<u64>) -> Result<Vec<AuditEntry>, StorageError> {
        let conn = self.conn.lock().expect("audit mutex poisoned");
        let since: i64 = since_id.unwrap_or(0) as i64;
        let mut stmt = conn.prepare(
            "SELECT id, finding_id, tier, decision, reason, created_micros, payload_json \
             FROM audit_entry \
             WHERE id > ?1 \
             ORDER BY id ASC",
        )?;
        let rows = stmt.query_map(params![since], |row| {
            let id: i64 = row.get(0)?;
            let finding_id: String = row.get(1)?;
            let tier: String = row.get(2)?;
            let decision: String = row.get(3)?;
            let reason: String = row.get(4)?;
            let created_micros: i64 = row.get(5)?;
            let payload_json: String = row.get(6)?;
            Ok((
                id,
                finding_id,
                tier,
                decision,
                reason,
                created_micros,
                payload_json,
            ))
        })?;

        let mut out = Vec::new();
        for row in rows {
            let (id, finding_id, tier_s, decision_s, reason, created_micros, payload_json) = row?;
            let tier = parse_tier(&tier_s)?;
            let decision = AuditDecision::from_wire_str(&decision_s)?;
            let payload: serde_json::Value = serde_json::from_str(&payload_json)?;
            out.push(AuditEntry {
                id: id as u64,
                finding_id,
                tier,
                decision,
                reason,
                created_micros,
                payload,
            });
        }
        Ok(out)
    }
}
