# Product Brief: Agent On the Fly (AOTF)

> ## ℹ️ HISTORICAL CONCEPT BRIEF — see [PRD v2.0](./PRD.md) for current spec
>
> This brief was written **2026-04-04** as the initial product concept. Since then:
>
> - The product evolved into a **four-pillar vision** (added AI QA Agent, Authorized Pre-Production Autonomy, claude-bridge Telegram adapter, AOTF MCP server) during Party-Mode refinement 2026-04-17.
> - The **implementation stack changed from Python → Rust + TypeScript/Bun** (brew/curl single-binary installer, Ratatui TUI, cargo+bun pipelines). The "extracted from xMainframe Python CI/CD system" lineage below describes the *core engine patterns* (regex watcher, rate limiting, notification dispatch), not the production language choice.
> - MVP scope now includes features this brief listed as Growth/Vision: `full_auto` fix mode (gated by five-gate Auto-Fix Safety + action-tier taxonomy), pre-push risk assessment, multi-language fix engine (TS/JS/Python/Go/Rust/Java-Kotlin/Ruby from v1.0).
>
> The vision, problem statement, competitive analysis, target users, and success metrics below **remain valid**. The "In Scope" / "Out of Scope" / "Technical Foundation" sections are **out of date** — refer to [PRD.md](./PRD.md) §4 (Product Scope), §9 (Project Scoping & Phased Development), §8 (Developer Tool + CLI Specific Requirements) for the current breakdown.
>
> *(Forward-pointer banner added 2026-04-19 during PRD v1.1 → v2.0 reconciliation.)*

---

**Date:** 2026-04-04 (concept brief); banner added 2026-04-19
**Author:** hieutrungdao
**Context:** Open-Source Developer Tools / AI-Powered DevOps

---

## Executive Summary

**Agent On the Fly (AOTF)** is an open-source, AI-powered SDLC health platform that autonomously monitors, diagnoses, and fixes software issues across the full development lifecycle. While existing tools force engineers to context-switch between 5-10 separate platforms for error tracking, log analysis, CI/CD, and MLOps, AOTF unifies these concerns under a single CLI-first tool with an AI agent that acts as an "always-on SRE."

**The Problem:** Modern software teams are drowning in operational complexity. When a production error occurs, engineers must jump between Sentry (errors), Datadog (logs), PagerDuty (alerts), ArgoCD (deployments), and custom scripts (CI) to trace root causes. Existing tools _monitor_ but do not _act_ — autonomous remediation is either absent or locked behind expensive enterprise tiers. Meanwhile, MLOps/LLMOps tooling remains completely disconnected from CI/CD pipelines.

**Our Solution:** An AI agent that watches your logs, detects anomalies, traces bugs through distributed systems, diagnoses root causes, and either reports findings or autonomously creates PRs to fix issues — all from a single CLI. Three remediation modes (diagnosis-only, fix-and-PR, full-auto) let teams choose their comfort level with AI autonomy.

**Competitive Advantage:** AOTF is the first open-source platform to combine real-time error detection, AI-powered root-cause analysis, autonomous code remediation, and MLOps monitoring in a single, pluggable tool. Its plugin architecture supports any AI backend (Claude, GPT, Gemini, local models), any CI provider (GitHub Actions, GitLab CI, Azure DevOps, Jenkins), and any notification channel — preventing vendor lock-in while enabling deep integration.

**MVP Strategy:** Prove core value (detect → diagnose → fix) before expanding. Launch with error tracking, log reading, AI diagnosis, auto-fix (diagnosis_only + fix_and_pr modes), CLI interface, GitHub Actions plugin, and Claude AI backend. Web dashboard and MLOps features follow in growth phase.

**Market Opportunity:** The AI-powered DevOps market is exploding, but no open-source tool owns the "autonomous remediation" space. GitHub Copilot and Cursor focus on code generation; Sentry and Datadog focus on monitoring; ArgoCD focuses on deployment. None combine monitoring with autonomous fixing. This is our gap.

**Success Metrics:** Reduce mean-time-to-detection (MTTD) to <60 seconds, reduce mean-time-to-resolution (MTTR) by 70% via auto-fix, achieve 1,000+ GitHub stars in first 6 months, build 5+ community-contributed plugins within 12 months.

