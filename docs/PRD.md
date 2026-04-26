---
stepsCompleted: ["step-01-init", "step-02-discovery", "step-02b-vision", "step-02c-executive-summary", "step-03-success", "step-04-journeys", "step-05-domain", "step-06-innovation", "step-07-project-type", "step-08-scoping", "step-09-functional", "step-10-nonfunctional", "step-11-polish", "step-12-complete", "step-e-01-discovery", "step-e-02-review", "step-e-03-edit"]
inputDocuments:
  - "docs/product-brief.md"
  - "_bmad-output/research/claude-bridge-monitor-tool-research.md"
  - "_bmad-output/planning-artifacts/research/market-sre-agent-competitive-research-2026-04-17.md"
  - "_bmad-output/brainstorming/brainstorming-session-2026-04-04-1000.md"
  - "_bmad-output/brainstorming/brainstorming-session-2026-04-04-1000-distillate.md"
  - "docs/architecture.md"
  - "docs/PRD.md"
  - "docs/epics.md"
briefCount: 1
researchCount: 2
brainstormingCount: 2
projectDocsCount: 3
workflowType: 'prd'
lastEdited: '2026-04-19'
editHistory:
  - date: '2026-04-19'
    source: 'bmad-edit-prd workflow + Party Mode (John/Winston/Amelia/Mary) — pre-architecture cleanup'
    changes: >
      Resolved 3 open decisions from prd-review-2026-04-19.md.
      D-1 (C-2 gate model): hybrid "seven-control trust surface" in pitch (five pipeline gates + two cross-cutting controls — Audit, Consent Token), 5+2 typing preserved in architecture. Rewrote Innovation claim #4 + §5 policy matrix; fixed NFR-S04 pipeline-gate range to FR-23-27 (was wrongly FR-23-35).
      D-2 (M-1 MCP): read-only MCP in v1.0. FR-65 rewritten — tools: get_risk_score, get_finding, list_recent_runs, get_daemon_status. No mutation in v1.0. NFR-S07 updated for read-scope tokens. Exec summary qualified. Write-capable MCP deferred to v1.1 pending design-partner validation.
      D-3 (M-5 FR-105 mechanism): softened. FR-105 rewritten with invariant ACs (AC.1-AC.5); file paths (plugin_bridge.rs, tier_map.rs) removed from PRD — mechanism moves to architecture + ADRs. NFR-S10 rewritten with invariant "memory-safe, capability-bounded core whose failure domain is isolated from the orchestrator"; specific mechanism is architectural choice. Core scope test: "if this code is wrong, can reversible escalate to irreversible without authorization?"
      Additional pre-architecture items (M-2/M-3/M-4/M-6/m-3/m-5): FR-107 added (three-token-model unification with AuthTypeMismatch enforcement, Telegram ACL clarification). Weekly Active Teams operational definition added (team identity = SHA-256(remote URL + config hash), active = ≥1 of 5 loop actions in 7d, telemetry opt-in with disclosed coverage share). FR-53 per-channel bidirectional specification (Slack/PagerDuty/Teams = bidirectional; Email = output-only). Growth Command Surface subsection added to §8 (aotf ml/qa/simulate/agent phase-gated; aotf consent issue is MVP for J11+FR-106). FR Phase Summary table added: 81 MVP / 20 Growth / 6 Vision / 107 total. FR-18 cross-reference to §6 confidence formula.
      Deferred: C-1 strict section-monotonic FR renumber — held until after architecture regeneration to minimize cross-artifact churn (ux-research-inputs, readiness reports, upcoming architecture + epics).
      Net: +1 FR (107 total), ~4 NFRs expanded, 0 FRs removed. PRD ready for bmad-create-architecture.
  - date: '2026-04-17'
    source: 'bmad-edit-prd workflow + Party Mode (Mary/John/Winston/Amelia)'
    changes: >
      Sync with market research (market-sre-agent-competitive-research-2026-04-17.md).
      §1 Exec Summary: added Runframe/NeuBird/Gravitee evidence anchors (toil-rose-post-adoption framing, 78% no-alert, 14.4% security approval).
      §4 Product Scope: added "Non-Goals / Scope Boundary" subsection — pre-production SDLC surface explicit, SRE-expansion deferred to 2027-06-30 gated on 4-criterion triad (LOI / funding / hyperscaler partner / lost-deals).
      §5 User Journeys: added J11 Proactive Pre-Push Prevention (supports FR-05/FR-14/FR-105/FR-106/FR-66), J12 AI QA Agent Self-Healing (supports FR-71–FR-75 + FR-105, closes validation warning on AI QA pillar).
      §7 Innovation: updated claim #1 competitive comparison for 2026 landscape (Resolve.ai $1B, HolmesGPT CNCF Sandbox, Keep 11.6k, hyperscaler agents); added Tracer opensre lineage; added new claim #5 Authorized Pre-Production Autonomy (action-tier taxonomy + tier-to-gate policy matrix + enforcement placement); added CNCF Sandbox strategic-moat bullet; expanded Risk Mitigation table (Resolve.ai, OSS-SRE, hyperscaler, CNCF-procurement-bias rows).
      §8 Developer Tool + CLI: added actionTier declaration for plugin manifests, manifest schema migration note (v1.0.x grace -> v1.1 required), IPC contract note on consentTokenId field.
      §9 Project Scoping: promoted FR-14 pre-push risk assessment to MVP, added J11 to core MVP journeys, v1.0 phased-roadmap description updated, Market Risks table expanded.
      §10 FRs: FR-14 un-deferred (Journey 11 supports it, tier-gating integrated); added FR-105 (action-tier taxonomy), FR-106 (production consent token with WebAuthn attestation); strengthened FR-66 with approver/timestamp/reason (min 20 chars verbatim) + token hash + WebAuthn assertion hash on EXECUTE_IRREVERSIBLE.
      §11 NFRs: strengthened NFR-S04 (force-flag does not bypass FR-105/FR-106); added NFR-S10 (Rust FFI enforcement + mutation test requirement) and NFR-S11 (production-designated is binary).
      Wedge naming decision: "SDLC x Production Fusion" explicitly rejected (Resolve.ai language collision); adopted "Authorized Pre-Production Autonomy" throughout.
classification:
  projectType: "developer_tool+cli_tool"
  domain: "ai-devops-mlops-platform"
  complexity: "high"
  projectContext: "greenfield-v1.0"
  prdGoal: "Authoritative v1.0 PRD for AOTF; Rust+TS stack, CLI-agent-first, claude-bridge, MCP interface, four-pillar product vision"
---

# Product Requirements Document - aotf

**Author:** hieutrungdao
**Date:** 2026-04-16 (Party-Mode edit; restored 2026-04-19)

**Version:** 2.0.1
**Status:** Canonical (pre-architecture cleanup)

### Changelog

- **2.0.1 (2026-04-19)** — Pre-architecture cleanup. Resolves open decisions from `_bmad-output/planning-artifacts/prd-review-2026-04-19.md` via `bmad-edit-prd` + Party Mode (John/Winston/Amelia/Mary). **Decisions:** D-1 gate model → hybrid "seven-control trust surface" in pitch (five pipeline gates + two cross-cutting controls: Audit, Consent Token) with 5+2 typing in architecture; D-2 MCP scope → **read-only in v1.0** (FR-65 revised: `get_risk_score`, `get_finding`, `list_recent_runs`, `get_daemon_status` only; no mutation), write-capable deferred to v1.1; D-3 FR-105 architecture pre-commit → **softened to invariant-based** (mechanism moves to architecture/ADR; AC.1-AC.5 specify contract; core scope test: "if wrong, can reversible escalate to irreversible?"). **Also:** added FR-107 (three-token-model unification with `AuthTypeMismatch` enforcement), Weekly Active Teams operational definition (team identity = SHA-256(remote URL + config hash); active = ≥1 of 5 loop actions in 7d; telemetry opt-in with disclosed coverage), FR-53 per-channel bidirectional spec (Slack/PagerDuty/Teams/Email), Growth Command Surface subsection (`aotf ml`/`qa`/`simulate`/`agent` phase-gated), `aotf consent issue` added to v1.0 command table (supports J11/FR-106), FR Phase Summary table (**81 MVP / 20 Growth / 6 Vision / 107 total** FRs), FR-18 cross-reference to §6 confidence formula, NFR-S04 pipeline-gate range corrected (FR-23-27, not FR-23-35), NFR-S10 rewritten with invariants instead of file paths. **Deferred:** C-1 strict section-monotonic FR renumber — held until after architecture regeneration to minimize cross-artifact doc churn (ux-research-inputs, readiness reports, upcoming architecture + epics). Net delta: +1 FR (107 total), ~4 NFRs expanded, no FRs removed. PRD ready for `bmad-create-architecture`.
- **2.0 (2026-04-19)** — Scope reconciliation. Restored from recovered legacy draft (`_bmad-output/planning-artifacts/prd.md`, 1,433 lines, Party-Mode edit 2026-04-17 by Mary/John/Winston/Amelia). Reverses the 2026-04-18 narrowing (commit `95aa4b0`) in response to the Apr-17 readiness recommendation to regenerate architecture + epics from this PRD. Introduces **106 FRs** (was 69 in v1.1) across 15 categories, **41 NFRs** (was 12) across 7 categories, 12 user journeys, 12 TS criteria, Rust+TS/Bun stack, AOTF MCP server, claude-bridge Telegram adapter, action-tier reversibility taxonomy (FR-105), production consent token with WebAuthn (FR-106), AI QA Agent pillar (FR-71–FR-75), AI Agent Operations pillar (FR-76–FR-80), pre-push risk assessment (FR-14). **Architecture v1.1 and Epics v1.1 are now out-of-date** against this PRD and require regeneration — see scope-shift banners at the top of each. Pre-architecture cleanup (C-1 FR renumber, C-2 five-vs-seven gate, M-1–M-6 scope clarifications) tracked in [`_bmad-output/planning-artifacts/prd-review-2026-04-19.md`](../_bmad-output/planning-artifacts/prd-review-2026-04-19.md); address via `bmad-edit-prd` before `bmad-create-architecture`.
- **1.1 (2026-04-13) — DEPRECATED** — Narrowed scope removed here. See `_bmad-output/planning-artifacts/prd.md` recovery header for provenance; that copy of the legacy is now this file.
- **1.0 (2026-04-04)** — Initial draft.

---

## Executive Summary

**Agent On the Fly (AOTF)** is the missing operational layer of the AI SDLC — an open-source platform that ensures applications, AI agents, and models all work correctly in production. As teams move from shipping *code* to shipping *AI-native systems*, the operational gap has widened: existing DevOps tools don't understand agent behavior, existing MLOps tools don't close the remediation loop, and no tool connects all three layers. AOTF closes that gap.

**Why Now:** Between 2023 and 2026, LLM-native applications moved from experimental to mission-critical. The category's incumbent AI-SRE tooling is visibly failing its central promises: **78% of production incidents fire no alert at all** (NeuBird 2026 survey) — the alerting layer SRE agents hook into is blind to the majority of production pain. **Only 14.4% of AI agents go live with full security/IT approval** (Gravitee 2026) — the authorization layer is under-productized. Independent survey data (Runframe 2026, n=1,039) shows SRE toil *continuing to rise post-AI-adoption* — AI SRE has not yet demonstrated the operational outcome it promises. Teams now run AI agents in production without the observability infrastructure that code deployments take for granted. Prompt regressions go undetected. Agent drift is discovered by customers, not monitoring tools. The AI SDLC is missing its operational edge — and that edge is now a business risk, not a nice-to-have.

**The Problem:** Modern engineering teams running AI-native systems operate three disconnected operational layers with no cross-layer correlation:
1. **Application layer** — errors, logs, CI failures, deployment issues
2. **AI agent layer** — agent run telemetry, behavior deviation, cost overruns, agent drift
3. **Model/prompt layer** — model drift, prompt regression, A/B test results, shadow pipeline divergence

These layers require 5–10 separate tools (Sentry, Datadog, Langfuse, Arize, MLflow, ArgoCD), producing isolated alerts with no unified remediation path. Autonomous fixing is either absent or enterprise-priced. The result: a platform engineer fielding 3am pages while their AI agent silently hallucinates in production; an MLOps engineer discovering prompt regression via customer complaints, not tooling.

**The Solution — Four Pillars:**
1. **Proactive DevOps Loop** — pre-push risk scoring, auto-fix PRs, CI pipeline intelligence; predicts and prevents issues before production
2. **AI QA Agent** — Playwright + LLM-powered autonomous shadow QA; self-healing tests; user bug reports auto-converted to regression tests
3. **ML/LLM Lifecycle Manager** — git-native model registry; AOTF *orchestrates* closed-loop MLOps (drift→retrain→gate→canary→promote); prompt versioning, A/B testing, injection firewall
4. **AI Agent Operations** — agent run tracking, behavior logging, deviation detection, cost tracking, shadow pipelines

**Target Users:**
- **Software engineers** at teams shipping AI-native applications — the primary adoption surface (bottom-up PLG)
- **Platform engineers / SREs** running AI systems in production without adequate observability
- **MLOps / LLMOps engineers** stitching together siloed tools to manage model and prompt lifecycles
- **Engineering managers** who need a unified view of SDLC health across code, agents, and models

**Architecture:** Rust binary CLI + TypeScript/Bun agent layer. Single static binary (`brew install aotf`). **CLI-agent-first AI backends:** Claude Code CLI (v1.0 default), Codex CLI, Gemini CLI — all subprocess-wrapped with no framework SDK dependency. Local-model backends (Ollama, llama.cpp) on the roadmap. Interface-agnostic: CLI, **MCP server (read-only, v1.0)**, Telegram (v1.0); WebUI, TUI (Growth). Write-capable MCP surface deferred to v1.1 pending design-partner validation — see FR-65 rationale.

**Cloud dependency disclosure:** AOTF itself has no hosted service dependency — all state, findings, and learning remain on the user's machine. AI diagnosis and fix generation route through the user's configured AI CLI, which does transmit code to that provider (e.g., Anthropic for Claude Code CLI). Local-model backends (Growth roadmap) close this loop fully for air-gapped use.

**Distribution model:** v1.0 ships fully open-source under Apache 2.0. Commercial tiers (hosted intelligence, SLA, SSO) are post-launch considerations, not v1.0 scope.

### What Makes This Special

AOTF occupies a category-defining gap that fragmented tools cannot fill individually:

- **Langfuse, Helicone, Arize** observe agent/LLM behavior. AOTF observes *and remediates* — it creates fix PRs, triggers retraining pipelines, and heals tests autonomously. Observation without remediation is an alert, not a solution.
- **Sentry, Datadog** monitor application errors. They have no model of AI agent behavior, prompt regressions, or model drift — and their AI features are enterprise-only.
- **MLflow, W&B** track experiments. They don't monitor production behavior or close the loop back to CI/CD.

No open-source tool combines all three layers with autonomous remediation. AOTF's moat is integration — the cross-layer correlation that siloed tools structurally cannot provide.

**The Proactive Loop:** AOTF predicts and prevents, not reports and alerts. Pre-push risk scoring catches issues before they leave your machine. Autonomous fix PRs resolve them before they escalate. AI agent behavior monitoring detects drift before users do.

**Intelligence Compounds:** AOTF learns from every incident — building a failure library, codebase personality profile, and (opt-in) cross-org anonymized intelligence tier. The longer teams use AOTF, the more precisely it understands their system. All data stays local by default; the hosted intelligence tier is fully opt-in.

