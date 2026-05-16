// Fixtures for missing-version-check-ts
// Run: semgrep --test rules/nostr/nip-44/

import { base64 } from "@scure/base";

// BAD: slices past the version byte without checking it.
function nip44DecryptBad(payload: string): Uint8Array {
  const bytes = base64.decode(payload);
  // ruleid: missing-version-check-ts
  const nonce = bytes.slice(1, 33);
  const ciphertext = bytes.slice(33, bytes.length - 32);
  void nonce;
  return ciphertext;
}

// OK: explicit version byte check before slicing.
function nip44DecryptGood(payload: string): Uint8Array {
  const bytes = base64.decode(payload);
  if (bytes[0] !== 0x02) {
    throw new Error(`unsupported NIP-44 version: ${bytes[0]}`);
  }
  // ok: missing-version-check-ts
  const nonce = bytes.slice(1, 33);
  return nonce;
}

// BAD: function whose name matches decrypt|decode|parse|unwrap and slices past
// byte 0 without checking — even with a generic name.
function decodePayload(payload: string): Uint8Array {
  const bytes = base64.decode(payload);
  // ruleid: missing-version-check-ts
  const body = bytes.slice(1);
  return body;
}

// OK: alias-then-check pattern (the idiomatic style in nostr-tools).
function nip44DecodeAliased(payload: string): Uint8Array {
  const bytes = base64.decode(payload);
  const vers = bytes[0];
  if (vers !== 2) {
    throw new Error("unknown encryption version " + vers);
  }
  // ok: missing-version-check-ts
  const nonce = bytes.slice(1, 33);
  return nonce;
}

// OK: function name does NOT match decrypt|decode|parse|unwrap, so rule does not apply
// (Example: ECDH shared-secret extraction in getConversationKey style.)
function getConversationKey(secret: Uint8Array): Uint8Array {
  const sharedPoint = secret;
  // ok: missing-version-check-ts
  const xCoord = sharedPoint.subarray(1, 33);
  return xCoord;
}
