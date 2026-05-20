# btc-sec-toolkit — rule pack

Semgrep rules for security failure modes specific to the Bitcoin / Lightning / Cashu / Nostr ecosystem.

Companion to the parent project. For project rationale, scope, and ruled-out branches see [`docs/project-context.md`](../docs/project-context.md). For the strategic plan, customer + internal FAQ, and the verdict on the alpha see [`docs/prfaq.md`](../docs/prfaq.md). The full 255-failure-modes catalog and AOS preparation artifacts live locally under `_bmad-output/` (not tracked).

## Layout

```
rules/
  <ecosystem>/             # nostr, lightning, bitcoin, cashu, cross-layer
    <spec-or-area>/        # nip-44, bolt-12, bip-340, ...
      <rule-name>.yaml     # the Semgrep rule(s); may contain one rule per language
      <rule-name>.rs       # Rust fixtures (semgrep --test markers)
      <rule-name>.ts       # TypeScript fixtures
      README.md            # human-readable rule documentation
```

## Running

```bash
# Run all rules against a target codebase
semgrep --config rules/ /path/to/your/code

# Test rules against their fixtures (CI gate)
semgrep --test rules/
```

## Rule index

| Rule ID | Layer | Severity | Languages | Status |
|---|---|---|---|---|
| `nostr.nip-44.missing-version-check` | Nostr | ERROR | Rust, TypeScript / JavaScript | v1.1 |

## Conventions

- **Rule ID:** `<ecosystem>.<spec>.<short-bug-name>` (e.g. `nostr.nip-44.missing-version-check`). When the same conceptual rule has per-language variants whose AST patterns diverge, suffix the language: `-rust`, `-ts`.
- **Severity:** `ERROR` for findings that imply loss of funds or compromise of keys; `WARNING` for privacy / DoS / silent misbehavior; `INFO` for hygiene.
- **Confidence / likelihood / impact** in `metadata` follow Semgrep registry semantics so rules are reusable upstream.
- **Fixtures:** every rule ships at least one `// ruleid:` (BAD) and one `// ok:` (GOOD) case per supported language. CI runs `semgrep --test rules/` and fails on regression.
- **Documentation:** every rule has a `README.md` next to the YAML explaining what it detects, why it matters, how to fix, and known limitations.

## License

MIT OR Apache-2.0 (matches parent project; permissive for ecosystem reuse).
