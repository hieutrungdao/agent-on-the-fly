# Agent On the Fly — Product Requirements Document

**Author:** hieutrungdao
**Date:** 2026-04-13
**Version:** 1.1
**Status:** Draft

### Changelog

- **1.1 (2026-04-13)** — Aligned with Architecture v1.1: added FR-49–FR-69 covering Plugin Security (allowlist + checksum + capabilities), Polling Watcher abstraction and implementations (Loki/Grafana/Datadog/CloudWatch/Elasticsearch), CLI Agent backend abstraction (Claude Code / Codex CLI / OpenCode), Context Window Management, Auto-Fix Safety (approval/dry-run/scope-limit/mutex/rollback), Git Failure Mode Handling, multi-user dashboard auth + audit log. Updated FR-04, FR-05, FR-16, FR-30 to reflect new ABCs and security model. Updated Security Considerations and Strategic Positioning accordingly.
- **1.0 (2026-04-04)** — Initial draft.

---

## Executive Summary

**Agent On the Fly (AOTF)** is an open-source, AI-powered SDLC health platform that autonomously monitors, diagnoses, and fixes software issues across the full development lifecycle. It combines real-time error detection, intelligent log analysis, AI-powered root-cause diagnosis, autonomous code remediation, CI/CD pipeline automation, and MLOps/LLMOps monitoring into a single CLI-first tool with a pluggable architecture.

AOTF addresses the fundamental gap in the DevOps toolchain: existing tools detect problems but require human intervention to fix them. By deploying an AI agent that understands code context, AOTF reduces mean-time-to-resolution (MTTR) by 70%+ and transforms reactive incident response into proactive autonomous remediation.

### Strategic Positioning

| Dimension | AOTF Position |
|---|---|
| **Primary value** | Autonomous error detection + AI diagnosis + code remediation |
| **Interface** | CLI-first with optional web dashboard |
| **Architecture** | Plugin-driven (allowlist + checksum + capability scoping); swappable AI backends (CLI agents + SDK), CI providers, notifications, watchers (streaming + polling); five-gate Auto-Fix Safety model |
| **Deployment** | Self-hosted (zero cloud dependencies) or cloud-connected |
| **Pricing** | Free and open-source (MIT/Apache 2.0) |
| **Target market** | Platform engineers, SREs, backend developers, MLOps engineers |

### Competitive Landscape

| Tool | Category | Detection | AI Diagnosis | Auto-Fix PRs | CI Integration | MLOps | OSS | Cost |
|---|---|---|---|---|---|---|---|---|
| **Sentry** | Error tracking | Yes | No | No | Limited | No | SDK only | $26-89/user/mo |
| **Datadog** | Full observability | Yes | Enterprise | No | Limited | Enterprise | No | $23-34/host/mo |
| **ArgoCD** | GitOps deployment | No | No | No | Deploy only | No | Yes | Free |
| **MLflow** | ML experiment tracking | No | No | No | No | Yes | Yes | Free |
| **PagerDuty** | Incident management | Via integrations | No | No | No | No | No | $21-41/user/mo |
| **GitHub Copilot** | Code generation | No | No | No | No | No | No | $19-39/user/mo |
| **AOTF** | **SDLC health platform** | **Yes** | **Yes (3 modes)** | **Yes** | **Yes** | **Yes (Vision)** | **Yes (Full)** | **Free** |

**Market Gap:** No open-source tool combines real-time error detection, AI-powered root-cause diagnosis, and autonomous code remediation with a pluggable architecture. AOTF owns this intersection.

---

## Success Criteria

