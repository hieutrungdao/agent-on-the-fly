// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! JSON-RPC 2.0 wire types.
//!
//! Per the JSON-RPC 2.0 spec the envelope is fixed: `jsonrpc: "2.0"`,
//! `method`, `params`, and `id` for requests; `jsonrpc: "2.0"`, exactly one of
//! `result` / `error`, and `id` for responses.
//!
//! `id` is intentionally `serde_json::Value` because the spec allows
//! number, string, or null — no type discriminant needed for an opaque
//! correlation token.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// JSON-RPC 2.0 protocol version constant.
pub const JSONRPC_VERSION: &str = "2.0";

/// Stable JSON-RPC error codes. The first block is the spec-mandated set;
/// `-32000` and below are AOTF application-specific.
pub mod error_codes {
    /// Spec: invalid JSON received.
    pub const PARSE_ERROR: i32 = -32700;
    /// Spec: JSON sent is not a valid request.
    pub const INVALID_REQUEST: i32 = -32600;
    /// Spec: method does not exist.
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Spec: invalid method parameters.
    pub const INVALID_PARAMS: i32 = -32602;
    /// Spec: internal JSON-RPC error.
    pub const INTERNAL_ERROR: i32 = -32603;

    /// AOTF: gatekeeper denied the action.
    pub const GATE_DENY: i32 = -32001;
    /// AOTF: a token was submitted of the wrong authorization type
    /// (`AuthTypeMismatch` per FR-107).
    pub const AUTH_TYPE_MISMATCH: i32 = -32002;
    /// AOTF: an `EXECUTE_IRREVERSIBLE` audit entry is missing one of the six
    /// mandatory fields (D-ENF-2).
    pub const INCOMPLETE_IRREVERSIBLE_RECORD: i32 = -32003;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct JsonRpcRequest {
    /// Always `"2.0"`.
    pub jsonrpc: String,
    /// Method name from [`crate::methods`].
    pub method: String,
    /// Method parameters; shape is method-specific.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub params: serde_json::Value,
    /// Correlation token — number, string, or null. Echoed in the response.
    pub id: serde_json::Value,
}

impl JsonRpcRequest {
    /// Build a request with the protocol version pre-set.
    pub fn new(
        method: impl Into<String>,
        params: serde_json::Value,
        id: serde_json::Value,
    ) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params,
            id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct JsonRpcResponse {
    /// Always `"2.0"`.
    pub jsonrpc: String,
    /// Present iff the call succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Present iff the call failed. Mutually exclusive with `result`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Correlation token from the originating request.
    pub id: serde_json::Value,
}

impl JsonRpcResponse {
    pub fn success(result: serde_json::Value, id: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub fn failure(error: JsonRpcError, id: serde_json::Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            result: None,
            error: Some(error),
            id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct JsonRpcError {
    /// Error code; see [`error_codes`].
    pub code: i32,
    /// Short human-readable summary.
    pub message: String,
    /// Optional structured payload (e.g. validation failures).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trip() {
        let req = JsonRpcRequest::new(
            "daemon.ping",
            serde_json::json!({"echo": "hi"}),
            serde_json::json!(1),
        );
        let s = serde_json::to_string(&req).unwrap();
        let back: JsonRpcRequest = serde_json::from_str(&s).unwrap();
        assert_eq!(req, back);
    }

    #[test]
    fn response_success_round_trip() {
        let resp =
            JsonRpcResponse::success(serde_json::json!({"pong": true}), serde_json::json!("a"));
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
        let s = serde_json::to_string(&resp).unwrap();
        let back: JsonRpcResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(resp, back);
    }

    #[test]
    fn response_failure_round_trip() {
        let err = JsonRpcError::new(error_codes::METHOD_NOT_FOUND, "no such method");
        let resp = JsonRpcResponse::failure(err, serde_json::json!(42));
        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        let s = serde_json::to_string(&resp).unwrap();
        let back: JsonRpcResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(resp, back);
    }

    #[test]
    fn null_params_omitted_on_wire() {
        let req = JsonRpcRequest::new("daemon.ping", serde_json::Value::Null, serde_json::json!(1));
        let s = serde_json::to_string(&req).unwrap();
        // Spec allows omission of params; we omit when null to keep wire compact.
        assert!(
            !s.contains("\"params\""),
            "expected no params key, got: {s}"
        );
    }

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(error_codes::PARSE_ERROR, -32700);
        assert_eq!(error_codes::INVALID_REQUEST, -32600);
        assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
        assert_eq!(error_codes::INVALID_PARAMS, -32602);
        assert_eq!(error_codes::INTERNAL_ERROR, -32603);
        assert_eq!(error_codes::GATE_DENY, -32001);
        assert_eq!(error_codes::AUTH_TYPE_MISMATCH, -32002);
        assert_eq!(error_codes::INCOMPLETE_IRREVERSIBLE_RECORD, -32003);
    }
}
