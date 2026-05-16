# btc-sec-toolkit — Project Context

> Loaded as a persistent fact by BMad agents. Keep this file tight: it ships into every agent session.

## What this project is

An OSS toolkit for security analysis of code in the **Bitcoin / Lightning / Cashu / Nostr** ecosystem. Two artifacts, one story:

1. **Detection rules pack** (Semgrep-format) covering protocol-specific failure modes: BIP / BOLT / LUD / NUT / NIP patterns, HTLC / PTLC lifecycle bugs, transaction construction pitfalls, LNURL/Nostr handler vulnerabilities, etc.
2. **MCP server** that exposes the rules + contextual explanation to LLM agents (Claude Code, Cursor, Continue, any MCP-aware tool). Calls out to `bkb-mcp` / `bitcoinknowledge.dev` for spec lookup — does NOT build its own knowledge layer.

Distribution: Semgrep Registry, crates.io, MCP server registries. Zero infrastructure to operate.

## What this project is NOT

- **Not a SaaS.** No multi-tenancy, no hosted backend, no billing.
- **Not VC-scale.** Lifestyle / boutique scope. Optional consulting layer later if traction warrants. Explicitly NOT a venture play.
- **Not horizontal SAST.** Will not try to compete with Snyk / Semgrep / GHAS / Corgea on general-purpose code security.
- **Not EVM / DeFi / altcoin** tooling. The ecosystem focus is deliberate.
- **Not a fork of loupe.** Inspired by it; may copy small scaffolding pieces (loupe is MIT OR Apache-2.0). Different architecture (rules-first, LLM-on-demand) and different audience.

## Differentiation thesis

Value vs. generic-LLM alternatives (including self-hosted loupe):

| Axis | Generic LLM scanner | btc-sec-toolkit |
|---|---|---|
| Cost per run | $100-$400 in LLM tokens (per-file agent fan-out) | $0 for the rules, ~cents for optional MCP calls |
| Cadence enabled | Occasional deep audit | Per-commit / per-PR |
| Determinism | LLM emission is not reproducible | Rules give CI-friendly deterministic gates |
| Setup | Multi-binary, mTLS, server, worker, sandbox | One Semgrep config entry; install MCP server in agent |
| Domain fit | Generic CWE awareness, thin on protocol specifics | Expert-written Bitcoin/Lightning patterns |
| Detection class | Emergent (LLM finds what it notices) | Deterministic (only what's encoded) |
| Trade-off accepted | — | Loses ~5-15% long-tail "emergent" detection. Mitigation: optional augmented LLM mode |

**Positioning:** complement to, not substitute for, LLM-driven deep audit. A serious team could use both — rules pack on every commit, deep audit before major release.

## Origin context

Came from analyzing [loupe](https://github.com/<owner>/loupe), a FOSS Rust LLM-driven security scanner. Loupe has only **generic CWE-style prompts** (`DISCOVERY` / `VERIFY` in `crates/loupe-worker/src/llm/prompts.rs`) and a single AWS-key regex rule. Bitcoin-awareness in loupe lives entirely in the optional `bkb-mcp` hook (which is a knowledge layer, not detection content). **btc-sec-toolkit fills the gap loupe leaves open.**

The project owner (Ifuensan) is a contributor to loupe (added the Gemini backend), not its founder. btc-sec-toolkit is a separate project, not a fork.

## Status (as of 2026-05-16)

**Exploration / analysis phase.** Not committed to building. Currently running BMad analysis workflows:

- ✅ Architectural fitness analysis (Winston) — done
- ✅ Strategic validation (Mary) — done; thesis refined through several pushback rounds
- ⏳ Next candidate: brainstorm the failure modes catalog to validate substance (`bmad-brainstorming`)
- ⏳ After: PRFAQ stress-test (`bmad-prfaq`) if catalog has substance

## Ruled-out branches (do not re-litigate)

- **Loupe-as-SaaS-multi-tenant** — huge rework, not Ifuensan's project to pivot anyway
- **Horizontal SAST** — saturated, no path against freemium incumbents
- **EVM / DeFi vertical** — loupe not equipped, market saturated, culturally misaligned
- **Bitcoin/Lightning vertical SaaS premium pricing** — TAM too small (~10-20 paying entities globally), grant-funded projects don't buy commercial tooling

See `/home/ifuensan/.claude/projects/-mnt-datos-home-data-Work-myprojects-research-loupe/memory/` for the full analysis history.

## Conventions

- Communication language: Spanish (despite English config default — match user)
- Naming: `btc-sec-toolkit` covers the whole ecosystem; sub-modules may be named per-protocol (e.g. `bolt-rules`, `nut-rules`)
- License: MIT OR Apache-2.0 (mirror loupe's, permissive for ecosystem reuse)
