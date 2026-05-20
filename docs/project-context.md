# btc-sec-toolkit — Project Context

> Loaded as a persistent fact by BMad agents. Keep this file tight: it ships into every agent session.

## What this project is

An OSS toolkit for security analysis of code in the **Bitcoin / Lightning / Cashu / Nostr** ecosystem. Two artifacts, one story:

1. **Detection rules pack** (Semgrep-format) covering protocol-specific failure modes: BIP / BOLT / LUD / NUT / NIP patterns, HTLC / PTLC lifecycle bugs, transaction construction pitfalls, LNURL/Nostr handler vulnerabilities, etc.
2. **MCP server** (deferred to v2 post-alpha) that would expose the rules + smoke results + suppression metadata as a queryable rule registry. Spec lookup is delegated to [`bkb-mcp`](https://docs.rs/crate/bkb-mcp) (BIP/BOLT/NUT/LUD/bLIP) and [`@nostrbook/mcp`](https://nostrbook.dev/mcp) (NIP/kinds/tags), both already mature — this project does NOT build its own knowledge layer.

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

Came from analyzing [loupe](https://github.com/tnull/loupe), a FOSS Rust LLM-driven security scanner by tnull (who also maintains `bkb-mcp`). Loupe has only **generic CWE-style prompts** (`DISCOVERY` / `VERIFY` in `crates/loupe-worker/src/llm/prompts.rs`) and a single AWS-key regex rule. Bitcoin-awareness in loupe lives entirely in the optional `bkb-mcp` hook (which is a knowledge layer, not detection content). **btc-sec-toolkit fills the gap loupe leaves open.**

The project owner (Ifuensan) is a contributor to loupe (added the Gemini backend), not its founder. btc-sec-toolkit is a separate project, not a fork.

## Status (as of 2026-05-20)

**Exploration phase, alpha scoped, verdict pending post-conference.** See [`docs/prfaq.md`](prfaq.md) and [`docs/prfaq-distillate.md`](prfaq-distillate.md) for the strategic record.

- ✅ Failure modes catalog: 255 candidates brainstormed, 17 Tier-1 prioritized (catalog local; see PRFAQ for summary).
- ✅ PRFAQ stress-test: customer + internal FAQs, narrative verdict (Forged / Needs Heat / Cracked), sustainability commitments.
- ✅ First rule shipped: `nostr.nip-44.missing-version-check` (commit `dad89ba`), smoke-tested against `rust-nostr` and `nostr-tools`.
- 🟡 Alpha scope: 3 rules total (NIP-44 done, Cashu to-build, MuSig2 PSBT to-build), Rust + TypeScript at launch.
- 🟡 Launch blocker pending: 2-3 ecosystem peer reviewers (Lightning + Nostr + Cashu) as CODEOWNERS before alpha publicization.
- ⏳ Validation pending: attending **And Other Stuff** (Nostr conference, ~2026-05-28) in adversarial-listening mode; commit / pivot / close decision deferred to post-conference.

## Ruled-out branches (do not re-litigate)

- **Loupe-as-SaaS-multi-tenant** — huge rework, not Ifuensan's project to pivot anyway
- **Horizontal SAST** — saturated, no path against freemium incumbents
- **EVM / DeFi vertical** — loupe not equipped, market saturated, culturally misaligned
- **Bitcoin/Lightning vertical SaaS premium pricing** — TAM too small (~10-20 paying entities globally), grant-funded projects don't buy commercial tooling

See [`docs/prfaq.md`](prfaq.md) for the strategic record and the ruled-out branches explained in depth (including the Loupe attribution correction from the original adversarial research).

## Conventions

- Communication language: Spanish (despite English config default — match user)
- Naming: `btc-sec-toolkit` covers the whole ecosystem; sub-modules may be named per-protocol (e.g. `bolt-rules`, `nut-rules`)
- License: MIT OR Apache-2.0 (mirror loupe's, permissive for ecosystem reuse)
