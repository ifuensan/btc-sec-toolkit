---
title: "PRFAQ: btc-sec-toolkit"
status: "complete"
created: "2026-05-16"
updated: "2026-05-17"
stage: "5-verdict"
concept_type: "open-source project (with optional consulting layer)"
inputs:
  - "{project-root}/docs/project-context.md"
  - "{project-root}/_bmad-output/brainstorming/brainstorming-session-2026-05-16-1316.md"
  - "memory: project_bitcoin_security_toolkit_thesis.md"
  - "implementation: rules/nostr/nip-44/missing-version-check (commit 294add4)"
  - "adversarial market research (web + ResearchGate / IACR / arXiv, 2026-05-16)"
---

# PRFAQ: btc-sec-toolkit

<!-- coaching-notes-stage-1 -->
**Stage 1 — Ignition closure (2026-05-16).**

**Concept type:** open-source project, possibly with later consulting layer. Explicitly NOT a SaaS, NOT VC-scale, NOT a commercial product. Author (Ifuensan) is in exploration mode, contributor (not founder) of `loupe`.

**Customer / Problem / Stakes / Solution captured pre-PRFAQ:**
- **Customer:** developers writing code in the Bitcoin / Lightning / Cashu / Nostr ecosystem — specifically maintainers/contributors of canonical implementations (`nostr-sdk`, `nostr-tools`, `cdk`, LDK, CLN, Mostro, Whitenoise/Marmot). User confirmed this scope.
- **Problem:** generic SAST tools don't understand BIPs/BOLTs/NIPs/NUTs. LLM-driven scanners are expensive (~$100-$400/run) and non-deterministic — don't fit per-PR CI. Result: protocol-conformance bugs reach production and are caught post-incident.
- **Stakes:** direct loss of funds, key compromise, privacy leaks. Ecosystem operates on grant funding, can't afford Trail of Bits / Chaincode audits regularly.
- **Solution:** Semgrep rule pack + MCP server, distributed via Semgrep Registry + crates.io + MCP registries. Zero infrastructure. Companion to LLM deep audit, not substitute.

**Pre-PRFAQ assets validated:** 255 failure modes brainstormed and prioritized (17 Tier-1); 1 rule implemented and smoke-tested (`nostr.nip-44.missing-version-check` v1.1). Workflow proven end-to-end.

**Adversarial research findings that will challenge the press release in Stage 2 (HIGH PRIORITY):**

1. **Loupe (tnull) is the elephant in the room.** _[Correction 2026-05-19: original adversarial research (sourced from glitchwire.com) attributed Loupe to Block/Spiral; the canonical Cargo.toml + README show tnull as the author with no Spiral affiliation documented.]_ Same target repos (Bitcoin Core, BDK, LDK, rust-bitcoin, Cashu). The author is a contributor (added the Gemini backend) — must be honest about positioning. Loupe will likely subsume high-prestige bug patterns into its prompt library, not into public Semgrep rules. Risk: 6 months from now, this project's marginal value evaporates if Loupe absorbs the patterns. Tnull also maintains bkb-mcp, which gives him a natural position to layer Bitcoin-specific context into Loupe.

2. **Target audience does NOT use Semgrep.** Rust shops (LDK, cdk, nostr-sdk, rust-bitcoin) live in clippy + cargo-audit. CLN's CI workflow (verified) does not run Semgrep. **Adoption is the dominant problem, not rule quality.**

3. **TAM is brutally small.** ~30-60 active Lightning impl maintainers + ~50-150 serious Nostr client devs + ~12 Cashu mint implementations. Combined serious audience: **low hundreds globally**. This is recognition/reputation work, not a product.