| ID | Criterion | Target | Measurement | Phase |
|---|---|---|---|---|
| SC-01 | Detect regex-matchable errors within N seconds of log emission | <60s | Integration tests with timed error injection | MVP |
| SC-02 | AI root-cause diagnosis accuracy | >80% correct | Human review of 100 sampled diagnoses | MVP |
| SC-03 | Auto-generated fix PRs that pass CI | >60% pass rate | Track PR merge rates across test repos | MVP |
| SC-04 | Time from CLI install to first error detection | <5 minutes | User onboarding tests | MVP |
| SC-05 | Plugin development time for new CI provider | <4 hours | Contributor onboarding sessions | MVP |
| SC-06 | Memory footprint of watch daemon | <100MB RSS | Monitoring over 24h with 10+ log sources | MVP |
| SC-07 | GitHub stars within 6 months of launch | 1,000+ | GitHub metrics | Growth |
| SC-08 | Community-contributed plugins within 12 months | 5+ | Plugin registry count | Growth |

---

## Product Scope

### Phase 1: MVP (Epics 1-5, ~12 weeks)

Core detect → diagnose → fix loop with CLI interface, GitHub Actions integration, and Claude AI backend.

### Phase 2: Growth (Epics 6-7, ~6 weeks)

Web dashboard, additional AI backends (GPT, Gemini, local models), additional CI providers (GitLab, Azure DevOps, Jenkins), expanded notification plugins.

### Phase 3: Vision (Epics 8-10, ~8 weeks)

MLOps/LLMOps integration, E2E testing framework, distributed tracing and cross-service bug tracing.

---

## User Journeys

### UJ-1: First Detection

**Persona:** Alex, a backend developer who just deployed a new microservice

```
1. Alex installs AOTF: `pip install aotf`
2. Alex initializes for the project: `aotf init`
   → AOTF scans the project, detects Docker containers, creates .aotf/config.yaml
3. Alex starts watching: `aotf watch`
   → AOTF begins tailing Docker logs for the configured containers
4. A 500 error occurs in the API service
   → AOTF detects it within 60s via regex pattern matching
   → Console notification: "ERROR detected in api-service: HTTP 500 at /users/123"
   → Error stored with ID ERR-001
5. Alex views errors: `aotf errors list`
   → Sees ERR-001 with timestamp, service, error message, occurrence count
```

**Acceptance:** Error detected <60s after emission; deduplication prevents alert fatigue; error stored with queryable ID.

### UJ-2: Diagnose and Fix

**Persona:** Alex continues from UJ-1

```
1. Alex requests diagnosis: `aotf diagnose ERR-001`
   → AOTF collects: error context, relevant source files, recent git changes, log snippets
   → Sends structured prompt to Claude AI backend
   → Receives: root cause, affected files, suggested fix, confidence score (87%)
2. Alex reviews diagnosis output:
   → Root cause: "NullPointerException in UserService.getUser() — user_id parameter
      not validated before database query"
   → Affected files: src/services/user_service.py:45, src/api/users.py:23
   → Suggested fix: "Add null check for user_id before query execution"
3. Alex requests fix: `aotf fix ERR-001 --mode fix_and_pr`
   → AOTF creates branch: aotf/fix-ERR-001
   → AI applies code changes to affected files
   → Opens PR: "fix: Add null validation for user_id in UserService.getUser()"
   → Notification: "Fix PR created: #42 (confidence: 87%)"
4. Alex reviews the PR, approves, merges
```

**Acceptance:** Diagnosis completes <2 minutes; structured output includes root cause + files + confidence; PR contains only relevant changes.

### UJ-3: Full Auto Mode

**Persona:** Priya, an SRE running AOTF in production with full_auto mode enabled

```
1. AOTF watch daemon detects repeated 503 errors in payment-service
2. AOTF auto-triggers diagnosis → root cause identified (connection pool exhaustion)
3. AOTF creates fix branch and PR → increases pool size, adds connection timeout
4. CI runs automatically → all tests pass
5. AOTF auto-merges the PR
6. Slack notification: "Auto-fixed ERR-047: connection pool exhaustion in
   payment-service. PR #89 merged. CI passed. No human intervention required."
7. Priya reviews the morning summary and confirms the fix was correct
```

**Acceptance:** End-to-end fix without human intervention; CI must pass before merge; notification includes full context; audit trail preserved.

### UJ-4: Plugin Author

**Persona:** Jordan, a developer who wants to add GitLab CI support

