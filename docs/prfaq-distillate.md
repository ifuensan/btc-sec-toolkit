---
title: "PRFAQ Distillate: btc-sec-toolkit"
type: llm-distillate
source: "prfaq.md"
created: "2026-05-17"
purpose: "Token-efficient context for downstream PRD creation or closure retrospective"
verdict: "needs-heat"
key_risks:
  - "I10 commit/close decision deferred to 2026-06-01 (2-week cooldown test)"
  - "I3 maintainer outreach unstarted (placeholder names only); launch blocker"
  - "Q5 peer reviewers unnamed; launch blocker (no alpha publicization without CODEOWNERS for rule directories)"
  - "Loupe (tnull) may absorb the high-value patterns into the LLM scanner within 6 months, evaporating differential value. Tnull also maintains bkb-mcp, natural position to layer Bitcoin context into Loupe."
  - "Target audience (Rust-heavy: LDK/CLN/cdk/nostr-sdk) does not currently run Semgrep in CI; adoption is the dominant problem, not rule quality"
  - "Maintenance burden at 20 production rules (~50-80 h/month) is incompatible with 5-10 h/week author bandwidth without contributors"
  - "TAM brutally small (low hundreds globally) — recognition work, not product"
open_questions:
  - "Will any of the 5 placeholder integration targets (Mostro, Whitenoise, cdk, rust-nostr, LDK) actually be receptive?"
  - "Will tnull (Loupe + bkb-mcp maintainer) coordinate, absorb, or compete on the rule patterns? (Decision blocked until post-cooldown commit)"
  - "Does MCP show enough value-add to justify investment? (3-month post-ship evaluation; pre-emptive drop from alpha scope strongly recommended)"
  - "Are C/Swift wallet teams reachable as a wave-2 audience, or does the project effectively never reach them?"
  - "Will the alias-then-check Rust suppression pattern (currently broken due to `*$BYTES.first()$T` parser limitation) be fixable in Semgrep, or does it require a v1.2 dataflow rewrite?"
---

# PRFAQ Distillate — btc-sec-toolkit

## Concept summary

Open-source Semgrep rule pack covering protocol-conformance failure modes for the Bitcoin / Lightning / Cashu / Nostr ecosystem (BIPs, BOLTs, NIPs, NUTs, LUDs). Companion MCP spec-context server is on the roadmap but **recommended for drop-from-alpha** per Stage 4 analysis. Distribution: Semgrep Registry + GitHub. Zero infrastructure to operate. Author is one person (Ifuensan), contributor (not founder) of `loupe` (an LLM-driven security scanner by tnull; not Spiral as originally mis-attributed in the adversarial research). Currently in exploration mode, commit-or-close decision pending 2026-06-01.

## Customer / problem / stakes / solution

- **Customer:** maintainers of canonical ecosystem implementations (`nostr-sdk`, `nostr-tools`, `cdk`, LDK, CLN, Mostro, Whitenoise/Marmot). Combined serious audience: ~low hundreds globally.
- **Problem:** generic SAST does not understand BIPs/BOLTs/NIPs/NUTs. LLM scanners (Loupe) cost $100-$400 per repo audit and are not deterministic for CI gating. Protocol-conformance bugs reach production and are caught post-incident.
- **Stakes:** direct loss of funds (Lightning HTLC mismanagement, Cashu retired-keyset acceptance), key compromise (RNG, scheme confusion), privacy leaks (linkability). Ecosystem mostly grant-funded, cannot afford regular Trail of Bits / Chaincode audits.
- **Solution:** Semgrep rule pack at alpha (3 rules); each rule cites a spec clause, ships per-language fixtures (Rust + TS at launch), smoke-tested against canonical implementations, published with explicit known-limitations and suppression guidance.

## Alpha scope (locked)

- **3 specific rules, each tied to a public 2025-2026 disclosure:**
  1. `nostr.nip-44.missing-version-check` — **DONE** (commit `294add4`, smoke-tested against rust-nostr + nostr-tools); ties to IACR ePrint 2025/1459 (Damus/Iris signature-verification bypass).
  2. `cashu.nut-13.keyset-id-derivation-flaw` — **TO BUILD** (~16-24 h); ties to `conduition.io` disclosure July 2025 (Minibits, Cashu.me, Nutstash affected).
  3. `bitcoin.musig2.psbt-nonce-pubkey-validation` — **TO BUILD** (~20-30 h); ties to Bitcoin Core PR #34219 (Mar 2026).
