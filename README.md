# Agent on the Fly (AOTF)

> A proactive, self-improving multi-agent SDLC companion. Prevents failures instead of reacting to them — and gets smarter the longer it runs.

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
[![Status: Pre-Alpha](https://img.shields.io/badge/status-pre--alpha-orange.svg)](#project-status)

---

## Project Status

**Pre-alpha — walking-skeleton phase.** The v0.0.2-alpha.1 release ships the first runnable code: an 8-crate Rust workspace + Bun agent proving the file-watch → finding → gate → audit-log path end-to-end. The four product pillars and the full MVP scope are not yet implemented. If you're interested in shaping the design, feedback on the docs is extremely welcome.

See [`docs/`](./docs) for the current product thinking and [`CHANGELOG.md`](./CHANGELOG.md) for what's in each release.

## What It Is

AOTF applies software engineering rigor to both code *and* ML/LLM artifacts across the full SDLC. Instead of a chatbot that answers questions, it's a background orchestrator that spawns specialized sub-agents to catch problems before they ship.

### Four Core Pillars

1. **Proactive DevOps Loop** — Pre-push simulation, risk scoring, auto-fix PRs, smart CI that focuses test effort where risk is highest.
2. **AI QA Agent** — LLM + Playwright + Chrome DevTools for autonomous exploration, bug narration, self-healing tests, and automated bug tracing.
3. **ML/LLM Lifecycle Manager** — Git-native model versioning (Git LFS + DVC), drift detection, closed-loop retraining, and prompt-engineering rigor (versioning, A/B testing, injection firewall).
4. **AI Agent Operations** — Agent run tracking, behavior logging, deviation detection, cost tracking, and shadow pipelines.

### Why This, Why Now

Existing tooling treats code, ML models, and LLM prompts as separate worlds — each with its own disconnected observability, versioning, and quality story. AOTF unifies them behind one agent that reads your logs, traces your bugs, runs your tests, and learns your codebase's failure patterns over time. The moat is compounding: every interaction makes AOTF smarter about *your* project specifically.

## Architecture (Summary)

- **Deployment model:** Hybrid — local agent (your data stays in your org) + optional hosted intelligence layer (anonymized cross-org patterns).
- **Agent framework:** [Claude Agent SDK](https://docs.anthropic.com/en/api/agent-sdk) — AOTF is the orchestrator; specialized sub-agents spawn on demand.
- **Open core:** The local agent is Apache-2.0. Hosted intelligence is a separate subscription tier.

Full architecture lives in [`docs/architecture.md`](./docs/architecture.md). PRD in [`docs/PRD.md`](./docs/PRD.md). Epics in [`docs/epics.md`](./docs/epics.md).

## Planned Tech Stack

| Area | Tooling |
|---|---|
| Agent framework | Claude Agent SDK |
| Browser / QA | Playwright, Chrome DevTools MCP |
| Observability | Vector → ClickHouse, OpenTelemetry → Jaeger, Prometheus → Grafana |
| ML tracking | Git LFS + DVC, W&B / MLflow (abstracted) |
| LLM monitoring | LangSmith + custom prompt registry |
| Data | ClickHouse (time-series), Qdrant / Weaviate (vector), PostgreSQL |
| Interfaces | VS Code extension, GitHub / GitLab app, Slack bot, CLI, Next.js dashboard |

## MVP (v0.1) Scope

1. Git hook integration — pre-push analysis
2. Risk scoring engine — LLM + historical-data driven
3. Playwright + Chrome AI QA test runner with LLM evaluation
4. Basic dashboard (errors, risk scores, test results)

Tracking in [`docs/epics.md`](./docs/epics.md).

## Getting Involved

The walking skeleton (v0.0.2-alpha.1) is runnable — see `scripts/install.sh` or `cargo build`. The highest-leverage contributions right now are:

- Feedback on the planning docs (PRD, architecture, epic scope)
- Challenge the four-pillar framing — is anything missing, or over-scoped for v0.1?
- Share related prior art (tools that tried this, tools that failed, research papers)
- Bug reports and review findings against the walking skeleton

A proper `CONTRIBUTING.md` will be added before the v0.1 milestone.

## License

Apache License 2.0 — see [`LICENSE`](./LICENSE).

Copyright © 2026 Hieu Trung Dao.
