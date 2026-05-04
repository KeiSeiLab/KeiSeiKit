---
title: identity.rs
path: kei-pet/src/identity.rs
dna_hash: sha256:67c8a8aaf17f1979
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-pet/src/identity.rs

Ed25519 identity (RFC 8032) — no proprietary crypto, no matrix math.

Identity flow:
1. Client generates `Keypair` on first run (`generate_keypair`).
2. `user_id` is the first 16 hex chars of `blake3(public_key_bytes)`.
3. Requests are signed with the private key; the server verifies using
the advertised public key.

The public key is safe to publish; the private key is stored locally in
`~/.keisei/identity.key` (filesystem permissions `0600`).

## Public API

- `pub fn from_secret_hex` — Reconstruct from a 32-byte secret hex string.
- `pub fn user_id_from_pubkey` — Derive a stable 16-hex-char user id from a 32-byte Ed25519 public key.
- `pub fn generate_keypair` — Generate a fresh Ed25519 keypair using the OS RNG.
- `pub fn verify` — Verify a signature against a public key and message.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: ed25519_dalek, rand_core

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
