# CLAUDE.md — Agent on the Fly

Behavioral guidelines for Claude working in this repository. Adapted from [forrestchang/andrej-karpathy-skills](https://github.com/forrestchang/andrej-karpathy-skills/blob/main/CLAUDE.md); project-specific additions below.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## Project Context

Agent on the Fly (AOTF) is a pre-alpha, open-source (Apache-2.0) multi-agent SDLC companion built on the Claude Agent SDK. The repo currently holds planning artifacts — PRD, architecture, epics, product-brief — and **no runnable code yet**. Canonical product thinking lives in [`docs/`](./docs).

Every scope decision should anchor to one of three pillars:

1. **Proactive DevOps Loop** — pre-push simulation, risk scoring, auto-fix PRs
2. **AI QA Agent** — LLM + Playwright + Chrome DevTools for autonomous exploration
3. **ML/LLM Lifecycle Manager** — git-native versioning, drift detection, closed-loop retraining

Work outside the pillars belongs in an issue, not a PR.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them — don't pick silently.
- If a simpler approach exists, say so. Push back when warranted. Adversarial pushback and disconfirming conclusions are welcomed on this project.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it — don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:

```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria enable independent looping. Weak criteria ("make it work") force constant clarification.

## 5. Dogfood the Reversibility Discipline

AOTF will enforce reversibility on the agents it spawns. Code written *for* AOTF should hold the same standard:

- Classify every action as **reversible**, **soft-reversible** (undoable with cost), or **irreversible**.
- Irreversible actions require explicit human authorization — never auto-execute.
- Reversible actions may auto-execute but must log a clear undo path.

Full taxonomy and per-finding authorization model: [`docs/architecture.md`](./docs/architecture.md).

## Repo Conventions

- **Canonical docs:** [`docs/`](./docs) — keep PRD, architecture, epics, and product-brief in sync with any scope change.
- **Gitignored (local working state):** `_bmad/`, `_bmad-output/`, `.claude/`, `node_modules/`.
- **License:** Apache-2.0. New source files should carry the standard Apache header once the codebase lands.
- **Commits:** Conventional style (`chore:`, `feat:`, `fix:`, `docs:`). Subject short; body explains *why*, not *what*.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.
