// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Regenerate the TypeScript bindings under
//! `packages/aotf-agent/src/generated/`.
//!
//! Each `#[derive(TS)]` type with `#[ts(export)]` writes a `.ts` file at the
//! path declared in its `export_to` attribute. This binary calls
//! [`TS::export`] on every cross-IPC type so the bindings can be regenerated
//! deterministically (CI gate: `git diff --exit-code` after running).

use aotf_core_types::{ActionTier, Decision, Finding, FindingId, FindingSource};
use aotf_ipc_protocol::wire::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use ts_rs::TS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // aotf-core-types
    ActionTier::export_all()?;
    FindingSource::export_all()?;
    FindingId::export_all()?;
    Finding::export_all()?;
    Decision::export_all()?;
    // aotf-ipc-protocol
    JsonRpcRequest::export_all()?;
    JsonRpcResponse::export_all()?;
    JsonRpcError::export_all()?;

    eprintln!("ts bindings regenerated under packages/aotf-agent/src/generated/");
    Ok(())
}