- Languages: Rust + TypeScript at alpha.
- MCP server: dropped from alpha per Internal FAQ I7 recommendation (saves 60-120 h = 1-2 more rules or 2x faster ship). Press release frames as "in early development" — compatible with later evaluation.
- Public roadmap of 200+ candidate failure modes (from `_bmad-output/brainstorming/brainstorming-session-2026-05-16-1316.md`) is a navigable artifact (TODO: convert MD to GitHub project board, ~8 h).
- License: MIT OR Apache-2.0.

## Process invariants (every rule must satisfy)

1. Spec-clause citation in YAML metadata.
2. Per-language fixtures (`.rs`, `.ts`, etc.) with `// ruleid:` (BAD) and `// ok:` (OK) markers.
3. Passing `semgrep --test` enforced by `.github/workflows/rules.yml` CI gate.
4. Smoke test against at least one canonical implementation, results published in the rule README.
5. Explicit known-limitations + suppression guidance in the rule README.
6. **FP-rate threshold:** no rule ships with >20% FP rate against canonical implementations as measured by smoke test.

## Engineering patterns learned (from rule #1 smoke test)

- **Two-stage scoping** is essential: `paths.include` (file/dir) + `metavariable-regex` on function name. Path-only is too noisy; name-only misses idiomatic module-based code (rust-nostr has functions named `decrypt_to_bytes` inside `nips/nip44/v2.rs` — name regex must allow `decode|decrypt|parse|unwrap`).
- **Alias-then-check pattern** (`const vers = bytes[0]; if (vers !== 2) ...`) is the idiomatic style in real code. Suppression must cover both direct (`if (bytes[0] !== V)`) and aliased forms.
- **Cross-procedure validation** generates legitimate-but-noisy findings ("defense-in-depth weakness"). Document `// nosemgrep: <rule> — reason` as the accepted suppression with explicit trust contract.
- **Semgrep Rust parser limitations:** `match $X { ... }` patterns reject `...` arm placeholders; `let $X = *$BYTES.first()$T;` rejects the typed-call escape. For 5-10 of the Tier-1 catalog rules, the bug patterns are not cleanly expressible in current Semgrep Rust — fallbacks (`generic` mode, clippy dual-publish, wait for upstream) all imperfect.

## Resource / timeline reality

