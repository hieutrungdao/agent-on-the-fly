---
stepsCompleted: ["step-01-init", "step-02-context", "step-03-starter", "step-04-decisions", "step-05-patterns", "step-06-structure", "step-07-validation", "step-08-complete"]
status: 'complete'
lastStep: 8
completedAt: '2026-04-26'
inputDocuments:
  - "docs/PRD.md"                                                                # canonical PRD v2.0.1 (1519 lines, 107 FRs, 41 NFRs)
  - "docs/ux-research-inputs.md"                                                 # §3 architectural constraints — now redundant with PRD §8 but preserved
  - "_bmad-output/planning-artifacts/ux-design-specification.md"                 # Finding primitive, design tokens, Ratatui widgets, component strategy
  - "_bmad-output/planning-artifacts/research/market-sre-agent-competitive-research-2026-04-17.md"  # W1-W5 wedge scoring, competitive landscape
  - "_bmad-output/research/claude-bridge-monitor-tool-research.md"               # Goal Loop + Rust↔Bun split + MCP positioning
  - "docs/product-brief.md"                                                      # historical concept brief (2026-04-04) — vision/problem/market intact
  - "_bmad-output/planning-artifacts/implementation-readiness-report-2026-04-19.md"  # three-way stack+scope+numbering conflict analysis; C1-C4 findings
  - "_bmad-output/planning-artifacts/prd-validation-report-2026-04-19.md"        # PRD validation findings
  - "(git 985bafc) docs/architecture.md"                                         # v1.1 Python subset — reference shape only; do not carry forward Python assumptions
  - "docs/epics.md"                                                              # DEPRECATED v1.1 — documents coverage gaps for FR-14/71-75/105/106 etc.
  - "CLAUDE.md"                                                                  # 4 pillars, reversibility discipline, surgical-change rule
workflowType: 'architecture'
project_name: 'aotf'
user_name: 'hieutrungdao'
date: '2026-04-26'
prd_version: 'v2.0.1 (2026-04-19 Party-Mode resolution pass)'
supersedes: 'pre-2026-04-26 v1.1 Python subset (git ref 985bafc, scope-shifted 2026-04-19)'
---

# Agent on the Fly — Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

_Supersedes the pre-2026-04-26 v1.1 Python subset (git commit `985bafc`, scope-shifted 2026-04-19)._

_Target scope: canonical [PRD v2.0.1](../../docs/PRD.md) — 107 FRs (81 MVP / 20 Growth / 6 Vision), 41 NFRs, Rust + TypeScript/Bun stack, four-pillar vision (Proactive DevOps Loop → AI QA Agent → ML/LLM Lifecycle Manager → AI Agent Operations), 14-week MVP targeting Pillar 1._

## Project Context Analysis

### Requirements Overview

**AOTF's differentiating wedge is pre-production autonomous action gated by reversibility-typed authorization with WebAuthn production consent (FR-105 + FR-106 + FR-107).** This is Claim #5 in the PRD and the only axis that separates AOTF from the post-incident commercial cohort (Resolve.ai, Datadog Bits AI SRE, PagerDuty, Azure SRE Agent) and the investigative-but-not-autonomous OSS cohort (HolmesGPT, k8sgpt, Keep, Tracer opensre). The architecture exists to protect this wedge. Every structural decision downstream should trace back to either enabling autonomous action, enforcing the reversibility boundary, or reducing the friction on the two user flows that validate the wedge (the autonomous fix path and the pre-push consent path).

**Functional Requirements** — 107 total (81 MVP / 20 Growth / 6 Vision) across 15 categories. MVP delivers Pillar 1 of a four-pillar vision (Proactive DevOps Loop → AI QA Agent → ML/LLM Lifecycle Manager → AI Agent Operations). Category weight as MVP architectural load:
- Autonomous Fix & Safety — **19 FRs** (the wedge surface itself; highest rigor)
- Notifications & Team Coordination — 11 FRs (dual Telegram transports with bidirectional semantics)
- Project Init & Config — 10 FRs
- Plugin Ecosystem — 9 FRs (SHA-256, capability scoping, `actionTier` declaration)
- Log Monitoring & Detection — 7 FRs
- Audit & Observability — 7 FRs (irreversible-action provenance)
- AI Diagnosis — 6 FRs; Goal Loop — 6 FRs; AI Backend Management — 6 FRs

**Non-Functional Requirements** — 41 NFRs. Security is the defining pressure (11 NFRs); Performance (7) forces the compiled-binary stack choice; Reliability (6) forces restart-safety everywhere. Hard constraints that force architectural choices:

- **NFR-P01** <100MB RSS at 10 log sources over 24h and **NFR-P04** <50ms cold start together mandate a compiled Rust daemon; a Bun-only or Python host cannot meet both.
- **NFR-S10** requires a "memory-safe, capability-bounded core whose failure domain is isolated from the orchestrator" — *invariant, not mechanism*. This is a **constrained choice** within three explicit rails: memory-safe language (Rust presupposed), capability model not ambient authority, process/fault isolation from orchestrator. Mechanism candidates: library-in-binary (fails "isolated failure domain" on honest reading), separate subprocess (clean, costs an IPC hop), WASM-with-capability-imports (middle ground, adds ~8MB wasmtime runtime).
- **NFR-S04** enforces the seven-control trust surface: five pipeline gates (FR-23 Dry-Run, FR-24 Confidence Floor, FR-25 Scope Limit, FR-26 Rollback Ref, FR-27 Mutex) non-bypassable by `--force-local-auto-approve`, plus two cross-cutting controls (FR-66 Audit — a tap, not a gate; FR-106 Consent Token — always required for `EXECUTE_IRREVERSIBLE` against production-designated environments).
- **NFR-S11** makes "production-designated" a strictly binary per-environment flag — staging, canary, shadow, production-like, and ephemeral preview environments are NOT production for consent-token purposes.
- **NFR-D03** mandates zero data egress without explicit opt-in — a hard architectural property that excludes phone-home telemetry from the default path.
- **NFR-R02** + **NFR-R04** together require transactional boundary design that survives process termination at any point (SQLite WAL + in-flight state detection on unclean restart).

### Scale & Complexity

**Framing: security-dominant single-node platform with two high-rigor subsystems** — the seven-control trust surface (FR-23–27 + FR-66 + FR-106) and the Rust↔Bun boundary (orchestrator ↔ agent, shaped by NFR-S10 enforcement-core placement). Overall system complexity is **moderate** (3 runtime processes plus subprocess agents; SQLite + Unix socket; one-tenant-per-install). The rigor-density in those two subsystems is what distinguishes AOTF architecturally, not the surface area of the system.

Explicitly not: HA, multi-tenant, horizontally scaled, or enterprise-operated. The >20% individual→team conversion validation target is bottom-up OSS adoption, not top-down enterprise sales.

- **Primary domain:** cross-process CLI + daemon + agent platform with autonomous code-modification capability.
- **Components (approximate, to be refined in step-06):** Rust side — CLI, config resolver, `notify`-based file watcher, Docker-SDK log watcher, pre-push git hook integration, IPC server, orchestrator enforcement core (placement TBD per step-03), plugin loader + SHA-256 verifier, SQLite+WAL persistence layer, audit log (append-only), rollback ref manager, fix mutex, git2 worktree manager. Bun side — agent orchestrator, AI CLI subprocess manager, confidence computer, MCP server (read-only v1.0), Goal Loop runtime, Telegram adapters (claude-plugins-official + claude-bridge), GitHub Actions CI provider plugin, notification event bus, decision-log writer. Cross-cutting — `AotfChannelEvent` schema, Finding v1 schema, structured JSON daemon-log schema, plugin manifest schema with `actionTier` declaration, IPC JSON-RPC protocol schema.

### Technical Constraints & Dependencies

**Stack fixed by PRD + claude-bridge research:**
- Rust + TypeScript/Bun. Unix domain socket with versioned JSON-RPC for IPC; messages for `EXECUTE_IRREVERSIBLE` carry `consentTokenId`.
- SQLite with WAL mode for findings, decision log, audit log, rollback refs.
- Single static binary distribution (`brew install`, `curl | sh`, GH Releases prebuilt). TS agent bundled as compressed asset, extracted to `~/.aotf/agent/` on first run. `aotf update` pulls new TS bundle without touching Rust binary.
- SLSA provenance on releases; `cargo audit` + `bun audit` blocking in CI on critical/high severity.

**External dependencies fixed:**
- Claude Code CLI (v1.0 default backend); Codex + Gemini CLI (Growth).
- Claude Code Monitor tool (v2.1.98+) for event-driven wake-ups.
- Both `telegram@claude-plugins-official` (zero-infra Channels adapter) AND `claude-bridge` (self-hosted Goal Loop + worktree + cost tracking) — normalized behind `AotfChannelEvent`.
- GitHub Actions as v1.0 CI provider; GitHub API (5000 req/hour, HMAC-SHA256 webhook validation).
- WebAuthn for FR-106 consent tokens (pre-registered operator keys).
- OS credential stores (macOS Keychain, Linux libsecret) with SOPS fallback for headless/CI.
- seccomp-bpf (Linux) / import-hook + entitlement-gated OS sandbox (macOS, weaker, published).

**Decisions the PRD already settled (architecture documents, does not choose):**
- Three distinct non-interchangeable authorization primitives (FR-107) with cross-type = `AuthTypeMismatch`.
- Five action tiers (FR-105): `READ` / `DRAFT` / `PROPOSE` / `EXECUTE_REVERSIBLE` / `EXECUTE_IRREVERSIBLE`. Plugin manifest declares ceiling; runtime classifies; violation = audit entry + plugin termination.
- Finding schema v1 is frozen (eight fields, stable across minors).
- Confidence formula (FR-18): `0.40 × static_analysis_agreement + 0.40 × test_signal + 0.20 × (1 − normalized_diff_complexity)`. AOTF-computed, never AI-self-reported.
- JSON-RPC 2.0 as the IPC wire protocol (semver on method namespace is the conventional answer; no architectural gate).
- SQLite append-only enforcement at the storage layer (write-only views + revoke DELETE; three-line design, not an architectural gate).
- FR-53 per-channel webhook surfaces (Slack Block Kit / PagerDuty ack / Teams Adaptive Card / Email output-only) — PRD prescribes per-channel interactivity model; architecture names the adapter interface.
- WebAuthn key lifecycle — PRD + WebAuthn Level 2 spec already prescribe registration, attestation verification, and revocation via the user token layer; architecture documents the flow, does not redesign cryptography.

**Genuine architectural gates handed to this workflow (4 decisions, not 7):**
1. **NFR-S10 enforcement-core placement** — library-in-binary vs. separate subprocess vs. WASM-with-capability-imports. This is the **first architectural decision** because it shapes the IPC boundary that follows; see the critical-path reframing below.
2. **IPC message-size and backpressure policy** — NFR-P01 (<100MB RSS at 10 sources) and NFR-P07 (1000 errors/min burst) require bounded channels and explicit drop/overflow behavior at every hop. The wire protocol is chosen (JSON-RPC 2.0); flow-control semantics are not.
3. **Concrete source locations for the tier-classification table** — the PRD removed `crates/orchestrator/src/tier_map.rs` from its text; architecture names the file and the review convention (standard code review, not config).
4. **MCP write-capable surface threat model for v1.1+** — v1.0 read-only is locked; the v1.1 design model is this workflow's to specify so the v1.0 code does not foreclose it. Tracked as a Growth decision but decided *now* to avoid Growth-era rework.

**Critical-path reframing** (departure from PRD §9 ordering, documented here for traceability):
The PRD §9 names *"Rust↔Bun IPC contract → before any agent work"* as critical path #1. The architecture roundtable (step-02 Party Mode) contested this ordering on the grounds that the IPC boundary shape is **downstream** of the NFR-S10 enforcement-core placement decision. Architecture ordering:
  - **step-03** picks the starter shape *and* the enforcement-core placement (they are coupled; library-in-binary vs. subprocess vs. WASM each produces a different starter skeleton).
  - **step-04** derives the IPC boundary shape from that placement.
  - The **first concrete deliverable that unblocks downstream stories** is neither the socket nor the schema — it is the `Finding v1` wire struct with a Rust↔Bun round-trip golden test (<200 LOC, byte-equal). Both sides can mock against the fixture while the boundary hardens.

### Cross-Cutting Concerns Identified

Concerns that touch ≥3 components and must appear in every component-level decision. Organized by category after the Party-Mode revision:

**A. Trust-boundary spine (the wedge — every decision touches these):**
1. **Action-tier reversibility taxonomy (FR-105)** — traverses plugin manifest, IPC contract, enforcement core, decision log, notification adapters, MCP tools, audit log.
2. **FR-105.2 mutation-target manifest** — the mutation test required by NFR-S10 is writable *only if* gate allow/deny branches are isolated into named functions with a manifest naming them. Testability is architectural, not a test-team concern.
3. **Three-token authorization model (FR-107)** — distinct lifecycles, storage, verification; cross-type submission produces `AuthTypeMismatch` with audit entry and rate limit.
4. **Audit log (FR-66, NFR-S06)** — tap on every action; append-only at storage layer; `EXECUTE_IRREVERSIBLE` entries carry six additional fields (approver identity, UTC timestamp, stated reason ≥20 chars verbatim, consent-token hash, WebAuthn assertion hash, classified tier). **PII filtering (FR-68, NFR-D01) is subsumed here** — it is an audit-log/daemon-log feature, not a cross-cutting concern in its own right.

