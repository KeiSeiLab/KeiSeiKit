---
title: mod.rs
path: keisei/src/adapters/mod.rs
dna_hash: sha256:1eddf6d80b80f435
language: rust
size_loc: 14
generated: by-keidocs
---

# keisei/src/adapters/mod.rs

Concrete `ClientAdapter` implementations, one file per client.

Constructor Pattern: this file is the module declaration hub only —
no logic lives here. `jsonmcp` owns the shared JSON merge helpers
used by every JSON-keyed adapter (claude-code, cursor, zed).
`_registry` is the single canonical adapter list (v0.22).

## Related

- parent: `keisei/Cargo.toml`

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
