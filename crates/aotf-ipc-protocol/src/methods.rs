// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Stable method-name constants for the AOTF JSON-RPC namespace.
//!
//! These string values are part of the v1 wire contract. Renaming or removing
//! any constant requires a v2 envelope and coordinated bump in every caller.

/// Liveness probe served by `aotfd`. Tier: `READ`.
pub const DAEMON_PING: &str = "daemon.ping";

/// Producer (watcher) emits a [`crate::Finding`]. Routed to gatekeeper for
/// classification, then to storage. Tier: `DRAFT`.
pub const FINDING_EMIT: &str = "finding.emit";

/// Append a record to the audit log. Only the gatekeeper writes; callers route
/// through it. Tier: `DRAFT`.
pub const AUDIT_APPEND: &str = "audit.append";

/// Read recent audit entries. Daemon serves read-only. Tier: `READ`.
pub const AUDIT_LIST: &str = "audit.list";

/// Ask the gatekeeper to evaluate an [`crate::ActionTier`] and return a
/// [`crate::Decision`]. Tier: `READ` (the evaluation is itself read-only;
/// downstream side-effects are the caller's responsibility).
pub const GATE_EVALUATE: &str = "gate.evaluate";

/// All v1 method names, in stable insertion order. Useful for snapshot tests
/// and CLI completion.
pub const ALL: &[&str] = &[
    DAEMON_PING,
    FINDING_EMIT,
    AUDIT_APPEND,
    AUDIT_LIST,
    GATE_EVALUATE,
];