```
1. Jordan reads plugin development guide: `aotf docs plugin-guide`
2. Jordan creates a new Python package:
   → Implements CIProviderABC with 4 methods (create_pr, check_ci_status, merge_pr, get_workflow_config)
   → Adds entry_point: aotf.ci_providers = gitlab = aotf_gitlab:GitLabProvider
3. Jordan installs locally: `pip install -e ./aotf-gitlab-plugin`
4. Jordan tests: `aotf plugin list` → shows "gitlab" as available CI provider
5. Jordan configures: updates .aotf/config.yaml with ci_provider: gitlab
6. Jordan submits PR to AOTF plugin registry
```

**Acceptance:** Plugin ABC is well-documented with type hints; plugin discovery works via entry_points; local development workflow is <4 hours for experienced developer.

### UJ-5: MLOps Alert (Vision Phase)

**Persona:** Chen, an ML engineer monitoring a recommendation model

```
1. Chen configures AOTF to watch model metrics log file
2. AOTF detects accuracy drift: model accuracy dropped from 92% to 84% over 48 hours
3. AOTF cross-references with recent data pipeline changes
4. AI diagnosis: "Feature distribution shift in user_age column after ETL pipeline
   change merged 2 days ago. Commit abc123 modified age normalization logic."
5. AOTF creates alert with diagnosis, links to MLflow experiment, suggests rollback
6. Slack notification: "MODEL DRIFT: recommendation_model accuracy -8% over 48h.
   Root cause: age normalization change (commit abc123). Suggested: rollback ETL."
```

**Acceptance:** Drift detection within configurable time window; cross-reference with code changes; actionable diagnosis with commit links.

---

## Functional Requirements

### Core Engine (MVP)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-01 | Users can install AOTF via `pip install aotf` (PyPI) and optionally via `brew install aotf` (Homebrew) | P0 | 1 |
| FR-02 | Users can initialize AOTF for a project via `aotf init` which scans the project and creates `.aotf/config.yaml` with sensible defaults | P0 | 1 |
| FR-03 | AOTF provides a plugin registry that discovers plugins via Python entry_points and local `.aotf/plugins/` directory | P0 | 1 |
| FR-04 | AOTF defines abstract base classes for: `AIBackendABC` + `CLIAgentBackendBase` (subprocess CLI agent shared base), `CIProviderABC`, `NotificationChannelABC`, `StreamingLogWatcherABC`, `PollingLogWatcherABC`, `ErrorDetectorABC`, `RepositoryABC` | P0 | 1 |
| FR-05 | AOTF monitors multiple **streaming** log sources simultaneously: file tail (with rotation), Docker container logs, stdin (built-in). Polling sources covered by FR-65–FR-69. | P0 | 2 |
| FR-06 | AOTF detects errors using configurable regex patterns with sensible defaults for common frameworks (Python tracebacks, HTTP 5xx, Java exceptions, Node.js errors) | P0 | 2 |
| FR-07 | Detected errors are deduplicated using content hashing with configurable TTL window | P0 | 2 |
| FR-08 | Users can run `aotf watch` as a background daemon with configurable poll interval | P0 | 2 |
| FR-09 | Users can view error history via `aotf errors list` with filtering by service, severity, time range, and status | P0 | 2 |
| FR-10 | Rate-limited notifications are dispatched on error detection to configured channels | P0 | 2 |
| FR-11 | Users can invoke AI diagnosis on any detected error via `aotf diagnose <error-id>` | P0 | 3 |
| FR-12 | AI diagnosis produces structured output: root cause description, affected file paths with line numbers, suggested fix description, confidence score (0-100) | P0 | 3 |
| FR-13 | Users can invoke auto-fix via `aotf fix <error-id>` which creates a git branch and PR | P0 | 3 |
| FR-14 | AOTF supports three fix modes configurable per-project: `diagnosis_only`, `fix_and_pr`, `full_auto` | P0 | 3 |
| FR-15 | Diagnosis and fix results are stored persistently with full audit trail | P0 | 3 |
| FR-16 | Users can configure AI backend via plugin. **Claude Code CLI** is the MVP default; Codex CLI and OpenCode are additional built-in CLI agent backends in Growth (Epic 7), all sharing `CLIAgentBackendBase`. SDK-style backends (OpenAI/Gemini/local) are also supported in Growth. | P0 | 3 |

