// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! Snapshot tests for the v1 wire shape: method namespace and JSON-RPC
//! envelope shapes. Any unintentional change to these snapshots is a
//! wire-breaking change — review the snapshot diff carefully before
//! `cargo insta accept`.

use aotf_ipc_protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, error_codes, methods};
use serde_json::json;

#[test]
fn method_namespace_snapshot() {
    insta::assert_json_snapshot!("method_namespace", methods::ALL);
}

#[test]
fn error_codes_snapshot() {
    let codes = serde_json::json!({
        "PARSE_ERROR": error_codes::PARSE_ERROR,
        "INVALID_REQUEST": error_codes::INVALID_REQUEST,
        "METHOD_NOT_FOUND": error_codes::METHOD_NOT_FOUND,
        "INVALID_PARAMS": error_codes::INVALID_PARAMS,
        "INTERNAL_ERROR": error_codes::INTERNAL_ERROR,
        "GATE_DENY": error_codes::GATE_DENY,
        "AUTH_TYPE_MISMATCH": error_codes::AUTH_TYPE_MISMATCH,
        "INCOMPLETE_IRREVERSIBLE_RECORD": error_codes::INCOMPLETE_IRREVERSIBLE_RECORD,
    });
    insta::assert_json_snapshot!("error_codes", codes);
}

#[test]
fn request_shape_snapshot() {
    let req = JsonRpcRequest::new("daemon.ping", json!({"echo": "snap"}), json!(7));
    insta::assert_json_snapshot!("request_shape", req);
}

#[test]
fn response_success_shape_snapshot() {
    let resp = JsonRpcResponse::success(json!({"pong": true}), json!(7));
    insta::assert_json_snapshot!("response_success_shape", resp);
}

#[test]
fn response_failure_shape_snapshot() {
    let err = JsonRpcError::new(error_codes::METHOD_NOT_FOUND, "no such method")
        .with_data(json!({"hint": "see methods::ALL"}));
    let resp = JsonRpcResponse::failure(err, json!("req-1"));
    insta::assert_json_snapshot!("response_failure_shape", resp);
}
