// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Exhaustive tier coverage. Every `ActionTier` variant has a corresponding
//! `gate_*` function that returns the expected `Decision`. This test is the
//! mutation-testing target; the v0.0.3 weekly `cargo-mutants` run mutates
//! the per-tier branches and expects this file to fail on each mutation.

// `aotf-gatekeeper` is a binary crate. We re-import the gate module by file.
// To keep the per-tier functions testable from an integration test, we
// declare a thin shim module.
#[path = "../src/gate.rs"]
mod gate;

use aotf_core_types::ActionTier;

#[test]
fn read_allows() {
    let d = gate::gate_read();
    assert_eq!(d.tier, ActionTier::Read);
    assert!(d.allow);
}

#[test]
fn draft_allows() {
    let d = gate::gate_draft();
    assert_eq!(d.tier, ActionTier::Draft);
    assert!(d.allow);
}

#[test]
fn propose_allows() {
    let d = gate::gate_propose();
    assert_eq!(d.tier, ActionTier::Propose);
    assert!(d.allow);
}

#[test]
fn execute_reversible_allows_with_stub_reason() {
    let d = gate::gate_execute_reversible();
    assert_eq!(d.tier, ActionTier::ExecuteReversible);
    assert!(d.allow);
    assert!(
        d.reason.contains("walking-skeleton") || d.reason.contains("v0.0.3"),
        "reason should mention walking-skeleton scope: {:?}",
        d.reason
    );
}

#[test]
fn execute_irreversible_denies_for_missing_consent() {
    let d = gate::gate_execute_irreversible();
    assert_eq!(d.tier, ActionTier::ExecuteIrreversible);
    assert!(!d.allow);
    assert!(
        d.reason.contains("FR-106") || d.reason.contains("consent"),
        "reason should reference consent / FR-106: {:?}",
        d.reason
    );
}

#[test]
fn evaluate_dispatches_to_each_branch() {
    use ActionTier::*;
    for tier in [Read, Draft, Propose, ExecuteReversible, ExecuteIrreversible] {
        let d = gate::evaluate(tier);
        assert_eq!(d.tier, tier);
        let expected_allow = !matches!(tier, ExecuteIrreversible);
        assert_eq!(d.allow, expected_allow, "tier {tier:?}");
    }
}
