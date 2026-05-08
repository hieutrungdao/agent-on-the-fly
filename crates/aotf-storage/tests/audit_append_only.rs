// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Append-only audit log integration tests.
//!
//! Walking-skeleton enforcement is at the Rust API level: the public surface
//! exposes only `append` and `list`. There is no `delete` method, no `raw`
//! escape hatch, and no `update` — a v0.0.3 hardening pass adds the
//! storage-layer trigger as well.

use aotf_core_types::ActionTier;
use aotf_storage::{AuditDecision, NewAuditEntry, Storage};
use serde_json::json;
use tempfile::tempdir;

fn entry(finding_id: &str, tier: ActionTier, decision: AuditDecision) -> NewAuditEntry {
    NewAuditEntry {
        finding_id: finding_id.into(),
        tier,
        decision,
        reason: "test".into(),
        created_micros: 0,
        payload: json!({"src": "test"}),
    }
}

#[test]
fn append_then_list_returns_in_id_order() {
    let dir = tempdir().unwrap();
    let s = Storage::open(dir.path().join("aotf.db")).unwrap();

    let id1 = s
        .append(entry("f1", ActionTier::Draft, AuditDecision::Allow))
        .unwrap();
    let id2 = s
        .append(entry("f2", ActionTier::Propose, AuditDecision::Allow))
        .unwrap();
    let id3 = s
        .append(entry(
            "f3",
            ActionTier::ExecuteIrreversible,
            AuditDecision::Deny,
        ))
        .unwrap();

    assert!(id1 < id2 && id2 < id3, "ids must be monotonic");

    let all = s.list(None).unwrap();
    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, id1);
    assert_eq!(all[1].id, id2);
    assert_eq!(all[2].id, id3);

    assert_eq!(all[0].tier, ActionTier::Draft);
    assert_eq!(all[2].tier, ActionTier::ExecuteIrreversible);
    assert_eq!(all[2].decision, AuditDecision::Deny);
}

#[test]
fn list_since_id_filters() {
    let dir = tempdir().unwrap();
    let s = Storage::open(dir.path().join("aotf.db")).unwrap();

    let _ = s
        .append(entry("a", ActionTier::Read, AuditDecision::Allow))
        .unwrap();
    let id2 = s
        .append(entry("b", ActionTier::Read, AuditDecision::Allow))
        .unwrap();
    let id3 = s
        .append(entry("c", ActionTier::Read, AuditDecision::Allow))
        .unwrap();

    let after_id2 = s.list(Some(id2)).unwrap();
    assert_eq!(after_id2.len(), 1);
    assert_eq!(after_id2[0].id, id3);
}

#[test]
fn payload_roundtrips_through_json() {
    let dir = tempdir().unwrap();
    let s = Storage::open(dir.path().join("aotf.db")).unwrap();

    let payload = json!({"a": 1, "b": [true, false], "c": {"nested": "ok"}});
    let mut e = entry("payload-test", ActionTier::Draft, AuditDecision::Allow);
    e.payload = payload.clone();
    let _ = s.append(e).unwrap();

    let back = s.list(None).unwrap();
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].payload, payload);
}

#[test]
fn read_only_handle_can_list_but_not_append() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("aotf.db");
    let writer = Storage::open(&path).unwrap();
    writer
        .append(entry("seed", ActionTier::Draft, AuditDecision::Allow))
        .unwrap();
    drop(writer);

    let reader = Storage::open_read_only(&path).unwrap();
    let entries = reader.list(None).unwrap();
    assert_eq!(entries.len(), 1);

    // Append on a read-only handle must fail (SQLite rejects writes).
    let r = reader.append(entry("nope", ActionTier::Draft, AuditDecision::Allow));
    assert!(r.is_err(), "append on read-only handle must fail");
}