4. **Bug-class reality is STRONG:**
   - **Damus/Iris Nostr sig-verify bypass (IACR 2025/1459, Kimura et al.)** — exactly the type of bug a rules pack catches. Validates the thesis.
   - **Cashu NUT-13 keyset-ID collision (conduition.io, July 2025)** — affected Minibits, Cashu.me, Nutstash. Detectable by static rule.
   - **MuSig2 PSBT nonce/pubkey validation (Bitcoin Core PR #34219, Mar 2026)** — real recent fix.
   - **But** the highest-impact bugs (CVE-2023-40231 replacement cycle, Lightning payout races) are NOT static-analyzable. Catalog has to be honest about which tier of bugs the rules actually catch.

5. **MCP server may be REDUNDANT.** Researcher's verbatim: "Claude Code can already `grep` a rules repo. The MCP wrapper adds little if the rules pack is well-organized." This challenges a core thesis assumption — to be confronted in Stage 2.

6. **Empty space genuinely exists** at the rules-pack layer: zero Semgrep rules for BIPs/BOLTs/NIPs/NUTs in any public registry. No security-focused Bitcoin MCP server (general Bitcoin MCPs serve runtime ops, not security). The "translate academic papers (formal verification of Lightning, Nostr attacks) into Semgrep rules" angle is unoccupied.

7. **Researcher's brutal recommendation:** "Don't build it as a 'product.' Build it as a focused contribution and move on. Scope to the Semgrep rules pack only, drop the MCP server, aim for 20-40 high-quality rules tied to specific recent CVEs/papers, contribute upstream to `semgrep/semgrep-rules` Bitcoin-ecosystem subfolder. Set a 6-month checkpoint: if Loupe folds these patterns in, the project's value evaporates."

**Key tensions Stage 2 must resolve:**
- Scope: does the MCP server stay in the PRFAQ or get dropped?
- Customer: narrow to ~50 maintainers of canonical implementations, or broader downstream wallet devs?
- Positioning vs Loupe: explicit complement, explicit ignore, or pivot?
- Adoption strategy: contribute upstream to `semgrep/semgrep-rules` directly, or maintain separate registry?
- Honesty about which bug tier is caught (Tier-1 static-detectable vs Tier-3/4 needing dataflow or runtime).

---

# Press Release (v3, locked at end of Stage 2)

## Headline

**btc-sec-toolkit alpha launches: protocol-aware security checks for Bitcoin, Lightning, Cashu and Nostr code**

## Subheadline

An open-source Semgrep rule pack — and a planned MCP spec-context server — that help wallet, mint, and client maintainers catch BIP/BOLT/NIP/NUT misimplementation bugs at commit time. Complement to LLM-driven audits like Loupe.

**Date/Place placeholder** — The btc-sec-toolkit project is now available as an alpha release, hosted on GitHub and indexed via Semgrep Registry. Built by and for the maintainers of `nostr-sdk`, LDK, CLN, `cdk`, Mostro, and Whitenoise, the alpha ships **three** protocol-anchored security rules — each tied directly to a real 2025–2026 disclosure: missing NIP-44 version-byte validation (Damus/Iris signature-verification bypass, IACR ePrint 2025/1459); Cashu NUT-13 keyset-ID derivation flaw (`conduition.io` disclosure, July 2025; Minibits, Cashu.me, Nutstash affected); and MuSig2 nonce reuse in PSBT deserialization (Bitcoin Core PR #34219, March 2026). A public roadmap of 200+ additional candidate rules — sourced from a structured failure-modes catalog — invites contributors to claim and ship more. A Model Context Protocol (MCP) spec-context server, in early development, will expose the same corpus as queryable spec snippets to LLM coding assistants.

Implementing a BIP, BOLT, NIP, NUT, or LUD from spec is precise work. The path from "I read the document" to "my code satisfies every MUST clause" is long, and the bugs that survive are not what generic SAST catches. They are protocol-conformance bugs — a payload sliced past offset 0 without checking the version byte, a DLEQ proof skipped during Cashu token validation, an LNURL response trusted without verifying its HMAC. The Damus/Iris signature-verification bypass and the Cashu NUT-13 keyset-ID collision both pattern-match cleanly to static rules that did not exist anywhere when those bugs shipped. Teams writing this code mostly run on grant funding. They cannot afford a Trail of Bits engagement on every release. LLM-driven scanners catch many of these patterns but cost $100–$400 per repo audit and are not deterministic enough to gate a CI pipeline. Most protocol-conformance bugs are caught post-incident.

btc-sec-toolkit closes that gap by encoding ecosystem-specific failure modes as Semgrep rules. Each rule is anchored to a specific spec clause, ships per-language fixtures (Rust and TypeScript at launch), and documents why the bug matters, how to fix it, and when to suppress the warning. The alpha launches small on purpose — three rules, each tied to a publicly documented 2025–2026 disclosure. The published roadmap of 200+ candidate rules, derived from a structured failure-modes catalog, makes the next steps transparent and contribute-able. The MCP spec-context server is the next major artifact on the roadmap: when ready, it will let an LLM coding assistant query canonical spec text for any BIP/BOLT/NIP/NUT clause while writing code, instead of hallucinating it. Rules run in CI in seconds, cost nothing, and ship under MIT/Apache-2.0.

> "Loupe, where I contribute, is what you run before a major release: careful, thorough, expensive. btc-sec-toolkit is what you run on every commit: fast, deterministic, free. The two aren't competing; they live at different cadences and different costs. A Cashu mint operator with no security budget should not have to choose between an unmaintained CI gate and an unaffordable audit. The alpha is a starting point — three rules, each anchored to a real 2025 disclosure — and the roadmap is public so the ecosystem decides which rules to write next."
> — Ifuensan, project lead

### How It Works

1. **For teams already on Semgrep:** add one line to your CI config (`semgrep --config p/btc-sec-toolkit`). Findings appear inline in PRs, each citing the spec clause, the failure pattern, and a one-paragraph "why this matters."
2. **For teams on `cargo-audit`, `clippy`, or `eslint`:** btc-sec-toolkit runs alongside, not instead. Semgrep installs in under a minute and reads the same source tree. It is additive to your existing toolchain.
3. **AI-assisted coding (roadmap, in development):** once the MCP server ships, point your assistant at it for canonical spec snippets while writing protocol-handling code — no more hallucinated BIP/BOLT/NIP semantics.

> _Community quote placeholder — to be replaced by a real alpha-tester maintainer before publicization. The press release should not ship without one._

### How to Participate

- Run the rules: `pip install semgrep && semgrep --config p/btc-sec-toolkit your-code/`
- Read the roadmap and failure-modes catalog at `github.com/ifuensan/btc-sec-toolkit` (200+ candidate rules grouped by spec)
- Claim a rule, ship a PR, or open issues for new bug patterns
- MCP server: watch the repo; access opens when the alpha scope lands
- License: MIT OR Apache-2.0

<!-- coaching-notes-stage-2 -->
**Stage 2 — Press Release closure (2026-05-17). Three iteration rounds.**

**v1 → v2 changes:** v1 over-claimed ("today the toolkit ships," "dozens of rules"). v2 reframed to alpha launch, made MCP roadmap not feature, exposed Semgrep adoption obstacle in How It Works, marked community quote as placeholder.

**v2 → v3 changes:** committed to **three specific rules at launch**, each tied to a publicly verifiable 2025–2026 disclosure (Damus/Iris IACR paper, Cashu NUT-13 conduition.io disclosure, Bitcoin Core MuSig2 PSBT PR). User explicitly chose to keep "community-driven" framing aspirational despite the honest framing being "independent contributor" — accepted risk that if the community-driven framing is exposed as solo, credibility takes a hit; the bet is that public roadmap + permissive license bootstraps real contributors before that happens.

**Rejected framings:**
- "Today the toolkit ships" → too commercial, doesn't fit alpha OSS reality.
- "Built by an independent contributor" → user rejected in favor of community-driven aspirational.
- "Drop the MCP server entirely" (researcher's recommendation) → user rejected; reframed as "queryable spec snippets" with claim that grep + repo ≠ structured spec context.
- "Avoid mentioning Loupe in press release" → user chose explicit complement framing, which defuses conflict-of-interest critique up front.

**Competitive positioning explored but not used in press release (saved for Internal FAQ):**
- Trail of Bits / Chaincode high-end audit market (~$50k-$500k engagements; out of reach for grant-funded ecosystem projects).
- Olympix / Certora / OpenZeppelin EVM focus (intentionally out of scope by project framing).
- bkb-mcp as spec lookup (different layer; not security-focused, only spec retrieval).

**Out-of-scope details mentioned by user but not in PR:**
- Two-stage scoping (paths.include + function-name regex) and alias-then-check suppression learned from first rule's smoke test → belongs to engineering doc, not PRFAQ.
- 6-month checkpoint recommendation from researcher (if Loupe absorbs patterns, value evaporates) → belongs to Internal FAQ as risk register.
- Cashu mint operators vs Lightning impl maintainers as primary audience → both in scope per user; Customer FAQ should not pretend to choose.

**Open commitments the press release makes that must be honored before alpha:**
- Two additional rules besides NIP-44 (Cashu NUT-13 + MuSig2 PSBT) must be implemented and smoke-tested.
- Public roadmap must be a navigable artifact (the brainstorming session output is currently a Markdown file, may need to be converted to issues or a structured doc).
- A real alpha-tester quote must be obtained (i.e., at least one ecosystem maintainer must run it and respond).

---

# Customer FAQ (locked at end of Stage 3)

### Q1: Why not upstream these rules directly to Loupe? You contribute there.

A: Different beasts. Loupe is LLM-driven (DISCOVERY → VERIFY prompts, per-file fan-out, $100–$400 per repo); the protocol-specific knowledge lives in the optional `bkb-mcp` hook, not in detection content. "Upstreaming" these rules to Loupe would mean rewriting them as LLM prompts — losing determinism, per-PR latency, and gaining token cost. The 17 Tier-1 patterns are deterministic-detectable; running them through an LLM is a strict downgrade. Coordination with tnull on shared catalogs is on the table; merger is not — they are not substitutable. **(Status: accepted trade-off.)**

### Q2: My project uses `cargo-audit + clippy` and we already have alert fatigue. What false-positive rate should I expect in well-written code?

A: First rule we wrote (NIP-44 missing version check) had 3 findings in canonical implementations (rust-nostr, nostr-tools) — all 3 were false positives. v1.1 reduced to 1 (a cross-procedure trust pattern, suppressible with documented reason). We commit publicly: no rule ships with >20% FP rate against canonical implementations measured by smoke test. Every rule's README publishes its smoke test results. **(Status: accepted trade-off, with measurable threshold.)**

### Q3: Why Semgrep and not custom clippy lints? Our audience lives in Rust.

A: Two reasons. (1) Coverage: most target projects have both Rust and TypeScript (`cdk` is a Rust lib + JS bridge; `nostr-sdk` is Rust + WASM bindings; Mostro is a Rust daemon + TS mobile). Clippy lints would only catch Rust; you'd need a separate ESLint plugin for TS — duplicate work. Semgrep covers both with one rule per failure mode. (2) Distribution: custom clippy lints need procmacro plugin or `dylint` (nightly-only); Semgrep installs in 60 seconds and the rule pack lives at one path. **(Status: accepted trade-off. Dual-publication to clippy is a fast-follow if pressure builds.)**

### Q4: You're one person. What happens when you stop maintaining this in 12 months?

A: No guarantees. Mitigations: (a) MIT/Apache-2.0 license means anyone can fork and continue; (b) rule pack contributions accepted via PR from day one; (c) the failure-modes catalog is open, self-documenting reference even if the author disappears; (d) the explicit alpha framing tells you not to bet production-critical infrastructure on it yet. If the project gets pickup, it earns stronger commitments. If not, it dies graciously. **(Status: accepted trade-off. Honesty is the only credible answer.)**

### Q5: Who reviews rule correctness? An incorrect rule gives false confidence — worse than no rule.

A: Process per rule: (i) spec-clause citation, (ii) per-language fixtures (BAD + OK markers), (iii) passing `semgrep --test` enforced by CI, (iv) smoke test against at least one canonical implementation reported in the rule README, (v) explicit known-limitations section. New rules require all five in the PR. Peer review at alpha: author reviews own work — **this is a gap**. **(Status: LAUNCH BLOCKER.** Cannot ship without naming 2–3 ecosystem reviewers — one Lightning, one Nostr, one Cashu — as CODEOWNERS for rule directories. **Commit: identify reviewers before alpha publicization.)**

### Q6: Your NIP-44 rule had 0 true positives and 3 false positives in smoke testing. How do I justify adding this to CI?

A: That was v1. v1.1 dropped FPs from 3 → 1; the remaining one is a documented "defense-in-depth weakness" suppressible with reason. The published rule is v1.1 and the README shows both v1 and v1.1 smoke results transparently. CI integration is opt-in for `ERROR` severity; for rules with non-trivial FP rate against canonical code, severity should be `WARNING` with explicit `// nosemgrep` suppression docs. **(Status: FAST-FOLLOW.** Current rule is `ERROR`; should be downgraded to `WARNING` until dataflow tracking lands. Bug for v1.2.)

### Q7: Alpha only covers Rust and TypeScript. We're a C/Swift wallet team. When (if ever)?

A: Not at alpha. Most failure modes in the catalog (255 total) are language-agnostic at protocol level; converting to C/Swift Semgrep patterns is mostly mechanical but not trivial. Roadmap: C in wave 2 (after Rust+TS hits 20 rules); Swift further out. The catalog itself is language-neutral — a C maintainer can read the failure-mode descriptions and implement equivalent checks for their toolchain today. **(Status: accepted trade-off, explicit roadmap.)**

### Q8: Specs evolve (BIP-352, NIP-EE MLS, NUT-17). How do you keep rules from going stale?

A: Each rule pins spec version (e.g., "NIP-44 v2, version byte 0x02"). Spec drift detection is human work — the author watches spec repos. At alpha this is "best effort by the author." Roadmap: quarterly spec-review cycle as a public GitHub project board; community subscribes to spec-change alerts. **(Status: FAST-FOLLOW.** Author-watch is not scalable past ~10 rules in production. Quarterly review cycle must land before the catalog grows.)

### Q9: I have the BIPs repo cloned locally and `grep` works fine. My LLM assistant already reads those files. What does the MCP server give me that `grep -r "NIP-44 version byte" ~/specs/` doesn't?

A: Three concrete differentials over raw grep: (a) MCP responses include the spec clause **plus** known misimplementation patterns and corresponding rule IDs — grep returns text only; (b) cross-references between related clauses (querying NIP-44 returns NIP-17 and NIP-59 dependents); (c) structured output designed for the AI assistant's call-and-respond loop, not for human scanning. **But:** if a maintainer is already comfortable navigating BIP/BOLT/NIP repos manually, MCP adds incremental value, not transformative. The case is strongest for newcomers and for AI-assisted coding where the assistant doesn't know to look. **(Status: EXPLICIT RISK.** If MCP doesn't show clear value-add in the first 3 months post-ship, drop from scope.)

### Q10: Honest question: in 18 months, will this still be alive? And if you abandon, what do we do who integrated it into CI?

A: Public 6-month checkpoint (from adversarial research). If Loupe absorbs these patterns, OR the catalog is fully implemented, OR adoption is zero, the project ends or transfers to a willing maintainer. The author commits to a public retrospective at 6 months with one of three outcomes: continue / transfer / sunset. Sunset playbook documented in repo: archive with prominent README, suggest alternatives, no broken promises. For alpha users: assume project lifetime of 6 months minimum; anything beyond is opt-in confidence based on observed activity. **(Status: accepted trade-off. The public 6-month checkpoint is itself part of the commitment.)**

<!-- coaching-notes-stage-3 -->
**Stage 3 — Customer FAQ closure (2026-05-17). Single pass, user accepted draft.**

**Gaps revealed and triaged:**
- **Launch blocker (Q5):** peer review process. Must identify 2-3 ecosystem reviewers (Lightning, Nostr, Cashu) and add them as CODEOWNERS before alpha publicization. User did not name specific contacts during Stage 3 — this is open work.
- **Fast-follow (Q6):** severity downgrade for rules with non-trivial FP rates. Specific bug for v1.2: NIP-44 rule from `ERROR` → `WARNING`.
- **Fast-follow (Q8):** quarterly spec-review cycle must land before catalog grows past ~10 production rules. Currently "author watches spec repos" — not scalable.
- **Explicit risk (Q9):** MCP server value-prop is the weakest leg of the PRFAQ. Three differentials cited (rule cross-reference, related-clause linking, structured output for AI loops) are incremental, not transformative. 3-month post-ship evaluation: if not demonstrating clear value, MCP gets dropped. This contradicts the user's Stage 2 decision to keep MCP with reframed value-prop — the contradiction is intentional and noted.

**Accepted trade-offs that should NOT be re-litigated:**
- Q1 (positioning vs Loupe — complement, not merger).
- Q2 (FP threshold, smoke-test gating).
- Q3 (Semgrep vs clippy lints).
- Q4 (one-person sustainability).
- Q7 (Rust+TS at alpha, C/Swift in waves).
- Q10 (6-month public checkpoint commitment).

**Competitive intelligence surfaced (going into Internal FAQ):**
- Loupe (tnull) — Q1 is the consumer-facing version; Internal FAQ needs the strategic version (relationship with tnull as both Loupe + bkb-mcp maintainer, attribution risk, talent overlap).
- Clippy custom lints — Q3 deferred to fast-follow; Internal FAQ should evaluate effort vs ROI of dual publication.
- Raw grep + LLM tool access — Q9 deferred to 3-month evaluation; Internal FAQ should pre-commit kill criteria.

**Scope and requirements signals:**
- Alpha = exactly 3 rules (NIP-44, Cashu NUT-13, MuSig2 PSBT). Two of those don't exist yet (Cashu NUT-13 + MuSig2 are unwritten as of 2026-05-17).
- Per-rule README must include: spec citation, fixtures (BAD+OK), passing `semgrep --test`, smoke results against canonical impl, known limitations.
- CI must enforce all five checks before merging a new rule.

**Hardest question that survived:** Q9 (MCP value vs grep). Even the strongest answer relies on "structured output for AI loop" which is hand-wavy. The Internal FAQ must confront: is MCP a scope distraction that splits effort from the rule pack (where value is clearer)?

---

# Internal FAQ (locked at end of Stage 4)

### I1: What's the hardest technical problem here?

A: Two. (a) **Scaling rule maintenance against spec drift.** Each NIP/BOLT/NUT evolves; without a quarterly review cycle, rules go stale and silently miss bugs. Drift-detection tooling does not exist — would require spec-repo watchers, MUST-clause extraction, regression tests against existing rules. (b) **Semgrep's Rust pattern matcher is beta** for some constructs. v1 of our first rule hit this: `match $X { ... }` patterns reject `...` arm placeholders, blocking `match`-style version checks. For an estimated 5-10 Tier-1 rules in the catalog, the bug patterns are not cleanly expressible in current Semgrep Rust support. Fallback paths (Semgrep `generic` mode, dual-publish as clippy lint, wait for Semgrep to mature) are all imperfect.

### I2: Dependency and paywall risk

A: Three live dependencies. (a) **Semgrep OSS engine and free Registry continue to exist.** Semgrep Pro has been progressively paywalling features (cross-function dataflow, taint analysis); by v1.5 some Tier-1 rules will need these. Mitigation: keep rules compatible with `opengrep` (community fork) as a hedge. (b) **MCP protocol stability.** MCP v1 stabilized late 2024; if Anthropic ships incompatible v2 our server breaks. Pin to a stable spec version; budget migration effort. (c) **Spec repos preserve stable URLs and Markdown structure.** If BIPs/BOLTs/NIPs/NUTs repos restructure, snippet retrieval breaks. Cache spec snapshots in our own repo.

### I3: First 5 maintainer integrations — concrete names and contacts `[OPEN — user to fill before alpha]`

A (placeholder, needs validation): (1) **Mostro** — DM via Nostr to project lead, pitch NIP-44 + NIP-59 coverage. (2) **Whitenoise / Marmot** — GitHub issue on `parres-hq/whitenoise`, pitch MLS-on-Nostr primitives coverage. (3) **cdk** (Cashu Rust) — pitch Cashu NUT-13 rule with the conduition.io disclosure as the anchor. (4) **rust-nostr** — share smoke test results from their own codebase. (5) **LDK** — MuSig2 nonce-reuse rule via lightningdevkit Discord. **Open work:** validate receptiveness, identify right contact channels, decide which 5 to actually target first. This is real outreach work, not paper planning.

### I4: Distribution channels reality

A: (a) **Semgrep Registry** — PR to `semgrep/semgrep-rules` with our `nostr/`, `lightning/`, `bitcoin/`, `cashu/` subfolders. Low submission bar verified. First-mover advantage in the Bitcoin-ecosystem subfolder. (b) **MCP Registry** (Sept 2025, ~17K servers indexed) — noise floor high; skip aggressive marketing until value is proven. Aggregators (PulseMCP) for discovery. (c) **Conferences** — Bitcoin++ (security-focused), Lightning Summit, NostrCon. Lightning talk: "Automating BIP/BOLT/NIP compliance checks." (d) **Direct outreach** — each alpha rule fixes a real disclosure, which is a natural opener for an issue/PR on target repos. (e) **Awesome-bitcoin / awesome-nostr** list PRs.

### I5: Author bandwidth honest estimate

A: **5-10 hours/week realistic** (user-confirmed) outside Loupe contributions, day job, and life. At this pace, the alpha (3 rules + 2-3 reviewers + roadmap polish = 72-100 person-hours) ships in **8-20 weeks calendar time**, i.e., **2-5 months**. If actual sustained bandwidth lands in the 5h/week floor, expect the upper bound.

### I6: Timeline to alpha decomposition

A:
- Rule #1 (NIP-44 missing version check): **DONE.** ~12 hours invested, committed `294add4`.
- Rule #2 (Cashu NUT-13 keyset-ID validation): ~16-24 hours (read disclosure, write fixtures, smoke-test against `cdk` and Nutshell).
- Rule #3 (MuSig2 PSBT nonce/pubkey validation): ~20-30 hours (Bitcoin Core code review, write Rust + possibly C fixtures, smoke against `bitcoin-core`).
- Roadmap navigable artifact (convert brainstorming MD to GitHub project board or structured issues): ~8 hours.
- Identify + onboard 2-3 reviewers (Q5 launch blocker): ~10-20 hours of outreach.
- Repo polish (README, CONTRIBUTING.md, license headers, badges): ~6 hours.
- **Total: 72-100 person-hours.** At 5-10 h/week → 8-20 weeks calendar.
- Cut path if pressed: drop rule #3, ship alpha with 2 rules; drop reviewer outreach after 4 weeks if no responses (ship with author-only CODEOWNERS + explicit invitation in CONTRIBUTING).

### I7: MCP scope opportunity cost — honest drop-it analysis

A: Building a useful MCP server (spec snippet indexing + retrieval + cross-references across ~50 specs) is a **minimum 60-120-hour project** — equivalent to 1-2 additional rules' implementation cost.

**Strong case for dropping from alpha scope:**
- Q9 (Customer FAQ) already commits to 3-month post-ship evaluation with "drop if no demonstrable value."
- Making the drop pre-emptive saves the implementation cost outright.
- Alpha ships ~2x faster, OR with 2-4 more rules.
- The "serious commitment" signal can be sent post-alpha with the rule pack alone.

**Counter-argument:** MCP-from-day-one differentiates the toolkit story (vs. "just another Semgrep ruleset").

**Recommendation:** Drop MCP from alpha scope. Move to v2 explicitly. The 3-month evaluation now becomes "does the rule pack show enough traction to justify investing in MCP?" — better framing of the same risk.

### I8: Maintenance burden steady-state estimate (at 20 production rules)

A: Rough monthly estimate:
- Spec watch (5 active specs × 2-3 h/month each): **10-15 h/month**.
- FP triage from external user issues (5 heavy rules × 5-10 h/month): **25-50 h/month**.
- Spec-version regression testing (re-smoke against canonical impls when specs update): **~5 h/month**.
- New rule contribution review (assume 2 PRs/month × 3-5 h): **6-10 h/month**.
- **Total at 20 rules: 50-80 hours/month — a part-time job.**

Sustainable only if: contributors materialize and absorb portion of burden, OR a consulting layer funds part-time author hours, OR named reviewers (Q5) take primary ownership of their spec area.

### I9: If no contributors materialize in 6 months — specific scenario

A: Decision tree:
- **1-2 ecosystem integrations + finding value:** continue at reduced ambition (5-10 rules total, mature what exists, rest stays roadmap with explicit "PRs welcome").
- **0 integrations:** sunset per Q10 playbook (archive with prominent README, suggest alternatives, no broken promises).
- **Consulting opportunities have emerged:** pivot to consulting-first; rule pack becomes the showcase artifact.

Honest most-likely outcome at 6 months: "mild traction, hard maintenance" — neither clean success nor clean failure. The public checkpoint forces a binary decision, which is exactly its purpose.

### I10: La pregunta brutal — is this elegant procrastination?

A: **User-confirmed decision: 2-week project cooldown ending 2026-06-01.**

> ⚠️ **Superseded 2026-05-18 — see [Updates appendix](#updates-since-closure-2026-05-17--present).** The cooldown was cancelled in favor of adversarial validation at the **And Other Stuff** conference (~2026-05-28). The original mechanic below is preserved as historical record but no longer reflects the operational plan.

Mechanic: from 2026-05-17 to 2026-06-01, the author blocks all work on btc-sec-toolkit. At the end of the window, the test:
- If the project has been on the author's mind, missed, or pulled at — **commit to alpha** (90-day plan starts 2026-06-01, alpha target 2026-08-30).
- If the project has been forgotten or felt like relief from — **close the project**. Archive repo with the smoke-tested NIP-44 rule as a standalone OSS artifact + a retrospective README.
- If genuinely ambiguous — extend cooldown by 2 more weeks (one-time only; ambiguity twice = closure).

Honest framing of the test: this is the canonical "miss it or forget it" check for motivation vs. rationalization. The user explicitly declined to commit yes/no today, which is acceptable IF the cooldown is real (no work, no thinking-about-work).

**The forcing function from Working Backwards is preserved:** the PRFAQ exists as the artifact that the cooldown-revisit decision references. If the answer is commit, the PRFAQ is the plan. If the answer is close, the PRFAQ is the retrospective.

<!-- coaching-notes-stage-4 -->
**Stage 4 — Internal FAQ closure (2026-05-17). Single pass with user inputs for I5 and I10.**

**Confirmed user data:**
- I5 bandwidth: 5-10 hours/week, sustained, realistic.
- I10 decision: 2-week cooldown ending 2026-06-01, then commit-alpha-or-close.

**Strategic decision crystallized in this stage:**
- **Drop MCP server from alpha scope.** Internal FAQ I7 analysis (60-120 hours saved = 1-2 more rules OR 2x faster alpha) combined with Customer FAQ Q9 (MCP value-prop weakest leg) makes the case overwhelming. The 3-month post-alpha evaluation becomes "does rule pack traction justify investing in MCP?" instead of "does MCP justify itself in isolation."
- This **contradicts** the user's Stage 2 decision to keep MCP with reframed value-prop. The contradiction is intentional: Stage 2 was scoping the press release vision; Stage 4 is forcing engineering reality. The PRFAQ should reflect both — press release narrative includes MCP as roadmap, internal planning treats it as v2.

**Risks identified and triaged:**
- **High:** Semgrep Pro paywalling features needed by v1.5 (taint, dataflow). Hedge: `opengrep` compatibility.
- **High:** Q5 (peer review) launch blocker is unresolved — placeholder names in I3 are not commitments. Real outreach has not happened.
- **Medium:** 50-80 hours/month maintenance burden at 20 rules is part-time job; unsustainable at solo-author with 5-10 h/week unless contributors arrive.
- **Medium:** Loupe absorption of patterns within 6 months kills differential value (the original adversarial research finding).
- **Low:** MCP protocol breaking change (mitigable by pinning).

**Unknowns flagged with "what would it take to find out":**
- Are the 5 placeholder integration targets actually receptive? **Find out by: real outreach (1-2 DMs / GitHub issues each). 2-4 weeks. Blocked by Q10 cooldown decision.**
- Will tnull (Loupe + bkb-mcp maintainer) coordinate or absorb? **Find out by: conversation with tnull post-cooldown if decision is commit.**
- Does the maintenance burden estimate (50-80 h/mo @ 20 rules) hold? **Find out by: actually shipping 5-10 rules and measuring real triage burden.**

**Strategic positioning decisions:**
- Loupe: explicit complement (already in press release leader quote).
- vs Semgrep Pro: opengrep-compatible as hedge; treat Pro as nice-to-have not dependency.
- vs Trail of Bits / Chaincode: explicitly not competing (different price/cadence tier).
- vs clippy: dual-publish as fast-follow if pressure builds.

**Out-of-scope details surfaced for future:**
- Consulting layer monetization: deferred to post-alpha; not in PRFAQ.
- Specific conference talk preparation: deferred to post-commit.
- C/Swift language support: wave 2, post Rust+TS hitting 20 rules.

---

# The Verdict (Stage 5)

## Forged in steel — what survived without cracks

1. **Bug-class reality.** The three alpha-anchor rules (NIP-44 / Cashu NUT-13 / MuSig2 PSBT) are tied to public, verifiable 2025–2026 disclosures (IACR ePrint 2025/1459, conduition.io disclosure, Bitcoin Core PR #34219). Not theoretical. Strongest line of the press release.
2. **Press release narrative.** Opening + problem paragraph name specifics, avoid marketing jargon, pass the "so what?" test effortlessly. "Most protocol-conformance bugs are caught post-incident" is honest and painful.
3. **Loupe positioning.** Explicit-complement framing in the leader quote defuses the conflict-of-interest critique up front. Stronger than hiding it.
4. **Workflow proven end-to-end.** One rule written, smoke-tested through two iterations, committed (`294add4`). The PRFAQ is not vaporware — the process is validated at scale 1.
5. **Honest risk register.** Q5 launch blocker named, Q6/Q8 fast-follows triaged, Q9 explicit risk with 3-month evaluation, Q10 sunset playbook. Unusual maturity for a solo alpha.

## Needs more heat — promising but underdeveloped

6. **I3 — outreach to 5 maintainers.** Names in the doc are placeholders, not commitments. The work that converts the PRFAQ into reality is unstarted. Needs: real list (not the placeholder set), per-project pitch, validated contact channels.
7. **Maintenance math (I8 vs I5).** 50–80 h/month at 20 rules vs 5–10 h/week available = 2-4× gap. Strategy for "what if no contributors materialize" is reactive (sunset) not proactive (contributor acquisition plan). No design for attracting the first co-maintainer.
8. **Q5 peer reviewers.** Launch blocker still in placeholder. Real names for 2-3 reviewers (Lightning + Nostr + Cashu) must land before alpha publicization.
9. **Q8 quarterly spec review cycle.** Defined as a need, not as a design. Who does what, with what cadence, what triggers a rule update — all TBD.

## Cracks in the foundation — genuine risks and unresolved contradictions

10. **I10 decision deferred to 2026-06-01 cooldown.** _[Update 2026-05-18: deferral target moved to post-And-Other-Stuff (~2026-05-28); cooldown cancelled. See [Updates appendix](#updates-since-closure-2026-05-17--present).]_ Defensible (motivation test is real) but the risk is that cooldown produces "ambiguous" and gets extended indefinitely. **Mitigation already in I10:** ambiguous twice = closure by default. That rule must apply, not be negotiated when the moment comes.
11. **TAM is brutal.** Low hundreds of devs globally (serious Lightning + Nostr + Cashu maintainers combined). If alpha succeeds, the upside is "respected niche contribution," not "successful product." If the author's unconscious aspiration is larger than TAM allows, friction is built in that the PRFAQ does not resolve.
12. **"Community-driven" framing is aspirational.** Accepted risk per user. But a technical journalist or attentive reviewer can detect the gap between community framing and solo-author reality at any time post-launch. Credibility hit not recoverable.
13. **Adoption conversion.** Press release says "installs in 60 seconds, runs alongside cargo-audit." OK. The real conversion is "team decides to add to CI" — that requires internal advocacy at each target project. No concrete plan beyond generic outreach.
14. **MCP scope contradiction (mostly resolved).** Stage 2 kept MCP in press release; Stage 4 recommended dropping from alpha scope. v3 press release frames MCP as "in early development, will" — compatible with drop-from-alpha-evaluate-later. Contradiction is mostly resolved in the document but requires discipline to not let MCP devour pre-alpha time.

## Narrative verdict

The concept is **well-forged at the base, with two unforged cracks**: (a) the commit decision has not been made (I10 cooldown to 2026-06-01), (b) the outreach work has not been started (I3 placeholders).

Everything else — press release narrative, FAQ honesty, risk register, engineering process — is solid. The PRFAQ surfaces real risk and offers no false certainty. For a solo project in exploration mode, this is genuinely useful: it saves investing 2-5 months building if the commit-test lands "close," and gives the executable plan if it lands "commit."

**If the 2026-06-01 decision lands `commit`:**
- First 4 weeks in parallel: real outreach (I3, not placeholders) + rule #2 (Cashu NUT-13).
- Outreach receptiveness as leading signal: 0 responses at 4 weeks = soft no-go; 1-2 positive = full speed.
- Hold to 90-day alpha (2026-08-30 target), accepting that "alpha shipped" is itself a checkpoint, not validation.

**If `close`:**
- Archive PRFAQ as a public example of Working Backwards surfacing no-go early.
- The NIP-44 rule committed (`294add4`) survives as a standalone OSS contribution under MIT/Apache-2.0.
- Retrospective README explains why it was closed and where the alternatives are.

Both outcomes are **wins** of the process, not failures.

<!-- coaching-notes-stage-5 -->
**Stage 5 — Verdict closure (2026-05-17). PRFAQ complete; ready for either commit-or-close decision at 2026-06-01.**

**Critical artifacts produced this stage:**
- The Verdict section above, organized as Forged / Needs Heat / Cracked.
- Distillate at `docs/prfaq-distillate.md` for downstream PRD or retrospective consumption.

**The PRFAQ is binary-ready.** A PM or solo author can take this document and translate it directly into either:
- A 90-day alpha PRD (rule pack scope, milestones, reviewer list to fill, outreach plan).
- A closure retrospective (why exploration ended, what was learned, what survives as OSS artifact).

**No more iteration recommended.** Further stress-testing without new data (i.e., without real maintainer responses or post-cooldown decision) is procrastination dressed as rigor.

---

## Updates since closure (2026-05-17 → present)

The PRFAQ above is a snapshot of the thinking that closed Stage 5 on 2026-05-17. The sections below capture strategic changes that materialized after closure. The original PRFAQ is preserved as a historical record; this appendix is the delta.

### 2026-05-18 — Cooldown cancelled by external forcing function

I10 originally specified a 2-week cooldown ending 2026-06-01 as the commit/close test. That cooldown was cancelled on 2026-05-18 due to attendance at **And Other Stuff** (Nostr-centric conference, ~2026-05-28). The new test replaces the isolated motivation check with adversarial validation face-to-face with the exact PRFAQ audience.

The decision criterion remains the same (commit / pivot / close). The mechanism shifts: instead of asking "did I miss the project?" the question becomes "did the maintainers I talked to invalidate the thesis, reinforce it, or surface a pivot I hadn't considered?" The "ambiguous twice = closure" rule from I10 still applies if the conference signal is unclear.

Implication for the Verdict: crack #10 ("I10 decision deferred to 2026-06-01 cooldown") is partially superseded — the deferral target moves to post-conference (~2026-05-28). The other 4 cracks (TAM, community-driven framing, adoption conversion, MCP scope) remain unchanged.

The framing of the conference attendance is explicitly **adversarial-listening mode**, not selling. Prep artifacts (interview guide, hypothesis register, people-to-find, cheatsheet) live privately under `_bmad-output/planning-artifacts/`.

### 2026-05-19 — Loupe attribution correction (recap)

The Stage 1 coaching notes already document the inline correction (see the bracketed note in the Stage 1 findings): the original adversarial research mis-attributed Loupe to Block/Spiral; canonical repository is `github.com/tnull/loupe`. The correction is recapped here for completeness because all later sections (Customer FAQ Q1, Internal FAQ I3, Verdict #3) reference Loupe in their positioning analysis.

None of those analyses change in substance — the competitive elephant is still Loupe — but the institutional weight assumed ("Spiral backing") does not exist. Tnull's dual position as Loupe + bkb-mcp maintainer actually *strengthens* the 6-month pattern absorption risk noted in Internal FAQ I9 and Verdict crack #14: tnull has natural architectural position to integrate Bitcoin-specific context directly into Loupe via bkb-mcp.

### 2026-05-18 — Spec retrieval fully covered by two external MCPs

Customer FAQ Q9 framed the project's own MCP server as offering three differentials over raw grep: rule cross-reference, related-clause linking, and structured AI output. Post-Q9, two existing MCP servers were tested and registered as Claude Code tools:

- **`bkb-mcp`** (by tnull) covers BIPs, BOLTs, NUTs, LUDs, bLIPs.
- **`@nostrbook/mcp`** (by Soapbox Technology) covers NIPs, kinds, tags, protocol docs.

Together they cover Q9's "structured spec lookup" differential completely. The architectural implication: a future btc-sec-toolkit MCP (if built v2) should NOT be a spec knowledge base (already solved) — it should be a **rule registry** exposing failure-mode metadata, smoke results, and suppression playbooks as tools an AI assistant can combine with the two spec MCPs. This reduces the MCP scope from ~60-120 hours (Internal FAQ I7 estimate) to ~20-40 hours.

The 3-month post-alpha MCP evaluation gate (Q9 status: explicit risk) still applies, but the kill criterion shifts: instead of "does MCP justify itself in isolation," it becomes "does the rule pack show enough traction to justify a rule-registry MCP on top?"

### 2026-05-17 — NUT-13 vs NUT-02 anchor verification pending for rule #2

The Press Release and Internal FAQ I6 commit to "Cashu NUT-13 keyset-ID derivation flaw" as alpha rule #2. A first lookup against `bkb-mcp` returned the NUT-13 spec body, which describes **Deterministic Secrets** (deriving `secret` and `r` from a BIP-39 seed via HMAC-SHA256), not keyset-ID derivation. Keyset-ID derivation belongs more directly to **NUT-02 (Keysets and keyset_id)**.

The conduition.io disclosure may anchor to: (a) NUT-02 (mis-labeled in the adversarial research that surfaced the bug); (b) NUT-13 where secret derivation consumes `keyset_id_bytes` in the HMAC input; or (c) an intersection across NUT-02 / NUT-12 / NUT-13.

Action committed before implementing rule #2: re-verify the spec anchor via `bkb_lookup_nut(2)` + `bkb_get_references("NUT-13")` + re-reading the original conduition.io blog post. Implementation against "NUT-13" is paused until confirmation.

The bug class itself is real and verified (Minibits, Cashu.me, Nutstash were affected). The fix only addresses *which spec section the rule cites and pattern-matches against*. No change to the alpha 3-rule scope.

### Working state summary (as of 2026-05-20)

- I10 verdict deferred to **post-And-Other-Stuff** (~2026-05-28), not 2026-06-01.
- AOS prep artifacts (interview guide, hypothesis register, people-to-find, cheatsheet) ship privately under `_bmad-output/planning-artifacts/` (gitignored).
- Repo is public at `github.com/ifuensan/btc-sec-toolkit` with the NIP-44 rule shipped (commit `dad89ba`, rule-id refinement commit `1e114d0`).
- Launch blocker (Q5: 2-3 named CODEOWNERS reviewers) is unresolved; AOS outreach is the primary mechanism for closing it.
- All previous decisions in the PRFAQ above stand unless explicitly contradicted by this appendix.

---