### CI Integration (MVP)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-17 | GitHub Actions CI provider plugin can create PRs, monitor CI status, and report results | P0 | 4 |
| FR-18 | Auto-merge capability for `full_auto` mode with configurable policies (require CI pass, require N approvals) | P1 | 4 |
| FR-19 | AOTF can be integrated into existing CI workflows via GitHub Actions reusable workflow | P0 | 4 |

### Notifications (MVP)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-20 | Console notification plugin provides Rich-formatted terminal output for all events | P0 | 5 |
| FR-21 | Slack webhook notification plugin sends structured messages with error context | P1 | 5 |
| FR-22 | MS Teams Adaptive Card notification plugin sends interactive cards via webhook | P1 | 5 |
| FR-23 | Email SMTP notification plugin sends HTML-formatted error reports | P1 | 5 |
| FR-24 | Notification rate limiting prevents alert fatigue (configurable max per hour per channel) | P0 | 5 |
| FR-25 | Notification templates are customizable per channel | P2 | 5 |

### Web Dashboard (Growth)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-26 | Optional web dashboard displays real-time error timeline with SSE streaming | P1 | 6 |
| FR-27 | Dashboard shows diagnosis results with structured display (root cause, files, confidence) | P1 | 6 |
| FR-28 | Dashboard shows fix status tracker (PR link, CI status, merge state) | P1 | 6 |
| FR-29 | Users can launch dashboard via `aotf dashboard` CLI command | P1 | 6 |
| FR-30 | Dashboard requires **per-user token-based authentication** with scoped roles (viewer/operator/maintainer/admin) — see FR-63, FR-64. No shared password. | P1 | 6 |

### Additional Integrations (Growth)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-31 | OpenAI GPT AI backend plugin with API key configuration | P1 | 7 |
| FR-32 | Google Gemini AI backend plugin | P1 | 7 |
| FR-33 | Local model AI backend plugin (Ollama, llama.cpp) | P1 | 7 |
| FR-34 | GitLab CI provider plugin | P1 | 7 |
| FR-35 | Azure DevOps CI provider plugin | P1 | 7 |
| FR-36 | Jenkins CI provider plugin | P2 | 7 |

### MLOps / LLMOps (Vision)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-37 | AOTF can collect model performance metrics from log sources (accuracy, latency, throughput) | P2 | 8 |
| FR-38 | Data drift detection using statistical tests on feature distributions with configurable thresholds | P2 | 8 |
| FR-39 | MLflow integration plugin for experiment tracking correlation | P2 | 8 |
| FR-40 | LLM prompt versioning tracker with git-based prompt file monitoring | P2 | 8 |
| FR-41 | Prompt quality correlation: track prompt version changes vs output quality metrics | P2 | 8 |

### E2E Testing (Vision)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-42 | YAML-based test suite definition format for E2E tests | P2 | 9 |
| FR-43 | Test runner abstraction with plugins for Playwright, Cypress, Selenium | P2 | 9 |
| FR-44 | Deploy trigger integration: auto-run E2E tests after successful deployment | P2 | 9 |
| FR-45 | AI-powered test failure analysis (categorization, flakiness detection) | P2 | 9 |

### Distributed Tracing (Vision)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-46 | OpenTelemetry trace correlation: link detected errors to distributed traces | P2 | 10 |
| FR-47 | Cross-service error graph: visualize which service errors cascade to others | P2 | 10 |
| FR-48 | `aotf trace <error-id>` command for distributed root-cause analysis | P2 | 10 |