**CLI-Agent-First:** No SDK lock-in. AOTF wraps Claude Code CLI, Codex CLI, and Gemini CLI as interchangeable backends — swap your AI engine without changing your workflow.

## Project Classification

| Dimension | Value |
|---|---|
| **Project Type** | Developer tool + CLI tool (v1.0); WebUI dashboard in Growth |
| **Domain** | AI-powered DevOps / MLOps / LLMOps / AI Agent Operations platform |
| **Complexity** | High — autonomous remediation, multi-agent orchestration, ML lifecycle, plugin security |
| **Project Context** | Greenfield v1.0 — open-source ship |
| **Distribution** | `brew install aotf` / `curl install.aotf.dev` (single static binary) |
| **License** | Apache 2.0 (fully open-source, v1.0). Commercial tiers deferred to post-launch planning |

---

## Success Criteria

**North Star Metric: Weekly Active Teams** — the single number that unifies tradeoff decisions. Value compounds at the team level; individual installs are a leading indicator, not the goal.

**Weekly Active Teams — Operational Definition:**
- **Team identity:** SHA-256 of `(git remote URL + SHA-256 of committed `.aotf/config.yaml`)`. Two repos with identical config are counted as one team; two teams with the same repo but divergent configs are counted separately. This makes "team" a stable, local, privacy-preserving identifier.
- **Active threshold:** a team is Weekly Active if it performed ≥1 of the following within a trailing 7-day window: finding capture (FR-13), diagnosis (FR-19), fix apply (FR-32), rollback (FR-29), plugin install or trust update (FR-54). Passive watcher runs with zero findings do not count — the metric measures *engagement with the loop*, not daemon uptime.
- **Counting mechanism:** counted only via opt-in telemetry (NFR-S09 scope: command name + timing + team identity hash). Teams with telemetry disabled are *unmeasured, not assumed inactive* — reporting discloses the opt-in share each week so the published WAT number is transparent about coverage.
- **Dashboard surface:** admins view their own team's weekly activity via FR-103; aggregate cross-team WAT is reported only to the AOTF project itself, not to individual teams (no leakage of other teams' data).


**MTTR Definition:** Throughout this PRD, AOTF's MTTR means *time from first finding detected to fix-PR-merged*. Actual service recovery may include additional deployment or propagation time not attributable to AOTF. Reported MTTR improvements compare to user's opt-in pre-AOTF baseline where available.

**Target calibration approach:** Numeric targets in this PRD are *design anchors*, not contractual commitments. Final targets are calibrated from the first 60–90 days of real-world usage; community reporting (opt-in) adjusts published benchmarks accordingly.

### User Success (MVP Scope)

| ID | Criterion | Target | Measurement | Notes |
|---|---|---|---|---|
| US-01 | Time from `brew install aotf` to first actionable insight | <30 minutes (design anchor) | Opt-in onboarding session tracking | Primary TTV signal; calibrated from dogfood |
| US-02 | Error detection latency from log emission to storage | <60 seconds | Integration tests with timed error injection | Testable in CI |
| US-03 | AI root-cause diagnosis accuracy | >80% correct (design anchor) | Human review of sampled diagnoses; "correct" = root cause matches engineer's post-fix determination | Methodology: engineer reviews after closing the issue |
| US-04 | Auto-fix PRs that pass CI on first attempt | >75% (design anchor) | Track PR outcomes across dogfood repos and opt-in beta users | Calibrated from first 60 days of real use |
| US-05 | MTTR reduction visible to teams that opt in to baseline tracking | Team-reported improvement, no universal committed target | Opt-in pre-AOTF baseline collected at `aotf init`; tool computes delta per-team | AOTF instruments the measurement; teams interpret their own result |

### User Success (Growth Scope)

| ID | Criterion | Target | Pillar |
|---|---|---|---|
| US-06 | Agent behavior deviation caught before user impact | >80% of regressions caught proactively | AI Agent Operations |
| US-07 | Prompt regression detected within 1 deployment cycle | 1 cycle | ML/LLM Lifecycle |
| US-08 | Self-healing test repair without human intervention | >70% of broken selectors auto-repaired | AI QA Agent |
| US-09 | Model/agent drift alert latency | <24 hours at default thresholds | ML/LLM Lifecycle |

### Business Success

| ID | Criterion | Target | Timeframe | Notes |
|---|---|---|---|---|
| BS-01 | GitHub activity signal | Active issues + PRs + Discussions, trending upward | Monthly review | Community health; not a sales-driven star target |
| BS-02a | Published community plugins | Any community-contributed plugin | 12 months | Ecosystem signal |
| BS-02b | Community plugins with demonstrated use (install count reported) | 2+ plugins with reported use | 12 months | Validates ecosystem adoption beyond publication |
| BS-03 | Weekly active installs (teams that ran at least one AOTF command) | Measured and reported; target calibrated after 90-day dogfood window | 6 months | Anchor, not commitment |
| BS-04 | Plugin development time for new watcher or CI provider | <4 hours | Ongoing | Measured via contributor onboarding sessions |
| BS-05 | Teams using AOTF as first-response tool for >50% of production incidents | Qualitative signal from user interviews | 12 months | "Primary" = first tool opened when incident fires |
| BS-06 | 90-day team retention | Measured and reported; target calibrated in first 90 days | Ongoing | No committed target until baseline is established |
| BS-07 | Time to first actionable insight from install | <30 minutes | Onboarding tracking | Design anchor; measured via opt-in instrumentation |
| BS-08 | External (non-core-team) PRs merged to core | Demonstrable contribution from outside the core team | 12 months | Ecosystem health signal |

**Leading Indicator (weekly health check):** Errors reaching AI diagnosis stage per active install per week. If zero, detection pipeline is broken before AI contributes.

**Failure Criteria (pivot triggers):**
- 90-day retention below 30% at month 4 → diagnosis experience broken; fix UX before growth
- Auto-fix PR CI pass rate below 50% at month 3 → AI quality insufficient; gate fix behind diagnosis-only
- Zero community plugin submissions at month 9 → plugin DX broken; stop feature work and fix developer experience

### Technical Success

| ID | Criterion | Target | Notes |
|---|---|---|---|
| TS-01 | Watch daemon memory (10 log sources, 24h run) | <100MB RSS | Internal target: <50MB; 100MB is published SLA |
| TS-02 | CLI response time — non-AI ops (including config parse, SQLite read, plugin resolution) | <500ms | Excludes subprocess spawn for CLI agent backends |
| TS-03 | Watch daemon startup to first log read | <3s | Companion: daemon reconnection after crash <5s |
| TS-04 | Concurrent log sources with no degradation in TS-01/TS-02 | 20+ sources | Compound criterion: count + performance constraints both must hold |
| TS-05 | Core module test coverage | >80% branch coverage | Branch coverage; error-handling paths included |
| TS-06 | Rust binary startup including config file parse + plugin manifest load | <50ms | Meaningful target includes real init work |
| TS-07 | Plugin security: SHA-256 + allowlist; no plugin code executes before verification | 100% — zero silent loads | Enforced at Rust type-system level |
| TS-08 | Auto-fix safety gates enforced without exception before any merge operation | All configured gates pass | Enforcement guarantee; gate count can evolve |
| TS-09 | Agent backend failure isolation: watch daemon continues if CLI subprocess crashes or times out | Daemon uptime unaffected; timeout ≤30s | Most likely production failure mode |
| TS-10 | SQLite write durability under abrupt termination (SIGKILL) | No findings from last 30s silently lost | WAL mode + defined checkpoint interval |
| TS-11 | Plugin runtime boundary | Plugins may not access filesystem outside project root or make unapproved network calls | Enforced via capability declaration + import-hook |
| TS-12 | Structured log output | JSON lines; schema stable across minor versions; parseable by standard tooling | Critical for downstream integrations |

### Measurable Outcomes *(Post-MVP tracking targets — not MVP completion gates)*

Measured after 60 days of real-world usage and reported as product health indicators:

1. **Proactive ratio:** % of issues caught pre-production vs. post-production. Target: >50% within 6 months. Methodology: AOTF logs pre-push block events and post-deploy detections; ratio computed per team.
2. **Cross-layer correlation rate:** % of incidents where AOTF surfaces a correlation between application error and model/agent behavior change. Baseline TBD from first 60 days.
3. **Remediation closure rate:** % of detected issues fully remediated without human code changes (auto-fix PR merged + CI passed). Target: >40%. Requires feedback loop in GitHub Actions plugin.

---

## Product Scope

