// SPDX-License-Identifier: Apache-2.0

// Minimal JSON-RPC 2.0 client over a Unix domain socket. Uses Bun.connect
// directly; no streaming framework, no batching. Walking-skeleton scope: one
// request → one newline-terminated response → close.
//
// The wire types come from the Rust-generated bindings under ./generated/.
// Hand-written wire types are forbidden by the plan's anti-drift guardrails.

import type { JsonRpcRequest } from "./generated/JsonRpcRequest";
import type { JsonRpcResponse } from "./generated/JsonRpcResponse";

export const JSONRPC_VERSION = "2.0" as const;

/** Build a JSON-RPC 2.0 request envelope. Pure; no I/O. */
export function buildRequest(
  method: string,
  params: unknown,
  id: number | string,
): JsonRpcRequest {
  return {
    jsonrpc: JSONRPC_VERSION,
    method,
    // Cast through `unknown` to satisfy the generated `JsonValue` type.
    params: params as JsonRpcRequest["params"],
    id: id as JsonRpcRequest["id"],
  };
}

/**
 * Send a single JSON-RPC request to a Unix domain socket and resolve with the
 * parsed response. Closes the socket after the first newline-delimited line.
 */
export async function callOnce(
  socketPath: string,
  method: string,
  params: unknown,
  id: number | string = 1,
): Promise<JsonRpcResponse> {
  const req = buildRequest(method, params, id);
  const wire = JSON.stringify(req) + "\n";

  return new Promise<JsonRpcResponse>((resolve, reject) => {
    let buffer = "";
    let resolved = false;

    Bun.connect({
      unix: socketPath,
      socket: {
        open(socket) {
          socket.write(wire);
        },
        data(socket, data) {
          buffer += new TextDecoder().decode(data);
          const nl = buffer.indexOf("\n");
          if (nl < 0) return;
          const line = buffer.slice(0, nl);
          try {
            const resp = JSON.parse(line) as JsonRpcResponse;
            resolved = true;
            socket.end();
            resolve(resp);
          } catch (e) {
            reject(e instanceof Error ? e : new Error(String(e)));
          }
        },
        close() {
          if (!resolved) {
            reject(new Error("connection closed without response"));
          }
        },
        error(_socket, error) {
          reject(error);
        },
      },
    }).catch(reject);
  });
}

/**
 * `daemon.ping` shorthand. Resolves to `true` iff the response carries
 * `result.pong === true` and no `error`.
 */
export async function ping(socketPath: string): Promise<boolean> {
  const resp = await callOnce(socketPath, "daemon.ping", {}, 1);
  if (resp.error) return false;
  const result = resp.result as { pong?: boolean } | null | undefined;
  return result?.pong === true;
}
