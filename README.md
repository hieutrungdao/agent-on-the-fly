# Agent on the Fly (AOTF)

> A proactive, self-improving multi-agent SDLC companion. Prevents failures instead of reacting to them — and gets smarter the longer it runs.

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
[![Status: Pre-Alpha](https://img.shields.io/badge/status-pre--alpha-orange.svg)](#project-status)

---

## Project Status

**Pre-alpha — research and planning phase.** This repository currently contains the product brief, PRD, architecture, and epics. Runnable code has not yet landed. If you're looking to try AOTF, watch/star the repo and check back once the MVP milestone closes. If you're interested in shaping the design, feedback on the docs is extremely welcome.

See [`docs/`](./docs) for the current product thinking.

## What It Is

AOTF applies software engineering rigor to both code *and* ML/LLM artifacts across the full SDLC. Instead of a chatbot that answers questions, it's a background orchestrator that spawns specialized sub-agents to catch problems before they ship.

### Three Core Pillars

1. **Proactive DevOps Loop** — Pre-push simulation, risk scoring, auto-fix PRs, smart CI that focuses test effort where risk is highest.
2. **AI QA Agent** — LLM + Playwright + Chrome DevTools for autonomous exploration, bug narration, self-healing tests, and automated bug tracing.
3. **ML/LLM Lifecycle Manager** — Git-native model versioning (Git LFS + DVC), drift detection, closed-loop retraining, and prompt-engineering rigor (versioning, A/B testing, injection firewall).

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

Because the code isn't written yet, the highest-leverage contribution right now is **feedback on the planning docs**:

- Open an issue with comments on the PRD, architecture, or epic scope
- Challenge the three-pillar framing — is anything missing, or over-scoped for v0.1?
- Share related prior art (tools that tried this, tools that failed, research papers)

Once the MVP skeleton lands, a proper `CONTRIBUTING.md` will be added.

## License

Apache License 2.0 — see [`LICENSE`](./LICENSE).

Copyright © 2026 Hieu Trung Dao.
