// SPDX-License-Identifier: Apache-2.0

// Unit tests for the IPC client envelope construction. Live-daemon
// integration is in ./ping.test.ts and gated on `AOTF_TEST_SOCKET`.

import { describe, expect, it } from "bun:test";
import { JSONRPC_VERSION, buildRequest } from "../src/ipcClient";

describe("buildRequest", () => {
  it("emits a valid JSON-RPC 2.0 envelope", () => {
    const r = buildRequest("daemon.ping", { hello: "world" }, 7);
    expect(r.jsonrpc).toBe(JSONRPC_VERSION);
    expect(r.jsonrpc).toBe("2.0");
    expect(r.method).toBe("daemon.ping");
    expect(r.id).toBe(7);
    expect(r.params).toEqual({ hello: "world" });
  });

  it("serializes to a single line with trailing structure intact", () => {
    const r = buildRequest("audit.list", {}, "req-42");
    const json = JSON.stringify(r);
    const parsed = JSON.parse(json);
    expect(parsed.jsonrpc).toBe("2.0");
    expect(parsed.id).toBe("req-42");
    expect(parsed.method).toBe("audit.list");
    expect(json).not.toContain("\n");
  });

  it("accepts string ids per JSON-RPC 2.0 spec", () => {
    const r = buildRequest("daemon.ping", null, "abc");
    expect(r.id).toBe("abc");
  });
});
