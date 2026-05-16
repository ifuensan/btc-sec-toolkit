// Fixtures for nostr.nip-44.missing-version-check-rust
// Run: semgrep --test rules/nostr/nip-44/

use base64::{engine::general_purpose::STANDARD, Engine};

/// BAD: slices past the version byte without checking it.
fn nip44_decrypt_bad(payload: &str) -> Result<Vec<u8>, anyhow::Error> {
    let bytes = STANDARD.decode(payload)?;
    // ruleid: nostr.nip-44.missing-version-check-rust
    let nonce = &bytes[1..33];
    let ciphertext = &bytes[33..bytes.len() - 32];
    let _mac = &bytes[bytes.len() - 32..];
    let _ = nonce;
    Ok(ciphertext.to_vec())
}

/// OK: explicit version byte check before slicing.
fn nip44_decrypt_good(payload: &str) -> Result<Vec<u8>, anyhow::Error> {
    let bytes = STANDARD.decode(payload)?;
    if bytes[0] != 0x02 {
        return Err(anyhow::anyhow!("unsupported NIP-44 version: {}", bytes[0]));
    }
    // ok: nostr.nip-44.missing-version-check-rust
    let nonce = &bytes[1..33];
    Ok(nonce.to_vec())
}

/// OK: validation through `.first()`.
fn nip44_decrypt_first(payload: &str) -> Result<Vec<u8>, anyhow::Error> {
    let bytes = STANDARD.decode(payload)?;
    if bytes.first() != Some(&0x02) {
        return Err(anyhow::anyhow!("unsupported NIP-44 version"));
    }
    // ok: nostr.nip-44.missing-version-check-rust
    let nonce = &bytes[1..33];
    Ok(nonce.to_vec())
}

/// BAD: any function inside a nip44-path file whose name matches the
/// decrypt|decode|parse|unwrap regex and slices past byte 0 without checking it.
fn decode_payload(payload: &str) -> Result<Vec<u8>, anyhow::Error> {
    let bytes = STANDARD.decode(payload)?;
    // ruleid: nostr.nip-44.missing-version-check-rust
    let body = &bytes[1..];
    Ok(body.to_vec())
}

/// OK: alias-then-check pattern (idiomatic in real codebases).
fn nip44_decrypt_aliased(payload: &str) -> Result<Vec<u8>, anyhow::Error> {
    let bytes = STANDARD.decode(payload)?;
    let version = bytes[0];
    if version != 0x02 {
        return Err(anyhow::anyhow!("unsupported NIP-44 version: {}", version));
    }
    // ok: nostr.nip-44.missing-version-check-rust
    let nonce = &bytes[1..33];
    Ok(nonce.to_vec())
}

/// OK: function name does NOT match decrypt|decode|parse|unwrap, so rule does not apply
/// even though it slices past byte 0. (Example: ECDH shared-secret extraction.)
fn get_conversation_key(secret_key: &[u8], pubkey: &[u8]) -> Vec<u8> {
    let shared = STANDARD.decode("foo").unwrap_or_default();
    // ok: nostr.nip-44.missing-version-check-rust
    let x_coord = &shared[1..33];
    x_coord.to_vec()
}
