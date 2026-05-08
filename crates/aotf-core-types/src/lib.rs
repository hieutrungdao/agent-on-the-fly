// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Shared domain types for AOTF.
//!
//! - [`ActionTier`] — the 5-tier reversibility taxonomy (FR-105). Wire encoding
//!   is `SCREAMING_SNAKE_CASE` (`"READ"`, `"DRAFT"`, `"PROPOSE"`,
//!   `"EXECUTE_REVERSIBLE"`, `"EXECUTE_IRREVERSIBLE"`).
//! - [`Finding`] v1 — the frozen wire struct. Fields are `camelCase` on the wire,
//!   `snake_case` internally.
//! - [`Decision`] — the gatekeeper's allow/deny verdict.
//! - [`Clock`] — monotonic-friendly time source; [`SystemClock`] is the default.
//!
//! All cross-IPC types derive [`ts_rs::TS`] so the Bun agent imports generated
//! TypeScript instead of hand-writing wire types.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;
use uuid::Uuid;

// -- ActionTier ---------------------------------------------------------------

/// The 5 action tiers per FR-105. Plugin manifests declare a ceiling; the
/// gatekeeper classifies; violations produce an audit entry and plugin
/// termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export)]
pub enum ActionTier {
    /// Passive observation. No side effects.
    Read,
    /// Local-only artifacts (git branches, scratch files). Reversible by
    /// rollback.
    Draft,
    /// Public-facing objects (PRs, issues). Reversible by close/archive.
    Propose,
    /// Autonomous action with guaranteed programmatic rollback.
    ExecuteReversible,
    /// Autonomous action with no programmatic undo. Requires consent token.
    ExecuteIrreversible,
}

impl ActionTier {
    /// Wire encoding (`"READ"`, `"DRAFT"`, …). Useful for audit log columns
    /// where the encoding must match exactly across serializers.
    pub fn as_wire_str(&self) -> &'static str {
        match self {
            Self::Read => "READ",
            Self::Draft => "DRAFT",
            Self::Propose => "PROPOSE",
            Self::ExecuteReversible => "EXECUTE_REVERSIBLE",
            Self::ExecuteIrreversible => "EXECUTE_IRREVERSIBLE",
        }
    }
}

// -- FindingSource ------------------------------------------------------------

/// Where a [`Finding`] came from. Walking-skeleton scope is `FileTail`,
/// `DockerStdout`, and `Manual`. Polled sources (Loki, Datadog) are Growth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export)]
pub enum FindingSource {
    FileTail,
    DockerStdout,
    Manual,
}

// -- FindingId ----------------------------------------------------------------

/// A UUID-v7 string identifier for a [`Finding`]. Stored as a canonical UUID
/// string so the wire shape is identical for Rust and TypeScript without
/// requiring `ts-rs` ↔ `uuid` integration glue.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export, type = "string")]
pub struct FindingId(pub String);

impl FindingId {
    /// Generate a fresh UUID-v7 (time-ordered).
    pub fn new() -> Self {
        Self(Uuid::now_v7().to_string())
    }
}

impl Default for FindingId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for FindingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// -- Finding (v1, frozen) -----------------------------------------------------

/// Maximum length of [`Finding::summary`] in characters.
pub const FINDING_SUMMARY_MAX: usize = 200;

/// Wire-frozen Finding v1 struct. Fields are `camelCase` on the wire.
///
/// All eight fields are part of the v1 wire contract. Adding fields requires a
/// v2 envelope (out of scope for the walking skeleton).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Finding {
    /// Stable id (UUID v7).
    pub id: FindingId,
    /// Action tier the producer claims; gatekeeper re-classifies.
    pub tier: ActionTier,
    /// Where the finding came from.
    pub source: FindingSource,
    /// One-line human summary, ≤ [`FINDING_SUMMARY_MAX`] chars.
    pub summary: String,
    /// Free-form structured payload. Redacted before the audit log per FR-68.
    pub payload: serde_json::Value,
    /// Producer wall-clock micros at emit time. Used only for human display.
    pub created_at_micros: i64,
    /// Lamport timestamp; walking-skeleton stub returns 0.
    pub lamport: u64,
    /// Confidence in [0.0, 1.0]; walking-skeleton stub default is 0.0.
    pub confidence: f32,
}