---

## Core Vision

### Problem Statement

Software development teams face a convergence of operational challenges that waste engineering time and slow down delivery:

**1. Tool Fragmentation**
- Teams use 5-10 separate tools for errors, logs, alerts, CI/CD, and monitoring
- Context-switching between tools during incident response wastes critical minutes
- No single pane of glass for SDLC health across the entire pipeline
- Each tool requires its own configuration, credentials, and learning curve

**2. Reactive Monitoring Without Remediation**
- Existing tools detect problems but require human intervention to fix them
- Engineers are woken up at 3 AM to manually diagnose and patch issues
- Mean-time-to-resolution (MTTR) is dominated by human response time, not technical complexity
- AI-powered auto-remediation is either non-existent or locked behind enterprise pricing ($100K+/year)

**3. MLOps/LLMOps Disconnection**
- Model monitoring is completely separate from application monitoring
- Data drift and model degradation go undetected until customer complaints arrive
- Prompt versioning for LLMs is ad-hoc with no integration into CI/CD
- Experiment tracking (MLflow, W&B) doesn't connect to deployment pipelines

**4. CI/CD Brittleness**
- CI pipelines break silently or produce cryptic errors
- Test failures require manual investigation to determine root cause
- No intelligent retry or auto-fix when builds fail
- E2E test results are disconnected from deployment decisions

### Problem Impact

**This affects every software team.**

- Engineers spend **30-40% of their time** on operational tasks instead of building features
- The average MTTR for production incidents is **4-6 hours**, with most time spent on diagnosis
- **60% of CI pipeline failures** are caused by the same categories of errors (dependency issues, config drift, flaky tests)
- Organizations pay **$50K-500K/year per team** for monitoring tool subscriptions with no autonomous remediation
- MLOps teams discover model drift **days to weeks** after it begins affecting users

**The urgency:** As systems grow more distributed (microservices, serverless, edge computing), the operational complexity grows exponentially. AI-powered autonomous operations are not a luxury — they're becoming a necessity for teams to maintain velocity.

### Why Existing Solutions Fall Short

**vs. Sentry / Bugsnag (Error Tracking)**
- Detect errors and group them, but provide no automated root-cause analysis
- No code-level fix suggestions, let alone autonomous PR creation
- No CI/CD integration — errors are tracked separately from the build pipeline
- Expensive at scale ($26-89/month per user for enterprise features)

**vs. Datadog / New Relic (Full Observability)**
- Comprehensive monitoring but overwhelming — dashboards require expertise to interpret
- AI features (Watchdog, Applied Intelligence) are enterprise-only ($23-34/host/month)
- No autonomous remediation — alerts trigger PagerDuty, which triggers a human
- Lock-in to proprietary query languages and data formats

**vs. ArgoCD / Flux (GitOps Deployment)**
- Handle deployment orchestration but not error detection or diagnosis
- No AI-powered analysis of deployment failures
- No auto-rollback intelligence beyond simple health checks
- Focused solely on Kubernetes, not general-purpose

**vs. MLflow / Weights & Biases (ML Experiment Tracking)**
- Track experiments and model versions but don't detect production drift autonomously
- No integration with CI/CD pipelines for automated response
- No cross-correlation between model performance and application errors
- MLflow is open-source but has no AI-powered analysis capabilities

**vs. PagerDuty / OpsGenie (Incident Management)**
- Route alerts to humans but don't diagnose or fix anything
- Escalation policies are human-centric, not AI-augmented
- No code-level understanding of the systems they monitor

**The Gap:** No open-source tool combines:
1. Real-time error detection with AI diagnosis
2. Autonomous code remediation (PR creation)
3. CI/CD pipeline integration
4. MLOps/LLMOps monitoring
5. Pluggable architecture for any stack

---

## Proposed Solution

### Product Overview

AOTF is a **CLI-first, plugin-driven platform** that provides:

