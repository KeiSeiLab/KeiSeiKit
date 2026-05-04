---
title: dna_block.rs
path: kei-registry/src/dna_block.rs
dna_hash: sha256:6e8065347a52e12c
language: rust
size_loc: 78
generated: by-keidocs
---

# kei-registry/src/dna_block.rs

DNA composition for kit blocks.

Wire-format `<block_type>::<caps>::<scope_sha8>::<body_sha8>-<nonce8>`
delegates to `kei_shared::compose_dna` so the format string SSoT stays
in one place. `compose_for_block` is the only public surface — all
other crates that want a block DNA call this helper.

Determinism: scope_sha and body_sha are pure SHA-256 over canonical
inputs. The nonce is the only entropy source; callers that want
idempotency pass the existing row's nonce.

## Public API

- `pub fn short_sha8` — 8-hex (32-bit) prefix of SHA-256(`input`). Lowercase, deterministic.
- `pub fn compose_for_block` — Compose a block DNA. `block_type` becomes the wire `<role>` segment.
- `pub fn compose_for_block_with_nonce` — Deterministic variant — caller supplies the nonce. Used by `register`
- `pub fn fresh_nonce` — Generate a fresh 8-hex nonce from system entropy. Stable per-block: the

## Related

- parent: `kei-registry/Cargo.toml`
- imports: crate, kei_shared, sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