### Plugin Security (MVP — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-49 | Plugin allowlist: only plugins listed in `.aotf/plugins.lock.yaml` may load. Plugins discovered via entry points but absent from the lockfile are logged and skipped, never silently loaded. | P0 | 1 |
| FR-50 | SHA-256 checksum verification: on load, the validator computes SHA-256 of the resolved plugin module/wheel and compares to the lockfile entry; mismatch raises `PluginIntegrityError`. | P0 | 1 |
| FR-51 | Capability scoping: plugins declare required capabilities (`watcher`, `ai_backend`, `ci_provider`, `notifications`, `storage`, `network.outbound`, `git.write`, `subprocess`) in `pyproject.toml`. Lifecycle manager enforces via import-hook; violations raise `PluginCapabilityViolation`. `aotf plugin trust <name>` is the explicit user gesture to grant capabilities. | P0 | 1 |

### Polling Watcher Foundation (MVP — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-52 | `PollingLogWatcherABC` for query-response log sources (Loki, Grafana, Datadog, CloudWatch, Elasticsearch). Distinct from `StreamingLogWatcherABC`. Defines `query(since, until)`, `cursor()`, `poll_interval`. | P0 | 2 |
| FR-53 | `WatcherCursor` persistence so polling watchers can resume without duplicates after restart. Stored via `WatcherCursorRepo` keyed on `(source_name, watcher_name)`. | P0 | 2 |

### CLI Agent Backend Abstraction & Context Management (MVP — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-54 | `CLIAgentBackendBase` shared base class for subprocess-wrapped agentic CLIs. Handles subprocess lifecycle, JSON event streaming (`AgentEvent`), tool-call parsing, timeout enforcement. Subclasses override `build_command()`, `parse_event()`, `extract_result()`. | P0 | 3 |
| FR-55 | Codex CLI (`CodexCLIBackend`) and OpenCode (`OpenCodeBackend`) are built-in CLI agent backends alongside Claude Code, demonstrating the abstraction. Each declares its own `max_input_tokens` and event format. | P1 | 7 |
| FR-56 | Context Window Management with layered token budget. `ContextBuilder` apportions budget across error+log (10%), git history (10%), source files (70%), system (10%). When budget cannot be met, returns `DiagnosisResult` with `confidence=0` and `reasoning="context_overflow: ..."` — never silently drops context. | P0 | 3 |

### Auto-Fix Safety Model (MVP — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-57 | **Approval Gate**: fix operations above a configurable risk threshold require human approval. Defaults: confidence < 80 OR blast_radius > medium → queue for dashboard approval (`full_auto`) or PR-only (`fix_and_pr`). `aotf approve <fix_id>` is the explicit gesture. | P0 | 3 |
| FR-58 | **Dry-Run renderer**: every fix renders its unified diff + written summary before any application. Output stored on `FixResult`; requestable via `aotf fix <id> --dry-run`. | P0 | 3 |
| FR-59 | **Scope Limiter**: reject fixes that exceed declared limits — >10 files changed, >500 lines changed, paths matching `safety.protected_paths` (default: `**/migrations/**`, `**/*.lock`, `infra/**`, `.github/workflows/**`), or modifications to dependency files (`pyproject.toml`, `package.json`, `Pipfile`). | P0 | 3 |
| FR-60 | **Per-Repo Fix Mutex**: storage-backed advisory lock keyed on `repo_root` (via SQLite `INSERT OR IGNORE`) prevents concurrent fixes on the same repo. TTL = `fix.timeout_seconds + 60s` for crash recovery. Mutex protects the entire detect-source-state → apply → push sequence. | P0 | 3 |
| FR-61 | **Rollback Manager**: every applied fix is reversible. `aotf fix rollback <id>` deletes the branch (unmerged) or creates a revert PR (merged). Pre-apply records `(HEAD SHA, branch)` to `fix_rollback` table. | P0 | 3 |

### Git Failure Mode Handling (MVP — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-62 | `GitOperations` codifies handling for: dirty working tree on entry (refuse + suggest), push rejected (fetch + rebase + retry once), merge conflict applying AI diff (per-hunk fallback + report unapplied hunks), source file changed since detection (re-diagnose against current HEAD), detached HEAD (refuse), missing credentials in non-interactive env (fail-fast with `GIT_TERMINAL_PROMPT=0`), branch already exists (timestamp suffix). | P0 | 4 |

