// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! AOTF IPC protocol: JSON-RPC 2.0 schemas, method names, and the [`Finding`]
//! v1 envelope.
//!
//! Re-exports the wire-frozen types from [`aotf_core_types`] so consumers only
//! need a single dependency on this crate.
//!
//! Method namespace conventions:
//! - `daemon.*` — operations served by `aotfd`.
//! - `gate.*` — operations served by `aotf-gatekeeper`.
//! - `audit.*` — append-only audit log access (write through gatekeeper, read
//!   through daemon read-only).
//! - `finding.*` — finding emission and lookup.

pub mod methods;
pub mod wire;

pub use aotf_core_types::{
    ActionTier, Clock, Decision, FINDING_SUMMARY_MAX, Finding, FindingError, FindingId,
    FindingSource, SystemClock,
};
pub use wire::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, error_codes};
