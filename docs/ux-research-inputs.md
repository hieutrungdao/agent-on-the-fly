# UX Research Inputs — AOTF

> ## ℹ️ STATUS UPDATE 2026-04-19 — content now redundant with canonical PRD
>
> On 2026-04-19 the canonical [PRD.md](./PRD.md) was restored to its 99-FR Party-Mode scope (v2.0), which **already contains** all three blocks extracted into this file:
>
> - §1 User Journeys → PRD v2.0 §5 (Journeys 1–12 + edge-case stubs + journey summary)
> - §2 Domain-Specific Requirements → PRD v2.0 §6 (code safety, decision logging, security, privacy, worktree isolation, OSS trust model, integration constraints, risks)
> - §3 Developer Tool + CLI — Specific Requirements → PRD v2.0 §8 (command structure, exit codes, finding schema, output formats, telemetry, config schema, watch pipeline, plugin API, language matrix, install methods)
>
> The FR-reference numbers in this file (FR-05, FR-14, FR-49, FR-66, FR-68, FR-71–FR-75, FR-105, FR-106, NFR-D01) **now resolve correctly** against PRD v2.0. No renumbering needed.
>
> **This file is preserved as a historical extraction artifact.** If you want the authoritative current spec, read [PRD.md](./PRD.md). If you want to understand why this file was created (the 2026-04-18 PRD narrowing and its 2026-04-19 reversal), see:
>
> - [`_bmad-output/planning-artifacts/implementation-readiness-report-2026-04-17.md`](../_bmad-output/planning-artifacts/implementation-readiness-report-2026-04-17.md) — readiness check recommending 99-FR scope
> - [`_bmad-output/planning-artifacts/implementation-readiness-report-2026-04-19.md`](../_bmad-output/planning-artifacts/implementation-readiness-report-2026-04-19.md) — readiness check that discovered the scope fork
> - [`_bmad-output/planning-artifacts/prd.md`](../_bmad-output/planning-artifacts/prd.md) — the recovered source file (identical to current `docs/PRD.md` body)
>
> *(Redundancy banner added during PRD v1.1 → v2.0 reconciliation.)*

---

**Original header (preserved for provenance):**

**Purpose:** Source material for the UX design workflow. Preserves the high-value
persona journeys, CLI contract, and domain-specific constraints that were developed
during initial planning but condensed out of the canonical [PRD.md](./PRD.md).

**Status:** ~~Reference / input document. Not a spec. Where content here conflicts
with [PRD.md](./PRD.md), the PRD is canonical.~~ → **As of 2026-04-19: historical artifact. PRD v2.0 contains the authoritative copy.**

**Provenance:** Extracted on 2026-04-18 from the legacy draft PRD
(`_bmad-output/planning-artifacts/prd.md`, 1,433 lines) ~~before that file was
removed~~ **— that file has since been recovered (2026-04-19) and its content re-adopted as the canonical PRD v2.0**. The extract covers three blocks: User Journeys (§1), Domain-Specific
Requirements (§2), and Developer Tool + CLI — Specific Requirements (§3).

---

## 1. User Journeys

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

## 2. Domain-Specific Requirements

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

## 3. Developer Tool + CLI — Specific Requirements

### Command Structure

**Core command surface (v1.0):**

```
aotf init                          # Project setup + auto-discovery
aotf watch [--source <type>]       # Start daemon, attach log watchers
aotf diagnose [<finding-id>]       # AI diagnosis; interactive selection if no ID given
aotf fix [<finding-id>] [--mode <diagnosis_only|fix_and_pr>] [--scope <single|multi>] [--dry-run]
aotf rollback <finding-id>         # Revert a merged fix
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