*Full phased roadmap with rationale, resource requirements, and risk analysis is in [Project Scoping & Phased Development](#project-scoping--phased-development). This section provides a summary and the notification event interface specification.*

**v1.0 (~14 weeks) — Proactive DevOps Loop.** Core CLI surface (init, watch, diagnose, fix, rollback). Streaming log watchers. AI diagnosis + auto-fix via Claude Code CLI. Plugin system (SHA-256 + capability scoping). Five-gate safety model. Telegram (both adapters). GitHub Actions. MCP server. SQLite + WAL. Single static binary.

**Growth (v0.2–v0.5)** — AI QA Agent, AI Agent Operations, additional CLI backends (Codex, Gemini), WebUI dashboard, polling log watchers, additional CI providers and notification channels.

**Vision (post-v1.0)** — ML/LLM Lifecycle Manager, own agent runtime (local models), optional cross-org intelligence pool (community-governed, opt-in), AOTF recipes. Commercial tiers (if any) are future-state planning; v1.0 ships fully open-source.

**Unified notification event interface:**
```
AotfChannelEvent {
  source: 'claude-plugin' | 'claude-bridge'
  chatId: string
  message: string
  agentId?: string        // bridge-specific
  worktreeId?: string     // bridge-specific
  sessionCost?: number    // bridge-specific
}
```

### Non-Goals / Scope Boundary

AOTF's primary execution surface is the **pre-production SDLC**. Production-side runbook execution is a downstream orchestration target — AOTF operates at the READ and PROPOSE action tiers against production environments (see FR-105), not as an owned autonomous-remediation capability. This is a deliberate market boundary, not a capability gap.

**Specifically out of scope for v1.0 and Growth phases:** head-on competition with AI-SRE agents (HolmesGPT, opensre, Azure SRE Agent, Datadog Bits AI SRE, PagerDuty AI Agent Suite, Resolve.ai, Cleric) on their primary ground — post-incident autonomous production remediation. AOTF integrates *with* these agents as an upstream signal provider (feeds pre-production risk signals, failed-test predictions, regressed-model detections) rather than replacing them.

**Re-evaluation triggers — direct SRE-territory expansion reconsidered if any ONE of the following is demonstrated:**

- **(a) Customer pull:** ≥3 enterprise design-partner organizations request post-incident autonomous remediation with signed LOIs
- **(b) Capital:** external funding ≥$10M USD earmarked specifically for autonomous-remediation R&D + integration breadth
- **(c) Distribution:** a hyperscaler (AWS, Azure, or GCP) formal co-sell partnership granting production API access
- **(d) Lost-deal signal (leading indicator):** ≥3 design-partner prospects explicitly refuse adoption because AOTF lacks production-close-the-loop capability

Absent any of the above, the expansion decision is not revisited before 2027-06-30. This boundary constrains architecture, epic breakdown, and go-to-market decisions; attempting parallel SRE-track development without the triggers will dilute v1.0 focus and is explicitly forbidden.

---

## User Journeys

---

### Journey 1: Alex — First Detection to Fixed PR *(MVP — Happy Path)*

**Persona:** Alex, 3 years backend experience at a 12-person startup. Ships fast, breaks things, monitors nothing.

**Opening Scene:** 4:30pm Friday. A teammate messages: "hey are the /orders endpoints down?" Alex's stomach drops. Demo in 90 minutes. Three Slack threads open. He installs AOTF.

```
$ brew install aotf
$ cd my-service && aotf init
→ Detected: Python FastAPI service, PostgreSQL, Redis
→ Created .aotf/config.yaml

$ aotf watch
→ Daemon started in 2.1s. Watching 3 sources.
```

Eight minutes after install, before Alex has read the docs:

```
[AOTF] ERROR: api-service — HTTP 500 at POST /orders
  NullPointerException in OrderService.create()
  3x in 90s (deduplicated). Stored: ERR-001. Telegram: sent.
```

Alex runs `aotf diagnose ERR-001`. Confidence 91%. Root cause: `order_id` not validated before DB lookup, introduced in commit `a3f9c2` (2h ago). He opens `order_service.py:47` to verify. Exactly right. *Huh.*

```
$ aotf fix ERR-001 --mode fix_and_pr
→ [Dry-run] +4/-0 lines | Blast radius: low | Reversible: yes
→ PR #47 opened. CI triggered. Telegram: "Fix PR ready."
```

Alex reviews, approves, merges. Demo goes fine. Total: 23 minutes from install to merged fix.

*"I've been manually grepping logs for two years. This just did in 10 minutes what used to take me an hour."*

**Requirements:** Auto-discovery, Docker log watcher, deduplication, AI diagnosis with confidence + file/line attribution, dry-run diff with reversibility flag, auto-fix PR, Telegram push.

---

### Journey 2: Priya — 3am Approval, Back Asleep by 3:10 *(MVP — Edge Case)*

**Persona:** Priya, senior SRE at a growing B2B SaaS startup. On-call every third week. Burned before by automation — agreed to AOTF trial with approval gates enabled for anything above low blast radius, and the approved-fix merge gated on passing CI + a deploy-approval on production.

**Opening Scene:** 3:07am. Telegram alert via claude-bridge: connection pool exhausted in the core service, 47x 503s in 3 minutes. Confidence 84%, blast radius MEDIUM.

Priya taps "View diff" — 3 lines in `database.py`. She recognizes the pattern. She sees: *confidence 84%* and *rollback available: `aotf rollback ERR-089`*. There's an out. She taps **Approve** at 3:09am.

CI passes at 4:12am. PR #203 auto-merged to staging. Production deploy is gated separately; the error rate in staging immediately drops, giving Priya a clean signal. She schedules the production deploy to run with the next rotation.

At 9am she reviews the audit log: `03:09 | priya | FIX_APPROVED | ERR-089 | via=telegram`. Post-mortem note: *time from detection to merged fix 65 min → 3 min. No engineer fully woken during detection and staging merge.*

**Requirements:** Bidirectional Telegram (approve/reject/view diff from phone), inline diff in claude-bridge, rollback availability notice, audit log with channel + IP, CI pass as merge prerequisite, configurable approval gate threshold.

---

### Journey 3: Jordan — Building the Loki Plugin *(Growth — Plugin Author)*

**Persona:** Jordan, platform engineer. Their company runs Grafana + Loki. Six months of GitHub issues asking for Loki support. Jordan decides to build it.

Reads the plugin guide for 20 minutes. Implements `LokiWatcher` extending `PollingLogWatcherABC` — three methods, clean interface. Runs `aotf plugin trust loki`, SHA-256 verified. First query returns results. Total build time: 3h 45min.

Submits to the plugin registry at 11:30am and goes to lunch.

Six weeks later, a DM from Singapore: *"your plugin just saved our incident response."* 67 teams installed. A contributor Jordan doesn't know submits a PR adding Loki label filtering.

**Requirements:** `PollingLogWatcherABC` with cursor persistence, `aotf plugin trust` + `plugins.lock.yaml`, capability declaration, plugin registry submission, community contribution workflow.

---

### Journey 4: Sam — Team Setup and the VP Conversation *(MVP — Admin/Manager)*

**Persona:** Sam, engineering manager, 25-person team. Quiet fear: team is burning out on on-call toil. One bad quarter away from losing someone.

Configures multi-user tokens with roles. Sets up claude-bridge for team Telegram group. One engineer (Kenji) opts out of auto-fix initially — Sam respects it with per-service mode override. Two weeks later Kenji opts back in after watching Alex's workflow.

Week 4 digest: 23 errors captured, 21 diagnosed, 18 fix PRs auto-opened, 16 merged on first CI pass (76%), detection-to-merge MTTR 18 min vs 74 min opt-in baseline. Shows VP: detection-to-merge reduced 76% against the team's own baseline. *"Can we expand this to the infra team?"*

**Requirements:** Team init, multi-user token management, role-based scopes, per-service mode override, weekly digest, MTTR baseline tracking, team Telegram group notifications.

---

### Journey 5: Maya — The Near Miss *(Growth — AI Agent Operations)*

**Persona:** Maya, engineer at an AI-native startup. Their Claude-powered summarization service got a "tightening" prompt update (v1.4). Metrics look fine. No errors. No alerts. She would have found out when a customer complained.

Four days after deploy, AOTF detects: avg output tokens dropped 52%. Shadow pipeline comparison — v1.3 vs v1.4 on 100 real inputs — shows user acceptance rate 87% vs 64%. Correlation: prompt v1.4 deployed 4 days ago.

A teammate pushes back: "the shorter summaries are intentional." Maya checks the shadow data. 64% vs 87%. Not intentional enough.

```
$ aotf ml prompt rollback summarizer --to v1.3
→ Shadow test on 5% traffic: acceptance recovering 71% → 83%. Rollback complete.
```

The regression had been live 4 days. One customer complaint was already filed 6 hours before the AOTF alert. *Barely caught it.*

**Requirements:** Agent run log ingestion, behavioral baseline tracking, deviation detection, shadow pipeline comparison, prompt version tracking, rollback command, override/dismiss gesture for contested findings.

---

### Journey 6: AOTF Gets It Wrong — Rollback *(MVP — Failure Path)*

**Persona:** Alex, two weeks after his first AOTF success. He's developed trust — maybe too much.

AOTF fires on ERR-112 in billing service (Stripe webhook failures, confidence 73%). Alex approves the fix quickly from a meeting — 20-second glance. PR #51 merges. CI passes.

Twenty minutes later: *"billing webhooks are still failing — actually more now."* The fix changed webhook validation logic with an incorrect assumption about Stripe's header format. CI passed because unit tests mocked the Stripe client.

```
$ aotf rollback ERR-112
→ PR #51 is merged. Creating revert PR.
→ PR #52 opened: "revert: undo fix for ERR-112"
→ CI running... PASSED. Merged. Webhooks recovering.
```

Alex updates the team config: `approval_required_below_confidence: 80`. Adds to post-mortem: *"AOTF confidence below 80 means human review required. Don't approve from a meeting."*

The rollback worked cleanly. Trust wasn't broken — it was calibrated.

**Requirements:** Rollback manager (revert PR for merged fixes), pre-revert state recording, confidence threshold as configurable gate, audit trail showing who approved, Telegram rollback notification.

---

### Journey 7: Marcus — Goal Loop for Complex Remediation *(MVP — Telegram + claude-bridge)*

**Persona:** Marcus, senior backend engineer. Uses AOTF via claude-bridge's Goal Loop — he wants AOTF to keep trying until CI actually passes, not just make one attempt.

AOTF detects a race condition in auth service (confidence 61%, below auto-approval threshold). Marcus types in the team Telegram group:

```
/aotf goal ERR-134 --loop until:ci-pass --max-attempts 3
```

claude-bridge runs a Goal Loop in an isolated worktree:

- **Attempt 1:** Fix applied → CI fails (lock scope too narrow, still racing on cache)
- **Attempt 2:** Re-diagnoses with CI failure context → extends lock to cover `cache.invalidate()` → CI passes ✅

21 minutes later, Telegram summary:

```
✅ Goal Loop Complete — ERR-134
  Attempts: 2/3 | CI: passed | Cost: $0.047
  PR #67 ready. [Review] [Approve merge] [Reject]
```

Marcus reviews on his phone. Correct fix. Approves.

*"It's like pair programming where the pair doesn't need coffee breaks."*

**Requirements:** claude-bridge Goal Loop integration, worktree isolation per attempt, CI outcome fed back to next diagnosis attempt, max-attempts ceiling, per-loop token cost tracking, Telegram goal completion summary with cost breakdown.

---

### Journey 8: On-Call Team — Collaborative Telegram Triage *(MVP — Team Coordination)*

**Persona:** 4-person on-call rotation sharing a `#incidents` Telegram group. Cultural contract: primary on-call approves; others can weigh in.

AOTF posts a P1 alert to the group: checkout-service payment processor timeouts (confidence 77%, blast radius HIGH). Three engineers see it simultaneously.

Thread discussion unfolds naturally:
- Kenji: *"that's the right fix, I saw the SLA email this morning"*
- Priya: *"let's check impact first"* → runs `/aotf simulate ERR-201 --impact-check`
- AOTF responds: p99 latency +6.2s worst case, SLA headroom confirms it fits
- Priya approves

Fix merged in 12 minutes. The Telegram thread is the audit trail.

**Requirements:** AOTF posting to group Telegram chats, `/aotf simulate` impact check, multi-person approval visibility (who approved + timestamp), reassignment command, group context forwarded to AOTF core.

*Note: the "primary on-call approves first" cultural contract is encoded via role-based capability scopes (FR-49) — only users holding the on-call role token for the service can issue the final approval; others' comments are visible but non-authoritative. The PRD does not require AOTF to enforce the cultural norm beyond this role-based gate.*

---

### Journey 9: Priya — Incident Command via WebUI Dashboard *(Growth — Dashboard)*

**Persona:** Priya during a P0 — five services degraded simultaneously. CLI isn't enough.

```
$ aotf dashboard
→ http://localhost:8080 | Authenticated: priya (operator)
```

Real-time error timeline across all 6 services. Cross-service correlation graph makes the cascade immediately visible: cache eviction → auth retry storm → DB pool exhaustion → payment timeouts.

Priya clicks into the root cause (ERR-201), marks it "Investigating," assigns to herself. Status updates for everyone watching. She dispatches two engineers via Telegram directly from the dashboard. Approves the fix — the status tracker shows CI progress bar updating live: *4 of 9 test suites passing...*

At 11:47am all services green. The full cascade timeline is available as post-mortem export.

*"During a P0, dashboard is where I coordinate. Telegram is where I alert. CLI is where I configure."*

**Requirements:** Real-time error timeline (SSE streaming), cross-service correlation graph, fix status tracker with live CI progress, "mark as investigating" + assignee, Telegram dispatch from dashboard, post-mortem export, `aotf dashboard` CLI launch, operator-scoped auth.

---

### Journey 10: David (VP Engineering) — Executive Health View *(Growth — Dashboard)*

**Persona:** David, VP Engineering. Doesn't use CLI. Sam shares a read-only dashboard link before every quarterly review.

Opens the dashboard with a viewer-scoped token. Executive view — three panels (values below are example output from one team's 90-day window, not committed product targets):

**Operational Health:** MTTR trend Jan 52min → Feb 31min → Mar 18min. 312 incidents captured, portion auto-resolved without human intervention reported per team.

**AI Quality:** Diagnosis accuracy, fix-PR CI pass rate, rollback rate — each computed from this team's actual outcomes.

**Cost & Efficiency:** Engineering hours saved estimate (from opt-in baseline), total AI token cost, cost-per-resolved-incident — computed from this team's usage, not a universal benchmark.

David screenshots the cost panel for the board deck. Asks Sam: *"Can we get this to the infra team?"*

**Requirements:** Read-only viewer scope, shareable URL with token auth, quarterly time range selector, executive metrics aggregation, MTTR trend visualization, token cost per incident, dashboard export/screenshot layout.

---

### Journey 11: Priya — Proactive Pre-Push Prevention *(MVP — Pre-Production Authorization)*

**Persona:** Priya, the same senior SRE from J2 — six weeks into AOTF adoption. Her team has moved on-call MTTR from 65min to 3min on reactive fixes. She's now evaluating whether AOTF can prevent the incidents from shipping in the first place.

**Opening Scene:** 10:47am. A backend engineer (Amir) opens a PR adding a new migration: `ALTER TABLE orders DROP COLUMN legacy_status;`. Unit tests pass locally. Amir runs `git push`.

```
$ git push origin feat/drop-legacy-status
→ [AOTF pre-push hook] Analyzing 1 migration, 3 code changes...
→ Finding: ERR-PRE-042 (risk: HIGH, blast: IRREVERSIBLE, confidence: 88%)
    — Column `legacy_status` referenced in billing-service:reports.py:89
    — Column present in 3 of last 30 days of prod telemetry (non-zero reads)
    — Migration is irreversible without backfill
→ Action tier classified: EXECUTE_IRREVERSIBLE (schema change, prod-designated)
→ Push BLOCKED pending authorized consent.
→ Telegram: approval requested from on-call (Priya).
```

Priya's phone buzzes. Inline diff, risk breakdown, the single sentence that matters: **"This column is still being read in billing-service."** She does not approve. She replies in Telegram: `reject — coordinate with billing team first`. Amir gets the rejection with the evidence pointer, opens a coordination ticket, and the migration is held.

Three days later, after the billing team removes the reference, Amir re-pushes. AOTF re-runs the pre-push assessment:

```
→ [AOTF pre-push hook] Analyzing 1 migration, 0 new references detected.
→ Finding: ERR-PRE-042 (risk: LOW, blast: IRREVERSIBLE, confidence: 94%)
→ Action tier: EXECUTE_IRREVERSIBLE — production consent token required.
→ Priya requests token via CLI (WebAuthn touch on operator key):
    $ aotf consent issue --env prod --scope migration --ttl 1h
    → Token: cnst_0x8f9a... | Expires 2026-04-17T11:47Z | Single-use
→ Token presented; migration pre-approved.
→ Push PROCEEDS. Audit log: approver=priya, reason="billing dep removed, 0 refs verified"
```

The migration ships safely. Audit log captures approver identity, timestamp, stated reason (≥20 chars), and the WebAuthn assertion hash.

*"The two-step ceremony felt annoying the first time. After it caught a breaking migration, it became the most-loved line in our CI config."*

**Requirements:** Pre-push git hook (FR-05), pre-push risk assessment with cross-code/telemetry correlation (FR-14), action-tier declaration per operation (FR-105), production consent token with WebAuthn attestation (FR-106), strengthened audit log with approver/timestamp/reason on EXECUTE_IRREVERSIBLE (FR-66), Telegram bidirectional approval for pre-push findings, production-designated environment gating (NFR).

---

### Journey 12: Lin — AI QA Agent Self-Healing Test Run *(Growth — AI QA Agent)*

**Persona:** Lin, QA lead at a 40-person SaaS company. Weekly shadow QA runs Playwright against staging. Every time the login form UI shifts, half the selectors break and she spends a morning re-anchoring them.

**Opening Scene:** Monday 9am. Lin kicks off the weekly shadow QA run against staging.

```
$ aotf qa run --target staging --suite smoke
→ Action tier: EXECUTE_REVERSIBLE (staging-designated, non-irreversible)
→ 28 scenarios queued. Browser session: Chromium, AI narrator: enabled.
```

Three scenarios in, a selector fails: `button[data-testid="login-submit"]` not found. The UI team renamed it to `login-cta-primary` on Friday. AOTF's AI QA Agent reads the DOM, the previous selector history, the visible text, and the surrounding semantic context. It identifies the probable replacement, proposes a self-heal:

```
→ [Selector repair] data-testid=login-submit → data-testid=login-cta-primary
    Evidence: visible text "Sign In" unchanged, ARIA role=button, Friday commit
    3dba91 renamed selector but kept semantics
→ Confidence: 92%. Applying to test + retrying scenario.
→ Scenario PASSED. Finding: QA-087 (selector drift, auto-repaired).
```

Lin gets a Telegram summary at 9:14am: 26/28 scenarios passed, 1 auto-repaired (QA-087), 1 real regression (QA-088: checkout total incorrect). She reviews QA-088 — genuine bug; AOTF has already generated a regression test capturing the bad state. She files a ticket with the evidence.

Later that week, a customer submits a bug report: "password reset email links expire immediately." Lin types:

```
$ aotf qa from-report "password reset email links expire immediately"
→ Draft regression test generated from report text.
→ Action tier: DRAFT (no execution yet) — review required.
→ Review: tests/regression/qa_091_reset_email_expiry.ts
```

Lin reviews, tweaks the timing assertion, commits. The next shadow run includes it. Two weeks later, a dev change re-breaks it, and AOTF catches it before customer filing.

*"The selector stuff used to eat a morning a week. Now the only QA time I spend is on the regressions that matter."*

**Requirements:** Autonomous browser-driven test execution (FR-71), selector self-repair with semantic evidence (FR-72), user-bug-report to regression test conversion (FR-73), action-tier classification on QA operations (FR-105), shadow-mode execution against replica (FR-75), visual regression detection (FR-74), Telegram QA summary, evidence-backed audit trail for auto-repair decisions.

*Note: Production-target QA runs (not staging) would invoke FR-106 consent-token gating because action-tier is EXECUTE_IRREVERSIBLE when against production. Staging-like-prod is explicitly NOT production per the production-designated binary constraint.*

---

### Edge Case Journey Stubs

| # | Scenario | Key Assumption Tested | Phase |
|---|---|---|---|
| J-S1 | New dev onboarding to existing AOTF setup | Day-2 retention; second user has <10 min onboarding | MVP |
| J-S2 | Alert storm — 47 errors fire simultaneously | Notification batching prevents fatigue; 1 Telegram summary not 47 | MVP |
| J-S3 | Onboarding failure — GitHub token insufficient permissions | Error message quality at highest-churn moment | MVP |
| J-S4 | Permission conflict — repo excluded by org policy | Scope control and compliance accommodation | MVP |

---

### Journey Requirements Summary

| Capability Area | Journeys | Phase |
|---|---|---|
| Auto-discovery, project init, permission validation | J1, J-S3 | MVP |
| Streaming log watchers (file, Docker) | J1 | MVP |
| Error deduplication + batched notification | J1, J-S2 | MVP |
| AI diagnosis with confidence + file attribution | J1, J2, J6 | MVP |
| Dry-run diff + reversibility flag | J1, J2 | MVP |
| Auto-fix branch + PR | J1, J2, J6 | MVP |
| Rollback manager (revert PR for merged fixes) | J6 | MVP |
| Five-gate safety model + configurable confidence threshold | J2, J6 | MVP |
| Telegram push + bidirectional (approve/reject/view diff) | J1, J2, J8 | MVP |
| claude-bridge Goal Loop (multi-attempt, worktree, CI feedback) | J7 | MVP |
| Per-loop token cost tracking + Telegram cost summary | J7 | MVP |
| Group Telegram posting + threaded triage | J8 | MVP |
| `/aotf simulate` impact check — appears in J8 as optional flourish; scoped Growth | J8 | Growth (J8 core flow works without it) |
| Multi-approver visibility (who approved, when) | J8 | MVP |
| Audit log + `aotf user audit` | J2, J6 | MVP |
| Multi-user tokens + roles + per-service mode override | J4, J-S4 | MVP |
| Weekly digest + MTTR baseline tracking | J4 | MVP |
| New team member onboarding flow | J-S1 | MVP |
| Notification rate limiting + batching | J-S2 | MVP |
| Plugin security (allowlist, SHA-256, capabilities) | J3 | MVP |
| Polling log watchers (Loki) | J3 | Growth |
| Real-time error timeline (SSE) + correlation graph | J9 | Growth |
| Fix status tracker with live CI progress | J9 | Growth |
| "Mark as investigating" + assignee + Telegram dispatch | J9 | Growth |
| Post-mortem export from dashboard | J9 | Growth |
| Executive metrics view + read-only viewer scope | J10 | Growth |
| Token cost per incident tracking | J10 | Growth |
| AI agent run tracking + behavioral baseline | J5 | Growth |
| Agent deviation detection + shadow pipeline | J5 | Growth |
| Prompt version tracking + rollback | J5 | Growth |
| Pre-push git hook + pre-push risk assessment (FR-05 + FR-14) | J11 | MVP |
| Action-tier reversibility taxonomy + policy matrix (FR-105) | J11, J12 | MVP |
| Production consent token with WebAuthn attestation (FR-106) | J11 | MVP |
| Strengthened audit log — approver / timestamp / reason on EXECUTE_IRREVERSIBLE (FR-66) | J11 | MVP |
| Autonomous browser-driven QA + self-healing selectors (FR-71, FR-72) | J12 | Growth |
| User-bug-report to regression test conversion (FR-73) | J12 | Growth |
| Shadow-mode QA run against staging replica (FR-75) | J12 | Growth |

---

## Domain-Specific Requirements

### Autonomous Code Execution Safety

**This is AOTF's highest-stakes domain constraint.** AOTF can autonomously modify, commit, and push code. This requires a layered defense model, not just "ask before doing."

| Control | Requirement |
|---|---|
| **Blast radius limiter** | Auto-fix scope is bounded: single file edits only by default; multi-file requires explicit `--scope=multi` flag |
| **Confidence floor** | Fixes below 0.75 confidence require human approval (user-configurable). Confidence is AOTF-computed — NOT AI self-reported — via weighted formula: `confidence = 0.40 × static_analysis_agreement + 0.40 × test_signal + 0.20 × (1 − normalized_diff_complexity)`. Initial weights and 0.75 threshold are v1.0 defaults; to be calibrated from opt-in anonymized reporting |
| **Dry-run first** | Every fix plan rendered as a diff before execution. In interactive mode: displayed to user. In CI (`--auto-approve`): logged to audit trail and diff file — execution proceeds but the record is immutable |
| **Rollback guarantee** | Every auto-fix creates a tagged rollback ref before committing; `aotf rollback --last` always works. Startup integrity check detects and resolves any in-flight operation state from a crashed previous run |
| **Fix mutex** | Only one concurrent auto-fix per repository; prevents compound failures |
| **Goal Loop guard** | claude-bridge Goal Loop capped by three ceilings (all user-configurable): 3 attempts, 20 minutes wall time, $1.00 per-loop AI token cost. First ceiling hit terminates the loop. On cap: finding marked `human-required`, Telegram alert sent, final diff + CI trace preserved — not a silent failure |
| **Approval gate** | Interactive and non-interactive approval modes: `--auto-approve` for CI, default interactive for local |
| **Action cooldown** | Per-repo cooldown of 60 seconds between autonomous fix triggers (configurable); prevents runaway watcher/misconfiguration loops even within the attempt cap |
| **Auto-fix failure recovery** | When a fix fails post-apply (compilation error, test regression, git conflict): rollback ref invoked automatically, failure reason recorded with diff, finding escalated to `human-required` status |

### Auto-Fix Decision Logging

Every autonomous fix action records a structured decision log entry — this is the foundation of developer trust in an autonomous system:

| Field | Content |
|---|---|
| `finding_id` | Unique ID of the detected issue |
| `confidence_score` | Computed score with component breakdown (static analysis, diff complexity, test signal) |
| `diff` | Full before/after diff |
| `ai_reasoning_trace` | AI backend response captured verbatim as provided by the CLI. PII filtering applied before storage using the same filter as log outputs (FR-68 / NFR-D01). No AI-based summarization in v1.0 |
| `outcome` | `applied`, `rolled-back`, `human-required`, `cap-exceeded` |
| `timestamp` | ISO 8601 |

This log is queryable locally and is not optional.

### Security Requirements

| Area | Requirement |
|---|---|
| **API key handling** | Secrets stored in OS keychain. Fallback chain: macOS Keychain → Linux libsecret (interactive session) → SOPS encrypted file (headless/CI) → environment variable (last resort, logged as warning). Docker and GitHub Actions runners must use env var or SOPS path |
| **Plugin supply chain** | `plugins.lock.yaml` with SHA-256 checksums. Verification model: **full SHA-256 at install and at daemon startup**; a verified-plugins cache is held in memory for the daemon's lifetime to satisfy CLI performance targets. Cache invalidated on any `plugins.lock.yaml` change (detected via mtime + checksum) — which triggers full re-verification. Import-hook runtime enforcement. Capability scoping (filesystem, network, git per-plugin). On plugin update: new capabilities require re-consent |
| **Webhook authentication** | GitHub webhooks validated via HMAC-SHA256 signature; reject all unsigned payloads |
| **Git credential safety** | AOTF never stores or transmits git credentials; passes through to system git credential helper only |
| **MCP authorization** | AOTF MCP server is spawned by the CLI binary as a child process (not a separate daemon); binds to localhost only. `fix` tool calls require an explicit authorization token. MCP server exposes capability tier: read-only vs write. No MCP client can invoke `fix` without the same safety gates as the CLI path |
| **claude-bridge security** | Telegram webhook validated; bot token stored in keychain; per-user Telegram ID allowlist for multi-user setups; all bot commands logged to audit trail |
| **Audit log** | First-class capability: immutable, append-only log of every AOTF action (fix, rollback, plugin install, config change, user auth). Required for SOC 2 / ISO 27001 evaluation. Available from first release |

### Privacy Requirements

| Area | Requirement |
|---|---|
| **Log PII filtering** | PII filtering is **ON by default**. Code snippet content in logs is filtered/hashed unless user explicitly opts out via `--log-level=verbose` |
| **AI backend data transmission** | Code is transmitted to the AI provider when using CLI backends. Users acknowledge this on first fix action per repo (not just on binary install). Consent stored per-repo in `.aotf/consent.yaml` |
| **Telemetry** | All telemetry opt-in; anonymized; `--no-telemetry` disables entirely. No telemetry in CI mode unless explicitly enabled |
| **Cross-org intelligence** | Explicitly opt-in; described during onboarding with clear data flow explanation |
| **Data sovereignty** | Self-hosted deployment: no phone-home telemetry, no third-party CDN dependencies, documented data flow diagram published. Full data residency claim requires all three; "self-hosted" alone is insufficient |

### Worktree Isolation Contract

claude-bridge uses worktree isolation per agent task. Isolation boundaries are explicit requirements:

| Boundary | Requirement |
|---|---|
| **Filesystem** | Agent in a worktree has read/write access only to its worktree path; parent repo's `.env`, credential files, and `node_modules` are explicitly excluded via path allowlist |
| **Package installs** | Agents may not install packages into the parent repo's dependency tree; any installs are worktree-scoped and discarded on cleanup |
| **Secret access** | Worktree agents inherit no secrets from the parent environment except explicitly passed context |
| **Cleanup** | On task completion (success or failure), worktree is deleted; no residual state persists |

### Open-Source Trust Model

| Concern | Requirement |
|---|---|
| **License** | Core AOTF: **Apache 2.0** (chosen for explicit patent grant termination clauses required for enterprise legal review). Plugins: author's choice. Intelligence tier: proprietary |
| **Plugin ecosystem trust** | Community plugins require AOTF Hub submission + security scan before appearing in default registry. Plugin capability grants re-verified on update; new capabilities require re-consent |
| **Reproducible builds** | Rust binary built with locked `Cargo.lock`; released via GitHub Actions with SLSA provenance attestation |
| **Dependency audit** | `cargo audit` and `bun audit` in CI for known CVEs; blocking on critical/high severity |
| **SBOM generation** | Roadmap item (post-v1.0): AOTF generates SBOM for analyzed repos (EU Cyber Resilience Act compliance track) |
| **Fork safety** | Anyone can self-host the full intelligence tier with their own anonymized data pool |

### Integration Constraints

| Integration | Constraint |
|---|---|
| **GitHub API rate limits** | All GitHub API calls respect 5000 req/hour (authenticated); batch operations use conditional requests and ETags; rate limit headroom monitored |
| **git atomicity** | All git operations are atomic (commit + tag together); startup integrity check resolves partial state from previous run before proceeding |
| **CLI version compatibility** | Claude Code CLI minimum version documented and validated on startup; graceful error if unmet |
| **MCP protocol versioning** | AOTF MCP server declares supported protocol version; graceful degradation for older MCP clients |
| **Telegram Bot API** | Rate limits: 30 msg/sec global, 1 msg/sec per chat; queue-based dispatch required; backpressure handling documented |
| **Multi-repo resource limits** | AOTF caps concurrent active repos at 10 and concurrent AI backend calls at 3 (configurable) to prevent API quota exhaustion and CI infrastructure overload |
| **AI backend degraded mode** | On AI provider outage: AOTF falls back to detection-only mode automatically. Pending fix tasks are queued with TTL. Telegram notification sent. Degraded mode is a documented, tested state — not a silent failure |

### Domain-Specific Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Auto-fix breaks production build | High | Dry-run mandatory; rollback ref; startup integrity check; CI gate before auto-push |
| Developer distrust of autonomous changes | High | Decision log with reasoning trace; transparent diff previews; opt-in modes; human-in-the-loop default |
| Plugin supply chain compromise | Critical | SHA-256 + attestation; capability scoping; import-hook runtime enforcement; re-consent on update |
| AI backend provider outage | Medium | Detection-only fallback mode; task queue with TTL; Telegram alert |
| Personal data in code snippets transmitted to AI | High | PII filtering ON by default; per-repo consent prompt before first transmission |
| Goal Loop cap exceeded without resolution | High | Defined terminal state: `human-required` status + Telegram escalation; no silent failure |
| claude-bridge unauthorized access | Critical | Telegram user ID allowlist; webhook HMAC validation; audit log; per-user authorization |
| Worktree escape / filesystem boundary violation | High | Explicit path allowlist per worktree; no secret inheritance; cleanup on task end |

---

## Innovation & Novel Patterns

### Primary Innovation Claims

**1. Open-Source Autonomous Remediation — No Hosted Service Required**

The 2026 AI-SRE and autonomous-SDLC landscape has two distinct cohorts, and neither currently offers what AOTF does. The **commercial cohort** (Resolve.ai — $125M Series A @ $1B valuation, March 2026, deployed at Coinbase / MongoDB / Salesforce; Cleric; Datadog Bits AI SRE; PagerDuty AI Agent Suite; Azure SRE Agent) is SaaS-first, enterprise-priced, and structurally dependent on telemetry/data gravity — the business model forbids open-sourcing the core. The **OSS cohort** (HolmesGPT — 2.2k★, CNCF Sandbox, Microsoft contributions; k8sgpt — 5k★, CNCF Sandbox; Keep — 11.6k★ AIOps/alert management; Tracer opensre — 1.1k★) is genuinely open-source but **operates at the post-incident SRE layer**: observation, investigation, and suggested remediation, not autonomous pre-production action. AOTF's position — open-source + autonomous-remediation + **pre-production SDLC surface** + cross-layer (code + AI agent + model/prompt) — is not replicated by either cohort as of this PRD version.

The innovation is not the Apache 2.0 license. It's that AOTF's detection, diagnosis, fix, rollback, audit, and learning loop all run on the user's machine. There is no AOTF-hosted service to sign up for, no telemetry routed through AOTF infrastructure, no remote session state. Code does reach the user's chosen AI CLI provider (Anthropic for Claude Code CLI in v1.0) — that boundary is explicitly disclosed and will be closed by local-model backends on the roadmap (Ollama, llama.cpp).

*Why this matters to engineers:* OSS matters to developers not as a philosophy but as a trust prerequisite for code-modifying tools. When AOTF is about to open a PR against your repo, you can read exactly what it does. That's not a feature — it's the condition under which any autonomous tool earns trust.

*The `brew install, no account` mechanic:* removes the "I need to ask my manager to approve a SaaS vendor" friction. An individual engineer can evaluate it Friday afternoon without involving procurement. This is a structural wedge into regulated industries, air-gapped environments, and security-conscious teams that SaaS competitors cannot serve without rebuilding their stack.

*The actual moat:* Not the license — the **community failure library** and **plugin ecosystem network effects**. Every team that opts into the cross-org intelligence tier makes the failure library more useful for every other team. Every plugin contributed to the ecosystem compounds the platform's reach without requiring AOTF to build it. The Apache 2.0 license is the precondition that enables these to be contributed freely.

*The PLG ladder:*
- **Individual** (`brew install`) → proves value in 30 minutes, no commitment
- **Team** → shared `.aotf/` config, shared plugin registry, Telegram group notifications
- **Enterprise** → cross-org intelligence, audit log export, SLA, SSO — each stage makes the next obvious, not required

*Practitioner identity:* AOTF creates a new role — *the engineer who automates SDLC hygiene*. dbt named the "Analytics Engineer" and owned that community. AOTF should name and own its practitioner identity early.

*Plugin ecosystem prerequisite:* The plugin moat requires 5–10 well-maintained first-party plugins (GitHub Actions, Docker, file tail, Loki, Telegram, PagerDuty) that prove the interface is stable before community contribution becomes self-sustaining.

*Local LLM roadmap note:* The "zero cloud" claim has an asterisk until local model backends (Ollama, llama.cpp) are first-class. Currently AI diagnosis routes code to Claude Code CLI (Anthropic). Local model support is on the roadmap as a first-class backend to fully close data sovereignty for air-gapped environments.

---

**2. CLI-Agent-First — The Subprocess Abstraction**

AOTF wraps Claude Code CLI, Codex CLI, and Gemini CLI as interchangeable backends via subprocess, committing to subprocess-wrapping as a first-class design principle with a stable interface contract (structured JSON over stdout, versioned schema).

This sidesteps the SDK abstraction war entirely — no LangChain, no LlamaIndex, no framework lock-in. Teams swap AI engines (or run multiple) without code changes. **Positioned to outlast framework consolidation.**

*Architectural contract:* Interface specification published as a versioned schema. Conformance test suite validates each supported backend version. Version pinning + minimum version validation on startup; fallback to diagnosis-only if CLI unavailable.

---

**3. Goal Loop — Verified Resolution**

Multi-attempt autonomous remediation (worktree isolation per attempt, CI result fed back to next diagnosis attempt) changes the unit of value from "suggestion" to "verified resolution."

*Termination contract:* 3-attempt ceiling + per-loop time budget (default: 20 minutes) + cost ceiling per invocation. On cap hit: `human-required` status, Telegram escalation, final diff + CI trace preserved for human review.

*Convergence model:* Re-diagnosis at each attempt uses the previous CI failure output as context — enabling multi-step repair of complex issues that require structural changes in sequence.

---

**4. Autonomy UX — Calibrated Trust Architecture (Seven-Control Trust Surface)**

AOTF's **seven-control trust surface** combines *five pipeline gates* (Dry-Run, Scope Limit, Approval, Mutex, Rollback — sequential, blocking, per-action) with *two cross-cutting controls* (Audit — append-only observability sink active on every action regardless of outcome; Consent Token — authorization artifact consumed by the Approval gate when action-tier is `EXECUTE_IRREVERSIBLE`). Combined with per-finding confidence scores and an immutable decision log, this creates a trust ratchet: each successful autonomous fix raises the team's configured confidence threshold. Teams tune the autonomy dial per-service without losing safety guarantees.

**Typing:** the five pipeline gates are *runtime policy decision points* that gate whether an action proceeds (pass/fail). The two cross-cutting controls are categorically different: Audit is an observability sink that every gate emits into (a tap on every step, not a step you pass through); Consent Token is an authorization input that the Approval gate consumes when tier demands it (not a peer gate). This typing is the architectural source of truth — the seven-control surface is the enterprise-legible pitch artifact. FR-23 through FR-27 define the five pipeline gates; FR-66 defines Audit; FR-106 defines Consent Token. The action-tier taxonomy (claim #5) is the *type system* that selects which controls fire for which actions.

---

**5. Authorized Pre-Production Autonomy — Action-Tier Reversibility Taxonomy**

Every autonomous AOTF operation is classified at dispatch time into one of five **action tiers**: `READ` (safe, audit-only), `DRAFT` (creates artifact, not published), `PROPOSE` (opens PR/suggestion, human reviews), `EXECUTE_REVERSIBLE` (takes action, rollback exists), `EXECUTE_IRREVERSIBLE` (permanent — requires production consent token + audit capture of approver/timestamp/reason). The taxonomy is a *type system* over actions; the five-gate model (claim #4) is the *runtime policy* that consumes tiers and selects the gate matrix per tier.

*Policy matrix (tier → mandatory controls):*

| Tier | Audit | Dry-Run | Scope Limit | Approval Gate | Mutex | Rollback | Consent Token |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| READ | ✓ | — | — | — | — | — | — |
| DRAFT | ✓ | — | ✓ | — | — | — | — |
| PROPOSE | ✓ | ✓ | ✓ | — | — | — | — |
| EXECUTE_REVERSIBLE | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | — |
| EXECUTE_IRREVERSIBLE | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | **✓** |

*Column typing (see claim #4 for full treatment):* **Pipeline gates** (5): Dry-Run, Scope Limit, Approval Gate, Mutex, Rollback — sequential blocking decision points per action. **Cross-cutting controls** (2): Audit — append-only observability sink active on every gate regardless of outcome; Consent Token — authorization artifact consumed by the Approval gate for `EXECUTE_IRREVERSIBLE` tier, not a peer gate. The seven-column surface is the enterprise-legible trust artifact; the 5+2 typing is the architectural decomposition (see [architecture.md](./architecture.md) §Trust Boundary).

*Enforcement:* Declaration lives in the plugin capability manifest (`actionTier` field). Enforcement lives in the Rust orchestrator's FFI boundary (`crates/orchestrator/src/plugin_bridge.rs`) — the tier check occurs *before* dispatch, against the signed manifest, regardless of whether the plugin called through the TS SDK or bypassed it. SDK annotations are DX sugar, not the source of truth. The orchestrator owns a versioned tier-classification table (`crates/orchestrator/src/tier_map.rs`) that maps concrete syscalls and tool invocations to tiers — without this table, enforcement would be vacuous.

*Why this is distinctive:* As of this PRD version, none of the 10 surveyed competitor agents (Tracer opensre, Microsoft sre-agent, k8sgpt, HolmesGPT, Keep, Resolve.ai, Cleric, Datadog Bits AI SRE, PagerDuty AI Agent Suite, Azure SRE Agent) has productized a multi-tier action taxonomy as a first-class design constraint. HolmesGPT's "read-only + RBAC + memory-safe" posture is the closest public stance in the OSS cohort — but it is a single-tier stance, not a taxonomy. The moat window is estimated at 12–24 months before imitation converges.

*Production-designated is binary:* "Production" is a per-environment binary designation. Production-like staging, canary environments, and shadow replicas are explicitly NOT production for consent-token purposes. This prevents the blast-radius creep that would collapse consent-token security if engineers built a token-vending service to handle high-frequency staging runs.

*Interaction with `--force-local-auto-approve`:* The force flag bypasses the five enforcement gates (FR-23 through FR-35) for low-friction local use, but **never** bypasses the FR-105 tier ceiling for `EXECUTE_IRREVERSIBLE` actions. Irreversible action always requires tier enforcement — no flag route around it.

---

### Strategic Moat Opportunities *(future bets, not current innovations)*

**Intelligence Compounds:** Failure library + codebase personality profile create compounding accuracy. Local model: learns from your specific system's failure patterns. Cross-org tier (opt-in): anonymized failure signatures shared across teams. Data boundary: local learning stays entirely local; cross-org transmits only anonymized signatures, not code.

**CNCF / Linux Foundation Sandbox Pathway:** Both k8sgpt and HolmesGPT validated the CNCF Sandbox path as the OSS-infra credibility signal for AI-SRE tooling. AOTF's candidate submission criteria: ≥1 deployed production user outside the core team, documented governance model, first-party plugin set stable, action-tier taxonomy spec published. Target timeline: 6–12 months after v1.0 ship. Sandbox acceptance is a distribution and procurement-risk signal, not a commitment — it is a named future option in the PRD rather than a V1.0 gate.

---

### Positioning Context

**Scoped "first" claim:** As of v1.0 ship, AOTF is — to the best knowledge of this PRD — the first open-source tool to close the autonomous remediation loop across application, AI agent, and model/prompt layers in a single CLI, **gated by an explicit action-tier reversibility taxonomy with authorized pre-production consent**. Adjacent tools observe (Langfuse, Arize) or detect (Sentry, Datadog) but do not remediate autonomously. The OSS SRE-agent cohort (HolmesGPT, k8sgpt, Keep, Tracer opensre) investigates and proposes remediation at the post-incident SRE layer but does not operate autonomously at the pre-production SDLC surface. Commercial SDLC-analytics tools (Harness, LinearB) do not model AI agent behavior; commercial AI-SRE tools (Resolve.ai, Cleric, Datadog Bits AI SRE) are not open-source. This gap is specific; any stronger "first" claim is unnecessary for v1.0.

**Lineage note:** AOTF diverges from Tracer `opensre` and HolmesGPT — two reference OSS projects in the AI-SRE space — by shifting the autonomous-action surface *upstream* into the pre-production SDLC (pre-push, CI, pre-deploy) rather than post-incident production response, with a reversibility-typed action taxonomy (FR-105) and production consent token (FR-106) as the boundary mechanism. `opensre` and HolmesGPT are complementary, not competitive: AOTF's signal output integrates with their remediation surface as a natural downstream pairing.

**OSS model reference:** Grafana-style. Genuinely complete OSS core, optional managed tier for later. Apache 2.0 is a permanent commitment — no BSL relicensing is on the table.

**Developer workflow fatigue:** Engineers already run Copilot, Cursor, pre-commit hooks, and multiple observability tools. AOTF's approach: one CLI install, wraps existing AI CLIs (doesn't replace them), no new accounts, no new SaaS vendor to approve.

---

### Validation Approach

| Innovation | Validation Method | Signal | Timeline | Scope |
|---|---|---|---|---|
| Open-source autonomous remediation | Individual → team conversion rate | >20% of individual installs add a second user within 30 days (design anchor) | MVP + 30d | MVP |
| CLI-agent-first | Users swap backends without churning | Churn rate for backend-switchers ≤ baseline churn | MVP + 90d | MVP |
| Goal Loop | Relative CI pass rate lift vs. single-attempt | Goal Loop ≥10pp above team baseline | MVP + 60d | MVP |
| Proactive Loop adoption | 7-day pre-push hook retention | >30% of users who enable hook keep it at day 7 | MVP + 30d | MVP |
| Autonomy UX / trust calibration | Teams raising confidence threshold over time | >40% of teams adjust threshold by month 2 | MVP + 60d | MVP |
| Cross-layer correlation value | A/B: correlation alerts vs. single-layer | Time to root cause, false positive rate | Post-MVP beta | Growth |
| Intelligence Compounds | Diagnosis accuracy install vs. month 3 | >10% improvement (accuracy = % diagnoses confirmed correct by engineer post-fix) | Month 3 | Growth |

---

### Risk Mitigation

| Innovation Risk | Mitigation |
|---|---|
| "Zero cloud" claim has asterisk (LLM routes to Anthropic) | Disclose clearly at install; local LLM backend (Ollama) on roadmap as first-class |
| Plugin ecosystem fragmentation before moat is established | Ship 5–10 maintained first-party plugins; publish quality standards and deprecation policy |
| CLI subprocess contract broken by upstream update | Conformance test suite per supported backend; schema versioning |
| Goal Loop fails to converge | Termination contract (attempt cap + time + cost ceiling); failure state = `human-required`, not silent |
| Category window closes before PMF | MVP = DevOps Loop only (already-felt pain); AI agent observability follows after trust established |
| Intelligence Compounds privacy concern | Local by default; cross-org transmits anonymized signatures only; opt-in with full disclosure |
| **Resolve.ai ($1B valuation) extends into pre-production SDLC territory** | AOTF's positioning is *pre-production* autonomy with reversibility-typed gating (claim #5); Resolve's positioning is *post-incident* production response — orthogonal English phrases. Apache 2.0 + local-deploy + cross-layer moat are structurally unavailable to the commercial SaaS model. Execute the moat: publish taxonomy spec early, CNCF Sandbox track, first-incident-prevented case study within 6 months of v1.0 ship. |
| **OSS SRE-agent cohort (HolmesGPT, k8sgpt, Keep) occupies post-incident mindshare** | Reference-integrate with HolmesGPT / opensre as *downstream consumers* of AOTF's pre-production signals. AOTF + HolmesGPT is a natural complementary stack (prevention + investigation), not competitive. Publish an integration reference implementation at v1.1. |
| **HolmesGPT or a commercial competitor productizes a reversibility taxonomy** | 12–24 month imitation horizon. Mitigation: (a) publish the full taxonomy spec + tier-to-gate policy matrix as a public artifact at v1.0 so AOTF becomes the reference implementation; (b) maintain a benchmark comparing taxonomy depth across agents; (c) pursue CNCF Sandbox to anchor AOTF as the category-safety-narrative owner. |
| **Hyperscaler-embedded SRE agents (Azure SRE Agent, Datadog Bits AI, PagerDuty Suite) consolidate enterprise segment** | Accept the enterprise-segment loss for v1.0. Focus on Cloud-Native Mid-Market (200–2,000 eng, K8s + multi-cloud, OSS-bias culture) where hyperscaler lock-in is not yet a procurement default. Data-residency and self-host posture are structural defenses the hyperscalers cannot match. |
| **Procurement-risk bias toward CNCF-governed projects over Apache-2.0 startup** | Pursue CNCF Sandbox submission after 1 deployed production user; publish governance model; ship at least one cross-project collaboration (joint integration or spec) with k8sgpt or HolmesGPT to establish ecosystem credibility ahead of Sandbox review. |

---

## Developer Tool + CLI — Specific Requirements

### Command Structure

**Core command surface (v1.0):**

```
aotf init                          # Project setup + auto-discovery
aotf watch [--source <type>]       # Start daemon, attach log watchers
aotf diagnose [<finding-id>]       # AI diagnosis; interactive selection if no ID given
aotf fix [<finding-id>] [--mode <diagnosis_only|fix_and_pr>] [--scope <single|multi>] [--dry-run]
aotf rollback <finding-id>         # Revert a merged fix
aotf consent issue --env <e> --scope <s> --ttl <d>  # Issue production-consent token (FR-106, WebAuthn-gated)
aotf plugin trust <name>           # Verify SHA-256 + add to allowlist
aotf plugin list                   # List installed plugins with versions + capability grants
aotf plugin dev --local <path>     # Load unverified local plugin for development (dev mode only)
aotf telemetry status              # Show telemetry configuration + what is collected
aotf telemetry disable             # Persist opt-out to config
aotf user audit                    # Print immutable audit log (local only)
aotf dashboard                     # Launch WebUI (Growth, separate workstream)
```

**Global flags:**
```
--config <path>                # Override default .aotf/config.yaml
--no-telemetry                 # Disable telemetry for this invocation
--auto-approve                 # Non-interactive mode; honored ONLY when CI=true in env
                               # Local override requires explicit --force-local-auto-approve
--log-level verbose|info|warn|error
--output json|human            # Default: human
--dry-run                      # Available on any write operation; renders plan, no execution
```

**Ergonomics:**
- `aotf diagnose` and `aotf fix` without an ID: interactive finding selector in terminal; `--latest` semantics in non-interactive/piped context
- `--help` on every command; `--help --output json` returns machine-readable help schema

**Growth command surface (v0.2 through post-v1.0):**

The following commands appear in user journeys (§5) and are scoped to Growth/Vision phases. They are NOT in v1.0. Journey narratives that use them are aspirational demonstrations of the four-pillar vision; v1.0 delivers Pillar 1 (Proactive DevOps Loop) and a read-only MCP surface.

| Command | Pillar | Phase | Referenced in |
|---|---|---|---|
| `aotf qa run` / `aotf qa from-report` | AI QA Agent | Growth (FR-71-75) | J12 |
| `aotf ml prompt rollback` / `aotf ml drift` | ML/LLM Lifecycle | Vision (FR-85-89) | J5 |
| `aotf consent issue` | Proactive DevOps Loop (pre-push) | MVP (FR-14, FR-106) | J11 — **included in v1.0 surface** |
| `aotf simulate` (impact check) | Proactive DevOps Loop | Growth | J8 |
| `aotf agent run` / `aotf agent baseline` | AI Agent Operations | Growth (FR-76-80) | (Growth journeys) |
| `aotf dashboard` | — (cross-pillar) | Growth (FR-81-84) | J9, J10 |

**Notes:**
- `aotf consent issue` is MVP — it supports the pre-push prevention journey (J11) and is required to exercise FR-106 (production consent tokens). It is the one "Growth-looking" command in a v1.0 surface.
- Phase-gating is enforced by the CLI: unavailable commands print a clear message explaining the phase and where to track progress (e.g., `aotf qa run: available in v0.3+ (AI QA Agent pillar — see roadmap)`).

---

### First-Run Experience

`aotf init` is the highest-impact adoption moment:

| Scenario | Required Behavior |
|---|---|
| Fresh project, no config | Auto-detects stack, generates `.aotf/config.yaml` draft, prompts for confirmation |
| Already initialized | Prints current config summary, asks if reinitializing; default: no |
| Large monorepo (>10k files) | Scans top-level and `src/` by default; `--deep` flag for full scan |
| Insufficient git permissions | Explains exact permissions needed; links to docs |
| Clone with existing `.aotf/config.yaml` | Uses committed config; does not re-prompt |
| AI CLI not found | Clear error with install instructions per backend; exits non-zero |

Target: >85% of `aotf init` invocations that complete reach a running `aotf watch` state.

---

### Exit Codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Permission error |
| 4 | AI backend unavailable |
| 5 | No findings (not an error) |
| 6 | Safety gate triggered |
| 7 | Partial success |

Exit codes are stable across minor versions. Breaking changes require major version bump.

---

### Finding Schema (v1)

Core data contract flowing through watch → diagnose → fix → rollback:

```json
{
  "schema_version": "1",
  "id": "ERR-001",
  "timestamp": "2026-04-16T03:07:22Z",
  "source": { "type": "docker", "container": "api-service" },
  "severity": "error",
  "message": "NullPointerException in OrderService.create()",
  "file": "order_service.py",
  "line": 47,
  "confidence": 0.91,
  "confidence_components": {
    "static_analysis": 0.85,
    "diff_complexity": 0.95,
    "test_signal": 0.93
  },
  "status": "diagnosed|fix_pending|fix_applied|rolled_back|human_required",
  "fix": {
    "pr_url": "https://github.com/org/repo/pull/47",
    "rollback_ref": "aotf/rollback/ERR-001"
  }
}
```

Machine-readable JSON Schema published to `schema.aotf.dev/v1/finding.json`. CI validates all schema changes. Stable across minor versions; breaking changes require major version bump.

---

### Output Formats & Schema Stability

| Format | Use | Default |
|---|---|---|
| Human-readable | Interactive terminal | Yes |
| JSON lines (`--output json`) | CI pipelines, scripting | Via flag |
| Daemon structured log | Watch daemon output | Always |

**Breaking change definition:** removing a field, changing type, changing enum values, renaming a field. Non-breaking: adding optional fields or new enum values.

---

### Telemetry

Telemetry is **opt-in by default**. Zero telemetry in CI mode unless explicitly enabled.

| Control | Mechanism |
|---|---|
| `aotf telemetry status` | Prints: enabled/disabled, collected events, data destination |
| `aotf telemetry disable` | Persists opt-out to config |
| `--no-telemetry` | Disables for single invocation |
| `AOTF_NO_TELEMETRY=1` | Environment variable override |

What is collected (when opted in): command names (not arguments), error types (not messages), timing, OS/arch. Never: file contents, finding messages, error text, repo paths, git URLs.

---

### Configuration Schema

**`.aotf/config.yaml`:**
```yaml
project: my-service
watchers:
  - type: file
    path: ./logs/app.log
  - type: docker
    container: api-service
ai:
  backend: claude-code-cli   # claude-code-cli | codex-cli | gemini-cli
  min_version: "1.2.0"
  timeout_seconds: 30
  max_concurrent_calls: 3
fixes:
  mode: fix_and_pr
  confidence_threshold: 0.75
  scope: single
  auto_approve_in_ci: true   # Only honored when CI=true
  cooldown_seconds: 60
notifications:
  telegram:
    adapter: claude-bridge
    chat_id: "-1001234567890"
plugins:
  registry: https://hub.aotf.dev
  allowlist:
    - github-actions@1.2.0
telemetry:
  enabled: false
```

`plugins.lock.yaml`: SHA-256 checksums, auto-generated. `.aotf/consent.yaml`: per-repo AI consent, auto-generated on first fix. Env var overrides: `AOTF_` prefix. Precedence: env vars > config file > defaults.

---

### Watch-to-Diagnose Pipeline

| Parameter | Default | Notes |
|---|---|---|
| Error deduplication window | 90 seconds | Same error hash = single finding |
| Diagnosis trigger threshold | 3 occurrences in 90s | Prevents single-occurrence noise |
| Rate limit | 10 diagnoses/minute per repo | Prevents log spam triggering AI calls |
| Backpressure | 100-event buffer; drop oldest on overflow | Prevents memory growth |
| Diagnosis debounce | 5 seconds from first occurrence | Allows deduplication to accumulate |

All parameters configurable in `.aotf/config.yaml` under `watch:` section.

---

### Plugin API Surface

```typescript
abstract class LogWatcherABC {
  abstract name(): string
  abstract setup(): Promise<void>
  abstract start(emit: EmitFn): Promise<void>
  abstract stop(): Promise<void>
  abstract teardown(): Promise<void>
  onError?(error: Error): void   // Default: log + continue; plugin marked degraded
}

abstract class FindingEmitterABC {
  abstract emit(): AsyncGenerator<Finding>  // Yields Finding schema v1 objects
}

abstract class NotificationAdapterABC {
  abstract send(event: AotfChannelEvent): Promise<void>
}
```

**Plugin isolation:** Unhandled exception in a plugin marks it `degraded`, does not crash the watcher daemon.

**Local development:** `aotf plugin dev --local ./my-plugin` — skips SHA-256 verification, prints warning, unavailable in CI mode.

**Capability enforcement:** Linux: seccomp-bpf. macOS: advisory + import-hook level. All platforms: violations logged; plugin not killed on first violation (configurable).

**Conformance testing:** `aotf plugin test ./my-plugin` runs against synthetic event stream, validates output schema.

**Platform capability-enforcement note:** On Linux, capability declarations are runtime-enforced via seccomp-bpf. On macOS (the primary `brew install` target), OS sandbox APIs are entitlement-gated, so enforcement is advisory + import-hook level — a weaker posture than Linux. Users requiring strong plugin sandboxing on Mac fleets should run AOTF in a Linux VM or container. This gap is published in the security docs, not hidden.

**Plugin re-consent in CI:** When a plugin update in CI requests new capabilities beyond its previous grants, AOTF fails the CI run with a clear error. The user resolves by updating `plugins.lock.yaml` capability grants in a regular commit (reviewable via PR). No silent capability elevation.

**Action-tier declaration (per FR-105):** Every capability in a plugin manifest declares its maximum `actionTier` — one of `READ`, `DRAFT`, `PROPOSE`, `EXECUTE_REVERSIBLE`, `EXECUTE_IRREVERSIBLE`. The orchestrator enforces the declared tier as a ceiling: runtime operations that exceed the declared tier produce a `TierViolation` in the audit log and terminate the plugin. Enforcement is in the Rust orchestrator FFI layer (`crates/orchestrator/src/plugin_bridge.rs`), not the TS SDK — bypassing the SDK does not bypass enforcement. A versioned tier-classification table (`crates/orchestrator/src/tier_map.rs`) maps concrete syscalls and tool invocations to tiers.

**Manifest schema migration for `actionTier`:** `actionTier` is required in plugin manifests from v1.1. For the v1.0.x grace window, manifests omitting `actionTier` are accepted with an implicit `actionTier: UNKNOWN`, which the orchestrator treats as `READ` — preventing silent tier-escalation during the migration period. Plugin authors receive a deprecation warning at load time. From v1.1+, missing `actionTier` is rejected at plugin-trust time.

*IPC contract (Rust↔Bun):* Unix domain socket with versioned JSON-RPC protocol — must be specified before implementation; all plugin isolation and crash recovery depend on this boundary. IPC messages for `EXECUTE_IRREVERSIBLE` operations include a `consentTokenId` field referencing a valid, unexpired consent token (see FR-106).

---

### Language & Stack Matrix

Language-agnostic at the watcher layer. Fix engine is language-aware.

| Language | Watcher | Diagnosis | Auto-Fix | Phase |
|---|---|---|---|---|
| TypeScript / JavaScript | ✓ | ✓ | ✓ | v1.0 |
| Python | ✓ | ✓ | ✓ | v1.0 |
| Go | ✓ | ✓ | ✓ | v1.0 |
| Rust | ✓ | ✓ | ✓ | v1.0 |
| Java / Kotlin | ✓ | ✓ | ✓ | v1.0 |
| Ruby | ✓ | ✓ | ✓ | v1.0 |
| Any (log-only, no fix) | ✓ | ✓ | N/A | v1.0 |

*Note: AOTF is implemented in Rust + TypeScript/Bun. This is independent from the languages it monitors and fixes — all languages are supported via the AI CLI fix engine from v1.0.*

---

### Installation Methods

| Method | Target | Status |
|---|---|---|
| `brew install aotf` | macOS | v1.0 |
| `curl install.aotf.dev \| sh` | Linux, macOS | v1.0 |
| Prebuilt binary (GitHub Releases) | Linux x86_64/arm64; macOS arm64/x86_64 | v1.0 |
| Windows WSL2 | Deferred — validate demand first | Roadmap |
| Docker image | Deferred — validate CI use case first | Roadmap |

Single static binary. No runtime dependencies.

---

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach: Trust-First Problem-Solving MVP**

The assumption being validated: *teams will allow an AI to autonomously diagnose and attempt to fix production errors.* This is a high-trust ask. Everything in v1.0 is designed to earn that trust before expanding scope.

The MVP is intentionally narrow: one pillar (Proactive DevOps Loop), one notification channel (Telegram), one CI provider (GitHub Actions). Not because the other features aren't valuable, but because validating the trust assumption requires proving the core loop works flawlessly before adding surface area.

**Why this scope:**
- Proactive DevOps Loop is the most universally felt pain — every engineering team has production errors
- It requires no AI QA, no MLOps, no agent behavior knowledge — it's a clean first proof
- If teams won't accept an autonomous fix PR for a null pointer exception, they won't accept anything else
- Each growth pillar depends on the trust established in v1.0

---

### MVP Feature Set (v1.0 — ~14 weeks)

**Core User Journeys:**

| Journey | Path |
|---|---|
| J1 | First detection to fixed PR (happy path) |
| J2 | 3am approval via Telegram (approval gate) |
| J4 | Team setup and manager visibility |
| J6 | AOTF gets it wrong — rollback (trust calibration) |
| J7 | Goal Loop multi-attempt remediation via claude-bridge |
| J8 | Collaborative Telegram triage |
| J11 | Proactive pre-push prevention with tier-gated consent (pre-production authorization) |

**Must-Have Capabilities:**

| Capability | Why Non-Negotiable |
|---|---|
| `aotf init` / `watch` / `diagnose` / `fix` / `rollback` | Core loop; no value without all five |
| Streaming log watchers: file tail, Docker container | Minimum viable detection surface |
| Five-gate Auto-Fix Safety Model + Action-Tier Reversibility Taxonomy (FR-105) | Trust prerequisite; the taxonomy is the type system the gates consume |
| Production consent token with WebAuthn attestation (FR-106) — scoped to `EXECUTE_IRREVERSIBLE` actions against production-designated environments | Irreversible-in-production control is the distinctive safety primitive; no surveyed competitor has productized it |
| Pre-push risk assessment (FR-05 + FR-14) with tier-gated consent | Supports J11; without this, the "proactive" pillar is theater |
| Auto-fix decision log with approver / timestamp / reason on EXECUTE_IRREVERSIBLE (FR-66) | Developer trust + compliance mechanism; non-optional |
| Telegram notifications (both adapters) | Primary async interaction channel |
| claude-bridge Goal Loop | Multi-attempt remediation differentiator |
| Plugin system: SHA-256 + capability scoping | Security non-negotiable from day one |
| GitHub Actions CI provider plugin | MVP CI integration |
| SQLite local storage (WAL durability) | Finding persistence |
| All-language auto-fix via AI CLI (language-agnostic diff generation + post-apply validation via per-language test runners) | AI CLI generates diffs for any language; post-apply validation depth varies per language (deepest for Rust + TypeScript — AOTF's own stack — expanding with Growth) |
| Basic MCP server (`diagnose` + `fix` tools) | Highest-leverage IDE integration |
| Audit log | Security and compliance prerequisite |

**Explicit v1.0 Exclusions:**
AI QA Agent, ML/LLM Lifecycle Manager, AI Agent Operations, WebUI dashboard, TUI, Codex/Gemini CLI backends, `full_auto` merge mode, polling log watchers (Loki, Datadog, etc.), GitLab/CircleCI/Azure DevOps CI providers, Slack/Teams/Email/PagerDuty notifications, Windows/Docker install.

**Scope safety valve at week 12:** MCP server ships as stub (`diagnose` only). Plugin security and safety gates are cut-proof.

---

### Resource Requirements

**Minimum viable team:**

| Role | Scope |
|---|---|
| Rust engineer | Watch daemon, plugin system, safety gates, SQLite |
| TypeScript/Bun engineer | Agent layer, AI CLI subprocess management, fix engine, plugin adapters |
| Full-stack / DevOps | GitHub Actions plugin, claude-bridge integration, AOTF's own CI/CD |
| PM / tech lead (can overlap) | Scope guard, user feedback loop |

3–4 people at 14 weeks. 2-person team viable at ~20 weeks with scope discipline.

**Critical path (must be sequenced, not parallelized):**
1. Rust↔Bun IPC contract → before any agent work
2. Finding schema v1 frozen → before plugin API
3. Plugin capability model specified → before security enforcement
4. v1.0 config schema frozen → before GitHub Actions plugin

---

### Phased Roadmap

**v1.0 (~14 weeks) — Trust Gate**
Proactive DevOps Loop including pre-push risk assessment (FR-05 + FR-14, supporting J11). Core loop proven. All-language auto-fix. Telegram. GitHub Actions. 5-gate safety model + action-tier reversibility taxonomy (FR-105). Production consent token with WebAuthn attestation (FR-106). Goal Loop. Strengthened decision log (approver / timestamp / reason on irreversible actions — FR-66).

**v0.2–v0.5 (Growth)**
- AI QA Agent: Playwright + LLM test runner, self-healing tests, bug report → regression test
- AI Agent Operations: run tracking, behavior logging, deviation detection, cost tracking, shadow pipelines
- Additional AI backends: Codex CLI, Gemini CLI
- Polling log watchers: Loki, Datadog, CloudWatch, Grafana
- Additional CI providers: GitLab CI, Azure DevOps, Jenkins
- WebUI dashboard, TUI interface
- Additional notifications: Slack, PagerDuty, MS Teams

**v1.0+ (Vision)**
- ML/LLM Lifecycle Manager: git-native model registry, drift→retrain→gate→canary→promote, prompt versioning, A/B testing, injection firewall
- Own agent runtime with local model support (Ollama, llama.cpp — closes "zero cloud" fully)
- Cross-org anonymized intelligence tier (opt-in)
- Enterprise: SSO, compliance audit, self-hosted intelligence, SLA

---

### Risk Mitigation Strategy

**Technical Risks:**

| Risk | Probability | Mitigation |
|---|---|---|
| AI CLI subprocess contract breaks with upstream update | Medium | Versioned interface schema + conformance test suite per backend |
| Fix engine produces wrong diff | Medium | Confidence floor + mandatory dry-run; Goal Loop provides retry context |
| Goal Loop convergence failure | Low | Termination contract (3 attempts + time + cost ceiling) |
| Rust↔Bun IPC boundary bottleneck | Low | Define protocol in week 1; benchmark at 1000 events/sec |

**Market Risks:**

| Risk | Mitigation |
|---|---|
| Category too early — teams won't trust autonomous fix | Trust-first design: 5-gate model, action-tier taxonomy, decision log, human-in-loop defaults |
| OSS AI-SRE cohort (HolmesGPT — 2.2k★ + CNCF Sandbox + Microsoft contribs; k8sgpt — 5k★ + CNCF Sandbox; Keep — 11.6k★) captures adjacent OSS mindshare before AOTF ships | AOTF's surface is *pre-production* SDLC, not post-incident SRE — complementary, not competitive. Publish reference integration with HolmesGPT/opensre as downstream consumer of AOTF signals at v1.1. CNCF Sandbox submission after 1 deployed user. |
| Commercial AI-SRE incumbent (Resolve.ai — $125M Series A @ $1B valuation; Datadog Bits AI SRE; PagerDuty AI Agent Suite; Azure SRE Agent) extends pre-production | Apache 2.0 + local-deploy + cross-layer moat are structurally unavailable to SaaS-first models. Action-tier taxonomy (claim #5) and production consent token (FR-106) are productized safety primitives these incumbents have not shipped. |
| Hyperscaler-embedded SRE agents (Azure / Datadog / PagerDuty) consolidate enterprise segment | Accept enterprise-segment loss for v1.0; target Cloud-Native Mid-Market (200–2,000 eng, K8s + multi-cloud) where hyperscaler lock-in is not yet procurement default. |
| Low plugin adoption stalls ecosystem | Ship 5–10 maintained first-party plugins at v1.0 launch |

**Resource Risks:**

| Scenario | Contingency |
|---|---|
| 2-person team | Defer MCP server to stub; defer Goal Loop to v1.1; ship diagnosis + manual-approve-fix |
| Key dependency changes API | Adapter pattern (`NotificationAdapterABC`) isolates core from adapter changes |
| Week 12 scope overrun | Cut to: diagnosis-only mode + manual fix workflow; safety gates non-negotiable |

---

## Functional Requirements

### 1. Project Initialization & Configuration

- **FR-01:** Engineer can initialize AOTF in a project via CLI with auto-detection of stack, log sources, and CI provider
- **FR-02:** Engineer can review and confirm auto-detected configuration before any monitoring begins
- **FR-03:** Engineer can configure log watchers, AI backend, fix mode, confidence threshold, notification channels, and plugin allowlist via a project-level YAML config file
- **FR-04:** Engineer can override any configuration value via environment variables without modifying config files
- **FR-05:** Engineer can install pre-push git hooks that trigger pre-commit risk assessment
- **FR-06:** Engineer can manage per-repo AI data transmission consent independently of the binary install
- **FR-07:** Engineer can manage telemetry opt-in/opt-out with visibility into what data is collected
- **FR-08:** AOTF reports initialization failures (missing permissions, unsupported stack, AI CLI not found) with actionable resolution steps
- **FR-97:** AOTF communicates initial monitoring scope on first watch start — sources watched, severity filter, AI backend, fix mode — before the first finding fires
- **FR-99:** AOTF provides a guided interactive onboarding experience on first run that walks the engineer through config choices, verifies stack detection, and confirms readiness before entering watch mode

### 2. Log Monitoring & Detection

- **FR-09:** AOTF can watch streaming log sources (file tail, Docker container stdout/stderr) with watcher-layer ingestion latency <5 seconds from log emission
- **FR-10:** AOTF deduplicates identical errors within a configurable time window into a single finding
- **FR-11:** AOTF applies configurable trigger thresholds and rate limits before escalating a log pattern to AI diagnosis
- **FR-12:** AOTF assigns a unique, stable ID to each finding that persists across daemon restarts
- **FR-13:** AOTF stores all findings in local persistent storage with durability guarantees against abrupt termination
- **FR-14:** AOTF performs pre-push risk assessment on staged changes and reports findings before push completes *(Proactive Loop)*. Supports Journey 11. Findings classified at `EXECUTE_IRREVERSIBLE` tier (e.g., schema migrations, data deletions) against production-designated environments block the push pending authorized consent (see FR-105, FR-106); lower-tier findings surface as advisory with configurable block-vs-warn behavior.
- **FR-15:** AOTF watches multiple log sources concurrently up to a configurable limit
- **FR-16:** *(Growth)* AOTF can watch polling log sources (Loki, Datadog, CloudWatch, Grafana) on configurable intervals

### 3. AI Diagnosis

- **FR-17:** AOTF produces an AI-generated root cause diagnosis for a finding, including confidence score, file attribution, and line reference
- **FR-18:** AOTF computes a confidence score from static analysis agreement, diff complexity, and test signal — not from AI self-reporting. *(Formula, weights, and threshold defined in §6 Autonomous Code Execution Safety.)*
- **FR-19:** Engineer can request a diagnosis for any finding on demand via CLI
- **FR-20:** AOTF records a structured decision log entry for every diagnosis including confidence breakdown and AI reasoning trace
- **FR-21:** Engineer can query the local decision log to review all past diagnosis and fix decisions
- **FR-92:** AOTF delivers a human-readable reasoning explanation at the point of each autonomous action — distinct from the structured decision log
- **FR-22:** *(Growth)* AOTF correlates findings across ≥2 log sources or services to surface cross-layer relationships

### 4. Autonomous Fix & Safety

- **FR-23:** AOTF renders a diff preview of a proposed fix before any code is modified
- **FR-24:** AOTF enforces a configurable confidence floor — fixes below threshold require human approval before execution
- **FR-25:** AOTF enforces a blast radius limit on autonomous fixes (single file by default; multi-file requires explicit override)
- **FR-26:** AOTF creates a tagged rollback reference in git before applying any automated fix
- **FR-27:** AOTF enforces a per-repository fix mutex preventing concurrent autonomous fixes on the same repo
- **FR-28:** AOTF enforces a per-repository action cooldown between successive autonomous fix triggers
- **FR-29:** Engineer can revert any applied AOTF fix via a single rollback command regardless of merge status
- **FR-30:** AOTF presents an interactive approval step before executing a fix in non-CI contexts
- **FR-31:** AOTF supports non-interactive CI mode with an explicit opt-in honored only when a CI environment is detected
- **FR-32:** AOTF creates a fix branch and opens a pull request against the repository's target branch
- **FR-33:** AOTF detects fix failure post-apply (compilation error, test regression, git conflict), invokes rollback automatically, and escalates the finding to human-required status
- **FR-34:** AOTF detects and resolves in-flight operation state from a previous interrupted run before proceeding on restart
- **FR-35:** Engineer can configure confidence threshold, blast radius scope, approval mode, and cooldown per project and per service
- **FR-36:** AOTF records the outcome of every autonomous fix action in the decision log
- **FR-93:** AOTF communicates its operational state explicitly when degraded, blocked by a safety gate, or in fallback mode — not silently degraded
- **FR-94:** Engineer can reject a proposed fix with optional contextual feedback that is incorporated into a revised fix attempt
- **FR-105:** AOTF declares an action-tier on every autonomous operation — one of `READ`, `DRAFT`, `PROPOSE`, `EXECUTE_REVERSIBLE`, `EXECUTE_IRREVERSIBLE`. The declared tier is a ceiling: no operation executes at a tier higher than declared in the plugin capability manifest or at the call site. The contract expresses **invariants**, not mechanism — architecture selects the enforcement implementation (memory-safe capability-bounded core with isolated failure domain; see NFR-S10).
  - **AC FR-105.1:** No plugin-originating call path can cause an `EXECUTE_IRREVERSIBLE` action to execute without a prior `ALLOW` decision from the tier-policy gate in the same logical transaction.
  - **AC FR-105.2:** The tier-policy gate state (action → tier mapping, override ledger, consent-token ledger) is not writable from any plugin-reachable code path. Verified by mutation testing: mutating the gate's allow/deny branch MUST cause ≥1 integration test to fail.
  - **AC FR-105.3:** Violations of FR-105.1 or FR-105.2 detected in CI block merge. CI-detected violations additionally emit a `TierViolation` audit entry and terminate the offending plugin in runtime.
  - **AC FR-105.4:** Every decision-log entry records the classified tier (`READ` / `DRAFT` / `PROPOSE` / `EXECUTE_REVERSIBLE` / `EXECUTE_IRREVERSIBLE`) alongside the action, actor, and outcome.
  - **AC FR-105.5:** A versioned action-to-tier classification table is maintained in architecture-defined source locations and reviewed through standard code review (not config). The table is the authoritative mapping; runtime inference from call shape alone is forbidden.

  *Scope boundary for core (applies to NFR-S10 enforcement):* code belongs in the enforcement core iff **"if this code is wrong, a reversible-action tier could escalate to irreversible without authorization."** Code that fails this test belongs outside the core (agent orchestration, QA harness, drift detection, MCP server, notification adapters).
- **FR-106:** AOTF enforces a production-consent token before any `EXECUTE_IRREVERSIBLE` action targets a production-designated environment. Token properties: single-use, 1-hour TTL, issued per-approver. Token issuance requires a WebAuthn user-presence assertion from a pre-registered operator key; the assertion hash and approver identity are recorded in the audit log alongside the token hash. No assertion → no token → no execution. "Production-designated" is a binary per-environment flag (see NFR); staging-like-prod, canary, and shadow environments are NOT production for token purposes. The `--force-local-auto-approve` flag does NOT bypass FR-106 — irreversible-in-production always requires a consent token.
- **FR-107:** AOTF uses three **distinct, non-interchangeable** authorization primitives. Each has its own lifecycle, storage, verification, and revocation semantics:
  - **(a) User access tokens** (FR-49) — role-scoped, long-lived, rotatable, issued by team admin via `aotf user create`, stored as Argon2-hashed records; revocable via `aotf user revoke`
  - **(b) Production consent tokens** (FR-106) — single-use, 1-hour TTL, WebAuthn-gated, bound to a specific `EXECUTE_IRREVERSIBLE` action and approver identity; consumed once, cannot be re-presented
  - **(c) MCP invocation tokens** (NFR-S07) — localhost-bound, per-session, read-scope-only in v1.0, issued at MCP client handshake; revoked on session end or daemon restart

  **AC FR-107.1:** Tokens of one type MUST NOT be accepted in place of tokens of another type. Cross-type submission raises `AuthTypeMismatch`, is audit-logged with `(timestamp, submitted_type, expected_type, actor)`, and rate-limits the submitting identity.

  **AC FR-107.2:** No primitive has a superset-of relationship with another. A user access token with `admin` role cannot substitute for a WebAuthn-backed production consent token, even for the same approver identity. A Telegram approval (FR-44) provides audit trail and UX surface but is consumed as an *action on behalf of* an existing user access token — it is not a fourth primitive.

  **AC FR-107.3:** Telegram per-user allowlist (see §6 Security Requirements) is a *channel ACL*, not a token primitive. Approvals arriving via Telegram still verify against the approver's user access token scope (FR-49) for their role, and escalate to WebAuthn consent-token flow (FR-106) for `EXECUTE_IRREVERSIBLE`.

### 5. Goal Loop & Multi-Attempt Remediation

- **FR-37:** Engineer can trigger a Goal Loop that iterates autonomous remediation of a finding until CI passes or a ceiling is reached
- **FR-38:** AOTF isolates each Goal Loop attempt in a separate git worktree with defined filesystem boundaries and no secret inheritance
- **FR-39:** AOTF feeds CI failure output from each attempt back into the next diagnosis cycle within a Goal Loop
- **FR-40:** AOTF enforces a configurable maximum attempt ceiling, time budget, and cost ceiling per Goal Loop invocation
- **FR-41:** AOTF delivers a Goal Loop completion summary including attempt count, CI outcome, and per-loop token cost
- **FR-42:** When a Goal Loop exhausts its ceiling without success, AOTF escalates the finding to human-required status and delivers an alert with the final diff and CI trace

### 6. Notifications & Team Coordination

- **FR-43:** AOTF delivers finding alerts and fix status updates to configured notification channels
- **FR-44:** AOTF supports bidirectional Telegram interaction — recipients can approve, reject, or view diffs without leaving the messaging client
- **FR-45:** AOTF supports the zero-infrastructure Telegram adapter for individual use
- **FR-46:** AOTF supports the self-hosted Telegram adapter (claude-bridge) for team use with Goal Loop, worktree isolation, and per-task cost tracking
- **FR-47:** AOTF delivers notifications to group Telegram chats with multi-person approval visibility (who approved, when)
- **FR-48:** AOTF rate-limits and batches notifications to prevent alert fatigue during error storms
- **FR-49:** Team admin can create and manage per-user access tokens with role-based capability scopes
- **FR-50:** Team admin can configure per-service fix mode overrides independently of the global project configuration
- **FR-51:** AOTF produces periodic team digest reports summarizing findings, diagnoses, fix outcomes, and MTTR trends
- **FR-52:** AOTF tracks an MTTR baseline per team and reports improvement against that baseline
- **FR-103:** Team admin can view weekly activity metrics for their team scope (findings captured, diagnoses, fixes applied, rollbacks) — the data underlying the North Star metric of Weekly Active Teams
- **FR-53:** *(Growth)* AOTF supports additional notification channels with per-channel interactivity model: **Slack** (bidirectional via Block Kit interactive components — approve/reject/view-diff inline), **PagerDuty** (bidirectional via acknowledgment callbacks — ack/resolve; diff viewing out-of-band), **MS Teams** (bidirectional via Adaptive Card actions), **Email SMTP** (output-only — no inbound interaction; diff and approval via link-out to dashboard or Telegram). Bidirectional capabilities require a per-channel inbound-webhook surface specified in architecture.

### 7. Plugin Ecosystem

- **FR-54:** Engineer can install, trust, and manage AOTF plugins from a registry via CLI
- **FR-55:** AOTF verifies plugin integrity via SHA-256 checksum before allowing any plugin code to execute
- **FR-56:** Engineer can declare per-plugin capability grants (filesystem, network, git) that scope plugin access at runtime
- **FR-57:** Engineer can develop and test plugins locally before SHA-256 registration using a local development mode
- **FR-58:** Plugin author can validate plugin conformance against the AOTF plugin API contract via a CLI test runner
- **FR-59:** AOTF requires re-consent when a plugin update requests new capabilities beyond its previously granted scope
- **FR-60:** AOTF marks a plugin as degraded rather than crashing the watcher daemon when a plugin throws an unhandled exception
- **FR-96:** Engineer can audit all installed plugins for integrity and receive alerts when checksums no longer match the trust registry
- **FR-104:** AOTF publishes and enforces plugin quality standards (schema conformance, capability declaration completeness, documented deprecation path) applied at registry-submission time before a plugin appears in the default registry

### 8. AI Backend Management

- **FR-61:** Engineer can configure and switch between supported AI CLI backends without modifying fix or diagnosis logic
- **FR-62:** AOTF validates the configured AI CLI backend version against a declared minimum version on startup and reports incompatibility
- **FR-63:** AOTF falls back to detection-only mode automatically when the configured AI backend is unavailable
- **FR-64:** AOTF enforces a configurable ceiling on concurrent AI backend calls to prevent API quota exhaustion
- **FR-65:** AOTF exposes **read-only** diagnosis and status capabilities as MCP tools accessible from MCP-compatible clients. v1.0 tool set is strictly non-mutating: `get_risk_score(path_or_commit)`, `get_finding(finding_id)`, `list_recent_runs(limit)`, `get_daemon_status()`. **No v1.0 MCP tool may mutate a repository, create/modify/merge a PR, trigger a deploy, apply a fix, or invoke rollback** — these operations route exclusively through the CLI + safety-gate path (FR-23 through FR-27, FR-105, FR-106). Write-capable MCP surface (e.g., `trigger_run`, `approve_fix`, `apply_fix`) is **deferred to v1.1** pending a design-partner integration that validates the threat model (MCP chain-of-authorization is weaker than direct CLI authorization: the developer authorizes AOTF, but an MCP-called AOTF is authorized by the calling agent — an audit chain the developer cannot inspect at invocation time). MCP protocol version supported and lifecycle (bound to `aotf watch` daemon; separate process `aotf mcp-serve` available) specified in architecture
- **FR-95:** AOTF validates credential health for configured AI backends and notification channels on startup and alerts when credentials are invalid or nearing expiry

### 9. Observability, Status & Audit

- **FR-66:** AOTF maintains an immutable, append-only local audit log of all actions (fixes, rollbacks, plugin installs, config changes, auth events). For every action classified as `EXECUTE_IRREVERSIBLE` (per FR-105), the log additionally records: approver identity (principal + auth channel), UTC timestamp, stated reason (free-text, minimum 20 characters, logged verbatim with no truncation), the consent token hash (FR-106), the WebAuthn assertion hash, and the classified action tier. Missing any of these fields on an IRREVERSIBLE entry must block the action rather than record a partial entry.
- **FR-67:** Engineer can query and export the audit log via CLI
- **FR-68:** AOTF applies PII filtering to log content in its own outputs by default, with explicit opt-out for verbose logging
- **FR-69:** AOTF tracks per-session token cost for AI backend calls and surfaces cost data in summaries, audit log, and team digest
- **FR-70:** AOTF emits structured JSON log output from the watch daemon on a stable, versioned schema
- **FR-90:** Engineer can view current daemon operational status — monitored sources, last event processed, backend health, current cooldown state — reflecting events within 5 seconds of occurrence
- **FR-91:** Engineer can suppress or snooze a finding class or specific finding for a configurable duration without dismissing it permanently

### 10. AI QA Agent *(Growth)*

- **FR-71:** *(Growth)* AOTF can execute autonomous browser-driven test runs against a running application instance
- **FR-72:** *(Growth)* AOTF can detect and repair broken UI test selectors without human intervention
- **FR-73:** *(Growth)* AOTF can convert user-submitted bug reports into executable regression tests
- **FR-74:** *(Growth)* AOTF generates visual regression findings by comparing rendered output across deployments
- **FR-75:** *(Growth)* AOTF can run QA tests in shadow mode against a replica without affecting live traffic

### 11. AI Agent Operations *(Growth)*

- **FR-76:** *(Growth)* AOTF can ingest and store AI agent run telemetry including inputs, outputs, token counts, and latency
- **FR-77:** *(Growth)* AOTF establishes behavioral baselines for AI agents and detects deviation from baseline
- **FR-78:** *(Growth)* AOTF compares production AI agent behavior against a shadow pipeline to surface regressions proactively
- **FR-79:** *(Growth)* AOTF tracks per-agent cost and surfaces cost trend anomalies
- **FR-80:** *(Growth)* Engineer can configure alert thresholds for agent behavior deviation and cost overrun

### 12. Dashboard *(Growth)*

- **FR-81:** *(Growth)* Engineer can launch a local WebUI providing a live view of active findings, fix status, and CI progress updated within 5 seconds of state change
- **FR-82:** *(Growth)* Team admin can create read-only viewer tokens for sharing dashboard access without fix permissions
- **FR-83:** *(Growth)* AOTF streams live finding and fix events to the dashboard via server-sent events
- **FR-84:** *(Growth)* Engineer can export an incident timeline and correlation graph as a post-mortem document
- **FR-98:** *(Growth)* AOTF offers progressive autonomy escalation — suggests raising confidence thresholds or enabling auto-apply based on historical approval rate and fix success rate

### 13. Security Automation *(Growth)*

- **FR-100:** *(Growth)* AOTF detects known dependency CVEs via the project's language-ecosystem audit tooling (e.g., `cargo audit`, `bun audit`) and generates auto-fix PRs with impact-aware upgrade assessment
- **FR-101:** *(Growth)* AOTF performs context-aware code review on PRs using the accumulated failure library — flagging patterns that have caused incidents in this specific codebase

### 14. ML/LLM Lifecycle Manager *(Vision)*

- **FR-85:** *(Vision)* Engineer can register, version, and retrieve ML models in a git-native model registry
- **FR-86:** *(Vision)* AOTF detects model drift in production and triggers a configurable retraining pipeline
- **FR-87:** *(Vision)* Engineer can version, compare, and roll back prompt templates with production impact tracking
- **FR-88:** *(Vision)* AOTF can run A/B tests between prompt or model versions on live traffic with configurable splits
- **FR-89:** *(Vision)* AOTF detects and alerts on prompt injection attempts in production AI agent inputs

### 15. Ecosystem *(Vision)*

- **FR-102:** *(Vision)* Engineer can create, share, and import AOTF recipes — reusable workflow packages that bundle watcher configs, plugin sets, and fix policies for common stacks

---

### FR Phase Summary

| Phase | Count | FR IDs |
|---|---|---|
| **MVP (v1.0)** | **81** | FR-01–FR-15, FR-17–FR-21, FR-23–FR-52, FR-54–FR-70, FR-90–FR-97, FR-99, FR-103–FR-107 |
| **Growth (v0.2–v0.5)** | **20** | FR-16, FR-22, FR-53, FR-71–FR-84, FR-98, FR-100, FR-101 |
| **Vision (post-v1.0)** | **6** | FR-85–FR-89, FR-102 |
| **Total** | **107** | — |

*Note on FR numbering:* numbers reflect Party-Mode (2026-04-17) insertion order, not strict section order. Cross-section FR references are stable (FR-N uniquely identifies a requirement) but readers cannot infer scope or section from the FR number. A strict section-monotonic renumber is deferred to post-architecture-regeneration to minimize doc churn across architecture, epics, and external references (ux-research-inputs, readiness reports).

*MVP categories:* Project Initialization & Configuration (10), Log Monitoring & Detection (7), AI Diagnosis (6), Autonomous Fix & Safety (19), Goal Loop (6), Notifications & Team Coordination (11), Plugin Ecosystem (9), AI Backend Management (6), Observability/Audit (7) = **81 MVP FRs**.

*Growth categories:* AI QA Agent (5), AI Agent Operations (5), Dashboard (5), Security Automation (2), plus 3 scattered Growth items = **20 Growth FRs**.

*Vision categories:* ML/LLM Lifecycle Manager (5), Ecosystem (1) = **6 Vision FRs**.

---

## Non-Functional Requirements

*Note: Technical Success Criteria (TS-01 through TS-12) in the Success Criteria section define testable acceptance targets. The NFRs below define the quality attributes and architectural constraints that make those targets achievable. Where overlap exists, the TS-xx metric is the acceptance gate; the NFR is the design constraint.*

### Performance

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-P01** | Watch daemon memory: <100MB RSS under 10 concurrent log sources running 24h; internal target <50MB | Engineers run AOTF alongside IDEs and test runners — memory contention is a trust-killer |
| **NFR-P02** | CLI response time for all non-AI operations (config read, plugin resolve, finding lookup): <500ms | Excludes AI subprocess calls; fast CLI = feels native |
| **NFR-P03** | Watch daemon startup to first log read: <3s; daemon reconnection after crash: <5s | Slow startup makes engineers distrust the daemon is running |
| **NFR-P04** | AOTF binary cold-start including config parse + plugin manifest load: <50ms | First impression of tool responsiveness |
| **NFR-P05** | AI diagnosis latency (subprocess call to AI CLI, P95): <30s | Beyond 30s engineers move on; under 10s is ideal |
| **NFR-P06** | Concurrent log sources with no degradation in P01/P02 targets: 20+ sources | Teams with complex microservices need multi-source |
| **NFR-P07** | Finding burst handling: 1000 errors/minute ingested without data loss (backpressure model absorbs spikes) | Log storm during incidents is the worst time for data loss |

### Security

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-S01** | All secrets (API keys, tokens) stored in OS-native credential storage (macOS Keychain, Linux libsecret) or an encrypted secrets-management store (e.g., SOPS) for headless/CI environments; never written to disk in plaintext or logged | Supply chain attacks target credential files |
| **NFR-S02** | No plugin code executes before SHA-256 checksum verification; 100% enforcement, zero silent loads | Plugin ecosystem is the highest supply-chain risk surface |
| **NFR-S03** | No code content transmitted to AI backend without per-repo explicit consent stored in `.aotf/consent.yaml` | Data sovereignty requirement for enterprise users |
| **NFR-S04** | All AOTF autonomous actions pass the configured control surface before execution; controls are not bypassable via config except the explicit `--force-local-auto-approve` flag. The force flag bypasses the **five pipeline gates (FR-23 through FR-27:** Dry-Run, Scope Limit, Approval, Mutex, Rollback) for low-friction local use, but NEVER bypasses: (i) the FR-66 Audit cross-cutting control, (ii) the FR-105 action-tier ceiling, or (iii) the FR-106 production consent token requirement for `EXECUTE_IRREVERSIBLE` actions. Audit is non-bypassable by design (it is a tap on every action, not a gate); tier and consent enforcement are non-bypassable for irreversibility-class actions regardless of flag routes | Trust requires non-bypassable safety rail; irreversible action and audit logging always hold regardless of flag routes. Aligns with seven-control trust surface (five gates + two cross-cutting controls) per Innovation claim #4. |
| **NFR-S05** | GitHub webhook payloads validated via HMAC-SHA256; unsigned payloads rejected | Prevents webhook injection |
| **NFR-S06** | Audit log is append-only; AOTF itself cannot delete or modify past entries | Tamper-evidence for compliance |
| **NFR-S07** | MCP server binds to localhost only by default. v1.0 tool set is strictly read-only (see FR-65) — no MCP tool may mutate state, so no `fix` tool exists to authorize in v1.0. **MCP invocation tokens** (scoped per-session, localhost-bound) authorize *read* access; invalid or missing tokens produce `McpAuthRequired` error, logged and rate-limited. Write-capable MCP tools in v1.1+ will require a distinct write-scope token and pass through the same action-tier enforcement (FR-105) as CLI-originated operations | Read-only surface reduces v1.0 threat surface to info-disclosure; write authorization is additive in v1.1 after design-partner validation |
| **NFR-S08** | Plugin capability violations logged within 100ms of detection; on Linux enforced via seccomp-bpf; on macOS enforced at import-hook level | Defense-in-depth for plugin sandbox |
| **NFR-S09** | No PII (code snippet content, error message text) appears in telemetry payloads; telemetry contains only command names, timing, OS/arch | Privacy-first telemetry by design |
| **NFR-S10** | The action-tier taxonomy (FR-105) is enforced in a **memory-safe, capability-bounded core whose failure domain is isolated from the orchestrator** — not at the SDK/adapter layer and not by convention. The specific mechanism (Rust FFI, seccomp-bound subprocess, WASM sandbox with capability imports, or equivalent) is an architectural decision (see [architecture.md](./architecture.md) §Trust Boundary and its ADRs). Invariants that MUST hold regardless of mechanism: (i) the enforcement core's tier-policy state is not writable from any plugin-reachable code path (FR-105.2); (ii) bypassing any SDK/adapter layer MUST NOT bypass enforcement; (iii) a mutation test is required — mutating the gate's allow/deny branch MUST cause ≥1 integration test to fail (FR-105.2 verification). Plugins with a missing `actionTier` field in their capability manifest are rejected at plugin-trust time from v1.1+; v1.0.x accepts missing `actionTier` as implicit `UNKNOWN` mapped to `READ` with deprecation warning. **Core scope test:** code belongs in the enforcement core iff *"if this code is wrong, a reversible-action tier can escalate to irreversible without authorization."* Code that fails this test belongs outside the core | Enforcement "by convention" collapses to trust-the-plugin. The architect selects the strongest boundary available to the platform; the PRD specifies the invariant the boundary must uphold, not the mechanism |
| **NFR-S11** | "Production-designated" (for FR-106 consent-token gating) is a strictly binary per-environment flag in the AOTF config. Environments not explicitly designated `production: true` are NOT production for token purposes — this includes staging, canary, shadow, production-like, and ephemeral preview environments, regardless of name or similarity to production | Without a binary designation, blast-radius creep collapses consent-token security: engineers build token-vending services to handle frequent "production-like" runs and the token becomes ceremonial |

### Reliability

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-R01** | Watch daemon continues monitoring if the AI backend CLI subprocess crashes or times out; subprocess timeout ≤30s | Daemon uptime must be independent of AI quality |
| **NFR-R02** | SQLite WAL mode: no findings from the last 30 seconds silently lost on SIGKILL | Finding loss during incidents destroys trust |
| **NFR-R03** | Plugin crash: daemon continues; affected plugin marked `degraded`; no cascade to other plugins or watcher loop | One bad plugin must not take down the tool |
| **NFR-R04** | On startup after unclean shutdown: AOTF detects and resolves in-flight operation state before proceeding (no corrupt partial state) | Restart-safe by design |
| **NFR-R05** | Goal Loop worktree cleanup: all worktrees removed on task completion (success or failure); no residual state | Prevents disk accumulation in CI environments |
| **NFR-R06** | AI backend degraded mode is explicitly signaled to the user; AOTF does not silently reduce capability | Silent degradation destroys trust more than explicit errors |

### Privacy & Data Handling

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-D01** | PII filtering is ON by default in all AOTF log outputs; content is hashed unless user opts in to verbose logging | Wrong default costs adoption; right default builds trust |
| **NFR-D02** | Cross-org intelligence tier transmits anonymized failure signatures only — no source code, no error messages, no file paths | Network effects without data sovereignty risk |
| **NFR-D03** | All learning (failure library, codebase personality) remains entirely local by default; no data leaves the machine without explicit opt-in action | "Zero data egress by default" is a hard architectural property |
| **NFR-D04** | Self-hosted deployment path: no phone-home telemetry, no third-party CDN dependencies, documented data flow diagram published | Full data residency claim requires all three properties |

### Scalability

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-SC01** | Single-repo: handles repos up to 500k LOC without performance degradation in P01–P04 targets | Large monorepos are common in target companies |
| **NFR-SC02** | Multi-repo: supports up to 10 concurrent repos per AOTF instance with independent configs and fix mutexes | Teams with microservices run AOTF across multiple repos |
| **NFR-SC03** | Goal Loop: up to 3 concurrent worktrees per AOTF instance within the attempt ceiling | Parallel Goal Loop attempts on different findings |
| **NFR-SC04** | AI backend concurrent call ceiling: configurable (default 3); AOTF queues excess calls rather than dropping them | Prevents API quota exhaustion while preserving throughput |

### Operability

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-O01** | Single distributable binary that installs cleanly via `brew install` and `curl \| sh`. Exact size target set empirically from first functional build; optimize aggressively but do not commit to a hard ceiling until the real binary composition is measured | Hard size targets set before first build are guesses; measure and report |
| **NFR-O02** | `aotf doctor` command validates: config schema, git permissions, AI CLI availability and version, plugin integrity, Telegram connectivity | Engineers must be able to self-diagnose before filing support issues |
| **NFR-O03** | All AOTF error messages include an actionable resolution path; no bare stack traces surfaced to users | Error UX is a first-impression moment |
| **NFR-O04** | Update check: on CLI invocation, AOTF checks for newer version at most once per 24h (opt-out via config); update is never automatic | Keeps users current without surprising them |
| **NFR-O05** | Structured JSON log output schema is stable across minor versions; breaking changes require major version bump and CHANGELOG migration note | Downstream tooling depends on schema stability |

### Integration Reliability

| NFR | Requirement | Rationale |
|---|---|---|
| **NFR-I01** | GitHub API rate limit: AOTF backs off with exponential retry (max 5 retries) and surfaces headroom warnings before exhaustion | Rate limit surprises break CI pipelines |
| **NFR-I02** | Telegram delivery: queue-based dispatch with configurable TTL; expired messages dropped with audit log entry (not silently failed) | Silent notification loss is worse than no notification |
| **NFR-I03** | AI CLI subprocess interface: AOTF validates output schema conformance on each call; schema violation produces a logged finding, not a crash | Upstream breaking changes must not crash the daemon |
| **NFR-I04** | Plugin conformance: plugin schema version declared and validated at install time; version mismatch produces a clear error, not silent degradation | Plugin ecosystem must not regress silently across AOTF versions |
