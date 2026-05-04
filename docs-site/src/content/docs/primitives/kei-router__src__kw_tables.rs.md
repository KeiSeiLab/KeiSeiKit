---
title: kw_tables.rs
path: kei-router/src/kw_tables.rs
dna_hash: sha256:6c2e2cefe413b971
language: rust
size_loc: 197
generated: by-keidocs
---

# kei-router/src/kw_tables.rs

Per-domain keyword rule tables. Split from `keywords.rs` for Constructor
Pattern <200 LOC compliance. Each table is a `const` slice so the whole
router is built at compile time — zero allocation hot-path.

## Related

- parent: `kei-router/Cargo.toml`
- imports: crate

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