### Multi-User Dashboard Auth & Audit (Growth — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-63 | User and token management via `aotf user create/list/revoke <name> --role <viewer\|operator\|maintainer\|admin>`. Tokens are Argon2id-hashed in storage; roles map to scopes (`read`, `diagnose`, `fix.suggest`, `fix.apply`, `fix.merge`, `admin`). On first run with no users, a one-time bootstrap admin token is printed to stderr and the HTTP server refuses to start until consumed. | P1 | 6 |
| FR-64 | Append-only audit log: every state-changing action (token issuance, diagnosis trigger, fix apply, merge) recorded with `(timestamp, user, action, target, ip, user_agent, request_id)`. Surfaced in dashboard `/audit` view and queryable via `aotf user audit`. | P1 | 6 |

### Polling Watcher Implementations (Growth — Architecture v1.1)

| ID | Requirement | Priority | Epic |
|---|---|---|---|
| FR-65 | Loki polling watcher plugin — `/loki/api/v1/query_range` with LogQL filters; cursor = `(end_ts, last_id)`. | P1 | 7 |
| FR-66 | Grafana polling watcher plugin — Grafana data source proxy; resolves data source by UID, delegates to underlying Loki/Elasticsearch. | P1 | 7 |
| FR-67 | Datadog polling watcher plugin — `/api/v2/logs/events` with cursor-based pagination. | P1 | 7 |
| FR-68 | CloudWatch Logs polling watcher plugin — `FilterLogEvents` API with `nextToken` pagination. | P2 | 7 |
| FR-69 | Elasticsearch polling watcher plugin — `_search` with PIT + `search_after` for stable pagination. | P2 | 7 |

---

## Non-Functional Requirements

| ID | Requirement | Target | Rationale |
|---|---|---|---|
| NFR-01 | CLI command response time for non-AI operations | <500ms | Fast feedback loop for developer experience |
| NFR-02 | Error detection latency from log emission to storage | <60s | Real-time monitoring expectation |
| NFR-03 | Memory footprint of watch daemon | <100MB RSS with 10 log sources | Run alongside development workloads |
| NFR-04 | Platform support | Python 3.10+ on Linux, macOS, Windows (WSL) | Cover all major development platforms |
| NFR-05 | Plugin API backward compatibility | Guaranteed across minor versions (semver) | Protect plugin ecosystem from breaking changes |
| NFR-06 | Configuration method | Env vars + YAML + CLI flags (12-factor) | Standard DevOps practices |
| NFR-07 | Cloud dependencies | Zero mandatory; all cloud integrations optional | Self-hosted deployment must work offline |
| NFR-08 | Test coverage for core modules | >80% line coverage | Reliability for production use |
| NFR-09 | Startup time for watch daemon | <3s to first log read | Quick iteration during development |
| NFR-10 | Concurrent log source handling | 20+ simultaneous sources | Support microservice architectures |
| NFR-11 | Error storage retention | Configurable (default 30 days) | Disk space management for long-running daemons |
| NFR-12 | Structured logging | JSON format with configurable verbosity | Machine-parseable logs for AOTF monitoring itself |

---

## Technical Constraints

| Constraint | Rationale |
|---|---|
| Python 3.10+ only | Type hints, pattern matching, async/await mature support |
| SQLite as default storage | Zero-config local; pluggable interface for Postgres/MongoDB at scale |
| No JavaScript/TypeScript in core | Single-language codebase reduces contributor friction |
| Click + Rich for CLI | Proven combination; Rich provides excellent terminal formatting |
| FastAPI for dashboard | Async, Pydantic models, SSE support; proven in the source CICD module |
| Entry_points for plugins | Standard Python mechanism; no custom discovery needed |
| Pydantic v2 for config/models | Performance, validation, JSON schema generation |

---

## Security Considerations

