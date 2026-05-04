---
title: overlay.rs
path: kei-pet/src/overlay.rs
dna_hash: sha256:e000e2227cb14b2d
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-pet/src/overlay.rs

Render a validated `PetManifest` → system-prompt overlay string.

Used by any agent spawn / routine render: prepend this text to the agent's
base system prompt. Deterministic — same manifest → same overlay, byte-for-byte.

## Public API

- `pub fn system_prompt` — Build the overlay prefix that a `PetManifest` contributes to a system prompt.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