#[derive(Debug, Error, PartialEq)]
pub enum FindingError {
    #[error("summary exceeds {FINDING_SUMMARY_MAX} chars (got {0})")]
    SummaryTooLong(usize),
    #[error("confidence must be in [0.0, 1.0]; got {0}")]
    ConfidenceOutOfRange(f32),
}

impl Finding {
    /// Validate invariants. Cheap; call before serializing.
    pub fn validate(&self) -> Result<(), FindingError> {
        if self.summary.chars().count() > FINDING_SUMMARY_MAX {
            return Err(FindingError::SummaryTooLong(self.summary.chars().count()));
        }
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(FindingError::ConfidenceOutOfRange(self.confidence));
        }
        Ok(())
    }
}

// -- Decision -----------------------------------------------------------------

/// Allow/deny verdict from the gatekeeper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Decision {
    /// The tier the action was classified as.
    pub tier: ActionTier,
    /// `true` = proceed, `false` = block.
    pub allow: bool,
    /// Human-readable explanation. Recorded verbatim in the audit log.
    pub reason: String,
}

impl Decision {
    pub fn allow(tier: ActionTier, reason: impl Into<String>) -> Self {
        Self {
            tier,
            allow: true,
            reason: reason.into(),
        }
    }

    pub fn deny(tier: ActionTier, reason: impl Into<String>) -> Self {
        Self {
            tier,
            allow: false,
            reason: reason.into(),
        }
    }
}

// -- Clock --------------------------------------------------------------------

/// Time source. Production uses [`SystemClock`]; tests can substitute a fixed
/// clock to make audit-log ordering deterministic.
pub trait Clock: Send + Sync {
    /// Microseconds since UNIX_EPOCH.
    fn now_micros(&self) -> i64;
}

/// Wall-clock-based [`Clock`]. Walking-skeleton's only impl.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_micros(&self) -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before UNIX_EPOCH")
            .as_micros() as i64
    }
}

// -- Tests --------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_tier_wire_roundtrip() {
        for variant in [
            ActionTier::Read,
            ActionTier::Draft,
            ActionTier::Propose,
            ActionTier::ExecuteReversible,
            ActionTier::ExecuteIrreversible,
        ] {
            let s = serde_json::to_string(&variant).unwrap();
            let back: ActionTier = serde_json::from_str(&s).unwrap();
            assert_eq!(variant, back);
            // Ensure the wire form is the SCREAMING_SNAKE form.
            assert_eq!(s.trim_matches('"'), variant.as_wire_str());
        }
    }

    #[test]
    fn finding_id_is_uuid_v7() {
        let id = FindingId::new();
        // Canonical UUID is 36 chars: 8-4-4-4-12 hex with hyphens.
        assert_eq!(id.0.len(), 36);
        // v7 has '7' as the version nibble (position 14 in canonical form).
        let bytes = id.0.as_bytes();
        assert_eq!(bytes[14], b'7');
    }

    #[test]
    fn finding_summary_too_long_rejected() {
        let f = Finding {
            id: FindingId::new(),
            tier: ActionTier::Draft,
            source: FindingSource::Manual,
            summary: "x".repeat(FINDING_SUMMARY_MAX + 1),
            payload: serde_json::Value::Null,
            created_at_micros: 0,
            lamport: 0,
            confidence: 0.0,
        };
        assert!(matches!(f.validate(), Err(FindingError::SummaryTooLong(_))));
    }

    #[test]
    fn finding_confidence_out_of_range_rejected() {
        let f = Finding {
            id: FindingId::new(),
            tier: ActionTier::Draft,
            source: FindingSource::Manual,
            summary: "ok".into(),
            payload: serde_json::Value::Null,
            created_at_micros: 0,
            lamport: 0,
            confidence: 1.5,
        };
        assert!(matches!(
            f.validate(),
            Err(FindingError::ConfidenceOutOfRange(_))
        ));
    }

    #[test]
    fn decision_constructors() {
        let a = Decision::allow(ActionTier::Read, "ok");
        assert!(a.allow);
        assert_eq!(a.tier, ActionTier::Read);

        let d = Decision::deny(ActionTier::ExecuteIrreversible, "no consent");
        assert!(!d.allow);
        assert_eq!(d.tier, ActionTier::ExecuteIrreversible);
    }

    #[test]
    fn system_clock_advances_or_equal() {
        let c = SystemClock;
        let a = c.now_micros();
        let b = c.now_micros();
        assert!(b >= a);
    }
}
