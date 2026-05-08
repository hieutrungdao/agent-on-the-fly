# Changelog

All notable changes to AOTF are documented in this file. Format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); pre-alpha versions
are allowed to break wire and storage formats freely (per architecture line
545).

## v0.0.2-alpha.1 — Walking Skeleton (2026-05-08)

First runnable release. Proves the full Rust workspace + Bun agent compiles,
the 5-tier `ActionTier` enum is a real type with mutation-targetable per-tier
gate functions, and the file-watch → finding → gate → audit-log demo path
works end-to-end.

### Added

- **8-crate Cargo workspace** (`aotf-core-types`, `aotf-ipc-protocol`,
  `aotf-storage`, `aotf-gatekeeper`, `aotfd`, `aotf-cli`, `aotf-mcp-server`,
  `aotf-plugin-api`) + **Bun TypeScript agent** package (`@aotf/agent`).
- **`ActionTier`** enum (FR-105): `READ` / `DRAFT` / `PROPOSE` /
  `EXECUTE_REVERSIBLE` / `EXECUTE_IRREVERSIBLE`. Each tier has its own named
  `gate_*` function so future `cargo-mutants` runs can target each
  allow/deny branch.
- **Wire-frozen `Finding` v1 struct** with golden round-trip fixtures.
- **JSON-RPC 2.0 IPC** (`aotf-ipc-protocol`) with stable method namespace
  (`daemon.ping`, `finding.emit`, `audit.append`, `audit.list`,
  `gate.evaluate`) and AOTF-specific error codes (`GATE_DENY`,
  `AUTH_TYPE_MISMATCH`, `INCOMPLETE_IRREVERSIBLE_RECORD`).
- **TypeScript bindings auto-generated** from Rust via `ts-rs` +
  `gen_ts_types` binary; CI gates on `git diff --exit-code` to prevent
  drift.
- **SQLite WAL audit log** (`aotf-storage`) with append-only Rust API
  (`Storage::append` / `Storage::list` only — no `delete` on the public
  surface) and a read-only `Storage::open_read_only` for the CLI.
- **Gatekeeper subprocess** (`aotf-gatekeeper`) speaks JSON-RPC over a Unix
  domain socket, classifies every method via the FR-105 tier-map, and
  audits every gate evaluation.
- **Daemon** (`aotfd`) spawns the gatekeeper, watches a directory via
  `notify`, drains file events into `gate.evaluate` calls, and serves
  `daemon.ping` + `audit.list` to the CLI.
- **CLI** (`aotf`) with `init`, `watch`, `audit ls`, and `doctor`.
- **Bun agent** (`@aotf/agent`) one-shot JSON-RPC client + `daemon.ping`
  shorthand, using the auto-generated TS wire types.
- **End-to-end demo** (`scripts/e2e_demo.sh`) — runs `aotf init`, spawns the
  daemon, touches a file, asserts a `DRAFT` + `ALLOW` audit row appears
  within 5s, then SIGTERMs cleanly.
- **CI** (Linux + macOS, Rust stable + 1.85, Bun latest) plus a Linux-only
  e2e job that runs `scripts/e2e_demo.sh`.
- **Release pipeline** (`.github/workflows/release.yml`) — tag-triggered
  Linux x86_64 build that publishes the binaries and `install.sh` to the
  GitHub Release.
- **Install script** (`scripts/install.sh`) — `curl | sh`-friendly
  Linux x86_64 installer that drops `aotf` into `~/.local/bin`. macOS and
  aarch64 are not yet supported; the script exits with an error on those
  platforms.

### Walking-skeleton policy

The walking skeleton ships these *deliberate* gaps. Each is logged in the
plan's Decision Log with a v0.0.3+ remediation note.

- **`EXECUTE_IRREVERSIBLE` always denies** — the FR-106 consent-token /
  WebAuthn system is not implemented; irreversible actions wait until
  v0.0.3.
- **`EXECUTE_REVERSIBLE` always allows** — caller is responsible for
  rollback; FR-29 programmatic rollback is deferred.
- **Append-only enforced at Rust API only**, not via SQLite triggers /
  views. The `read_only_handle_can_list_but_not_append` integration test
  confirms read-only handles can't write, but a `BEFORE DELETE` trigger
  hardening pass is on the v0.0.3 list.
- **No notification adapters** — Telegram, GitHub Actions, claude-bridge
  integrations are post-walking-skeleton.
- **No plugin loading** — `aotf-plugin-api` is a library stub; `seccomp-bpf`
  isolation lands when plugins do.
- **No MCP server** — `aotf-mcp-server` exits 0; the v1.0 4-tool read-only
  surface lands in v0.0.3.
- **Goal Loop, AI QA Agent, ML/LLM Lifecycle Manager, AI Agent Operations**
  — none implemented. Walking skeleton scopes to the Pillar-1 (Proactive
  DevOps Loop) trust surface only, and within that, to the watch → gate →
  audit core.
- **Cross-crate `cargo-mutants`** — `mutants.toml` ships as a marker; the
  weekly run is wired in v0.0.3 along with cargo-deny dependency-direction
  gates.

### Architecture

- **MSRV bumped 1.80 → 1.85** because edition 2024 (architecture-mandated)
  requires it. CI matrix updated accordingly.
- **`TS_RS_EXPORT_DIR`** is set workspace-wide via `.cargo/config.toml` so
  generated TS bindings land in `packages/aotf-agent/src/generated/` from
  every crate's perspective.

### Verified

- 58 Rust tests pass across the workspace (`cargo test --workspace`).
- 3 Bun unit tests + 1 skipped live-daemon test (`bun test --cwd
  packages/aotf-agent`).
- `cargo install --path crates/aotf-cli --locked --force` produces a
  working `aotf --version` / `aotf doctor`.
- `bash scripts/e2e_demo.sh` passes 3/3 deterministic local runs.

### Known issues

- Live integration tests in Bun (`test/ping.test.ts`) are gated on
  `AOTF_TEST_SOCKET`; no CI runner sets this yet.
- `aotf watch` is foreground-blocking; detached mode (with pidfile + `aotf
  stop`) is on the v0.0.3 list.
- Linux x86_64 release artifact only. macOS / Linux aarch64 binaries land
  when CI minutes / SLSA signing are set up.
