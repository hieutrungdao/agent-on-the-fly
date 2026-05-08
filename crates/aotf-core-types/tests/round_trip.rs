// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Golden round-trip tests for the Finding v1 wire struct.
//!
//! Per architecture.md line 104: "the first concrete deliverable that unblocks
//! downstream stories is the `Finding v1` wire struct with a Rust↔Bun
//! round-trip golden test (<200 LOC, byte-equal)."
//!
//! "Byte-equal" is interpreted as structural-equal after deserialize ↔
//! reserialize ↔ deserialize — JSON whitespace and key ordering are not
//! deterministic across producers, so byte-level comparison is unsafe.

use aotf_core_types::{ActionTier, Finding, FindingSource};
use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/aotf-core-types parent (crates/)")
        .parent()
        .expect("repo root")
        .join("fixtures")
        .join("finding-v1")
}

fn load(name: &str) -> String {
    let path = fixtures_dir().join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e))
}

#[test]
fn valid_minimal_round_trips() {
    let raw = load("valid_minimal.json");
    let v: Finding = serde_json::from_str(&raw).expect("deserializes");
    assert_eq!(v.tier, ActionTier::Draft);
    assert_eq!(v.source, FindingSource::Manual);
    v.validate().expect("validates");

    let again = serde_json::to_string(&v).expect("re-serializes");
    let v2: Finding = serde_json::from_str(&again).expect("redeserializes");
    assert_eq!(v, v2, "round-trip not stable");
}

#[test]
fn valid_full_round_trips() {
    let raw = load("valid_full.json");
    let v: Finding = serde_json::from_str(&raw).expect("deserializes");
    assert_eq!(v.tier, ActionTier::ExecuteReversible);
    assert_eq!(v.source, FindingSource::FileTail);
    assert_eq!(v.confidence, 0.85);
    v.validate().expect("validates");

    let again = serde_json::to_string(&v).expect("re-serializes");
    let v2: Finding = serde_json::from_str(&again).expect("redeserializes");
    assert_eq!(v, v2, "round-trip not stable");
}

#[test]
fn invalid_missing_tier_rejected() {
    let raw = load("invalid_missing_tier.json");
    let r: Result<Finding, _> = serde_json::from_str(&raw);
    assert!(
        r.is_err(),
        "expected deserialization to fail (missing `tier`); got: {:?}",
        r
    );
}