- Author bandwidth: **5-10 h/week sustained** (confirmed by user; exploration mode + Loupe contributions + day job).
- Time to alpha (rule #1 done, rules #2 + #3 + reviewers + roadmap polish): **72-100 person-hours = 8-20 weeks calendar**. At 5 h/week → 20 weeks; at 10 h/week → 8 weeks.
- Maintenance burden at 20 rules in production: estimated **50-80 h/month** (spec watch + FP triage + regression tests + PR review). Unsustainable solo at 5-10 h/week; requires contributors or consulting funding.

## Decision pending (the gating one)

- **2-week project cooldown 2026-05-17 → 2026-06-01.** No work, no thinking-about-work.
- Cooldown test outcome at 2026-06-01:
  - Project pulled at / missed → **commit alpha** (90-day plan: 2026-06-01 to 2026-08-30).
  - Project forgotten / felt like relief → **close** (archive repo + retrospective + standalone NIP-44 rule survives as OSS contribution).
  - Genuinely ambiguous → extend cooldown 2 more weeks (one-time only; second ambiguity = closure by default).

## Positioning (locked decisions)

- **vs Loupe (tnull's LLM scanner):** explicit complement, named in leader quote. "Loupe before major release; btc-sec-toolkit per commit." Defuses conflict-of-interest critique up front. Not substitutable: Loupe is LLM-driven non-deterministic; toolkit is Semgrep-deterministic.
- **vs Trail of Bits / Chaincode high-end audit:** explicitly not competing (different price/cadence tier). Toolkit serves the audience that can't afford TOB.
- **vs Semgrep Pro:** treat as nice-to-have, not dependency. Hedge via `opengrep` (community fork) compatibility.
- **vs clippy custom lints:** Semgrep covers both Rust and TS with one rule; clippy would require per-language duplication. Dual-publish to clippy is a fast-follow if pressure builds.
- **vs raw grep on local spec repos (the MCP question):** MCP differential is incremental, not transformative. Strongest case is newcomers + AI-assisted coding. Drop from alpha; revisit at 3-month post-ship evaluation.
- **vs EVM / Solidity tooling (OpenZeppelin, Certora, Olympix):** explicit out-of-scope per project framing. Ruled out branch (see project memory).

## Rejected framings (do not re-litigate)

- "Today the toolkit ships" → too commercial for OSS alpha reality (Stage 2).
- "Built by an independent contributor" → user kept "community-driven" as aspirational despite the honest reframing (Stage 2). Accepted risk.
- "Dozens of rules" → too vague; committed to "three" with named anchors (Stage 2).
- "MCP server expanded into Semgrep wrapper" → researcher's redundancy critique; user re-framed as queryable spec snippets (Stage 2), then Stage 4 recommended drop-from-alpha-scope anyway (Stage 4 contradiction with Stage 2, mostly resolved by v3 press release wording).
- "Pivot to MCP-only" → considered, rejected (rules are the validated artifact, MCP is the speculative one).
- "Upstream rules to Loupe instead" (Q1) → rejected; LLM emission is strict downgrade from deterministic Semgrep rules.
- "Avoid mentioning Loupe in press release" (Q2 alt) → rejected in favor of explicit complement framing (Stage 2).
- "Loupe-as-SaaS-multi-tenant" pivot → ruled out branch (see project memory).
- "Bitcoin/Lightning vertical SaaS premium pricing" → ruled out (TAM too small per memory).

## Outstanding launch-blocking work (must close before alpha publicization)

1. **Identify 2-3 named ecosystem reviewers** (one Lightning, one Nostr, one Cashu) and onboard as CODEOWNERS for rule subdirectories. (Q5 launch blocker.)
2. **Write rule #2** (Cashu NUT-13 keyset-ID derivation flaw) with smoke test against `cdk` and Nutshell.
3. **Write rule #3** (MuSig2 PSBT nonce/pubkey validation) with smoke test against `bitcoin-core`.
4. **Convert brainstorming MD to navigable roadmap** (GitHub project board or structured issues).
5. **Obtain real alpha-tester quote** to replace press release placeholder (the press release explicitly states it should not ship without one).
6. **Repo polish:** README, CONTRIBUTING.md, license headers, badges.

## Fast-follows (post-alpha, before catalog scales)

- Quarterly spec-review cycle design (Q8): who, cadence, triggers, GitHub project board mechanism.
- NIP-44 rule severity downgrade `ERROR` → `WARNING` until dataflow tracking lands (Q6, v1.2 bug).
- Dual-publish key rules as clippy lints if pressure builds from Rust-heavy targets (Q3, optional).
- 3-month MCP evaluation: build / drop / defer further based on observed rule-pack traction (Q9).

## Out-of-scope at alpha (explicit)

- C / Swift / Go / Python language support → wave 2, after Rust+TS hits 20 rules.
- Consulting layer monetization → deferred to post-alpha if any traction.
- Cross-layer rules (LN ↔ Bitcoin, Cashu ↔ LN, zaps ↔ NWC) → catalog has them but they're not in the 3-rule alpha set.
- Race conditions / runtime bugs / supply-chain (catalog Tier-4) → not Semgrep scope; separate doc or CI tooling.

## Sustainability commitments (locked)

- **6-month public checkpoint** post-alpha launch: continue / transfer / sunset. Public retrospective regardless of outcome.
- **Sunset playbook** documented up front: archive repo with prominent README, suggest alternatives, no broken promises to users who integrated into CI.
- License MIT/Apache-2.0 → fork survives author.

## Cracks the PRFAQ leaves open (from The Verdict)

- TAM is brutal (low hundreds); if author's aspiration is larger than that, friction is built-in.
- "Community-driven" aspirational framing is detectable if scrutinized; accepted credibility risk.
- Adoption conversion (from "installs in 60s" to "team adds to CI") has no concrete plan beyond outreach.
- I10 cooldown could produce "ambiguous" indefinitely; the documented "ambiguous twice = closure" rule must be honored when the moment comes.
- I3 outreach is the work the author has not done. Until real maintainers respond, the customer story is theory.

## Sources of authority for this distillate

- Project memory: `~/.claude/projects/-mnt-datos-home-data-Work-myprojects-research-btc-sec-toolkit/memory/project_bitcoin_security_toolkit_thesis.md`
- Failure-modes catalog: `_bmad-output/brainstorming/brainstorming-session-2026-05-16-1316.md` (255 ideas)
- Adversarial research synthesis: in-session (web + IACR ePrint + arXiv + GitHub repos, 2026-05-16)
- First rule implementation: `rules/nostr/nip-44/` (commit `294add4`)
- Full PRFAQ: `docs/prfaq.md` (this repo)
