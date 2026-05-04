---
title: quality_constructor_pattern.rs
path: kei-agent-runtime/src/verifies/quality_constructor_pattern.rs
dna_hash: sha256:8906c39a449df5b8
language: rust
size_loc: 102
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/quality_constructor_pattern.rs

`quality::constructor-pattern` — walks the run dir, asserts every `.rs`
file ≤ 200 LOC and every top-level `fn` ≤ 30 LOC.

## Public API

- Extract `(fn_name, line_count)` for top-level `fn` definitions by tracking

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, std, walkdir

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
