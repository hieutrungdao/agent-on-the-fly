// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotf-gatekeeper` — the safety enforcement subprocess.
//!
//! Listens on a Unix domain socket; speaks line-delimited JSON-RPC 2.0; on
//! every gate decision, writes one append-only audit entry. The walking
//! skeleton serves a single method, [`methods::GATE_EVALUATE`].

mod gate;
mod tier_map;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use aotf_core_types::ActionTier;
use aotf_ipc_protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, error_codes, methods};
use aotf_storage::{AuditDecision, NewAuditEntry, Storage};
use clap::Parser;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

#[derive(Parser, Debug)]
#[command(version, about = "AOTF safety gatekeeper")]
struct Cli {
    /// Unix domain socket path to listen on.
    #[arg(long)]
    socket: PathBuf,
    /// SQLite audit-log database path.
    #[arg(long)]
    audit_db: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvaluateParams {
    tier: ActionTier,
    #[serde(default)]
    finding_id: Option<String>,
    #[serde(default)]
    reason_hint: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,aotf_gatekeeper=debug".into()),
        )
        .json()
        .init();

    let cli = Cli::parse();

    let storage = Arc::new(
        Storage::open(&cli.audit_db)
            .with_context(|| format!("open audit db at {:?}", cli.audit_db))?,
    );

    if let Some(parent) = cli.socket.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create socket parent {parent:?}"))?;
        }
    }
    let _ = std::fs::remove_file(&cli.socket);
    let listener =
        UnixListener::bind(&cli.socket).with_context(|| format!("bind socket {:?}", cli.socket))?;
    tracing::info!(socket = %cli.socket.display(), "gatekeeper listening");

    loop {
        let (stream, _addr) = listener.accept().await?;
        let storage = Arc::clone(&storage);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, storage).await {
                tracing::warn!(error = %e, "gatekeeper connection error");
            }
        });
    }
}

async fn handle_connection(stream: UnixStream, storage: Arc<Storage>) -> Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut lines = BufReader::new(read_half).lines();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let resp = process_line(&line, &storage);
        let mut bytes = serde_json::to_vec(&resp)?;
        bytes.push(b'\n');
        write_half.write_all(&bytes).await?;
    }
    Ok(())
}

fn process_line(line: &str, storage: &Storage) -> JsonRpcResponse {
    let req: JsonRpcRequest = match serde_json::from_str(line) {
        Ok(r) => r,
        Err(e) => {
            return JsonRpcResponse::failure(
                JsonRpcError::new(error_codes::PARSE_ERROR, format!("parse: {e}")),
                Value::Null,
            );
        }
    };

    // FR-105 invariant: unknown methods never reach a tier branch. The
    // tier-map is the authoritative method-namespace whitelist.
    if tier_map::classify(&req.method).is_none() {
        return JsonRpcResponse::failure(
            JsonRpcError::new(
                error_codes::METHOD_NOT_FOUND,
                format!("unknown method: {}", req.method),
            ),
            req.id,
        );
    }

    if req.method == methods::GATE_EVALUATE {
        let params: EvaluateParams = match serde_json::from_value(req.params) {
            Ok(p) => p,
            Err(e) => {
                return JsonRpcResponse::failure(
                    JsonRpcError::new(error_codes::INVALID_PARAMS, format!("params: {e}")),
                    req.id,
                );
            }
        };
        let decision = gate::evaluate(params.tier);
        let audit_decision = if decision.allow {
            AuditDecision::Allow
        } else {
            AuditDecision::Deny
        };
        let payload = json!({
            "method": methods::GATE_EVALUATE,
            "reasonHint": params.reason_hint,
        });
        let entry = NewAuditEntry {
            finding_id: params.finding_id.unwrap_or_else(|| "-".into()),
            tier: decision.tier,
            decision: audit_decision,
            reason: decision.reason.clone(),
            // Storage layer assigns wall-clock at append time via the caller; for
            // gatekeeper we use a fresh wall-clock — Lamport ordering work is
            // post-walking-skeleton (D-CLOCK-* in architecture).
            created_micros: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_micros() as i64)
                .unwrap_or(0),
            payload,
        };
        if let Err(e) = storage.append(entry) {
            return JsonRpcResponse::failure(
                JsonRpcError::new(error_codes::INTERNAL_ERROR, format!("audit append: {e}")),
                req.id,
            );
        }
        let result = serde_json::to_value(&decision).unwrap_or(Value::Null);
        return JsonRpcResponse::success(result, req.id);
    }

    if req.method == methods::DAEMON_PING {
        return JsonRpcResponse::success(json!({"pong": true}), req.id);
    }

    // Method is in the tier-map but the gatekeeper doesn't dispatch it here.
    // (E.g. `audit.append` / `audit.list` — those route through the daemon.)
    JsonRpcResponse::failure(
        JsonRpcError::new(
            error_codes::METHOD_NOT_FOUND,
            format!("not served by gatekeeper: {}", req.method),
        ),
        req.id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fresh_storage() -> (tempfile::TempDir, Storage) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("aotf.db");
        let s = Storage::open(&path).unwrap();
        (dir, s)
    }

    #[test]
    fn process_line_allows_draft_and_audits() {
        let (_dir, storage) = fresh_storage();
        let req = JsonRpcRequest::new(
            methods::GATE_EVALUATE,
            json!({"tier": "DRAFT", "findingId": "abc"}),
            json!(1),
        );
        let line = serde_json::to_string(&req).unwrap();
        let resp = process_line(&line, &storage);
        assert!(resp.error.is_none(), "unexpected error: {:?}", resp.error);
        let entries = storage.list(None).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].decision, AuditDecision::Allow);
        assert_eq!(entries[0].tier, ActionTier::Draft);
    }

    #[test]
    fn process_line_denies_irreversible_and_audits() {
        let (_dir, storage) = fresh_storage();
        let req = JsonRpcRequest::new(
            methods::GATE_EVALUATE,
            json!({"tier": "EXECUTE_IRREVERSIBLE", "findingId": "xyz"}),
            json!(2),
        );
        let line = serde_json::to_string(&req).unwrap();
        let resp = process_line(&line, &storage);
        assert!(resp.error.is_none());
        // Result should serialize a Decision with allow=false
        let result = resp.result.unwrap();
        assert_eq!(result.get("allow"), Some(&json!(false)));
        let entries = storage.list(None).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].decision, AuditDecision::Deny);
    }

    #[test]
    fn process_line_method_not_found() {
        let (_dir, storage) = fresh_storage();
        let req = JsonRpcRequest::new("nope", json!({}), json!(3));
        let line = serde_json::to_string(&req).unwrap();
        let resp = process_line(&line, &storage);
        assert_eq!(
            resp.error.unwrap().code,
            error_codes::METHOD_NOT_FOUND,
            "expected METHOD_NOT_FOUND"
        );
    }

    #[test]
    fn process_line_parse_error_returns_null_id() {
        let (_dir, storage) = fresh_storage();
        let resp = process_line("{not json", &storage);
        assert_eq!(resp.error.unwrap().code, error_codes::PARSE_ERROR);
        assert_eq!(resp.id, Value::Null);
    }
}
