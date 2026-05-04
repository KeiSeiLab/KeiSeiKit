---
title: hit.rs
path: frustration-matrix/src/hit.rs
dna_hash: sha256:1e4138ea7e412386
language: rust
size_loc: 42
generated: by-keidocs
---

# frustration-matrix/src/hit.rs

Normalised per-file parse product.

Both markdown and jsonl parsers emit their own line types. The scan
loop only needs a common shape: `{file, line_no, text, timestamp?}`.
This cube is the only place that knows how to unify the two.

`timestamp` is `Some` only for jsonl entries (runtime writes an
ISO 8601 `.timestamp` field). Markdown falls back to file mtime,
applied by the scan loop — keep this struct dumb.

## Public API

- `pub struct Hit` — One candidate line for category matching.

## Related

- parent: `frustration-matrix/Cargo.toml`
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
