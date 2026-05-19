# `nostr.nip-44.missing-version-check`

| | |
|---|---|
| **Severity** | ERROR |
| **Languages** | Rust, TypeScript / JavaScript |
| **Category** | security · audit |
| **Confidence / Likelihood / Impact** | MEDIUM / MEDIUM / HIGH |

## What this rule detects

Manual NIP-44 decryption implementations that slice the payload past the version byte (offset 0) without first validating it against the supported version constant.

## Why it matters

[NIP-44](https://github.com/nostr-protocol/nips/blob/master/44.md) (current version: v2, version byte `0x02`) states that decoders **MUST** reject unsupported version bytes. Without an explicit check, two classes of failure become exploitable:

1. **Future-version downgrade confusion.** When NIP-44 v3 ships, a v3 ciphertext fed to a v2-only decoder will be silently misparsed — offsets shift, lengths differ, and HMAC verification fails in ways that may surface as decryption errors rather than version errors. In layered protocols (NIP-17 DMs, NIP-59 gift wrap, NIP-EE MLS) the confusion propagates upward.
2. **Malformed-input handling.** Corrupted payloads where the version byte is non-`0x02` get parsed as if valid v2. MAC verification then fails, but the failure path may be observable (timing, error message, log line) — providing oracle behaviour to an attacker probing for weaknesses.

The bug is most common in **custom NIP-44 implementations**: clients that reimplemented the primitive before `nostr-sdk` (Rust) / `nostr-tools` (TS) matured, or wrappers that strip the version byte before delegating to a library decrypt.

## Affected reference projects

- **[Mostro](https://github.com/MostroP2P/mostro)** — P2P Bitcoin/Lightning exchange. Uses NIP-59 gift wrap + NIP-44 as primitive for all order/trade messages.
- **[Whitenoise](https://github.com/parres-hq/whitenoise) / [Marmot Protocol](https://github.com/marmot-protocol/marmot)** — Secure messenger built on MLS-over-Nostr ([NIP-EE](https://nips.nostr.com/EE)). NIP-44 is the encryption primitive underneath MLS.
- Any [NIP-17](https://github.com/nostr-protocol/nips/blob/master/17.md) client (private DMs combining NIP-44 + NIP-59).

## How to fix

Add an explicit version byte check before slicing:

**Rust:**

```rust
let bytes = STANDARD.decode(payload)?;
if bytes[0] != 0x02 {
    return Err(anyhow!("unsupported NIP-44 version: {}", bytes[0]));
}
let nonce = &bytes[1..33];
```

**TypeScript:**

```typescript
const bytes = base64.decode(payload);
if (bytes[0] !== 0x02) {
  throw new Error(`unsupported NIP-44 version: ${bytes[0]}`);
}
const nonce = bytes.slice(1, 33);
```

Preferred when possible: delegate to the canonical libraries (`nostr-sdk::nips::nip44::decrypt` in Rust; `nip44.decrypt` from `nostr-tools` in TS), which handle version validation internally.

## Scoping (v1.1)

Two-stage filter:

1. **Path-based.** Only files / directories whose path contains `nip44`, `nip-44`, or `nip_44` are scanned. Catches the idiomatic module/file naming used by `rust-nostr` (`crates/nostr/src/nips/nip44/v2.rs`) and `nostr-tools` (`nip44.ts`).
2. **Function-name regex.** Within those files, only functions whose name matches `(?i).*(decrypt|decode|parse|unwrap).*` are scanned. Eliminates false positives on unrelated functions in the same file (e.g. `getConversationKey` which slices `[1, 33)` from a secp256k1 shared point, not from a NIP-44 payload).

Suppression heuristics (the rule does NOT fire if the enclosing function contains any of these):

- Direct `if`-style check: `if bytes[0] != V`, `if bytes[0] == V`, plus `!==` / `===` / `!=` / `==` variants (TS).
- `.first()`-style check (Rust): `if bytes.first() != V`, `if bytes.first() == V`.
- `assert_eq!(bytes[0], V)` (Rust).
- **Alias-then-check** (the idiomatic style in `nostr-tools::decodePayload`): `const vers = bytes[0]; if (vers !== V) ...`. Covered for both `let` and `const` bindings in TS, and `let $X = $BYTES[0]` in Rust.

## Known limitations (v1.1)

- **Method definitions** (`class Decoder { nip44Decrypt() { ... } }`) and **arrow functions** are not covered. Only top-level `fn`/`function` declarations match.
- **`match` / `switch` checks not recognized.** Validation via `match bytes[0] { ... }` (Rust) or `switch (bytes[0]) { ... }` (TS) will fire a false positive. Semgrep's parser rejects `match`/`switch` with `...` arm placeholders. Workaround: refactor to `if`-style or suppress with `// nosemgrep`.
- **Cross-procedure trust** (acceptable false positive — judgement call). When the public dispatcher validates the version byte and delegates to a version-specific internal function (e.g. `impl::decrypt_to_bytes` validates and calls `v2::decrypt_to_bytes`), the internal function still trips the rule because it can't see the validation in the caller. This is arguably a true positive ("defense-in-depth weakness — the internal function trusts its input") and the canonical fix is to either re-validate or accept a `ValidatedPayload` newtype proving validation happened. Suppress with:
  ```rust
  // nosemgrep: nostr.nip-44.missing-version-check-rust — version validated in dispatcher impl.rs:N
  ```
- **`.first()` alias-then-check not recognized.** The pattern `let v = *bytes.first()?; if v != 2 { ... }` (used in `rust-nostr::impl::decrypt_to_bytes`) does not currently suppress, due to Semgrep Rust parser issues with `*foo()?` syntax in metavariable position. To be addressed in v1.2.
- **Per-language rule split.** AST patterns diverge enough between Rust and TypeScript that the rule is split into `-rust` and `-ts` variants in the same YAML. Conceptually one rule.

## Smoke test results (2026-05-16)

Run against canonical NIP-44 implementations:

| Repo | Findings | Notes |
|---|---|---|
| `rust-nostr/nostr` (`crates/nostr/src/nips/nip44/`) | 1 | `v2::decrypt_to_bytes:199` — cross-procedure trust pattern (see limitations above) |
| `nbd-wtf/nostr-tools` (`nip44.ts`) | 0 | clean — `decodePayload` covered by alias-then-check suppression |

## References

- [NIP-44 spec](https://github.com/nostr-protocol/nips/blob/master/44.md)
- [NIP-17 — Private Direct Messages](https://github.com/nostr-protocol/nips/blob/master/17.md)
- [NIP-59 — Gift Wrap](https://github.com/nostr-protocol/nips/blob/master/59.md)
- [NIP-EE — E2EE Messaging using MLS](https://nips.nostr.com/EE)
- [Marmot Protocol](https://github.com/marmot-protocol/marmot)
- [Mostro](https://github.com/MostroP2P/mostro)

## Provenance

This rule is item **#231 / #232 / #233** (NIP-44 cripto bugs cluster) in the local failure-modes catalog (255 candidates) referenced in the project's [PRFAQ](../../../docs/prfaq.md). The full catalog is a working artifact under `_bmad-output/` (not tracked). First implementation: 2026-05-16.