1. **Watch Mode** (`aotf watch`) — Continuously monitors log sources (files, Docker containers, stdout) for errors using configurable regex patterns with smart deduplication
2. **AI Diagnosis** (`aotf diagnose <error-id>`) — Sends error context, relevant source files, and log snippets to an AI backend for structured root-cause analysis
3. **Auto-Fix** (`aotf fix <error-id>`) — Creates a git branch, applies AI-suggested code changes, and opens a PR — with three modes:
   - `diagnosis_only` — Analyzes and reports; no code changes
   - `fix_and_pr` — Creates a branch with fixes and opens a PR for human review
   - `full_auto` — Creates PR, waits for CI to pass, and auto-merges
4. **Plugin System** — Extensible via Python entry_points for AI backends, CI providers, notification channels, and log watchers
5. **Web Dashboard** (optional) — Real-time error timeline, diagnosis results, and fix status via HTMX-powered UI

### Architecture Philosophy

- **CLI-first, dashboard-optional** — Lower barrier to adoption; works in any environment
- **Zero mandatory cloud dependencies** — SQLite default storage, local AI models supported
- **Plugin everything** — AI backends, CI providers, notifications, log sources all pluggable
- **12-factor configuration** — YAML + environment variables with hierarchical override
- **Extracted from production** — Core engine proven in production at xMainframe CI/CD system

---

## Competitive Analysis

| Capability | Sentry | Datadog | ArgoCD | MLflow | PagerDuty | **AOTF** |
|---|---|---|---|---|---|---|
| Error detection | Yes | Yes | No | No | Via integrations | **Yes** |
| Log analysis | Limited | Yes | No | No | No | **Yes (AI-powered)** |
| Bug tracing across services | Limited | Yes | No | No | No | **Yes** |
| Root-cause AI diagnosis | No | Enterprise only | No | No | No | **Yes (3 modes)** |
| Autonomous code fix (PR) | No | No | No | No | No | **Yes** |
| CI pipeline automation | No | Limited | Deployment only | No | No | **Yes** |
| E2E test orchestration | No | Synthetics ($) | No | No | No | **Yes (Vision)** |
| MLOps/model drift | No | Enterprise only | No | Yes | No | **Yes (Vision)** |
| LLMOps/prompt versioning | No | No | No | No | No | **Yes (Vision)** |
| Open source | Partial (SDK) | No | Yes | Yes | No | **Yes (Full)** |
| CLI-first | No | No | Yes | Partial | No | **Yes** |
| Pluggable AI backend | N/A | N/A | N/A | N/A | N/A | **Yes** |
| Self-hosted (zero cloud) | Yes | No | Yes | Yes | No | **Yes** |
| Cost | $26-89/user/mo | $23-34/host/mo | Free | Free | $21-41/user/mo | **Free** |

---

## Target Users

### Primary: Platform Engineers / SREs
- **Pain:** Drowning in alerts, spending nights on incident response, context-switching between tools
- **Value:** Autonomous error detection and remediation reduces on-call burden by 70%+
- **Adoption path:** `pip install aotf && aotf init && aotf watch`

### Secondary: Backend Developers
- **Pain:** CI failures with cryptic error messages, slow feedback loops, manual debugging
- **Value:** AI diagnoses build failures and suggests fixes instantly
- **Adoption path:** Add AOTF to GitHub Actions workflow, get fix PRs automatically

### Tertiary: MLOps Engineers
- **Pain:** Model drift goes undetected, experiment tracking disconnected from CI/CD
- **Value:** Unified monitoring of model performance alongside application health (Vision phase)
- **Adoption path:** Configure model metrics log source + drift detection thresholds

### Quaternary: DevOps Leads / Engineering Managers
- **Pain:** No unified view of SDLC health, hard to measure operational efficiency
- **Value:** Single dashboard showing error rates, fix success rates, MTTR trends
- **Adoption path:** Deploy web dashboard, configure team notifications

### Open-Source Contributors
- **Pain:** Want to contribute to impactful developer tools
- **Value:** Clean plugin architecture makes it easy to add new integrations
- **Adoption path:** Fork, implement a plugin ABC, submit PR

---

## MVP Scope

