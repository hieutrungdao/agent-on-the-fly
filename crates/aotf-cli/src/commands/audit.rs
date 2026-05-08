// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotf audit ls` — read-only inspection of the audit log.

use std::path::Path;

use anyhow::{Context, Result};
use aotf_storage::Storage;

use crate::paths;

pub fn ls(runtime_dir: &Path, limit: usize, since: Option<u64>) -> Result<()> {
    let db_path = paths::audit_db(runtime_dir);
    if !db_path.exists() {
        anyhow::bail!(
            "no audit db at {} — run `aotf watch` first",
            db_path.display()
        );
    }
    let storage =
        Storage::open_read_only(&db_path).with_context(|| format!("open ro {db_path:?}"))?;
    let mut entries = storage.list(since)?;
    // Newest-first.
    entries.reverse();
    entries.truncate(limit);

    if entries.is_empty() {
        println!("(no audit entries)");
        return Ok(());
    }

    println!(
        "{:>6}  {:<22}  {:<6}  {:<24}  reason",
        "id", "tier", "verd.", "finding"
    );
    for e in &entries {
        let verdict = match e.decision {
            aotf_storage::AuditDecision::Allow => "ALLOW",
            aotf_storage::AuditDecision::Deny => "DENY",
        };
        let finding = if e.finding_id.len() > 24 {
            format!("{}…", &e.finding_id[..23])
        } else {
            e.finding_id.clone()
        };
        let tier = e.tier.as_wire_str();
        let reason = if e.reason.len() > 80 {
            format!("{}…", &e.reason[..79])
        } else {
            e.reason.clone()
        };
        println!(
            "{:>6}  {:<22}  {:<6}  {:<24}  {}",
            e.id, tier, verdict, finding, reason
        );
    }
    Ok(())
}