**B. Cross-process substrate (the two high-rigor subsystems' supporting infrastructure):**
5. **Rust↔Bun IPC contract** — shape derived from enforcement-core placement (concern A1/handoff #1); message-size, protocol versioning, and `consentTokenId` propagation all ride this boundary.
6. **Clock & ordering semantics** — with 3+ processes writing to an append-only audit log and an IPC-ordered event bus, the choice between monotonic-per-process, Lamport timestamps, or wall-clock-with-skew-tolerance is a step-06 prerequisite. Without it, the audit log becomes unauditable under concurrent writes.
7. **Backpressure & queue-depth policy** — NFR-P01 (<100MB) implies bounded channels everywhere; NFR-P07 (1000 errors/min) implies explicit drop-oldest overflow. Flow control is not free at any hop (log watcher → dedup → diagnosis queue → fix mutex → notification bus).
8. **Transactional boundary design** — FR-34 restart-safe + NFR-R02 no-loss-on-SIGKILL + NFR-R04 in-flight state detection together require every autonomous action to be interrupt-safe at any point. Rollback ref written *before* commit; audit entry written *before* action.
9. **Degraded-mode signaling (FR-63, FR-93, NFR-R06)** — AI backend outage, plugin crash, integrity recovery — never silent. Each component declares its degraded states and paths.
10. **Confidence formula (FR-18)** — computed once in Bun intelligence core; propagated verbatim through diagnosis → gate decision → notification → audit log.

**C. External trust boundaries (distinct from internal trust-boundary spine):**
11. **External-surface trust boundary** — webhook signing (GitHub HMAC-SHA256 per NFR-S05), Telegram bot vs. user-session distinction, per-channel replay protection (FR-53 Slack/PagerDuty/Teams + the claude-plugins-official/claude-bridge dual transport). Distinct from 3-token auth (internal) and from IPC contract (inter-process).

**D. User-validation surfaces (the MVP assumption test):**
12. **Override, rejection & rollback UX surface** — the human-facing loop the MVP hypothesis depends on. Serves FR-29 (rollback single command), FR-30 (interactive approval), FR-91 (suppress/snooze), FR-94 (reject-with-feedback). If this surface is clunky, the validation metric fails regardless of infra quality. Touches CLI, Telegram, MCP, and audit-log UX simultaneously.

**Defer-and-flag** (tracked but not treated as foundational):
- **Multi-tenancy lite** — PRD FR-49 (per-user tokens) and FR-50 (per-service overrides) are MVP, but true multi-tenancy (isolated state, cross-tenant RBAC) is not. Treat as v0.2+ concern; the v1.0 architecture should avoid foreclosing it but should not build for it.
- **Formal schema-stability contract** — v1.0 is pre-alpha; breaking SQLite schema is acceptable with a migration note. The *wire* schemas (Finding v1, IPC JSON-RPC) are frozen; the *internal* schemas are not. This distinction is architectural discipline but not a day-one concern.

### Stakeholder Lens

Voices whose unstated requirements shape architectural choices even when they do not appear as FRs:

- **Plugin authors** (J3 Jordan) — capability-manifest stability is day-one concern because plugins are moat infrastructure. The plugin API surface (FR-54–60, FR-96, FR-104) must be treated as a public contract with semver and deprecation policy from v1.0, not a post-hoc stabilization.
- **Compliance-curious mid-market buyers** — not SOC 2 scope, but audit-log exportability (FR-67) and data residency (NFR-D04) must be answerable at install time. Affects the SQLite append-only schema decision and the telemetry default path (NFR-S09, NFR-D03).
- **CNCF Sandbox submission readiness** (post-MVP but influenced now) — license (Apache 2.0, confirmed), governance model, SBOM track (roadmap item per PRD Open-Source Trust Model), and reproducible builds (SLSA provenance). Do not trade architectural decisions for sandbox-submission-optimization, but do not block them either.

### Party-Mode Roundtable Record (step-02)

This analysis was stress-tested in a BMad Party-Mode roundtable with Winston 🏗️ (Architect), Amelia 💻 (Dev), Mary 📊 (Analyst), and John 📋 (PM). Unanimous findings and the accepted revisions are baked above. Key traceable decisions that emerged:

- **"Enterprise complexity" was unanimously rejected.** The four agents each offered a distinct replacement framing; the accepted blend — *"security-dominant single-node platform with two high-rigor subsystems"* — preserves the architectural signal without triggering SOC 2 / over-build reflexes.
- **The handoff-decision list was cut 7 → 4.** JSON-RPC versioning, SQLite append-only enforcement, FR-53 per-channel webhook surfaces, and WebAuthn key lifecycle were downgraded to "architecture documents, does not decide" because the PRD text (or the governing spec) already constrains them sufficiently.
- **The moat articulation was promoted.** Claim #5 (pre-production autonomous action with reversibility-typed gating) now leads the Requirements Overview rather than being diluted across concerns #1 and #7.
- **The PRD §9 critical-path ordering was contested and reframed.** All three technical agents (Winston, Amelia, John) disagreed with "Rust↔Bun IPC contract is critical path #1." The accepted reframing: enforcement-core placement (step-03) precedes IPC boundary shape (step-04); the first concrete unblocker is the Finding v1 wire-struct golden test.
- **Six new cross-cutting concerns were added** (mutation-target manifest, clock/ordering semantics, backpressure policy, external-surface trust boundary, override/rollback UX surface, plus PII collapsed into audit-log).
- **Two stakeholders were added** (plugin authors, compliance-curious buyers); CNCF Sandbox submission tracked as post-MVP but influencing-now.

## Starter Template Evaluation

### Primary Technology Domain

Cross-process developer platform — Rust CLI + Rust daemon + Rust enforcement subprocess + TypeScript/Bun agent + subprocess AI CLI. No conventional third-party starter template matches this shape; the "starter" is a small custom Cargo + Bun monorepo scaffold whose skeleton is coupled to the NFR-S10 enforcement-core placement decision (this coupling is resolved here — see Option B below).

### Starter Skeleton Options Considered

Three options evaluated, one per NFR-S10 enforcement-core placement candidate. All share a Cargo workspace + `packages/aotf-agent` Bun package + Turborepo outer shell; they differ only in where the enforcement core lives.

| Option | Shape | NFR-S10 satisfaction | NFR-P01/P04 cost | Mutation test FR-105.2 | Verdict |
|---|---|---|---|---|---|
| **A. Library-in-binary** | `aotf-core` crate linked into `aotfd`; failure-domain isolation argued from Rust type safety + `panic=abort` | 2 of 3 rails (memory-safe + capability-typed); fails address-space-isolation reading | Lowest overhead | Writable but verifies logical not fault isolation | Rejected — weakest honest reading of NFR-S10 invariant |
| **B. Separate enforcement subprocess** ✅ | Three Rust processes (`aotf-cli`, `aotfd`, `aotf-gatekeeper`); gatekeeper self-confined via seccomp-bpf on Linux; gate-IPC over dedicated Unix socket | All 3 rails cleanly | +5–15 MB RSS, +1 process in cold-start (non-blocking for gate-free commands) | Clean — mutate `aotf-gatekeeper::gate::allow_deny`, integration test fails | **Selected** (see expanded block below) |
| **C. WASM with capability imports** | Enforcement core compiled to `wasm32-wasi`, loaded by `aotfd` via wasmtime; capability boundary = Wasm linear memory + explicit import surface | Capability-bounded by construction | +~8 MB binary, +3–8 MB RSS per instance | Sharp — mutate Wasm binary, host capability checks fail | Deferred — architecturally elegant, strategically premature for 3–4-person MVP; Option B preserves migration path because the enforcement core is already a separate crate |

#### Option B — Separate enforcement subprocess (expanded)

Three Rust processes: `aotf-cli` (short-lived), `aotfd` (daemon + plugin host), `aotf-gatekeeper` (long-lived enforcement sidecar). Bun agent is a subprocess of `aotfd`. Plugins load in `aotfd`; a compromised plugin cannot reach the gatekeeper's address space or its SQLite handles — the worst it can do is submit bad requests over the gate-IPC channel, which the gatekeeper validates and audit-logs.

**Cross-platform posture (stance X3 — decided in step-03 Party Mode):** the gatekeeper is **a single cross-platform binary**; the IPC protocol and the crate implementation are identical on Linux and macOS. What differs is the layered confinement:

| Platform | Isolation layers |
|---|---|
| Linux | Process isolation + POSIX privilege drop + **seccomp-bpf syscall filter** (additive, NFR-S08 Linux clause) |
| macOS | Process isolation + POSIX privilege drop + `RLIMIT_*` + close-on-exec; **no kernel-level syscall filter** (NFR-S08 macOS clause preserved; gap is published, not hidden) |

Rejected alternatives: **X1** (Linux-only gatekeeper with library-in-binary fallback on macOS) was rejected because two codepaths double FR-105.2 mutation-test surface and fracture the primary-install-target security story. **X2** (cross-platform with `sandbox_init(3)` on macOS) was rejected because `sandbox_init` is deprecated since macOS 10.7, requires Developer ID signing + notarization hostile to OSS forks, and creates false confidence that makes NFR-R02 fault-injection tests meaningless.

X3 is consistent with the precedent the PRD already set for *plugin* sandboxing (NFR-S08 macOS clause): enforcement is advisory-at-kernel-level but honest about the gap. The architectural boundary is the *process*; the *kernel filter* is additive.

### Selected Starter: Custom Cargo + Bun Monorepo with Separate Enforcement Subprocess (Option B / X3)

**Rationale for Selection:**

1. **Satisfies NFR-S10 on the strongest reading** — address-space isolation + (Linux) seccomp confinement + capability-bounded IPC surface. Options A and C satisfy different combinations of the three rails; Option B satisfies all three on Linux and 2.5-of-3 on macOS (process isolation + capability-bounded IPC surface without kernel syscall filter).
2. **Boring technology discipline.** Three Rust processes + Unix sockets is mature, debuggable, low team-learning cost — satisfies CLAUDE.md simplicity-first and Winston's "embrace boring" principle.
3. **Preserves Option C migration path at zero marginal cost.** Because `aotf-gatekeeper` is already a separate crate, it can be retargeted to `wasm32-wasi` later without touching `aotfd` or `aotf-cli`. Optionality without premature complexity.
4. **Best fit for required tests.** FR-105.2 mutation test and NFR-R02 SIGKILL-durability fault injection both produce natively clean scenarios in a three-process architecture. The cross-platform IPC contract (AC-PLAT-3 below) enables fault-injection test parity.
5. **Matches the seccomp-bpf Linux enforcement story fixed by NFR-S08.** Same machinery enforces plugin capabilities and gatekeeper self-confinement.
6. **Platform posture is honest, not hidden.** X3 accepts the macOS gap the PRD already accepted for plugin sandboxing — one asymmetry across the codebase, not two.

**Risks accepted:**

- Cold-start for gate-requiring operations waits on gatekeeper readiness (mitigated: readiness probe, gate-free CLI commands don't wait).
- +1 IPC hop per `EXECUTE_*` tier operation (~100 μs on Unix socket; negligible against NFR-P05's 30 s budget).
- **macOS security posture is weaker than Linux** — process isolation only, no kernel syscall filter. Mitigation: `aotf doctor` warns; published in `docs/security/platform-matrix.md`; README flags the platform matrix at install time; recommend Linux VM for production Mac fleets. This gap is consistent with the PRD's existing NFR-S08 macOS plugin-sandbox concession; we do not invent a new asymmetry.

**Risks rejected:**

- Speculative WASM adoption ahead of demonstrated need.
- Linking against Apple's deprecated `sandbox_init(3)` to appear stronger on macOS than we are.
- Linux-only gatekeeper (fractures primary-install-target story; doubles the security-boundary codepath).

### Initialization Command (to execute as first implementation story)

No single third-party CLI creates this layout. The scaffolding story executes:

```bash
# 1. Create Cargo workspace
mkdir aotf && cd aotf
git init
# Workspace members: aotf-cli (bin), aotfd (bin), aotf-gatekeeper (bin),
#                    aotf-storage (lib), aotf-ipc-protocol (lib),
#                    aotf-plugin-api (lib), aotf-core-types (lib)
for m in aotf-cli aotfd aotf-gatekeeper; do cargo new --bin crates/$m; done
for m in aotf-storage aotf-ipc-protocol aotf-plugin-api aotf-core-types; do cargo new --lib crates/$m; done

# 2. Create root Cargo.toml workspace manifest with shared dependencies
#    (see dependency matrix in §Architectural Decisions Provided by Starter below)

# 3. Create Bun agent package
mkdir -p packages/aotf-agent && cd packages/aotf-agent
bun init -y
bun add @anthropic-ai/claude-agent-sdk @modelcontextprotocol/sdk
cd ../..

# 4. Add Turborepo pipeline
bun add -d turbo
# Write turbo.json with cargo:build + bun:build + build pipelines

# 5. Add dev-env pinning (flake.nix OR .tool-versions)
# 6. Add .github/workflows/ for cargo audit, bun audit, SLSA provenance
```

**Versions to pin at scaffold time** (knowledge cutoff 2026-01; verify against current stable before pinning):

| Dependency | Target | Notes |
|---|---|---|
| `rustc` edition | 2024 | `rust-version = "1.80"` floor |
| `tokio` | 1.x | async runtime |
| `clap` | 4.x (derive) | CLI parsing |
| `notify` | 6.x | file-watch (Rust side) |
| `git2` | 0.18.x | libgit2 bindings (pre-push hook, worktree ops) |
| `rusqlite` | 0.31.x, bundled | SQLite + WAL |
| `serde`/`serde_json` | 1.x | JSON-RPC wire format |
| `reqwest` | 0.12.x, rustls-tls | HTTP client |
| `tracing` + `tracing-subscriber` | 0.1.x / 0.3.x | structured logging |
| `libseccomp` | 0.3.x, Linux feature-gated | gatekeeper self-confinement (Linux only) |
| `webauthn-rs` | 0.5.x | FR-106 consent token verification |
| `anyhow` + `thiserror` | 1.x | error handling |
| Bun | latest stable | TS runtime |
| `@anthropic-ai/claude-agent-sdk` | latest stable | agent layer |
| `@modelcontextprotocol/sdk` | latest stable | MCP server |
| `turbo` | latest stable | build orchestration |

### Architectural Decisions Provided by Starter

**Language & Runtime:**
- Rust stable (edition 2024, `rust-version = "1.80"` floor) for all host-side code: CLI, daemon, gatekeeper, storage, IPC protocol, plugin API.
- TypeScript with Bun as the runtime for the agent layer. No Node-compatibility requirement in v1.0.
- Two-language boundary runs through exactly one Unix domain socket: `aotfd` ↔ `aotf-agent`. All other IPC (`aotf-cli` ↔ `aotfd`, `aotfd` ↔ `aotf-gatekeeper`) is Rust-to-Rust.

**Styling Solution** (N/A — this is a CLI + daemon + agent, no GUI surface in v1.0):
- CLI uses Ratatui for any interactive prompts per UX design spec §CLI-Specific Ratatui Widgets.
- No web UI in v1.0. Growth dashboard (FR-81–84) gets its own starter evaluation when it's picked up.

**Build Tooling:**
- Cargo workspace with resolver `2` and shared `[workspace.dependencies]` for single-source-of-truth version pinning.
- Turborepo (`turbo.json`) orchestrates cross-language builds with `cargo:build`, `bun:build`, and `build` (dependsOn) pipelines. Caching on the cross-build boundary avoids rebuilding the TS agent on pure Rust changes and vice versa.
- Final distribution: `aotf-cli` binary embeds the Bun agent's built `dist/` as a compressed asset (`include_bytes!`); first-run extracts to `~/.aotf/agent/`. `aotf update` replaces the extract without re-downloading the Rust binary.

**Testing Framework:**
- Rust side: `cargo test` with `tokio` multi-thread runtime for integration tests. `cargo-mutants` for the FR-105.2 mutation test on `aotf-gatekeeper::gate::allow_deny`. `cargo-nextest` recommended for faster CI feedback.
- TS side: `bun test` (Bun's native runner, Jest-compatible).
- Fault injection: a test-only `aotf-test-harness` crate provides deterministic kill-points for NFR-R02 SIGKILL-durability tests; WAL replay assertions in integration tests.
- Golden-fixture corpus: `fixtures/finding-v1/` directory holds canonical `Finding v1` payloads for round-trip tests between Rust and Bun (Amelia's "first deliverable that unblocks stories" per step-02).

**Code Organization (Cargo workspace layout):**

```
aotf/
├── Cargo.toml                                # workspace
├── crates/
│   ├── aotf-cli/                             # binary: user-facing CLI
│   ├── aotfd/                                # binary: daemon (watchers, orchestrator, plugin host)
│   ├── aotf-gatekeeper/                      # binary: enforcement core — tier map, gate, audit writer
│   ├── aotf-storage/                         # library: SQLite+WAL wrapper, append-only audit impl
│   ├── aotf-ipc-protocol/                    # library: JSON-RPC schemas + Finding v1 wire struct
│   ├── aotf-plugin-api/                      # library: plugin loader, SHA-256 verify, seccomp hooks
│   └── aotf-core-types/                      # library: shared domain types (Finding, ActionTier, etc.)
├── packages/
│   └── aotf-agent/                           # Bun/TS: Claude Agent SDK, MCP server, adapters,
│                                             #         Goal Loop runtime, confidence computer
├── fixtures/
│   └── finding-v1/                           # canonical payloads for cross-language golden tests
├── turbo.json
├── Cargo.lock                                # committed (SLSA reproducibility)
├── bun.lockb                                 # committed
├── flake.nix                                 # dev-env pinning (alt: .tool-versions)
└── .github/workflows/
    ├── ci.yml                                # cargo test + bun test + cargo audit + bun audit
    ├── release.yml                           # SLSA provenance, signed binaries, brew formula push
    └── codeql.yml                            # security scanning
```

**Development Experience:**
- `cargo watch -x test` + `bun --watch test` for continuous test runs.
- `cargo insta` for snapshot tests on IPC protocol + audit-log record formats.
- `cargo doc --workspace` + `typedoc` for the TS agent — both published to the docs site on release.
- Pre-commit hook runs `cargo fmt`, `cargo clippy -- -D warnings`, `bun format`, `bun lint`. This is AOTF dogfooding its own pre-push discipline (CLAUDE.md §5 reversibility).
- First-boot script: `./scripts/bootstrap.sh` installs toolchain (via `rustup` + `curl -fsSL bun.sh`) and runs the full scaffold-verification test suite. Target: <2 minutes from `git clone` to green test run for new contributors.

### Platform-Posture Acceptance Criteria (X3)

The cross-platform gatekeeper decision is concrete enough to land as acceptance criteria on the scaffolding epic:

- **AC-PLAT-1:** `aotf --version --verbose` prints `capability_tier: enforced` on Linux (seccomp-bpf active) or `capability_tier: isolated` on macOS (process-isolation only). The tag is read by `aotf doctor`, the release notes, and the telemetry `os/arch` payload.
- **AC-PLAT-2:** `aotf doctor` emits a WARN on macOS: *"syscall-level capability enforcement unavailable on this platform; relying on process isolation per NFR-S08 macOS clause. See docs/security/platform-matrix.md."*
- **AC-PLAT-3:** The `aotfd ↔ aotf-gatekeeper` IPC contract (message schemas, readiness protocol, crash-recovery handshake) is byte-identical across platforms. This is the enabling invariant for NFR-R02 fault-injection test parity — the same test suite runs on both OSes.
- **AC-PLAT-4:** A `docs/security/platform-matrix.md` ships with v1.0 and is linked from the README, the install-script output, and `aotf doctor --output json`. The matrix lists each platform × isolation mechanism × known gap × mitigation × roadmap item.
- **AC-PLAT-5:** v1.0 release notes include a "Platform Security Posture" section that names the macOS tier downgrade explicitly, following the precedent set by the existing plugin-sandbox gap disclosure.

These ACs are owned by the scaffolding epic (first implementation story) and verified in CI matrix runs on ubuntu-latest and macos-latest.

**Note:** Project scaffolding using the layout above is the first implementation story (Epic 1 / Story 1). No architectural work should proceed to step-04 (component decisions) without this skeleton in place or a clear skeleton-PR-in-flight.

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**
- D-IPC-1/2/3/4/5 — IPC contract shape (schemas, versioning, framing, backpressure, crash recovery)
- D-ENF-1/2/3 — Enforcement core API (tier map, gate surface, mutation manifest)
- D-DB-1/2/3/4 — SQLite schema (audit-log append-only, findings, decision log, token storage)
- D-CLK-1 — Clock & ordering semantics

**Important Decisions (Shape Architecture):**
- D-DB-5 — Schema migration policy for pre-alpha
- D-MCP-1 — v1.0/v1.1 MCP write-surface boundary
- D-INFRA-1/2 — CI matrix + release pipeline scope

**Deferred Decisions (Post-MVP):**
- Web dashboard storage model (FR-81–84 Growth)
- Multi-tenancy isolation model (v0.2+ per step-02 defer-and-flag)
- Cross-org intelligence transport (NFR-D02 Vision tier)

---

### Category 1: IPC Contract

**D-IPC-1. Message schemas and topology.** Three IPC channels, three distinct schemas. All use JSON-RPC 2.0 over Unix domain sockets.

| Channel | Endpoint | Owner (server) | Client(s) | Purpose |
|---|---|---|---|---|
| CLI↔Daemon | `~/.aotf/run/aotfd.sock` | `aotfd` | `aotf-cli` | Command dispatch, finding queries, config reads |
| Daemon↔Gatekeeper | `~/.aotf/run/gatekeeper.sock` | `aotf-gatekeeper` | `aotfd` only | Gate evaluation, audit writes, consent-token verify |
| Daemon↔Agent | `~/.aotf/run/agent.sock` | `aotfd` | `aotf-agent` (Bun) | Finding ingestion, diagnosis dispatch, Goal Loop coordination |

Rust↔Rust channels (1, 2) use `serde_json` with shared `aotf-ipc-protocol` crate types. Rust↔Bun channel (3) uses the same shared types, serialized identically; cross-language round-trip verified by golden fixtures in `fixtures/finding-v1/`.

Critical invariant — the gatekeeper socket (channel 2) is accessible only to the `aotfd` UID; `aotf-cli` and plugins never connect to it. Filesystem permissions + SO_PEERCRED verification enforce this.

**D-IPC-2. Protocol versioning.** Semver on the method namespace, not on the JSON-RPC wire protocol.

- Method names: `v1.finding.get`, `v1.gate.evaluate`, `v1.diagnosis.request`, etc.
- Breaking changes = new namespace (`v2.*`), with both served side-by-side during a deprecation window.
- Additive changes (new optional fields, new methods in a namespace) = minor-version semver on the crate only; wire compat preserved.
- JSON-RPC 2.0 itself is frozen (RFC-equivalent); no custom extensions.

Rationale: versioning on namespace rather than wire protocol means old clients stay working through a transition without protocol translation.

**D-IPC-3. Message framing and size.**

- Transport: length-delimited framing over UDS. Each message = `u32-be length` + `JSON payload`. No streaming within a single message (streaming flows use separate paired request/notification messages).
- **Maximum message size: 1 MiB.** Findings with diffs larger than this are *paged* via a second request (`v1.finding.get_diff_chunk`) rather than inlined. Diagnosis reasoning traces larger than 1 MiB are written to SQLite blob storage and referenced by hash in the IPC message.
- Rationale: 1 MiB is generous for any single protocol message (the largest realistic payload is an AI reasoning trace, which should be stored-not-passed); caps the memory-amplification attack surface against NFR-P01.

**D-IPC-4. Backpressure policy (bounded channels, named variants).** *[Revised post-Party-Mode: reduced 4 → 2 policies.]*

All inter-component channels are bounded. Two named variants ship in v1.0, implemented as a single enum with exhaustive match:

```rust
// In crates/aotf-core-types/src/backpressure.rs
pub enum Backpressure {
    /// Producer blocks until space is available. Use for request/response flows
    /// where caller latency is acceptable and losing a message is not.
    Block,
    /// On overflow, drop the oldest queued item and accept the new one.
    /// Emits an explicit eviction event to the caller-provided audit callback.
    /// Use for event streams where freshness wins (log watcher, Telegram dispatch).
    DropOldest,
}

impl Backpressure {
    pub fn apply<T>(&self, queue: &mut VecDeque<T>, capacity: usize, item: T,
                    on_evict: impl FnOnce(T)) -> Result<(), QueueFull>;
}
```

Depth is specified at each channel construction site, not in a central registry. The enum is the test-leverage unit: exhaustive match + property tests on `{empty, full, overflow}` × `{Block, DropOldest}` = 6 table-driven cases.

**Usage sites (v1.0):**

| Consumer | Policy | Depth | Rationale |
|---|---|---|---|
| Gate evaluation (`aotfd → aotf-gatekeeper`) | `Block` with 200ms timeout on `try_send` | 8 | Must not drop (fail-closed); 200ms timeout becomes `GateEvaluationTimeout` error routed through the same outcome path as `Deny`. |
| Log watcher → dedup queue | `DropOldest` | 100 | Absorbs NFR-P07 burst (1000 errors/min); dedup mostly absorbs the flood anyway. |
| Telegram dispatch | `DropOldest` | 32 | `on_evict` callback emits `DroppedNotification` audit entry (satisfies NFR-I02). |
| CLI → daemon command queue | `Block` | 8 | Caller-visible wait acceptable. |

**Extension policy.** New variants are added to the enum only when a new consumer demonstrates genuinely distinct overflow semantics. First candidate likely to need a new variant: a hypothetical write-capable MCP tool that must fail-fast rather than block (`Fatal`) — deferred to v1.1 per D-MCP-1.

**Enforcement against unbounded channels.** CI gate in `ci.yml`:

```yaml
- name: No unbounded channels
  run: |
    if rg 'unbounded_channel|unbounded\(\)|flume::unbounded|crossbeam_channel::unbounded' \
       --type rust -g '!**/tests/**' -g '!**/mutants.toml' .; then
      echo "::error::Unbounded channel detected. Use Backpressure::{Block,DropOldest} from aotf-core-types."
      exit 1
    fi
```

Five lines of CI config; catches the same bug class a custom `dylint` would have caught at 2–3 dev-weeks of cost. Custom clippy lint deferred to v1.1+ or until a clear regression demonstrates the grep check is insufficient.

**D-IPC-5. Crash recovery handshake.**

Each long-lived process exposes a `v1.daemon.status` (or `v1.gatekeeper.status`) method returning `{pid, generation_id, started_at, last_clean_shutdown}`. On daemon/gatekeeper restart, the `generation_id` bumps; clients detect stale sockets and reconnect.

`aotfd` startup sequence: (1) acquire exclusive lock on `~/.aotf/run/aotfd.pid`; (2) verify gatekeeper readiness via `v1.gatekeeper.status` (timeout 5s per NFR-P03); (3) run SQLite in-flight-state detector per NFR-R04 (detect half-applied fixes, orphaned rollback refs, uncommitted audit entries); (4) only after (1)+(2)+(3) succeed, accept CLI connections.

If (3) finds partial state, the recovery log path is documented per FR-34: for each detected orphan, `aotfd` either completes the transaction (if all side-effects can be verified idempotent) or invokes the rollback ref and marks the finding `human-required`.

---

### Category 2: Enforcement Core API

**D-ENF-1. Tier-classification table: source of truth and review process.**

The action-to-tier mapping table lives in `crates/aotf-gatekeeper/src/tier_map.rs` as a compile-time `&'static [(&str, ActionTier)]` slice. This is:

- **Source of truth:** the Rust file, reviewed via standard code review (FR-105.5 explicit requirement: "reviewed through standard code review, not config").
- **Not runtime-mutable:** no SQL table, no config file, no live reload. Changing the tier classification requires a PR + review + release.
- **Greppable and auditable:** a deliberate choice to prefer compile-time discipline over runtime flexibility.
- **Coverage gate in CI:** every variant of the `Action` enum must appear in `tier_map` — enforced by a `cargo test` coverage test using the `strum::IntoEnumIterator` trait.

The gatekeeper API that consumes this table is `gate::evaluate(action: Action, context: ActionContext) -> GateDecision` — no other code path consumes the table, no other path can bypass the gate for an `Action`.

**D-ENF-2. Gate API surface (the only surface `aotfd` can call on `aotf-gatekeeper`).**

Minimalist by design — 7 methods total.

```rust
// Called by aotfd before every autonomous operation
v1.gate.evaluate(action: Action, context: ActionContext) -> GateDecision
    // Returns: Allow | Deny(reason) | RequireConsent(required_tier) | RequireApproval(channel)

// Called by aotfd when it has a consent token to present
v1.gate.present_consent_token(action: Action, token: ConsentToken, assertion: WebAuthnAssertion) -> GateDecision

// Called by aotfd after every autonomous operation
v1.gate.record_outcome(action_id: ActionId, outcome: Outcome, metadata: RecordMetadata) -> ()

// Called by aotf-cli (via aotfd) for user audit queries
v1.audit.query(filter: AuditFilter, limit: u32) -> AuditPage

// Called by aotf-cli for issuing a consent token
v1.consent.issue(env: Environment, scope: ConsentScope, ttl: Duration, approver: UserId, assertion: WebAuthnAssertion) -> ConsentToken

// Called by aotfd to check platform capability tier (for AC-PLAT-1)
v1.gatekeeper.capability_tier() -> CapabilityTier  // Enforced | Isolated

// Health check
v1.gatekeeper.status() -> GatekeeperStatus
```

No "generic" or "raw" methods. No way to write to the audit log except through `record_outcome` (which validates `action_id` against a prior `gate.evaluate`). This is the enforcement guarantee that makes NFR-S06 tractable: the API surface itself ensures the audit log is a tap on evaluated actions, not a raw write surface.

**D-ENF-3. Mutation-target manifest (FR-105.2 testability invariant).**

The gate's allow/deny logic is factored into three named pure functions in `crates/aotf-gatekeeper/src/gate.rs`:

```rust
pub(crate) fn classify(action: &Action) -> ActionTier { /* reads tier_map */ }
pub(crate) fn evaluate_tier_ceiling(tier: ActionTier, ceiling: ActionTier) -> Decision { /* ... */ }
pub(crate) fn require_consent_if_irreversible(tier: ActionTier, env: &Environment) -> ConsentRequirement { /* ... */ }
```

A manifest file `crates/aotf-gatekeeper/mutants.toml` lists these three functions as the **required mutation targets**. CI runs `cargo mutants --file src/gate.rs` and fails if any mutation of the allow/deny branch survives all integration tests (satisfies AC FR-105.2 per PRD).

Any future additions to gate logic must be added to both `src/gate.rs` and `mutants.toml` in the same PR. A CI check validates this invariant.

---

### Category 3: SQLite Schema High-Level

**D-DB-1. Audit log append-only enforcement.** SQLite-layer, not application-layer.

The `audit_log` table has:
- No UPDATE trigger (any UPDATE via connection fires a BEFORE UPDATE trigger that RAISEs ABORT).
- No DELETE trigger (any DELETE via connection fires a BEFORE DELETE trigger that RAISEs ABORT).
- A separate application-user with INSERT-and-SELECT-only privileges via SQLite's URI-based credential model.
- Hash chain: each row includes `prev_hash` = SHA-256 of the previous row's serialized contents. Chain break is detectable on read.

Enforcement is at the storage layer (triggers) + schema layer (user privileges) + verification layer (hash chain). Three-line design; three defense layers.

**D-DB-2. Core tables (v1.0).**

| Table | Purpose | Key Columns | Notes |
|---|---|---|---|
| `findings` | Detected issues (Finding v1 wire schema as native rows) | `id`, `schema_version`, `source_json`, `severity`, `message`, `file`, `line`, `confidence`, `confidence_components_json`, `status`, `created_at`, `updated_at` | Primary key: `id` (TEXT, stable across restarts per FR-12). Indexes on `status`, `created_at`. |
| `audit_log` | Append-only action log (FR-66, NFR-S06) | `seq`, `prev_hash`, `timestamp_ns`, `action`, `action_tier`, `actor`, `outcome`, `context_json`, `irrev_fields_json` | INSERT-only enforced by triggers (D-DB-1). `irrev_fields_json` is non-null only for `EXECUTE_IRREVERSIBLE` and contains the 6 required fields. |
| `decision_log` | AI diagnosis + fix decisions (FR-20, FR-36) | `finding_id`, `decision_type`, `confidence_score`, `confidence_components_json`, `ai_reasoning_trace_hash`, `outcome`, `timestamp_ns` | AI trace stored by hash; actual trace in `ai_traces` blob table (below). |
| `ai_traces` | AI reasoning-trace blob storage | `hash`, `trace_pii_filtered`, `size_bytes`, `created_at` | PII filter applied before insert (FR-68). Content-addressed by SHA-256. |
| `rollback_refs` | Git rollback pointers (FR-26) | `finding_id`, `ref_name`, `commit_sha`, `pre_state_json`, `created_at`, `invoked_at` | `invoked_at` non-null = rollback was executed. |
| `user_tokens` | User access tokens (FR-49, FR-107a) | `token_hash` (Argon2), `user_id`, `role`, `scopes_json`, `created_at`, `revoked_at`, `last_used_at` | Argon2 hash; no plaintext storage. |
| `consent_tokens` | Production consent tokens (FR-106, FR-107b) | `token_hash`, `action_id`, `approver_user_id`, `webauthn_assertion_hash`, `issued_at`, `expires_at`, `consumed_at` | Single-use (`consumed_at` non-null after use); 1-hour TTL. |
| `mcp_sessions` | MCP invocation tokens (FR-107c) | `session_id`, `token_hash`, `scope`, `started_at`, `ended_at` | In-memory during daemon run; persisted only for audit reconstruction. |
| `plugins` | Installed plugins + SHA-256 + capabilities | `name`, `version`, `sha256`, `capabilities_json`, `action_tier_ceiling`, `installed_at`, `trust_status` | Matches `plugins.lock.yaml`; cache-invalidated on lock-file mtime/checksum change per PRD §Security. |
| `findings_dedup` | Deduplication index | `error_hash`, `last_seen_at`, `occurrence_count`, `latest_finding_id` | 90-second window per PRD §Watch Pipeline. |

All tables `WITHOUT ROWID` where the natural primary key is a TEXT hash (`audit_log`, `ai_traces`); others use `ROWID` for index-efficiency.

**D-DB-3. Connection model.**

- `aotf-gatekeeper` owns the write-connection to `audit_log`, `consent_tokens`, and `decision_log` (IRREVERSIBLE portion). No other process has write access to these tables.
- `aotfd` owns the write-connection to `findings`, `findings_dedup`, `ai_traces`, `rollback_refs`, `decision_log` (non-IRREVERSIBLE portion), `plugins`.
- `aotf-cli` is read-only on all tables (opens SQLite in `?mode=ro` URI).
- Enforced at SQLite layer via `ATTACH DATABASE ... AS audit MODE readonly` and per-table privilege views — not application-layer honor system.

**D-DB-4. WAL + durability.**

- `journal_mode = WAL`, `synchronous = NORMAL` (WAL is crash-safe at NORMAL; FULL unnecessary and slower).
- `wal_autocheckpoint = 1000` (default; acceptable for our write rate).
- **Critical NFR-R02 guarantee:** `aotf-gatekeeper::audit::write()` calls `PRAGMA wal_checkpoint(PASSIVE)` on a timer every 30 seconds, plus immediately after every `EXECUTE_IRREVERSIBLE` record. This ensures NFR-R02's "no findings lost in last 30s on SIGKILL" for audit entries specifically; non-audit findings rely on standard WAL durability (data loss window ≤ one checkpoint, acceptable per NFR).
- Fault-injection tests (per step-03 test framework) verify this: SIGKILL mid-transaction, restart, verify audit log hash chain integrity.

**D-DB-5. Schema migration policy (pre-alpha).**

- v1.0 is pre-alpha. Breaking SQLite schema changes are **acceptable** within v1.0.x releases with a clear CHANGELOG migration note and a one-shot migration script shipped with the release.
- From v1.0 stable → v1.1, schema changes follow standard additive-only discipline or ship a verified migration.
- Wire schemas (Finding v1, IPC JSON-RPC method namespaces) are frozen independently of SQLite schema — they do not migrate with it.
- Migration script location: `crates/aotf-storage/migrations/NNNN_description.sql`, applied at daemon startup, version tracked in `schema_migrations` table.

---

### Category 4: Clock & Ordering Semantics

**D-CLK-1. Audit log ordering and timestamps.**

Cross-cutting concern 6 from step-02 resolved:

- **Total order within the audit log:** monotonically-increasing `seq` column (`INTEGER PRIMARY KEY AUTOINCREMENT`). SQLite enforces monotonicity under single-writer (gatekeeper-owned) writes. This is the authoritative order.
- **Human-readable timestamp:** `timestamp_ns` column = `CLOCK_MONOTONIC_RAW` on Linux / `mach_absolute_time` on macOS, rebased to wall-clock at boot. Stored in nanoseconds. Never used for ordering; only for display/audit queries.
- **Cross-process ordering for pre-audit events:** gate evaluations and finding detections in `aotfd` emit a Lamport timestamp on the IPC message to the gatekeeper. Gatekeeper's `seq` assignment absorbs this Lamport timestamp into the canonical ordering. Consumers that need cross-process causal order read Lamport; consumers that need system-wide total order read `seq`.
- **Wall-clock timestamps** appear only in (a) user-facing display, (b) `issued_at`/`expires_at` for TTL-based objects (consent tokens), (c) external export formats (JSON-lines logs). Never used for internal ordering decisions — skew would silently corrupt audit order.

Rationale: single-writer + autoincrement gives total order at the cheapest cost. Lamport for causal traceability across `aotfd` ↔ `aotf-gatekeeper` ↔ `aotf-agent`. Wall-clock only for humans and TTLs.

---

### Category 5: v1.0/v1.1 Boundary Decisions

**D-MCP-1. MCP write-surface: what v1.0 code must NOT foreclose.**

v1.0 ships with 4 read-only MCP tools per FR-65: `get_risk_score`, `get_finding`, `list_recent_runs`, `get_daemon_status`. The architectural decision is what structure v1.0 implements so that v1.1 write-capable tools can be added without rewrite.

Accepted constraints on v1.0 MCP implementation:

1. MCP server runs as a child process of `aotfd`, separate binary crate `aotf-mcp-server` (new v1.0 crate), communicating over a dedicated MCP socket `~/.aotf/run/mcp.sock`. Not bundled into `aotfd`, not into `aotf-agent` — its own process so v1.1 write tools can get their own capability scoping.
2. MCP invocation tokens (FR-107c) are issued by `aotfd` and presented to `aotf-mcp-server` at session start. v1.0 tokens are read-scoped; the token schema includes a `scope: {read, write}` field from day one, even though only `read` is accepted in v1.0.
3. Write-tool call paths (not implemented in v1.0) would route through `aotf-mcp-server` → `aotfd` → `aotf-gatekeeper` — the same gate as CLI-originated operations. FR-107's AuthTypeMismatch enforcement is already in place in the gatekeeper, so v1.1 just needs write-scoped MCP tokens to pass through unchanged.
4. **Threat model for v1.1 write surface (design now, implement later):** MCP chain-of-authorization is weaker than direct CLI authorization because the developer cannot inspect the calling agent's authority at invocation time. v1.1 mitigation: write-capable MCP tools require a write-scoped MCP token AND a human approval prompt (similar to FR-30 interactive approval, routed via Telegram or CLI-stdin) for every `EXECUTE_*` tier. No silent autonomous write through MCP ever.

**D-INFRA-1. CI matrix (scoped to MVP safety-valve minimum).**

- Matrix: `{ubuntu-latest, macos-latest}` × `{rust-stable, rust-MSRV}` × `{bun-latest}`.
- Gates: `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --check`, `cargo audit`, `cargo mutants --file crates/aotf-gatekeeper/src/gate.rs` (weekly, not per-PR — mutation testing is expensive), `bun test`, `bun audit`.
- **Cross-platform parity gate:** the `aotfd ↔ aotf-gatekeeper` IPC integration test runs on both platforms and must byte-match. Satisfies AC-PLAT-3.

**D-INFRA-2. Release pipeline.**

- `release.yml` triggers on tags `v*.*.*`. Builds matrix `{linux-x86_64, linux-aarch64, macos-x86_64, macos-aarch64}`; produces static binaries signed with project GPG key.
- SLSA provenance attestation via `slsa-framework/slsa-github-generator` GitHub Action.
- Homebrew formula auto-push to `hieutrungdao/homebrew-aotf` tap (separate repo, auto-PR not auto-merge).
- `curl | sh` install script auto-updated in gh-pages branch of `install.aotf.dev`.
- SBOM generation (per PRD Open-Source Trust Model roadmap) deferred to post-v1.0.

### Decision Impact Analysis

**Implementation Sequence (story ordering implied by these decisions):**

1. **Scaffold** (Epic 1 / Story 1) — monorepo skeleton per step-03. AC-PLAT-* verified on both platforms.
2. **Finding v1 wire struct + golden round-trip** — Amelia's "first deliverable that unblocks stories." Enables parallel work on Rust and Bun sides.
3. **IPC protocol crate** (`aotf-ipc-protocol`) — implements D-IPC-1/2/3 method namespaces and framing. Consumer crates mock against these types.
4. **Gatekeeper scaffold** (`aotf-gatekeeper`) — implements D-ENF-1/2/3 with stub logic; `tier_map.rs` + `gate.rs` + `mutants.toml` land here. FR-105.2 mutation test passes (against stubs) in this PR.
5. **SQLite storage crate** (`aotf-storage`) — implements D-DB-1/2/3/4 schema + connection model. Append-only enforcement verified by negative test.
6. **Daemon scaffold** (`aotfd`) — connects CLI socket, gatekeeper socket, agent socket. Crash-recovery handshake (D-IPC-5) verified by fault-injection test.
7. **CLI scaffold** (`aotf-cli`) — thin client; `aotf --version`, `aotf doctor`, `aotf user audit`. AC-PLAT-1/2 verified.
8. **Bun agent scaffold** (`aotf-agent`) — Claude Agent SDK integration; AI CLI subprocess manager; confidence computer.
9. From here: feature-driven development per PRD epics.

Stories 1–7 must be sequenced; stories can only parallelize from step 8 onward (with Bun agent work able to start at step 2 against golden fixtures).

**Cross-Component Dependencies:**

- `aotf-ipc-protocol` is dependency of every crate except `aotf-core-types` (where domain types live). Change-risk: high. Mitigation: semver discipline + golden fixtures.
- `aotf-core-types` is dependency of `aotf-ipc-protocol`. This is the only one-way allowed dependency direction; enforced by `cargo-deny` workspace rules.
- `aotf-gatekeeper` has no dependency on `aotfd` (gatekeeper serves; daemon calls). Enforced by `cargo-deny`.
- `aotf-agent` (Bun) depends on Finding v1 wire struct via generated TypeScript types (`cargo run --bin gen-ts-types`); no direct Rust dependency.
- Plugin crates (future) depend on `aotf-plugin-api` only; never on `aotf-gatekeeper` or `aotfd` directly. Enforced by `cargo-deny`.

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

For AOTF, the patterns that matter most are not generic style guides but **invariant call patterns** that enforce the seven-control trust surface (NFR-S04) at the code level. Generic web-app conflict points (REST endpoint naming, API response wrappers, frontend state management, loading-state UX) are N/A in v1.0.

Critical conflict-point categories:

1. Language & cross-boundary naming (Rust + TS share a wire, must not diverge)
2. Error handling & Result shapes (typed in libraries, anyhow in binaries, stable JSON-RPC codes at IPC)
3. Logging & observability (tracing discipline, PII-safe by default, structured fields)
4. Gate-evaluation discipline (the invariant that makes FR-105/NFR-S04 enforceable)
5. Audit emission discipline (the invariant that makes NFR-S06 append-only guarantee meaningful)
6. IPC method & message format (strict naming, request/response shape, timestamp format)
7. Time & ID formats (monotonic vs wall-clock via `Clock` trait, Finding IDs, hashes)
8. Test organization & fixtures
9. Commit & branch discipline
10. Anti-patterns explicitly rejected

### Naming Patterns

**Rust (all crates):**

- Functions, modules, local variables, fields: `snake_case`.
- Types (structs, enums, traits), type aliases: `UpperCamelCase`.
- Constants and statics: `SCREAMING_SNAKE_CASE`.
- Lifetimes: single lowercase letters (`'a`, `'b`); longer names only when disambiguation is required.
- Crate-internal types: prefer `pub(crate)` over `pub` unless the item is part of the documented external surface.
- Test modules: `#[cfg(test)] mod tests { use super::*; ... }` at the bottom of the file under test.

**TypeScript (`packages/aotf-agent`):**

- Functions, local variables, object keys: `camelCase`.
- Types, interfaces, classes: `UpperCamelCase`.
- Constants: `SCREAMING_SNAKE_CASE` only for true compile-time constants; otherwise `camelCase`.
- File names: `camelCase.ts` for modules; `UpperCamelCase.ts` for single-class-export files.

**Cross-boundary types (Rust → TS auto-generation):**

- All shared types (Finding, ActionTier, GateDecision, etc.) are defined in Rust in `crates/aotf-core-types` and `crates/aotf-ipc-protocol`.
- TS types are generated via `ts-rs` (or `specta`) — AI agents MUST NOT hand-write these types on the TS side. Regeneration command: `cargo run --bin gen-ts-types` (writes to `packages/aotf-agent/src/generated/`). CI fails on stale generated files (`git diff --exit-code`).
- Wire serialization uses `serde` with `#[serde(rename_all = "camelCase")]` on Rust types that cross the IPC boundary — TS sees camelCase, Rust stays idiomatic snake_case internally.
- Rationale: Rust is the single source of truth; divergence is impossible by construction.

**IPC method naming:**

- Format: `v{major}.{domain}.{verb}`. Example: `v1.finding.get`, `v1.gate.evaluate`, `v1.diagnosis.request`.
- Domains: `finding`, `gate`, `audit`, `consent`, `diagnosis`, `daemon`, `gatekeeper`, `plugin`, `agent`, `mcp`.
- Verbs: prefer `get`, `list`, `create`, `update`, `cancel`, `status`, `evaluate`, `record`, `issue`, `consume`. Avoid ambiguous verbs like `do`, `handle`, `process`.
- Method name MUST be registered in the `aotf-ipc-protocol` method registry via `inventory`/`linkme` macro — referencing an unregistered method is a compile error (Layer-1 design-enforced per §Enforcement Guidelines below).

**Finding IDs and other identifiers:**

- Finding IDs: `ERR-NNN` where NNN is a zero-padded monotonic counter within a single daemon run; stable across restarts (FR-12). The counter is persisted in `findings_seq` sidecar table.
- Pre-push findings use a distinct prefix: `ERR-PRE-NNN` (matches UX examples like `ERR-PRE-042`).
- QA findings (Growth): `QA-NNN`.
- Consent token IDs: `cnst_` + 16 lowercase hex chars. Example: `cnst_0x8f9a12...`.
- Action IDs (for gate evaluation): UUIDv7 (time-ordered UUIDs) — enables natural sort by creation time without a separate timestamp.
- All IDs are strings on the wire, never integers.

**Database naming (SQLite):**

- Table names: `snake_case`, plural consistently (`findings`, `audit_logs`, `consent_tokens`, `rollback_refs`).
- Column names: `snake_case`. Timestamps: `_at` suffix for wall-clock (`created_at`, `expires_at`); `_ns` suffix for monotonic nanoseconds (`timestamp_ns`). JSON columns: `_json` suffix. Hash columns: `_hash` suffix.
- Indexes: `idx_{table}_{columns}` (e.g., `idx_findings_status`, `idx_audit_logs_timestamp_ns`).
- Migration files: `NNNN_snake_case_description.sql` under `crates/aotf-storage/migrations/`.

**Config file field naming:**

- YAML config: `snake_case` (matches Rust serde default).
- Environment variable overrides: `AOTF_` prefix + SCREAMING_SNAKE_CASE. Example: `AOTF_FIXES__CONFIDENCE_THRESHOLD=0.80` (double-underscore for nested paths).
- Plugin manifest fields (`aotf-plugin.toml`): `snake_case`.

### Error Handling Patterns

**Library crates (`aotf-core-types`, `aotf-ipc-protocol`, `aotf-storage`, `aotf-plugin-api`):**

Use `thiserror` for typed error enums. Every error is a variant with a `#[error("...")]` derive. No `anyhow` in library crates — callers should be able to match on specific error variants.

```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("audit log chain break at seq {seq}")]
    AuditChainBreak { seq: i64 },
    #[error("append-only violation: {attempted_op}")]
    AppendOnlyViolation { attempted_op: String },
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}
```

**Binary crates (`aotf-cli`, `aotfd`, `aotf-gatekeeper`, `aotf-mcp-server`):**

Use `anyhow::Result<T>` at main.rs + command-handler boundaries. Use `.context("what was happening")` or `.with_context(|| format!(...))` to build an error chain. Context messages are present-tense imperative describing the operation that failed. Match on specific library errors at decision points; convert to `anyhow::Error` at boundaries that don't care.

**IPC boundary (JSON-RPC error codes):**

Standard JSON-RPC 2.0 codes (`-32700` parse error, `-32600` invalid request, `-32601` method not found, `-32602` invalid params, `-32603` internal error) plus AOTF-specific codes in the reserved range `-32000` to `-32099`:

| Code | Name | Meaning |
|---|---|---|
| -32001 | `GateDenied` | gate evaluation returned Deny |
| -32002 | `GateEvaluationTimeout` | gatekeeper did not respond within timeout |
| -32003 | `ConsentRequired` | IRREVERSIBLE action reached gate without consent token |
| -32004 | `ConsentInvalid` | consent token expired, already-consumed, or mismatched action |
| -32005 | `AuthTypeMismatch` | FR-107 cross-type token submission |
| -32006 | `TierViolation` | plugin exceeded declared actionTier |
| -32007 | `QueueSaturated` | backpressure rejected the message (from `Block`-policy channels after timeout) |
| -32008 | `McpAuthRequired` | MCP request missing or with invalid session token |
| -32009 | `SchemaViolation` | IPC message failed JSON Schema validation |
| -32010 | `AppendOnlyViolation` | attempted write to append-only table outside the authorized path |

Every code has a corresponding Rust enum variant in `aotf-ipc-protocol::IpcErrorCode` and a human message; both are auto-propagated across the Rust↔TS boundary.

**Never:**

- `.unwrap()` or `.expect()` in production code paths. Exception: `.expect("checked at line N above")` when the invariant is genuinely proven above and documented in the message. Enforced by `clippy::unwrap_used` + `clippy::expect_used` at `-D warnings` level.
- `panic!` in library code. Library code returns errors; only binary code may panic, and only on truly unrecoverable conditions.
- Silent error swallowing. Every `Err(...)` either propagates, logs at `warn!`+, or is explicitly annotated with a comment explaining why silent is correct.

### Logging & Observability Patterns

**Library choice:** Rust uses `tracing` + `tracing-subscriber` (never `log`). TS uses a wrapper in `packages/aotf-agent/src/observability/logger.ts` that adds structured fields (OpenTelemetry candidate for v0.2+).

**Level discipline:**

- `TRACE` — verbose internal state changes; off by default.
- `DEBUG` — useful for `aotf doctor` and bug reports; off by default at user level.
- `INFO` — user-visible state changes (daemon started, finding detected, fix applied). Default minimum.
- `WARN` — recoverable issues (degraded mode, retry, dropped notification).
- `ERROR` — unrecoverable within the current operation.

**Structured fields (not formatted strings):**

```rust
// GOOD
tracing::info!(finding_id = %f.id, confidence = f.confidence, "finding detected");
// BAD (unsearchable, less efficient)
tracing::info!("finding {} detected with confidence {}", f.id, f.confidence);
```

Standard field names: `finding_id`, `action_id`, `tier`, `actor` (UserId or `"daemon"`/`"agent"`), `plugin`, `duration_ms`, `error` (Display, not Debug).

**Daemon structured log output (FR-70):** `aotfd` emits JSON-lines to stderr by default; schema stable within major version. Each line includes `timestamp_ns`, `level`, `target`, `span`, `fields`, `message`. Schema frozen in `aotf-ipc-protocol::DaemonLogLine`.

**PII filtering (FR-68, NFR-D01):**

- INFO+ level: no file contents, error message text from user code, environment variables, API keys, email addresses, or URLs from user configs.
- DEBUG level: above is still filtered; file paths are OK.
- TRACE level (opt-in via env): content may appear, with `pii_visible=true` field as auditable marker.
- `#[tracing::instrument(skip(self, secret))]` strips sensitive args from auto-generated spans.

**Spans for tracing:** Every public fn that crosses a module boundary SHOULD be wrapped in `#[tracing::instrument(level = "debug")]` or manual span creation. IPC request handlers on the server side create a root span with the method name as span name and request_id as a field. `aotfd` → `aotf-gatekeeper` requests propagate request_id via an `x-aotf-request-id` JSON-RPC metadata field.

### Gate-Evaluation Discipline (the FR-105 invariant call pattern)

**MANDATORY sequence for every `EXECUTE_REVERSIBLE` or `EXECUTE_IRREVERSIBLE` action:**

```
1. action_id = Uuid::now_v7()
2. decision = gate.evaluate(Action::<X>, context { action_id, ... }).await?
3. match decision:
     Allow           -> proceed to step 4
     Deny(reason)    -> return Err(GateDenied(reason))
     RequireConsent  -> fetch consent token, present via gate.present_consent_token,
                        loop back to step 2 with updated context
     RequireApproval -> send notification, wait for response, loop back
4. <perform the action>
5. gate.record_outcome(action_id, outcome, metadata).await?
6. return Ok(result)
```

**Violations are bugs, not style choices:**

- Calling step 4 without step 2 preceding it in the same function.
- Calling step 2 but not step 5 for a completed action.
- Using a different `action_id` in step 5 than step 2.
- Retrying the action without re-running step 2 with a fresh `action_id`.

**Code helper that makes the pattern ergonomic:**

```rust
// In crates/aotfd/src/gate_scope.rs
pub async fn gated<F, T, E>(action: Action, ctx: ActionContext, body: F) -> Result<T, GateError>
where F: FnOnce(ActionId) -> Pin<Box<dyn Future<Output = Result<T, E>>>>
{
    let action_id = Uuid::now_v7();
    let decision = GATE.evaluate(action.clone(), ctx.with_action_id(action_id)).await?;
    handle_decision(decision)?;
    let outcome = body(action_id).await;
    GATE.record_outcome(action_id, outcome.as_ref().map(Into::into).unwrap_or_default(), ...).await?;
    outcome.map_err(GateError::from)
}
```

All `EXECUTE_*` code paths use `gated(...)` — the invariant is in the scope guard, not in developer discipline.

**Testability enforcement (AC FR-105.1):** The `aotf-test-harness` crate provides a `MockGatekeeper` recording every `evaluate`/`record_outcome` call. Integration test `tests/gate_invariant.rs` asserts every `evaluate` is followed by exactly one matched-`action_id` `record_outcome`, and no `EXECUTE_*` side-effect is observable without both calls.

### Audit Emission Discipline (the NFR-S06 invariant)

**The only way to write to `audit_log`:** via `gate.record_outcome(...)` in the gatekeeper process. No other code path has a write connection. Enforced at three layers:

- **Rust-side:** `audit_log` write-handle is owned by `aotf-gatekeeper::audit::Writer` and is `pub(crate)`. Never passed out of the crate.
- **SQL-side:** BEFORE UPDATE / BEFORE DELETE triggers reject mutation at the SQLite layer.
- **Schema-side:** `aotfd` and `aotf-cli` open SQLite read-only on the `audit_log` table (D-DB-3).

**API-surface test (`tests/audit_invariant.rs`):** asserts via type reflection that no public writer for `audit_log` exists outside `aotf-gatekeeper::audit::Writer`. Not a behavior-observation test (which could be bypassed) but a type-level invariant check.

**IRREVERSIBLE audit entries (FR-66):** The six required fields (approver identity, UTC timestamp, stated reason ≥20 chars verbatim, consent-token hash, WebAuthn assertion hash, classified tier) are populated in `irrev_fields_json`. Missing any of the six MUST block the action via `IncompleteIrreversibleRecord` error before any side-effect. The stated-reason field is stored verbatim with no truncation, summarization, or AI-based rewriting.

**Hash chain:** Each row's `prev_hash` = SHA-256 of the previous row's canonical serialization. Chain verification runs at daemon startup; chain break causes daemon to refuse startup with a P0 error. Chain break recovery is human-only via `aotf admin audit verify` + `aotf admin audit reseal`.

### IPC Message Format Patterns

**Request/response shape:**

- Request parameters are always named objects (`{ "field1": ..., "field2": ... }`), never positional arrays.
- Response `result` field is always an object; single-scalar responses wrap in `{"value": <scalar>}`.
- Notifications (fire-and-forget) tag `"method": "v1.x.y"` with `"id"` omitted per JSON-RPC 2.0.

**Timestamps on the wire:**

- User-facing: ISO 8601 UTC string (`2026-04-22T13:47:00Z`). Example: `created_at`, `expires_at`.
- Internal monotonic: nanosecond integer (`i64`). Example: `timestamp_ns`.
- Never epoch seconds/millis on the wire.

**Optional fields:** `Option<T>` in Rust with `#[serde(skip_serializing_if = "Option::is_none")]`. Absent = None / undefined. `null` reserved for "explicitly cleared" semantics.

### Time & ID Format Reference

| Purpose | Format | Example | Rationale |
|---|---|---|---|
| User-facing timestamp | ISO 8601 UTC string | `2026-04-22T13:47:00Z` | Human-readable |
| Internal monotonic time | nanosecond `i64` via `Clock` trait | `1713789420123456789` | Ordering-safe, no skew |
| Audit log ordering | `seq` integer (DB-assigned) | `42` | Single-writer total order (D-CLK-1) |
| Finding ID | `ERR-NNN` / `ERR-PRE-NNN` | `ERR-001`, `ERR-PRE-042` | Stable, greppable |
| Action ID (gate eval) | UUIDv7 string | `018f23a1-b2c3-7d4e-8f5a-6b7c8d9e0f1a` | Time-ordered sortable |
| Consent token | `cnst_` + 16 hex | `cnst_0x8f9a12b3c4d5e6f7` | Recognizable prefix |
| Hash | lowercase hex | SHA-256 64 chars (12 for short-prefix refs) | — |

### Test Organization Patterns

- **Unit tests:** `#[cfg(test)] mod tests { use super::*; ... }` at the bottom of each file. Shared helpers in `crates/<crate>/src/test_helpers.rs` gated by `#[cfg(any(test, feature = "test-helpers"))]`.
- **Integration tests:** `crates/<crate>/tests/*.rs`; file name describes scenario.
- **Cross-crate integration:** workspace-level `tests/` directory, `aotf-integration-tests` crate.
- **Fault-injection:** `aotf-test-harness` crate with helpers (`kill_at_point`, `drop_fsync`, `corrupt_row`). Attribute: `#[aotf_test_harness::fault_test]`.
- **Fixture corpus:** `fixtures/finding-v1/*.json`, `fixtures/audit-chain/*.jsonl`, `fixtures/plugin-manifest/*.toml`.
- **Golden tests:** `cargo insta` for snapshot stability on IPC messages and audit entries.
- **Mutation tests (FR-105.2):** `cargo mutants --file crates/aotf-gatekeeper/src/gate.rs` against the three functions in `mutants.toml` (D-ENF-3).
- **TypeScript tests:** `bun test`, `*.test.ts` co-located with source.

### Commit & Branch Discipline (from CLAUDE.md)

- Commit message format: Conventional style — `type(scope): subject`. Types: `chore`, `feat`, `fix`, `docs`, `refactor`, `test`, `build`, `ci`, `perf`.
- Subject: imperative mood, under 72 chars, no trailing period.
- Body: explains *why*, not *what*. Why includes specific incident/constraint, FR/NFR/ADR being satisfied, tradeoff accepted.
- Every PR references at least one: story ID (`AOTF-NNN`), PRD FR/NFR ID, or ADR number.
- Branch names: `{type}/{short-desc}` — example `feat/gate-invariant-test`, `fix/audit-chain-recovery`.

### Enforcement Guidelines (three-layer model, revised post-Party-Mode: 10 → 4 mandatory)

AOTF patterns are enforced in three layers. The type system and build system do the heavy lifting; explicit guidelines are reserved for what cannot be mechanically enforced.

**Layer 1 — Design-enforced (no explicit rule needed; code design makes violation impossible or a compile/build error):**

- **Only path to `EXECUTE_*` actions is `gated(...)` scope helper.** Backed by: `pub(crate)` on the underlying gate IPC client; the only exported API for mutating operations is the scope helper.
- **Only writer to `audit_log` is `aotf-gatekeeper::audit::Writer`.** Backed by: `pub(crate)` Rust visibility + SQLite BEFORE UPDATE/DELETE triggers + per-process connection model (D-DB-3). Verified by API-surface test in `tests/audit_invariant.rs`.
- **Bounded channels only.** Backed by: `Backpressure` enum has no `Unbounded` variant; calls to `tokio::sync::mpsc::unbounded_channel` caught by CI grep (D-IPC-4).
- **No handwritten TS types across IPC.** Backed by: build step `cargo run --bin gen-ts-types`; CI fails if generated file is stale (`git diff --exit-code packages/aotf-agent/src/generated/`).
- **IPC method registry.** Backed by: `inventory`-macro auto-registration at compile time; referencing an unregistered method is a compile error via the typed method-handle pattern.

**Layer 2 — Explicit mandatory guidelines (4, cannot be fully mechanically enforced):**

1. **Route every `EXECUTE_REVERSIBLE` or `EXECUTE_IRREVERSIBLE` operation through `gated(...)`.** The scope helper enforces the preceding `gate.evaluate` + trailing `gate.record_outcome` call pair with a single `action_id`. Verified by `tests/gate_invariant.rs` (MockGatekeeper asserts matched pairs).
2. **Use monotonic `timestamp_ns` via the `Clock` trait** for any ordering decision; wall-clock only for display, TTL, and serialization to external formats. Concrete: `aotf-core-types::Clock` trait, production impl uses `CLOCK_MONOTONIC_RAW` (Linux) / `mach_absolute_time` (macOS), tests inject `TestClock`. No direct `std::time::SystemTime::now()` in `aotf-gatekeeper` or `aotf-storage` — CI enforces via clippy `disallowed_methods`.
3. **Add every new gate allow/deny branch to `crates/aotf-gatekeeper/mutants.toml` in the same PR.** Enforced by CI check: `cargo mutants --list` must include every function in `src/gate.rs` that has conditional logic. Missing entries fail the PR check.
4. **PII discipline in logs.** No file contents, error message text, environment variables, API keys, or user URLs at INFO level or above. TRACE-level exposure tags with `pii_visible=true`. Reviewed in PR; partial clippy enforcement via `disallowed_types` (e.g., deny `std::env::vars()` in `aotfd`/`aotf-gatekeeper` unless explicitly skipped).

**Layer 3 — Review-caught (not gate-blocking in CI):**

- `thiserror` in libs, `anyhow::Result + .context()` in bins; no `.unwrap()` outside tests — `clippy::unwrap_used` + `clippy::expect_used` deny, but merges with `#[allow(...)]` annotations are permitted with reviewer justification.
- `tracing` with structured fields, not format strings — reviewed in PR.
- One-line invariant comments where the WHY is non-obvious — reviewed in PR.
- Commit message Conventional style — `commitlint` in pre-commit (review-caught, not CI-gating).

**Verification gates (real vs theater):**

| Gate | Catches | Classification |
|---|---|---|
| `tests/gate_invariant.rs` (MockGatekeeper matched-pair assertion) | Layer-2 guideline #1 violations | Real |
| `tests/audit_invariant.rs` (type-level API-surface reflection test) | Layer-1 audit-writer invariant | Real (only if written as API-surface, not behavior-observation) |
| `cargo mutants --file crates/aotf-gatekeeper/src/gate.rs` (weekly CI) | Gate logic weakening (AC FR-105.2) | Real |
| `tier_map` coverage test | Missing ActionTier classification | Real |
| CI grep for `unbounded_channel` / direct unbounded-channel constructors | Direct bypass of Backpressure | Partial (catches common-case; determined re-export bypass escapes) |
| `cargo clippy --workspace -- -D warnings` with `unwrap_used`, `expect_used`, `disallowed_methods` | Style + forbidden API calls | Real at workspace level |
| `gen-ts-types` freshness check | Rust↔TS type drift | Real |

**Deleted guidelines** (behavior preserved via design, explicit rule removed):

- ~~Never write directly to `audit_log`~~ → `pub(crate)` + triggers enforce (Layer 1).
- ~~Use Backpressure enum~~ → enum has no Unbounded variant (Layer 1).
- ~~Register IPC methods~~ → `inventory` macro auto-registers (Layer 1).
- ~~Document non-obvious invariants in a one-line comment~~ → review-caught taste (Layer 3).
- ~~Commit message format as explicit guideline~~ → `commitlint` + review (Layer 3).

### Anti-Patterns (rejected)

**Direct audit write:**

```rust
// WRONG — bypasses gatekeeper, violates NFR-S06
storage::audit_log::insert(AuditEntry { ... })?;
```

**Unbounded channel:**

```rust
// WRONG — CI gate rejects this
let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Finding>();
// RIGHT
let (tx, rx) = backpressure::channel::<Finding>(Backpressure::DropOldest, 100);
```

**Handwritten TS type crossing IPC:**

```typescript
// WRONG — diverges from Rust source of truth
interface Finding { id: string; confidence: number; /* ... */ }
// RIGHT
import type { Finding } from './generated/types';
```

**Silent error swallow:**

```rust
// WRONG — error dropped, no audit, no signal
let _ = send_telegram(message).await;
// RIGHT
match send_telegram(message).await {
    Ok(()) => {},
    Err(e) => {
        tracing::warn!(error = %e, "telegram delivery failed");
        audit_dropped_notification(message_id, &e).await?;
    }
}
```

**Wall-clock for ordering:**

```rust
// WRONG — skew can silently reorder
entries.sort_by_key(|e| e.created_at);  // wall-clock ISO string
// RIGHT
entries.sort_by_key(|e| e.seq);  // single-writer autoincrement, total order
```

**Direct `SystemTime::now()` in gate/audit path:**

```rust
// WRONG — not injectable, tests flake
let now = std::time::SystemTime::now();
// RIGHT — use the injected Clock trait
let now = self.clock.now_ns();
```

**Good example — fully gated irreversible action:**

```rust
pub async fn apply_fix(finding: &Finding, diff: &Diff, approver: UserId) -> Result<PrUrl, GateError> {
    gated(
        Action::ApplyFix { finding_id: finding.id.clone() },
        ActionContext::new().with_approver(approver),
        |action_id| Box::pin(async move {
            let pr = git_ops::create_fix_pr(finding, diff).await?;
            tracing::info!(finding_id = %finding.id, action_id = %action_id, pr_url = %pr.url, "fix applied");
            Ok(pr.url)
        }),
    ).await
}
```

## Project Structure & Boundaries

### Complete Project Directory Structure

```
aotf/
├── README.md                             # Project overview; links platform-matrix + security
├── LICENSE                               # Apache-2.0
├── SECURITY.md                           # Vuln disclosure + platform-gap summary
├── CONTRIBUTING.md                       # Contributor onboarding; bootstrap.sh pointer
├── CLAUDE.md                             # Existing — Claude-specific behavioral guidelines
├── CHANGELOG.md                          # Keep-a-changelog; SQLite migration notes
├── Cargo.toml                            # Workspace: resolver=2, shared [workspace.dependencies]
├── Cargo.lock                            # Committed (SLSA reproducibility)
├── rust-toolchain.toml                   # stable, rust-version pin (1.80 floor)
├── clippy.toml                           # disallowed_methods, disallowed_types config
├── rustfmt.toml                          # edition = 2024, max_width = 100
├── deny.toml                             # cargo-deny: licenses, advisories, bans, layering
├── package.json                          # Root Bun package (turborepo + gen-ts-types runner)
├── bun.lockb                             # Committed
├── turbo.json                            # cargo:build + bun:build + build pipelines
├── flake.nix                             # Nix dev env (alt: .tool-versions)
├── .tool-versions                        # asdf fallback: rust, bun, node
├── .gitignore                            # target/, node_modules/, dist/, .aotf/, tmp/
├── .env.example                          # DAEMON_LOG, WEBAUTHN_RP_ID, GITHUB_APP_ID, ...
│
├── crates/                               # Rust workspace members
│   │
│   ├── aotf-cli/                         # Binary: user-facing CLI
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                   # clap parser; dispatches to commands/
│   │   │   ├── commands/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── init.rs               # FR-01, FR-02, FR-97, FR-99 — first-run wizard
│   │   │   │   ├── watch.rs              # FR-09 — start/attach daemon
│   │   │   │   ├── diagnose.rs           # FR-19 — on-demand diagnosis
│   │   │   │   ├── fix.rs                # FR-29, FR-30 — interactive approval UX
│   │   │   │   ├── rollback.rs           # FR-29
│   │   │   │   ├── consent.rs            # FR-106 — aotf consent issue
│   │   │   │   ├── plugin.rs             # FR-54-59, FR-96 — trust/list/dev/test
│   │   │   │   ├── telemetry.rs          # FR-07, NFR-S09
│   │   │   │   ├── user.rs               # FR-49 (admin), FR-67 — audit query
│   │   │   │   ├── doctor.rs             # NFR-O02 — platform matrix check (AC-PLAT-2)
│   │   │   │   └── version.rs            # AC-PLAT-1 — capability_tier tag
│   │   │   ├── ipc_client.rs             # Thin client to aotfd over Unix socket
│   │   │   ├── output/                   # Human + JSON-lines renderers (D-IPC-3)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── human.rs              # Ratatui-based interactive prompts
│   │   │   │   └── json.rs               # --output json renderer
│   │   │   ├── ratatui_widgets/          # Per UX design spec §CLI-Specific Ratatui Widgets
│   │   │   │   ├── mod.rs
│   │   │   │   ├── finding_card.rs
│   │   │   │   ├── tier_tag.rs
│   │   │   │   ├── consent_ceremony.rs   # FR-106 WebAuthn touch UX
│   │   │   │   └── command_palette.rs
│   │   │   └── bootstrap.rs              # Extracts bundled TS agent to ~/.aotf/agent/ on first run
│   │   └── tests/
│   │       ├── commands/                 # Integration tests per command
│   │       └── platform_matrix.rs        # AC-PLAT-1/2/3 cross-platform assertions
│   │
│   ├── aotfd/                            # Binary: daemon (FR-09-16, plugin host, orchestrator)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs                   # Startup sequence (D-IPC-5), pid lock, sig handlers
│   │   │   ├── ipc/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── cli_server.rs         # Serves aotfd.sock to aotf-cli
│   │   │   │   ├── gatekeeper_client.rs  # Client to gatekeeper.sock
│   │   │   │   └── agent_server.rs       # Serves agent.sock to aotf-agent (Bun)
│   │   │   ├── watchers/                 # FR-09, FR-15
│   │   │   │   ├── mod.rs
│   │   │   │   ├── file.rs               # notify-crate file tail
│   │   │   │   ├── docker.rs             # Docker SDK stdout/stderr
│   │   │   │   └── polling.rs            # Stub for Growth (Loki/Datadog/CW) — v0.2+
│   │   │   ├── dedup/                    # FR-10, FR-11 — 90s window, 3/90s trigger
│   │   │   │   ├── mod.rs
│   │   │   │   └── hash.rs               # Error-hash canonicalization
│   │   │   ├── gate_scope.rs             # `gated(...)` helper (Patterns Layer-2 #1)
│   │   │   ├── fix/                      # FR-23-36 (orchestration; gate-enforced)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── orchestrator.rs       # Five-gate pipeline caller
│   │   │   │   ├── scope_limit.rs        # FR-25 blast radius
│   │   │   │   ├── mutex.rs              # FR-27 per-repo fix mutex
│   │   │   │   └── rollback_ref.rs       # FR-26 — coordinates with aotf-gatekeeper
│   │   │   ├── pre_push/                 # FR-14 + J11 — pre-push risk assessment
│   │   │   │   ├── mod.rs
│   │   │   │   └── hook_integration.rs   # git2-based hook install + invocation
│   │   │   ├── plugin_host/              # FR-54-60, FR-96, FR-104, NFR-S02, NFR-S08
│   │   │   │   ├── mod.rs
│   │   │   │   ├── loader.rs             # SHA-256 verify + capability grant load
│   │   │   │   ├── seccomp.rs            # Linux seccomp-bpf filter construction (unsafe-allowed)
│   │   │   │   ├── import_hook.rs        # macOS advisory import-hook
│   │   │   │   └── degraded_state.rs     # FR-60 — degraded-mode semantics
│   │   │   ├── notification_bus/         # FR-43, FR-48 — routes to aotf-agent adapters
│   │   │   │   └── mod.rs
│   │   │   ├── recovery.rs               # FR-34, NFR-R04 — in-flight state detector
│   │   │   ├── config/                   # FR-03, FR-04 — config resolver
│   │   │   │   ├── mod.rs
│   │   │   │   ├── schema.rs             # .aotf/config.yaml serde types
│   │   │   │   └── env_override.rs       # AOTF_* env var overrides
│   │   │   └── credentials.rs            # NFR-S01 keychain/libsecret/SOPS fallback
│   │   └── tests/
│   │       ├── startup_integrity.rs      # NFR-R04 — unclean shutdown recovery
│   │       ├── backpressure.rs           # D-IPC-4 policy tests
│   │       └── dedup_window.rs           # 90s dedup behavior
│   │
│   ├── aotf-gatekeeper/                  # Binary: enforcement core (D-ENF, NFR-S10)
│   │   ├── Cargo.toml                    # [features] test-helpers = [] for MockGatekeeper
│   │   ├── mutants.toml                  # Layer-2 guideline #3 — mutation targets
│   │   ├── src/
│   │   │   ├── main.rs                   # seccomp self-confinement (Linux), POSIX drop (both)
│   │   │   ├── ipc_server.rs             # Serves gatekeeper.sock — D-ENF-2 7-method surface
│   │   │   ├── tier_map.rs               # D-ENF-1 — SSoT action→tier table
│   │   │   ├── gate.rs                   # D-ENF-3 — classify/evaluate_ceiling/require_consent
│   │   │   ├── audit/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── writer.rs             # pub(crate) Writer — Layer-1 audit-writer invariant
│   │   │   │   ├── chain.rs              # SHA-256 hash chain; verify on startup
│   │   │   │   └── fields.rs             # FR-66 six-field IRREVERSIBLE validator
│   │   │   ├── consent/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── issuer.rs             # FR-106 — single-use, 1h TTL, WebAuthn-attested
│   │   │   │   ├── webauthn.rs           # webauthn-rs wrapper
│   │   │   │   └── ledger.rs             # consent_tokens table writes
│   │   │   ├── capability_tier.rs        # AC-PLAT-1 — Enforced | Isolated
│   │   │   ├── capability/               # Capability dispatch to plugin_host
│   │   │   │   └── mod.rs
│   │   │   └── test_helpers.rs           # MockGatekeeper — gated by feature = "test-helpers"
│   │   │                                 #                  consumed via dev-deps only
│   │   └── tests/
│   │       ├── gate_invariant.rs         # AC FR-105.1 + #1 gated-scope test
│   │       ├── audit_invariant.rs        # API-surface reflection test (NFR-S06)
│   │       ├── audit_chain_recovery.rs   # SIGKILL mid-chain recovery
│   │       ├── irrev_six_fields.rs       # FR-66 blocker test
│   │       └── platform_parity.rs        # AC-PLAT-3 — IPC byte-identical
│   │
│   ├── aotf-mcp-server/                  # Binary: MCP server (FR-65, D-MCP-1) — v1.0 read-only
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── tools.rs                  # get_risk_score, get_finding, list_recent_runs, get_daemon_status
│   │   │   ├── session.rs                # MCP invocation token (FR-107c) handshake
│   │   │   └── daemon_client.rs          # Talks to aotfd via cli_server IPC
│   │   └── tests/
│   │
│   ├── aotf-storage/                     # Library: SQLite+WAL, append-only audit impl (D-DB-*)
│   │   ├── Cargo.toml
│   │   ├── migrations/
│   │   │   ├── 0001_initial_schema.sql
│   │   │   ├── 0002_audit_triggers.sql   # BEFORE UPDATE/DELETE = ABORT
│   │   │   ├── 0003_hash_chain.sql
│   │   │   └── …
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── schema.rs                 # D-DB-2 — all 10 tables typed
│   │   │   ├── findings.rs               # FR-12, FR-13
│   │   │   ├── audit_log.rs              # Write impl (pub(crate) upward)
│   │   │   ├── decision_log.rs           # FR-20, FR-36
│   │   │   ├── ai_traces.rs              # Blob storage, content-addressed
│   │   │   ├── rollback_refs.rs          # FR-26
│   │   │   ├── tokens.rs                 # user_tokens, consent_tokens, mcp_sessions
│   │   │   ├── plugins.rs                # Plugin registry persistence
│   │   │   ├── dedup.rs                  # findings_dedup
│   │   │   ├── wal.rs                    # PRAGMA wal_checkpoint timer
│   │   │   └── migrations.rs             # Migration runner, version table
│   │   └── tests/
│   │       ├── append_only.rs            # Negative test — UPDATE/DELETE rejected
│   │       ├── wal_durability.rs         # NFR-R02 — SIGKILL + replay (uses aotf-fault)
│   │       └── chain_integrity.rs
│   │
│   ├── aotf-ipc-protocol/                # Library: JSON-RPC schemas + Finding v1 wire struct
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── finding_v1.rs             # Frozen schema per PRD §Finding Schema
│   │   │   ├── methods.rs                # inventory-macro method registry
│   │   │   ├── framing.rs                # D-IPC-3 length-delimited u32-be + JSON
│   │   │   ├── error_codes.rs            # -32001..-32010 IpcErrorCode enum
│   │   │   ├── channels/
│   │   │   │   ├── cli_daemon.rs         # v1.finding.*, v1.config.*, v1.user.audit, ...
│   │   │   │   ├── daemon_gatekeeper.rs  # v1.gate.*, v1.audit.*, v1.consent.*
│   │   │   │   └── daemon_agent.rs       # v1.diagnosis.*, v1.goal_loop.*, …
│   │   │   └── versioning.rs             # D-IPC-2 namespace semver
│   │   ├── bin/
│   │   │   └── gen-ts-types.rs           # ts-rs emit into packages/aotf-agent/src/generated/
│   │   └── tests/
│   │       └── round_trip.rs             # Finding v1 Rust ↔ Bun byte-equal golden
│   │
│   ├── aotf-plugin-api/                  # Library: plugin ABCs + capability model (FR-54-60)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── log_watcher.rs            # LogWatcherABC equivalent trait
│   │   │   ├── notification_adapter.rs   # NotificationAdapterABC
│   │   │   ├── finding_emitter.rs
│   │   │   ├── manifest.rs               # aotf-plugin.toml schema + actionTier ceiling
│   │   │   └── capability.rs             # Filesystem/network/git capability declarations
│   │   └── tests/
│   │       └── conformance.rs            # FR-58 — `aotf plugin test` harness logic
│   │
│   ├── aotf-core-types/                  # Library: shared domain types (Finding, ActionTier, Clock)
│   │   ├── Cargo.toml                    # [features] test-helpers = [] for TestClock
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── finding.rs                # Domain type (wire type is in aotf-ipc-protocol)
│   │       ├── action_tier.rs            # FR-105 enum
│   │       ├── confidence.rs             # FR-18 formula
│   │       ├── severity.rs
│   │       ├── user_id.rs
│   │       ├── action_id.rs              # UUIDv7 newtype
│   │       ├── backpressure.rs           # D-IPC-4 enum + apply()
│   │       ├── clock.rs                  # Layer-2 #2 — Clock trait + monotonic prod impls
│   │       └── test_clock.rs             # TestClock — gated by feature = "test-helpers"
│   │
│   ├── aotf-test-macros/                 # proc-macro-only (Rust-required for #[fault_test])
│   │   ├── Cargo.toml                    # [lib] proc-macro = true
│   │   └── src/
│   │       └── lib.rs                    # #[fault_test] proc macro definition
│   │
│   └── aotf-fault/                       # Test-only: unsafe fault-injection (#![deny(...)] ceiling)
│       ├── Cargo.toml                    # The ONLY non-production audit-allowed unsafe crate
│       ├── src/
│       │   ├── lib.rs                    # #![deny(unsafe_op_in_unsafe_fn, missing_docs)]
│       │   ├── kill_point.rs             # Inserts deterministic SIGKILL points
│       │   ├── drop_fsync.rs             # Test-only fsync interception
│       │   └── corrupt_row.rs            # SQLite row-corruption injection for chain-integrity tests
│       └── tests/
│
├── packages/                             # Bun workspace members
│   └── aotf-agent/
│       ├── package.json
│       ├── tsconfig.json
│       ├── biome.jsonc                   # Linter/formatter config (or eslint/prettier)
│       ├── src/
│       │   ├── index.ts                  # Entry — connects to agent.sock
│       │   ├── ipc/
│       │   │   ├── daemonClient.ts       # JSON-RPC client to aotfd
│       │   │   └── schema.ts             # Re-exports from generated/
│       │   ├── generated/                # Rust-generated types (DO NOT HANDWRITE)
│       │   │   ├── finding.ts
│       │   │   ├── actionTier.ts
│       │   │   └── …
│       │   ├── intelligence/
│       │   │   ├── diagnosis.ts          # FR-17 — AI root-cause
│       │   │   ├── confidence.ts         # FR-18 — formula evaluator
│       │   │   └── reasoningTrace.ts     # FR-20, FR-68 PII-filter pre-store
│       │   ├── aiCli/
│       │   │   ├── backend.ts            # FR-61 pluggable interface
│       │   │   ├── claudeCode.ts         # v1.0 default
│       │   │   ├── codex.ts              # Stub — Growth
│       │   │   ├── gemini.ts             # Stub — Growth
│       │   │   └── subprocessManager.ts  # FR-62 version check, FR-64 concurrent cap
│       │   ├── goalLoop/                 # FR-37-42
│       │   │   ├── runtime.ts
│       │   │   ├── worktreeIsolation.ts  # FR-38
│       │   │   ├── ciFeedback.ts         # FR-39
│       │   │   ├── ceilings.ts           # FR-40 attempts/time/cost
│       │   │   └── summary.ts            # FR-41
│       │   ├── mcp/                      # MCP tool implementations (thin layer over daemonClient)
│       │   │   ├── tools.ts
│       │   │   └── session.ts
│       │   ├── adapters/
│       │   │   ├── telegram/
│       │   │   │   ├── index.ts          # AotfChannelEvent bus
│       │   │   │   ├── officialPlugin.ts # claude-plugins-official adapter (FR-45)
│       │   │   │   └── claudeBridge.ts   # claude-bridge adapter (FR-46)
│       │   │   ├── slack/                # Stub — Growth FR-53
│       │   │   ├── pagerDuty/            # Stub — Growth FR-53
│       │   │   ├── teams/                # Stub — Growth FR-53
│       │   │   └── email/                # Stub — Growth FR-53
│       │   ├── ci/
│       │   │   ├── githubActions.ts      # FR-32 PR creation + CI-pass gate
│       │   │   └── githubWebhooks.ts     # HMAC validation
│       │   ├── observability/
│       │   │   └── logger.ts             # Structured-field wrapper around console
│       │   ├── qa/                       # Stub only v1.0 — Growth FR-71-75
│       │   │   └── README.md
│       │   └── agentOps/                 # Stub only v1.0 — Growth FR-76-80
│       │       └── README.md
│       └── tests/                        # bun test, co-located *.test.ts + scenario files
│           ├── diagnosis.test.ts
│           ├── confidence.test.ts
│           ├── goalLoop.test.ts
│           └── ipcRoundTrip.test.ts
│
├── fixtures/
│   ├── finding-v1/
│   │   ├── minimal.json
│   │   ├── irreversible.json
│   │   ├── with-diff.json
│   │   └── …
│   ├── audit-chain/
│   │   └── valid-chain.jsonl
│   ├── plugin-manifest/
│   │   ├── v1.0-grace-no-action-tier.toml
│   │   └── v1.1-strict.toml
│   └── consent-token/
│       └── webauthn-assertion-sample.bin
│
├── docs/
│   ├── architecture.md                   # This document
│   ├── security/
│   │   ├── platform-matrix.md            # AC-PLAT-4 — X3 cross-platform posture
│   │   ├── threat-model.md               # Formal STRIDE for NFR-S10 core
│   │   └── disclosure.md
│   ├── plugin-development-guide.md       # Stakeholder-lens #1 (plugin authors)
│   ├── adr/                              # Architecture Decision Records
│   │   ├── 0001-enforcement-core-placement.md     # Option B / X3
│   │   ├── 0002-dual-telegram-transports.md
│   │   ├── 0003-backpressure-two-policies.md
│   │   ├── 0004-clock-trait-determinism.md
│   │   ├── 0005-test-organization-fault-isolation.md
│   │   └── …
│   ├── ux-research-inputs.md             # Existing
│   ├── PRD.md                            # Existing
│   └── epics.md                          # Existing (deprecated, to be regenerated)
│
├── scripts/
│   ├── bootstrap.sh                      # Contributor onboarding (<2min to green test)
│   ├── gen-ts-types.sh                   # Thin wrapper around cargo run --bin gen-ts-types
│   ├── run-mutants.sh                    # Weekly cargo mutants invocation
│   ├── check-no-test-features.sh         # CI gate — verifies no test-helpers feature in release
│   └── install.sh                        # Published to install.aotf.dev (gh-pages)
│
├── tests/                                # Workspace-level cross-crate integration tests
│   ├── Cargo.toml                        # aotf-integration-tests crate
│   └── src/
│       ├── common/
│       │   └── mod.rs                    # fixtures::load() — ~20 lines, shared helper
│       ├── end_to_end_fix.rs             # J1 full-path integration
│       ├── pre_push_consent.rs           # J11 consent-token-gated push
│       └── goal_loop_convergence.rs      # J7 multi-attempt scenario
│
└── .github/
    ├── CODEOWNERS
    ├── pull_request_template.md          # Checklist: gate discipline, IPC registry, mutants.toml
    ├── dependabot.yml
    └── workflows/
        ├── ci.yml                        # cargo test + clippy + fmt + audit + bun test + audit
        │                                 # + unbounded-channel grep + gen-ts-types freshness
        │                                 # + tier_map coverage + check-no-test-features.sh
        ├── mutants.yml                   # Weekly cargo mutants (expensive)
        ├── release.yml                   # SLSA provenance, signed binaries, brew auto-PR
        ├── codeql.yml
        └── nightly.yml                   # Long-running fault-injection suite
```

### Architectural Boundaries

**Process boundaries (runtime):**

| Boundary | Mechanism | Invariant |
|---|---|---|
| `aotf-cli` ↔ `aotfd` | Unix socket `aotfd.sock`, JSON-RPC 2.0 | Short-lived client; daemon owns long-term state |
| `aotfd` ↔ `aotf-gatekeeper` | Unix socket `gatekeeper.sock`, JSON-RPC 2.0 | Gatekeeper owns audit log and consent ledger; daemon cannot write audit directly |
| `aotfd` ↔ `aotf-agent` (Bun) | Unix socket `agent.sock`, JSON-RPC 2.0 | Agent runs as subprocess of daemon; no shared filesystem outside `~/.aotf/agent/` |
| `aotfd` ↔ `aotf-mcp-server` | Child process spawn; named session token | MCP reads only; write path not implemented v1.0 |
| `aotfd` ↔ plugins (loaded in `aotfd` address space) | seccomp-bpf (Linux) / import-hook (macOS); capability grants | Plugin crash ≠ daemon crash (NFR-R03); plugin tier-violation → termination + audit |
| `aotf-agent` ↔ AI CLI subprocess | `child_process.spawn`; subprocess stdin/stdout | Subprocess dies → fallback to detection-only (FR-63, NFR-R01) |
| `aotf-agent` ↔ Goal Loop worktree agents | git worktree + child Claude Code processes | Filesystem + secret isolation per PRD §Worktree Isolation Contract |

**Crate dependency boundaries (compile-time):**

Enforced via `deny.toml` `[[bans]]` and `[workspace.dependencies]` audit:

```
aotf-core-types         (root — depends on nothing AOTF-internal)
    ↑
aotf-ipc-protocol       (depends on aotf-core-types; exposes wire types)
    ↑
aotf-storage            (depends on aotf-ipc-protocol, aotf-core-types)
    ↑
aotf-plugin-api         (depends on aotf-core-types; NEVER on aotf-gatekeeper or aotfd)
    ↑
aotf-gatekeeper         (depends on aotf-ipc-protocol, aotf-storage, aotf-core-types;
                         NEVER depended on by aotfd — gatekeeper serves, daemon calls)
    ↑
aotf-mcp-server         (depends on aotf-ipc-protocol, aotf-core-types)

aotfd                   (depends on aotf-ipc-protocol, aotf-storage, aotf-plugin-api,
                         aotf-core-types; calls aotf-gatekeeper via IPC only)

aotf-cli                (depends on aotf-ipc-protocol, aotf-core-types; calls aotfd via IPC)

aotf-test-macros        (proc-macro crate; depended on by aotf-fault and any crate using #[fault_test])

aotf-fault              (test-only crate; the only audit-allowlisted location for `unsafe` outside
                         production crates; depends on aotf-core-types, aotf-test-macros)
```

Production crates set `#![forbid(unsafe_code)]` except where explicitly justified (`aotf-gatekeeper` for seccomp-bpf, others must be empty). Circular deps forbidden; layering violations caught by `cargo-deny check bans`.

**Test-feature isolation (CI gate):** `cargo build --release --workspace` resolves zero `test-helpers` features. Verified via `cargo metadata --format-version 1` parsed by `scripts/check-no-test-features.sh`. This catches any production crate that transitively enables a `test-helpers` feature in a release build.

**Data boundaries (SQLite access model, per D-DB-3):**

| Process | `audit_log` | `consent_tokens` | `decision_log` | `findings` | `plugins` | `user_tokens` | others |
|---|---|---|---|---|---|---|---|
| `aotf-gatekeeper` | R/W (sole writer) | R/W (sole writer) | R/W (IRREV entries) | R | R | R | R |
| `aotfd` | R | R | R/W (non-IRREV) | R/W | R/W | R/W | R/W |
| `aotf-cli` | R (read-only URI) | R (masked) | R | R | R | R (masked) | R |
| `aotf-mcp-server` | R | — | — | R | — | — | R (via `aotfd`) |

Enforcement: SQLite `?mode=ro` URI + per-process SQLite connection owner + BEFORE UPDATE/DELETE triggers + `pub(crate)` Rust visibility.

### Requirements-to-Structure Mapping

**FR category → primary crate/directory:**

| FR Category (from PRD) | FRs | Primary location |
|---|---|---|
| Project Init & Config | FR-01-08, 97, 99 | `crates/aotf-cli/src/commands/init.rs` + `crates/aotfd/src/config/` |
| Log Monitoring & Detection | FR-09-16 | `crates/aotfd/src/watchers/`, `crates/aotfd/src/dedup/` |
| AI Diagnosis | FR-17-22, 92 | `packages/aotf-agent/src/intelligence/` |
| Autonomous Fix & Safety | FR-23-36, 93, 94, **105, 106, 107** | `crates/aotf-gatekeeper/*` + `crates/aotfd/src/fix/` + `crates/aotfd/src/gate_scope.rs` |
| Goal Loop | FR-37-42 | `packages/aotf-agent/src/goalLoop/` |
| Notifications & Coordination | FR-43-53, 103 | `packages/aotf-agent/src/adapters/` |
| Plugin Ecosystem | FR-54-60, 96, 104 | `crates/aotf-plugin-api/` + `crates/aotfd/src/plugin_host/` |
| AI Backend Management | FR-61-65, 95 | `packages/aotf-agent/src/aiCli/` + `crates/aotf-mcp-server/` |
| Observability & Audit | FR-66-70, 90, 91 | `crates/aotf-gatekeeper/src/audit/` + `crates/aotf-storage/src/audit_log.rs` |
| AI QA Agent (Growth) | FR-71-75 | `packages/aotf-agent/src/qa/` (stub in v1.0) |
| AI Agent Operations (Growth) | FR-76-80 | `packages/aotf-agent/src/agentOps/` (stub in v1.0) |
| Dashboard (Growth) | FR-81-84, 98 | Not in v1.0 — future `crates/aotf-dashboard/` |
| Security Automation (Growth) | FR-100, 101 | Future `crates/aotfd/src/security_automation/` |
| ML/LLM Lifecycle (Vision) | FR-85-89 | Post-v1.0 — new crates |
| Ecosystem (Vision) | FR-102 | Post-v1.0 |

**Cross-cutting concerns (from step-02) → specific locations:**

| Concern | Location |
|---|---|
| A1 Action-tier taxonomy | `crates/aotf-gatekeeper/src/tier_map.rs` + `crates/aotf-core-types/src/action_tier.rs` |
| A2 Mutation-target manifest | `crates/aotf-gatekeeper/mutants.toml` + `crates/aotf-gatekeeper/src/gate.rs` |
| A3 Three-token authorization | `crates/aotf-gatekeeper/src/consent/` + `crates/aotf-storage/src/tokens.rs` + `crates/aotf-mcp-server/src/session.rs` |
| A4 Audit log (FR-66, NFR-S06) | `crates/aotf-gatekeeper/src/audit/` + `crates/aotf-storage/src/audit_log.rs` |
| B5 Rust↔Bun IPC contract | `crates/aotf-ipc-protocol/` + `fixtures/finding-v1/` |
| B6 Clock & ordering semantics | `crates/aotf-core-types/src/clock.rs` + `crates/aotf-gatekeeper/src/audit/chain.rs` |
| B7 Backpressure policy | `crates/aotf-core-types/src/backpressure.rs` + usage sites per D-IPC-4 |
| B8 Transactional boundary design | `crates/aotf-gatekeeper/src/audit/writer.rs` + `crates/aotfd/src/fix/rollback_ref.rs` + `crates/aotfd/src/recovery.rs` |
| B9 Degraded-mode signaling | `crates/aotfd/src/plugin_host/degraded_state.rs` + `packages/aotf-agent/src/aiCli/subprocessManager.ts` |
| B10 Confidence formula | `crates/aotf-core-types/src/confidence.rs` + `packages/aotf-agent/src/intelligence/confidence.ts` |
| C11 External-surface trust boundary | `packages/aotf-agent/src/ci/githubWebhooks.ts` + `packages/aotf-agent/src/adapters/telegram/*` |
| D12 Override/rollback UX surface | `crates/aotf-cli/src/commands/fix.rs` + `crates/aotf-cli/src/commands/rollback.rs` + `crates/aotf-cli/src/ratatui_widgets/consent_ceremony.rs` + `packages/aotf-agent/src/adapters/telegram/*` |

**User journeys → integration test files:**

| Journey | Test file |
|---|---|
| J1 Alex — First detection to fixed PR | `tests/src/end_to_end_fix.rs` |
| J2 Priya — 3am Telegram approval | `tests/src/telegram_approval.rs` (to be added) |
| J6 Alex — Rollback after bad fix | `tests/src/rollback_failure_path.rs` (to be added) |
| J7 Marcus — Goal Loop | `tests/src/goal_loop_convergence.rs` |
| J8 On-Call Team — Telegram triage | `tests/src/group_chat_approval.rs` (to be added) |
| J11 Priya — Pre-push consent | `tests/src/pre_push_consent.rs` |

### Integration Points

**Internal communication:**

- **Rust↔Rust (3 channels):** all JSON-RPC 2.0 over Unix domain sockets, shared types in `aotf-ipc-protocol`.
- **Rust↔Bun (1 channel):** same JSON-RPC 2.0 + Unix socket; types generated from Rust via `cargo run --bin gen-ts-types` into `packages/aotf-agent/src/generated/`. No hand-written TS types.
- **Bun↔AI CLI subprocess:** `child_process.spawn` with stdin/stdout; output parsed via versioned schema conformance check (NFR-I03).
- **Event bus:** `AotfChannelEvent` type in `aotf-agent/src/adapters/telegram/index.ts` — unifies both Telegram adapters behind a single queue.

**External integrations:**

| External service | Client location | Concerns |
|---|---|---|
| GitHub API | `packages/aotf-agent/src/ci/githubActions.ts` | NFR-I01 rate-limit backoff; conditional requests |
| GitHub webhooks | `packages/aotf-agent/src/ci/githubWebhooks.ts` | NFR-S05 HMAC-SHA256 signature validation |
| Telegram Bot API (official plugin) | `packages/aotf-agent/src/adapters/telegram/officialPlugin.ts` | NFR-I02 rate-limit queue |
| Telegram Bot API (claude-bridge) | `packages/aotf-agent/src/adapters/telegram/claudeBridge.ts` | Same NFR-I02; bidirectional + Goal Loop |
| Claude Code CLI | `packages/aotf-agent/src/aiCli/claudeCode.ts` | FR-62 version validation; FR-63 degraded fallback |
| OS Keychain (macOS) / libsecret (Linux) | `crates/aotfd/src/credentials.rs` | NFR-S01 fallback chain |
| SOPS (encrypted file fallback) | same | Headless/CI secret path |
| WebAuthn (FR-106 consent) | `crates/aotf-gatekeeper/src/consent/webauthn.rs` | `webauthn-rs` crate; pre-registered operator keys |

**Data flow: Detect → Diagnose → Fix (happy path J1):**

```
1. Log watcher (aotfd) emits raw event → dedup/rate-limit queue
2. Dedup threshold reached → aotfd creates Finding in SQLite
3. aotfd notifies aotf-agent via agent.sock: v1.diagnosis.request(finding_id)
4. aotf-agent invokes Claude Code CLI subprocess with finding context
5. AI returns diagnosis → aotf-agent computes confidence via FR-18 formula
6. aotf-agent returns v1.diagnosis.response with diagnosis + confidence
7. aotfd stores diagnosis in decision_log (via gate_scope.rs → gate.record_outcome)
8. If user requests fix:
   a. aotfd calls gated(Action::ApplyFix, ctx) → gatekeeper.evaluate
   b. gatekeeper returns Allow (for REVERSIBLE) or RequireConsent (for IRREVERSIBLE)
   c. For REVERSIBLE: aotfd creates rollback ref, applies fix, opens PR
   d. aotfd calls gate.record_outcome(action_id, outcome) → gatekeeper writes audit_log
9. aotf-agent dispatches notification via AotfChannelEvent → Telegram adapter
```

### File Organization Patterns

- **Configuration files:** `.aotf/config.yaml` + `plugins.lock.yaml` + `.aotf/consent.yaml` at repo root. Daemon state in `~/.aotf/run/` (sockets, pid files).
- **Source organization:** one crate per process + one library crate per shared concern. Feature-by-feature within each crate, not type-by-type.
- **Test organization:** unit tests co-located in source files; integration tests per-crate in `tests/`; cross-crate in workspace-root `tests/`; shared helpers via `feature = "test-helpers"` on producer crates; unsafe fault-injection isolated in `aotf-fault`; proc macros in `aotf-test-macros`. See §Test Organization Patterns in Implementation Patterns section.
- **Asset organization:** `fixtures/` for test data; no runtime static assets in v1.0 (CLI-only).
- **Documentation placement:** `docs/` for architecture, ADRs, security; rustdoc emitted to `target/doc/`; TypeScript tsdoc emitted via `typedoc`.

### Development Workflow Integration

- `cargo watch -x "test --workspace"` for continuous Rust tests.
- `bun --watch test` for continuous TS tests.
- `cargo run --bin aotfd -- --foreground --config ./dev-config.yaml` for local daemon.
- `cargo run --bin aotf-cli -- doctor` to verify local setup.
- `turbo build` = `cargo build --workspace` + `bun run build` in `packages/aotf-agent` (parallel).
- Release build = `cargo build --release --workspace` + embed `packages/aotf-agent/dist` into `aotf-cli` via `build.rs` + `include_bytes!`.
- `scripts/bootstrap.sh` for new contributors — installs toolchain, runs full test suite, target <2 min.

**Deployment:**

- GitHub Releases: signed binaries for `{linux-x86_64, linux-aarch64, macos-x86_64, macos-aarch64}` + SLSA provenance attestation.
- `install.aotf.dev/sh` = `scripts/install.sh` hosted on gh-pages; detects platform, downloads verified binary.
- Homebrew tap: `hieutrungdao/homebrew-aotf` — auto-PR on release tag (not auto-merge; human review gates).
- No container image in v1.0 (PRD §Installation Methods defers Docker).

## Architecture Validation Results

### Coherence Validation ✅

**Decision compatibility:** all major architectural decisions (D-IPC-1 through D-INFRA-2) are mutually consistent. The Rust+Bun stack satisfies the joint NFR-P01 (<100MB RSS) + NFR-P04 (<50ms cold start) constraint that no Bun-only or Python-host alternative could; the cross-platform X3 gatekeeper stance is consistent with the existing NFR-S08 macOS plugin-sandbox concession (one published asymmetry, not two); the three-token authorization model (FR-107) maps cleanly onto the gate API surface (D-ENF-2's now-8-method surface — see Validation-Pass Architectural Additions below); the action-tier taxonomy (FR-105) propagates consistently across plugin manifest schema, IPC `consentTokenId` field, gate evaluation, and audit log; the backpressure policy (D-IPC-4 two-variant enum) satisfies NFR-P01 bounded-memory + NFR-P07 1000-errors/min burst + NFR-I02 audit-on-Telegram-drop without inventing speculative variants.

**Pattern consistency:** the three-layer enforcement model from §Implementation Patterns (Layer 1 design-enforced, Layer 2 four mandatory guidelines, Layer 3 review-caught) maps onto the workspace structure decisions: Layer-1 invariants are encoded by `pub(crate)` visibility + SQLite triggers + enum exhaustivity + `inventory` macro registration + `gen-ts-types` build freshness; Layer-2 guidelines (gated() scope, Clock trait, mutants.toml, PII discipline) have named verification tests in step-06.

**Structure alignment:** the workspace dependency graph (10 crates, layering enforced by `cargo-deny`) supports every architectural decision. `aotf-gatekeeper` having no dependency on `aotfd` enforces the "gatekeeper serves, daemon calls" invariant. `aotf-fault` being the only audit-allowlisted unsafe crate (outside production seccomp) gives reviewers a single audit surface. The CI gate `scripts/check-no-test-features.sh` prevents test-helpers feature leakage into release builds.

### Requirements Coverage Validation ✅

**Functional Requirements (107 total; 81 MVP / 20 Growth / 6 Vision):** all 81 MVP FRs map to a primary location in §Project Structure. FR category coverage:

| FR Category | MVP FRs | Coverage status |
|---|---|---|
| Project Init & Config (FR-01-08, 97, 99) | 10 | Full, except stack-detect module name (Gap G7) |
| Log Monitoring & Detection (FR-09-15) | 7 | Full |
| AI Diagnosis (FR-17-21, 92) | 6 | Full |
| Autonomous Fix & Safety (FR-23-36, 93, 94, 105-107) | 19 | Full — heaviest category, fully covered by gatekeeper + gate scope + consent ceremony |
| Goal Loop (FR-37-42) | 6 | Partial — attempt persistence unspecified (Gap G4) |
| Notifications & Coordination (FR-43-52, 103) | 11 | Full, except DroppedNotification audit-action question (Gap G10) |
| Plugin Ecosystem (FR-54-60, 96, 104) | 9 | Full for v1.0; FR-104 registry component is out of scope for v1.0 (Gap G8 documented) |
| AI Backend Management (FR-61-65, 95) | 6 | Full |
| Observability, Status & Audit (FR-66-70, 90, 91) | 7 | Full, except audit retention policy (Gap G6) and AI cost storage (Gap G3) |

Growth (20 FRs) and Vision (6 FRs) are tracked in stub locations; not v1.0 scope.

**Non-Functional Requirements (41 total):** all 41 NFRs are addressed. Spot summary:

| NFR Category | Count | Coverage |
|---|---|---|
| Performance (NFR-P01-P07) | 7 | Compiled Rust daemon, bounded channels, WAL, single binary |
| Security (NFR-S01-S11) | 11 | Keychain + SHA-256 + 5+2 trust surface + WebAuthn + Linux/macOS X3 posture |
| Reliability (NFR-R01-R06) | 6 | Degraded-mode signaling + WAL + recovery + worktree cleanup |
| Privacy (NFR-D01-D04) | 4 | PII filter + opt-in telemetry + local-default + no-phone-home (telemetry destination — Gap G1, BLOCKING) |
| Scalability (NFR-SC01-SC04) | 4 | 500k LOC + 10 repos + 3 worktrees + 3 concurrent AI calls |
| Operability (NFR-O01-O05) | 5 | Single binary + doctor + actionable errors + update check + stable JSON log schema |
| Integration Reliability (NFR-I01-I04) | 4 | GitHub backoff + Telegram queue + AI subprocess validation + plugin schema versioning |

**User journey coverage:** J1, J2, J3, J4, J6, J7, J8, J11 (the eight MVP journeys) all have explicit architectural support; J5, J9, J10, J12 are Growth/Vision in stub directories.

### Implementation Readiness Validation ⚠️ (READY with documented gaps)

**Decision completeness:** all critical D-* decisions documented with rationale. Versions pinned to "verify at scaffold" markers (knowledge cutoff 2026-01).

**Structure completeness:** every workspace member named, every PRD FR category mapped to a primary directory, every cross-cutting concern (12 from step-02) mapped to specific files. Workspace count: 10 (8 production crates + `aotf-test-macros` + `aotf-fault`).

**Pattern completeness:** three-layer enforcement model with explicit mandatory guidelines, design-enforced invariants, and review-caught conventions. Verification gates table distinguishes real gates from partial gates.

### Gap Analysis Results

The gaps below were surfaced during the validation pass. None block scaffold-stage stories (Story 1-9 per Updated Story Sequencing below).

**Blocking gaps (resolve before v1.0 ship — full panel converged on this):**

| Gap | Description | Resolution location | Story |
|---|---|---|---|
| **G1** | Telemetry destination + measurement of validation metrics not specified | New ADR + `crates/aotfd/src/telemetry/` | First-fix epic — without this, validation metrics unmeasurable AND audit-trail story collapses. **Reclassified important → BLOCKING** per John + Mary unanimous in step-07 Party Mode: "shipped on vibes" / "moat collapses to vibes." |

**Important gaps (resolve during MVP, before the relevant feature ships):**

| Gap | Description | Resolution location | Story |
|---|---|---|---|
| **G2** | GitHub auth model (App vs PAT) not chosen | New ADR + `packages/aotf-agent/src/ci/githubAuth.ts` | GitHub plugin epic |
| **G3** | AI cost tracking storage table not pinned | Add `ai_costs` table to D-DB-2 + `crates/aotf-storage/src/ai_costs.rs` | AI backend mgmt epic |
| **G4** | Goal Loop attempt persistence not specified | Add `goal_loop_attempts` table or extend `decision_log` | Goal Loop epic |
| **G5** | CLI↔daemon SO_PEERCRED auth not named | Specify in `crates/aotfd/src/ipc/cli_server.rs` | Scaffold epic addendum |
| **G7** | `aotf init` stack-detect module location not pinned | Decide: `aotf-cli/src/commands/init.rs` inline vs new `aotf-stack-detect` lib crate | Init epic |
| **G9** | Plugin manifest `actionTier` v1.0/v1.1 grace-window code location | Pin in `aotf-plugin-api/src/manifest.rs` with version-bump check | Plugin loader epic |
| **G10** | `DroppedNotification` audit entry — is it an `Action` in tier_map? | Add `Action::SystemAuditEvent` (tier `READ`) to `tier_map.rs`; route notification drops through `gate.record_outcome` | Notification epic |

**Deferred gaps (acknowledged, addressed post-v1.0):**

| Gap | Description | Defer to |
|---|---|---|
| **G6** | Audit log retention policy | v0.2 — local DB grows unbounded in v1.0 with documented note in `docs/security/audit-retention.md` |
| **G8** | Plugin registry submission service (FR-104) | v0.2 — registry is an HTTP service to be designed when first community plugin lands; v1.0 ships a static `https://hub.aotf.dev` allowlist |

**No critical gaps** beyond G1's reclassification. All gaps are addressable within the existing crate structure or with named additions.

### Validation-Pass Architectural Additions

The full-panel readiness check (step-07 Party Mode with Winston, Amelia, John, Mary) surfaced 5 architectural items not caught in earlier per-step probes. These are folded into the architecture before scaffold:

**A1 — 8th gate method: `gate.dry_run_evaluate`** (Winston). Updates D-ENF-2 from 7-method to 8-method surface. Non-mutating "would this be authorized?" probe for Goal Loop pre-flight; prevents the loop from either over-asking (audit noise from speculative authorize() calls) or speculatively executing (reversibility violation).

```rust
// Added to D-ENF-2 surface
v1.gate.dry_run_evaluate(action: Action, context: ActionContext) -> GateDecision
  // Same return shape as evaluate(); does NOT mutate gatekeeper state, does NOT emit audit entry.
  // Used by Goal Loop pre-flight, plugin tier-check warnings, "what if" UX flows.
```

**A2 — 4th gatekeeper IPC channel: structured decision-log emit** (Winston). Distinct from the audit log (which records actions); this records *reasoning* — why the gatekeeper denied or allowed each evaluation. Critical for human debugging at 2am ("why did the gatekeeper say no?"). New socket `~/.aotf/run/gatekeeper-decisions.sock`; subscribers include `aotf-cli doctor`, log aggregators, and (Growth) the dashboard. Read-only consumers; gatekeeper is sole publisher.

**A3 — Parent-death orphan detection** (Winston). `aotf-gatekeeper/src/main.rs` monitors `aotfd` PID via `prctl(PR_SET_PDEATHSIG)` on Linux / kqueue `NOTE_EXIT` on macOS. On parent-vanish, gatekeeper writes a final `aotfd-orphaned` audit entry and terminates cleanly. Prevents zombie-enforcer scenario. Added to Story 4 acceptance criteria.

**A4 — `AuditSink` trait breaks Story 4↔5 ordering dependency** (Amelia). New file `crates/aotf-gatekeeper/src/audit/sink.rs`:

```rust
pub trait AuditSink: Send + Sync {
    fn append(&self, entry: AuditEntry) -> Result<AuditSeq, AuditError>;
    fn verify_chain(&self) -> Result<ChainStatus, AuditError>;
}

// Story 4 ships:
pub struct InMemoryAuditSink { /* ... */ }

// Story 5 ships:
pub struct SqliteAuditSink { /* ... */ }
```

Story 4 (gatekeeper scaffold) consumes the trait and ships the in-memory impl. Story 5 (storage) replaces it with the SQLite-backed impl. `gate.rs` and `tests/gate_invariant.rs` are testable in isolation against the in-memory variant — no transitive dependency on the storage layer.

**A5 — `tests/audit_clock_monotonic.rs`** (Amelia). New integration test feeds the `Clock` trait a backwards-jumping clock impl (simulating NTP skew or container clock reset), asserts (a) hash chain still validates, (b) ordering by `seq` is preserved, (c) any user-facing display flagged with a "clock anomaly detected" indicator. Real bug class on long-running daemons; not covered by existing `audit_chain_recovery.rs` (which only handles crash recovery).

**Documentation addition: `docs/architecture/scaffold-reference.md`** (Amelia). Worked end-to-end example crate (Cargo.toml + 3-layer-pattern walkthrough + `mutants.toml` template + Layer-1/2/3 examples), so mid-junior devs can execute Story 1 without external reference. Added to Implementation Handoff documentation deliverables.

### Updated Story Sequencing (post-validation)

The 9-story sequence updates to reflect the AuditSink trait approach + Winston's hidden-dependency fix + Amelia's mid-junior gap fix:

1. **Scaffold** — monorepo skeleton, 10 crates + Bun package, AC-PLAT-* verified. **Includes new `docs/architecture/scaffold-reference.md`**.
2. **Finding v1 wire struct + golden round-trip** — `crates/aotf-core-types/tests/finding_golden.rs` + `fixtures/finding-v1/canonical.json`. **Stable SSoT for both wire and SQLite schema** (Winston's hidden-dep fix: `findings` table layout asserted to match wire-struct serde shape).
3. **IPC protocol crate** — D-IPC-1/2/3 method namespaces (now including `v1.gate.dry_run_evaluate`); framing; error codes.
4. **Gatekeeper scaffold** — D-ENF-1/2/3 with stub logic; `tier_map.rs` + `gate.rs` + `mutants.toml`. **AuditSink trait + InMemoryAuditSink ship here.** Parent-death orphan-detection wired (A3). FR-105.2 mutation test passes against stubs.
5. **SQLite storage** — D-DB-1/2/3/4 schema + connection model. **`SqliteAuditSink` replaces `InMemoryAuditSink` in gatekeeper.** Append-only verified by negative test; `audit_clock_monotonic.rs` added to test suite (A5).
6. **Daemon scaffold** — connects all 4 IPC sockets (D-IPC + new `gatekeeper-decisions.sock` per A2); in-flight recovery (D-IPC-5); gate_scope helper.
7. **CLI scaffold** — thin client; `aotf --version`, `aotf doctor`, `aotf user audit`. AC-PLAT-1/2 verified.
8. **Bun agent scaffold** — Claude Agent SDK integration; AI CLI subprocess manager; confidence computer.
9. From here: feature-driven development per regenerated PRD epics.

Stories 1-7 remain sequenced; Bun agent (Story 8) can start at Story 2 against the golden fixtures.

### Architecture Completeness Checklist

**✅ Requirements Analysis (step-02)**
- [x] Project context thoroughly analyzed (107 FRs / 41 NFRs / 4 pillars / 12 cross-cutting concerns)
- [x] Scale and complexity assessed ("security-dominant single-node platform with two high-rigor subsystems")
- [x] Technical constraints identified (Rust+Bun stack fixed, IPC pattern fixed, dual Telegram transports fixed)
- [x] Cross-cutting concerns mapped (12 concerns across 4 categories)
- [x] Stakeholder lens added (plugin authors, compliance-curious buyers, CNCF Sandbox)

**✅ Architectural Decisions (step-03 + step-04)**
- [x] Starter shape decided (Option B / X3 — separate enforcement subprocess, cross-platform with seccomp on Linux)
- [x] IPC contract decided (D-IPC-1 through D-IPC-5 + A2 new decision-log channel)
- [x] Enforcement core API decided (D-ENF-1 through D-ENF-3, with 8-method surface per A1)
- [x] SQLite schema decided (D-DB-1 through D-DB-5)
- [x] Clock & ordering decided (D-CLK-1)
- [x] v1.1 boundary decisions (D-MCP-1 + D-INFRA-1 + D-INFRA-2)

**✅ Implementation Patterns (step-05)**
- [x] Three-layer enforcement model (Layer 1 design / Layer 2 four mandatory / Layer 3 review-caught)
- [x] Naming conventions established (Rust + TS + cross-boundary auto-generated types)
- [x] Error handling specified (thiserror in libs, anyhow in bins, JSON-RPC error codes)
- [x] Logging discipline pinned (tracing with structured fields, PII level rules)
- [x] Gate-evaluation invariant call pattern (the gated() scope helper)
- [x] Audit emission discipline (single writer + API-surface reflection test)

**✅ Project Structure (step-06)**
- [x] Complete workspace tree (10 crates + Bun package + fixtures + docs + scripts + tests + .github)
- [x] Process boundaries (3 Rust IPC channels + Rust↔Bun + plugin sandbox + AI CLI subprocess + Goal Loop worktrees + new gatekeeper-decisions channel per A2)
- [x] Crate dependency layering (enforced by cargo-deny)
- [x] Data boundaries (per-process SQLite write privileges)
- [x] FR/epic-to-directory mapping (15 FR categories, 12 cross-cutting concerns, 8 user journeys)
- [x] Test-feature isolation CI gate

**✅ Validation Results (step-07)**
- [x] Coherence validated across all decisions
- [x] 81 MVP FRs covered (with 8 important + 1 blocking + 2 deferred gaps tracked)
- [x] 41 NFRs addressed
- [x] 8 MVP user journeys have explicit architectural support
- [x] 5 validation-pass architectural additions (A1-A5) folded in
- [x] 9-story implementation sequence updated for AuditSink trait approach

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION (with G1 BLOCKING for v1.0 ship and 7 important + 2 deferred gaps tracked).

**Confidence Level:** HIGH for v1.0 MVP scope. The wedge (FR-105 + FR-106 + FR-107 — pre-production autonomy with reversibility-typed gating + WebAuthn consent) has a clean architectural realization that survived four Party-Mode roundtables of pressure-testing.

**Key strengths:**

1. **The trust boundary is real, not narrative.** Gatekeeper as a separate process with `pub(crate)` audit-writer ownership + SQLite triggers + per-process connection privileges + API-surface reflection tests = NFR-S06 enforced at three layers.
2. **The wedge is named and protected structurally.** FR-105/106/107 lead the Requirements Overview; the gated() scope helper enforces the call pattern via type system; the mutation-target manifest verifies the gate logic survives mutation testing.
3. **Cross-platform discipline is honest.** X3 stance (cross-platform gatekeeper, Linux layers seccomp-bpf, macOS gap published) is consistent with the existing NFR-S08 plugin-sandbox concession.
4. **Workspace organization survives scrutiny.** 10 crates with single architectural justifications each.
5. **Validation discipline is internalized.** Four Party-Mode roundtables in this workflow caught and corrected: (a) "enterprise complexity" framing inflation, (b) D-IPC-4 four-policy over-engineering, (c) 10-guideline pattern bloat, (d) hidden architectural gaps in scaffold sequencing + gate API + observability.

**Areas for future enhancement (post-v1.0):**

- **Hyperscaler OSS-acquisition risk** (Mary, step-07 roundtable): if AWS/GCP forks AOTF and ports the gatekeeper to native macOS/Windows in 90 days, the X3-stance differentiator narrows. Mitigation: lean into the WebAuthn consent ceremony UX (Ratatui consent_ceremony widget + flow) as the harder-to-clone product artifact. Treat consent UX as first-class product surface, not plumbing.
- Goal Loop attempt persistence (G4) needs schema design before Goal Loop epic ships.
- AI cost storage (G3) needs schema design before cost-tracking telemetry rolls out.
- Plugin registry (G8) is a Growth-tier component requiring its own architecture pass.
- Audit log retention policy (G6) — local DB grows unbounded in v1.0; revisit before SOC 2 / ISO 27001 evaluation.
- WASM enforcement core (Option C from step-03) is a candidate for v0.3+ if the Linux/macOS posture asymmetry becomes user-visible pain.
- ML/LLM Lifecycle Manager (Pillar 3, FR-85-89) is post-v1.0 and will need its own architecture document.

### Implementation Handoff

**AI Agent Guidelines (for stories executing against this architecture):**

- Read this document before opening any AOTF code. The §Implementation Patterns section is the day-to-day reference; §Project Structure tells you where files go; §Core Architectural Decisions explains why.
- **Layer-1 design-enforced invariants** are NOT optional and NOT review-caught — they are encoded in types, traits, and build gates.
- **Layer-2 four mandatory guidelines** require human discipline — they CANNOT be fully mechanically enforced. Violations are bugs, not style choices.
- **Layer-3 review-caught conventions** are caught by clippy + reviewer judgment.

**Note on Pillar 4 (AI Agent Operations) cut concern:** raised in step-07 roundtable. Already addressed structurally — Pillar 4 ships as a stub directory `packages/aotf-agent/src/agentOps/` with no v1.0 code. The wedge (Pillars 1-3 + AI QA Agent stub for Growth) carries v1.0 alone.

**First implementation priority — 9-story sequence (per Updated Story Sequencing above):**

See "Updated Story Sequencing (post-validation)" section above for the canonical sequence with AuditSink trait approach.

**Documentation deliverables to ship alongside scaffold:**

- `docs/architecture.md` — this document (the source of truth)
- `docs/architecture/scaffold-reference.md` — worked end-to-end example for mid-junior devs (per Amelia)
- `docs/adr/0001-enforcement-core-placement.md` — Option B / X3 rationale
- `docs/adr/0002-dual-telegram-transports.md` — both adapters behind AotfChannelEvent
- `docs/adr/0003-backpressure-two-policies.md` — Block + DropOldest, deferred extension
- `docs/adr/0004-clock-trait-determinism.md` — monotonic + Lamport + wall-clock split
- `docs/adr/0005-test-organization-fault-isolation.md` — aotf-fault as unsafe-isolated test crate
- `docs/adr/0006-gate-api-dry-run.md` — A1 dry_run_evaluate rationale
- `docs/adr/0007-gatekeeper-decision-log-channel.md` — A2 reasoning-record channel
- `docs/security/platform-matrix.md` — Linux/macOS isolation table per AC-PLAT-4
- `docs/plugin-development-guide.md` — for plugin authors (stakeholder lens #1)

**Epics regeneration prerequisite:** the existing `docs/epics.md` is deprecated (scope-shifted from v1.1 Python). With this architecture document complete and promoted to `docs/architecture.md` (prior v1.1 Python subset lives at git ref `985bafc`), the next BMad workflow is `/bmad-create-epics-and-stories` against the canonical PRD v2.0.1 and this validated architecture. The 9-story sequence above seeds Epic 1 (Scaffold).
