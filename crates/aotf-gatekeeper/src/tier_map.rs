// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! FR-105 tier-classification table. Maps JSON-RPC method names to the action
//! tier the gatekeeper should evaluate.
//!
//! **Changes here require architecture-tier review per D-ENF-3.** Adding a
//! method without an entry yields `None`, which the JSON-RPC dispatcher must
//! treat as `METHOD_NOT_FOUND` — never default to a permissive tier (that
//! would create audit-log gaps).

use aotf_core_types::ActionTier;
use aotf_ipc_protocol::methods;

/// Returns the [`ActionTier`] for a known v1 JSON-RPC method, or `None` if
/// the method is unknown.
pub fn classify(method: &str) -> Option<ActionTier> {
    match method {
        m if m == methods::DAEMON_PING => Some(ActionTier::Read),
        m if m == methods::FINDING_EMIT => Some(ActionTier::Draft),
        m if m == methods::AUDIT_APPEND => Some(ActionTier::Draft),
        m if m == methods::AUDIT_LIST => Some(ActionTier::Read),
        m if m == methods::GATE_EVALUATE => Some(ActionTier::Read),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_methods_classify() {
        assert_eq!(classify(methods::DAEMON_PING), Some(ActionTier::Read));
        assert_eq!(classify(methods::FINDING_EMIT), Some(ActionTier::Draft));
        assert_eq!(classify(methods::AUDIT_APPEND), Some(ActionTier::Draft));
        assert_eq!(classify(methods::AUDIT_LIST), Some(ActionTier::Read));
        assert_eq!(classify(methods::GATE_EVALUATE), Some(ActionTier::Read));
    }

    #[test]
    fn unknown_method_returns_none() {
        assert_eq!(classify("nope.bogus"), None);
        assert_eq!(classify(""), None);
    }

    #[test]
    fn every_listed_method_has_a_classification() {
        // If a method is in `methods::ALL` but not in the tier-map, that's a
        // hole. This test fails on the first orphan, naming the offender.
        for m in methods::ALL {
            assert!(
                classify(m).is_some(),
                "method {m:?} is in methods::ALL but unclassified",
            );
        }
    }
}
