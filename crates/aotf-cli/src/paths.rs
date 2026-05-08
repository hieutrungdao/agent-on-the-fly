// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Shared path helpers for CLI commands.

use std::path::PathBuf;

/// `~/.aotf/run` (or `.aotf/run` if `$HOME` is unset, e.g. CI sandboxes).
pub fn default_runtime_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".aotf").join("run")
    } else {
        PathBuf::from(".aotf/run")
    }
}

/// `~/.aotf/config.toml`.
pub fn default_config_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".aotf").join("config.toml")
    } else {
        PathBuf::from(".aotf/config.toml")
    }
}

/// `<runtime>/audit.db`.
pub fn audit_db(runtime_dir: &std::path::Path) -> PathBuf {
    runtime_dir.join("audit.db")
}