| Concern | Approach |
|---|---|
| AI-generated code safety | Default to `diagnosis_only`. `fix_and_pr` and `full_auto` pass through five-gate Auto-Fix Safety Model (FR-57–FR-61): Approval Gate, Dry-Run Renderer, Scope Limiter, Per-Repo Fix Mutex, Rollback Manager. `full_auto` auto-merge requires `confidence>=80 AND blast_radius<=medium AND CI pass`. |
| API key storage | Environment variables or OS keyring; never stored in config files; `.aotf/` added to .gitignore template (except `plugins.lock.yaml` which is checked in). |
| Webhook authentication | Secret token validation for incoming triggers |
| Dashboard access | Per-user Argon2id-hashed tokens with scoped roles (FR-63); append-only audit log for state-changing actions (FR-64); CSRF + `SameSite=Strict` cookies; rate-limited login (5/min/IP). HTTPS recommended in production. |
| Plugin trust model | Allowlist (`.aotf/plugins.lock.yaml`) + SHA-256 checksum + capability declaration enforced via import-hook (FR-49–FR-51). Not a sandbox, but catches accidental capability creep and gives auditors a clear surface. `aotf plugin trust` is the explicit user gesture to grant capabilities. |
| Concurrency safety | Per-repo fix mutex (FR-60) prevents racing fixes from diverging working trees; polling watchers apply backpressure when event bus is saturated. |
| Git operation safety | `GIT_TERMINAL_PROMPT=0` and `GIT_ASKPASS=echo` to fail fast on missing credentials; explicit handling for dirty tree, push rejection, conflict, source drift since detection (FR-62). |
| Log data sensitivity | PII filtering plugin interface; configurable redaction patterns |

---

## FR-to-Epic Traceability Matrix

| Epic | Functional Requirements Covered |
|---|---|
| Epic 1: Project Scaffold & Plugin Architecture | FR-01, FR-02, FR-03, FR-04, FR-49, FR-50, FR-51 |
| Epic 2: Error Detection Engine | FR-05, FR-06, FR-07, FR-08, FR-09, FR-10, FR-52, FR-53 |
| Epic 3: AI Diagnosis & Auto-Fix | FR-11, FR-12, FR-13, FR-14, FR-15, FR-16, FR-54, FR-56, FR-57, FR-58, FR-59, FR-60, FR-61 |
| Epic 4: GitHub Actions CI Provider | FR-17, FR-18, FR-19, FR-62 |
| Epic 5: Notification Plugins & Event System | FR-20, FR-21, FR-22, FR-23, FR-24, FR-25 |
| Epic 6: Web Dashboard | FR-26, FR-27, FR-28, FR-29, FR-30, FR-63, FR-64 |
| Epic 7: Additional AI Backends & CI Providers | FR-31, FR-32, FR-33, FR-34, FR-35, FR-36, FR-55, FR-65, FR-66, FR-67, FR-68, FR-69 |
| Epic 8: MLOps & LLMOps Integration | FR-37, FR-38, FR-39, FR-40, FR-41 |
| Epic 9: E2E Testing Framework | FR-42, FR-43, FR-44, FR-45 |
| Epic 10: Distributed Tracing & Bug Tracing | FR-46, FR-47, FR-48 |

---

## Open Questions

| # | Question | Impact | Status |
|---|---|---|---|
| 1 | License choice: MIT vs Apache 2.0? | Apache 2.0 provides patent protection; MIT is simpler | Pending |
| 2 | Should `full_auto` mode be available in MVP or deferred? | Safety risk vs full feature demonstration | **Resolved 2026-04-13:** Available in MVP, gated by Auto-Fix Safety Model (FR-57–FR-61). Default config requires approval below confidence 80 or above blast_radius medium; `full_auto` auto-merge only when all safety gates pass. |
| 3 | Plugin marketplace: centralized registry or decentralized (PyPI-only)? | Discoverability vs maintenance burden | Pending |
| 4 | Should AOTF support Windows natively or WSL-only? | User reach vs development complexity | WSL-only for MVP |
| 5 | Telemetry: should AOTF collect anonymous usage stats? | Product improvement vs privacy concerns | Pending (opt-in if yes) |
