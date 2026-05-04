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
