---
title: ts_parse.rs
path: kei-db-contract/src/ts_parse.rs
dna_hash: sha256:4d25fc1c8225df44
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-db-contract/src/ts_parse.rs

TS parser cube: shallow regex extraction of `type X = { ... }` and `interface X { ... }`.

## Public API

- One declared field on a TS type / interface.
- One declared `type` alias or `interface` block.
- `pub fn parse_ts_glob` — Walk a directory recursively, parse every `.ts` and `.tsx` file.
- `pub fn extract_ts_types` — Extract every type/interface block from a single TS source string.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: anyhow, regex, serde, std, walkdir

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
