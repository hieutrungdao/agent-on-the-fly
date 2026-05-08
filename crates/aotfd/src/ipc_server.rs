// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! CLI-facing JSON-RPC server. Handles read-only methods served by `aotfd`
//! itself: `daemon.ping` and `audit.list`. Write methods (e.g. `finding.emit`)
//! are routed through the gatekeeper, not served here.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use aotf_ipc_protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, error_codes, methods};
use aotf_storage::Storage;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuditListParams {
    #[serde(default)]
    since_id: Option<u64>,
}

/// Listen on `socket_path`; accept JSON-RPC requests; serve from
/// `audit_db_path` in read-only mode. Loops forever.
pub async fn serve(socket_path: PathBuf, audit_db_path: PathBuf) -> Result<()> {
    if let Some(parent) = socket_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create cli socket parent {parent:?}"))?;
        }
    }
    let _ = std::fs::remove_file(&socket_path);
    let listener = UnixListener::bind(&socket_path)
        .with_context(|| format!("bind cli socket {socket_path:?}"))?;
    tracing::info!(socket = %socket_path.display(), "cli ipc listening");

    loop {
        let (stream, _addr) = listener.accept().await?;
        let db = audit_db_path.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, &db).await {
                tracing::warn!(error = %e, "cli ipc connection error");
            }
        });
    }
}

async fn handle_connection(stream: UnixStream, audit_db: &Path) -> Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut lines = BufReader::new(read_half).lines();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let resp = handle_request(&line, audit_db);
        let mut bytes = serde_json::to_vec(&resp)?;
        bytes.push(b'\n');
        write_half.write_all(&bytes).await?;
    }
    Ok(())
}

/// Process a single JSON-RPC line. Pure for testability.
pub fn handle_request(line: &str, audit_db: &Path) -> JsonRpcResponse {
    let req: JsonRpcRequest = match serde_json::from_str(line) {
        Ok(r) => r,
        Err(e) => {
            return JsonRpcResponse::failure(
                JsonRpcError::new(error_codes::PARSE_ERROR, format!("parse: {e}")),
                Value::Null,
            );
        }
    };

    if req.method == methods::DAEMON_PING {
        return JsonRpcResponse::success(json!({"pong": true}), req.id);
    }

    if req.method == methods::AUDIT_LIST {
        let params: AuditListParams = serde_json::from_value(req.params).unwrap_or_default();
        let storage = match Storage::open_read_only(audit_db) {
            Ok(s) => s,
            Err(e) => {
                return JsonRpcResponse::failure(
                    JsonRpcError::new(error_codes::INTERNAL_ERROR, format!("storage: {e}")),
                    req.id,
                );
            }
        };
        let entries = match storage.list(params.since_id) {
            Ok(e) => e,
            Err(e) => {
                return JsonRpcResponse::failure(
                    JsonRpcError::new(error_codes::INTERNAL_ERROR, format!("list: {e}")),
                    req.id,
                );
            }
        };
        let value = serde_json::to_value(&entries).unwrap_or(Value::Null);
        return JsonRpcResponse::success(value, req.id);
    }

    JsonRpcResponse::failure(
        JsonRpcError::new(
            error_codes::METHOD_NOT_FOUND,
            format!("not served by aotfd: {}", req.method),
        ),
        req.id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aotf_core_types::ActionTier;
    use aotf_storage::{AuditDecision, NewAuditEntry};
    use tempfile::tempdir;

    fn fresh_db() -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("audit.db");
        let s = Storage::open(&path).unwrap();
        s.append(NewAuditEntry {
            finding_id: "seed".into(),
            tier: ActionTier::Draft,
            decision: AuditDecision::Allow,
            reason: "test seed".into(),
            created_micros: 1,
            payload: json!({}),
        })
        .unwrap();
        drop(s);
        (dir, path)
    }

    #[test]
    fn ping_responds_pong() {
        let (_dir, path) = fresh_db();
        let req = JsonRpcRequest::new(methods::DAEMON_PING, json!({}), json!(1));
        let line = serde_json::to_string(&req).unwrap();
        let resp = handle_request(&line, &path);
        assert!(resp.error.is_none());
        assert_eq!(resp.result.unwrap(), json!({"pong": true}));
    }

    #[test]
    fn audit_list_returns_seeded_entry() {
        let (_dir, path) = fresh_db();
        let req = JsonRpcRequest::new(methods::AUDIT_LIST, json!({}), json!(1));
        let line = serde_json::to_string(&req).unwrap();
        let resp = handle_request(&line, &path);
        assert!(resp.error.is_none(), "got error: {:?}", resp.error);
        let arr = resp.result.unwrap();
        assert_eq!(arr.as_array().map(|a| a.len()), Some(1));
    }

    #[test]
    fn unknown_method_returns_not_found() {
        let (_dir, path) = fresh_db();
        let req = JsonRpcRequest::new("nope.bogus", json!({}), json!(1));
        let line = serde_json::to_string(&req).unwrap();
        let resp = handle_request(&line, &path);
        assert_eq!(resp.error.unwrap().code, error_codes::METHOD_NOT_FOUND);
    }

    #[test]
    fn parse_error_returns_null_id() {
        let (_dir, path) = fresh_db();
        let resp = handle_request("garbage}{", &path);
        assert_eq!(resp.error.unwrap().code, error_codes::PARSE_ERROR);
        assert_eq!(resp.id, Value::Null);
    }
}