### In Scope (v0.1.0)
- CLI tool installable via `pip install aotf`
- `aotf init` — Project initialization with `.aotf/config.yaml`
- `aotf watch` — Daemon mode monitoring log sources (file tail, Docker containers)
- `aotf errors list` — View detected errors with filtering
- `aotf diagnose <error-id>` — AI-powered root-cause analysis
- `aotf fix <error-id>` — Auto-fix with `diagnosis_only` and `fix_and_pr` modes
- Plugin system with entry_points discovery
- Claude AI backend (default)
- GitHub Actions CI provider plugin
- Console notification (default) + Slack/Teams plugins
- SQLite local storage
- Structured JSON event logging

### Out of Scope for MVP
- Web dashboard (Growth phase — Epic 6)
- GPT, Gemini, local model backends (Growth — Epic 7)
- GitLab, Azure DevOps, Jenkins CI providers (Growth — Epic 7)
- MLOps: model drift detection, experiment tracking (Vision — Epic 8)
- LLMOps: prompt versioning, quality correlation (Vision — Epic 8)
- E2E testing framework (Vision — Epic 9)
- Distributed tracing / cross-service bug tracing (Vision — Epic 10)
- `full_auto` fix mode (requires robust safety guardrails — Growth phase)

---

## Success Metrics

| Metric | Target | Measurement |
|---|---|---|
| Mean-time-to-detection (MTTD) | <60 seconds from log emission | Integration tests with timed error injection |
| Mean-time-to-resolution (MTTR) | 70% reduction vs manual | Compare fix_and_pr time vs manual debugging baseline |
| AI diagnosis accuracy | >80% correct root cause | Human review of 100 sampled diagnoses |
| Auto-fix PR CI pass rate | >60% of generated PRs pass CI | Track PR merge rates |
| Time to first detection | <5 minutes from install | User onboarding tests |
| GitHub stars | 1,000+ in 6 months | GitHub metrics |
| Community plugins | 5+ contributed plugins in 12 months | Plugin registry count |
| Plugin dev time | <4 hours for new CI provider | Contributor onboarding sessions |

---

## Key Differentiators

1. **Autonomous Remediation** — Not just monitoring; AOTF understands code, diagnoses issues, and creates PRs to fix them
2. **CLI-First** — Works everywhere (CI runners, SSH sessions, local dev) without requiring a web browser
3. **Plugin Architecture** — Avoid vendor lock-in; swap AI backends, CI providers, and notification channels freely
4. **Production-Proven Core** — Engine extracted from a production CI/CD system running 24/7 at xMainframe
5. **Unified SDLC Health** — One tool for errors, logs, CI, testing, and MLOps instead of 5-10 separate subscriptions
6. **Open Source** — Full platform, not just SDKs or agents; community-driven development
7. **Zero Cloud Dependencies** — Runs entirely self-hosted with SQLite + local AI models if desired

---

## Technical Foundation

AOTF's core engine is extracted and generalized from the **xMainframe CI/CD Automation System**, a production-proven platform that:
- Executes Claude Code CLI for AI-powered pipeline operations
- Monitors 54+ Docker containers with 3-tier health caching
- Scans logs with regex-based error detection, deduplication, and rate limiting
- Sends MS Teams Adaptive Card and email notifications
- Provides auto-fix in three modes (diagnosis_only, fix_and_pr, full_auto)
- Runs as a systemd service with HTMX+Tailwind dashboard

The extraction generalizes xMainframe-specific logic into pluggable abstractions while preserving battle-tested patterns for error detection, AI execution, and notification delivery.

---

## Risks and Mitigations

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| AI fix quality varies | Bad PRs erode trust | High | Default to diagnosis_only; require CI pass for auto-merge; confidence scores |
| Plugin API instability | Breaking changes frustrate contributors | Medium | Semver guarantees; comprehensive plugin test suite; deprecation warnings |
| Adoption competition from incumbents | Sentry/Datadog add AI remediation | Medium | Move fast on open-source moat; plugin ecosystem creates switching costs |
| Security concerns with auto-fix | Malicious or unsafe code changes | High | Sandboxed execution; branch protection enforcement; human review default |
| Cross-platform log watching | File tail differences across OS | Medium | Watchdog library for file monitoring; Docker SDK for containers |
| Local model quality for diagnosis | Smaller models produce poor diagnoses | Medium | Clear model requirements in docs; quality scoring with minimum thresholds |
