// SPDX-License-Identifier: Apache-2.0

// `@aotf/agent` walking-skeleton entry point. Opens the daemon's CLI socket,
// sends `daemon.ping`, prints the result, exits.
//
// The Claude Agent SDK is imported below as a type-only reference so v0.0.3
// can pick up where this leaves off. No live LLM call yet — see plan Decision
// Log; the agent is intentionally a no-op in v0.0.2-alpha beyond this ping.
//
// import type { /* TODO: v0.0.3 */ } from "@anthropic-ai/claude-agent-sdk";

import { ping } from "./ipcClient";

const DEFAULT_SOCKET = `${process.env.HOME ?? "."}/.aotf/run/cli.sock`;

function readSocketArg(argv: readonly string[]): string {
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--socket" && i + 1 < argv.length) return argv[i + 1] as string;
    if (a !== undefined && a.startsWith("--socket=")) {
      return a.slice("--socket=".length);
    }
  }
  return DEFAULT_SOCKET;
}

async function main(): Promise<number> {
  const socket = readSocketArg(Bun.argv.slice(2));
  console.error(`@aotf/agent: pinging ${socket}`);
  try {
    const ok = await ping(socket);
    console.log(ok ? "pong" : "no-response");
    return ok ? 0 : 1;
  } catch (err) {
    console.error(`ping failed: ${(err as Error).message ?? err}`);
    return 2;
  }
}

const code = await main();
process.exit(code);
