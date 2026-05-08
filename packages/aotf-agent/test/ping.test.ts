// SPDX-License-Identifier: Apache-2.0

// Live-daemon integration test. Skipped unless `AOTF_TEST_SOCKET` is set,
// which Story 9's `scripts/e2e_demo.sh` (or a manual run) provides.

import { describe, expect, it } from "bun:test";
import { ping } from "../src/ipcClient";

const sock = process.env.AOTF_TEST_SOCKET;

describe("daemon.ping (live)", () => {
  it.skipIf(!sock)("responds with pong when daemon is up", async () => {
    const ok = await ping(sock as string);
    expect(ok).toBe(true);
  });
});
