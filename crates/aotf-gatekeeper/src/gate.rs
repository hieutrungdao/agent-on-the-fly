// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! The pure gate function. Given an [`ActionTier`], returns a [`Decision`].
//!
//! Each tier's policy is implemented as its own named function (`gate_read`,
//! `gate_draft`, `gate_propose`, `gate_execute_reversible`,
//! `gate_execute_irreversible`) so that future `cargo-mutants` runs can
//! target each branch individually per architecture.md line 600 / FR-105.2.
//!
//! Walking-skeleton policy:
//! - `READ`, `DRAFT`, `PROPOSE` → always allow.
//! - `EXECUTE_REVERSIBLE` → allow, with the audit trail noting the rollback
//!   is the caller's responsibility until v0.0.3 wires programmatic rollback.
//! - `EXECUTE_IRREVERSIBLE` → always deny; consent-token system (FR-106) is
//!   not implemented in v0.0.2-alpha.

use aotf_core_types::{ActionTier, Decision};

const REASON_READ: &str = "READ tier has no side effects";
const REASON_DRAFT: &str = "DRAFT tier rolls back via local-only artifact removal";
const REASON_PROPOSE: &str = "PROPOSE tier rolls back via PR close/archive";
const REASON_EXEC_REV: &str =
    "walking-skeleton stub: caller-responsible rollback (FR-29 wiring deferred to v0.0.3)";
const REASON_EXEC_IRREV: &str = "consent-token system (FR-106) not implemented in v0.0.2-alpha — irreversible actions must wait";

/// Dispatch on tier; one branch per `ActionTier` variant.
pub fn evaluate(tier: ActionTier) -> Decision {
    match tier {
        ActionTier::Read => gate_read(),
        ActionTier::Draft => gate_draft(),
        ActionTier::Propose => gate_propose(),
        ActionTier::ExecuteReversible => gate_execute_reversible(),
        ActionTier::ExecuteIrreversible => gate_execute_irreversible(),
    }
}

pub fn gate_read() -> Decision {
    Decision::allow(ActionTier::Read, REASON_READ)
}

pub fn gate_draft() -> Decision {
    Decision::allow(ActionTier::Draft, REASON_DRAFT)
}

pub fn gate_propose() -> Decision {
    Decision::allow(ActionTier::Propose, REASON_PROPOSE)
}

pub fn gate_execute_reversible() -> Decision {
    Decision::allow(ActionTier::ExecuteReversible, REASON_EXEC_REV)
}

pub fn gate_execute_irreversible() -> Decision {
    Decision::deny(ActionTier::ExecuteIrreversible, REASON_EXEC_IRREV)
}
