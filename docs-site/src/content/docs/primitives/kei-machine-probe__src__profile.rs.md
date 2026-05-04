---
title: profile.rs
path: kei-machine-probe/src/profile.rs
dna_hash: sha256:cb41b8f8d690559c
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-machine-probe/src/profile.rs

Machine struct — the typed snapshot that `probe` emits.

Constructor Pattern: this cube owns the schema. Detectors fill in
their respective sub-struct; `recommend()` and `markdown()` consume
the whole thing read-only.

## Public API

- Top-level snapshot. `source_commands` records every shell-out the

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: serde

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
