---
title: evolution.rs
path: kei-pet/src/evolution.rs
dna_hash: sha256:e841bbe38700a0d2
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-pet/src/evolution.rs

Persona version history + manifest diff.

`PersonaVersion` records a single snapshot of a `PetManifest` with a
monotonic version number and an optional parent pointer — forming a linked
history chain. `diff` produces a minimal set of human-readable `Change`
entries between two manifests (voice tone, edge directness/initiative,
humor style, forbidden topics, identity languages). Persistence (file
layout, serialization target) is the caller's concern; this module is
pure data.

## Public API

- Unix seconds (UTC). Caller fills via `chrono::Utc::now().timestamp()`
- `pub fn diff` — Minimal ordered diff between two manifests.
- `pub fn fork_version` — Fork a new version off `parent`. `created_at` is left at 0 — caller

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate, serde

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
