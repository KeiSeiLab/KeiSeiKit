---
title: report.rs
path: frustration-matrix/src/report.rs
dna_hash: sha256:b21b2f76d2d91e06
language: rust
size_loc: 114
generated: by-keidocs
---

# frustration-matrix/src/report.rs

Report — read scan output, aggregate, print plaintext table.

Constructor Pattern: one responsibility = turn rows into a ranking.
Two group-by modes: `category` (default) and `session` (chatlog file).
Sort key = weighted score (count * weight), desc.

Output is plaintext with fixed-width columns so the user can grep it.

## Public API

- Group-by mode.
- `pub fn run` — CLI entry. Reads rows from `input`, prints the top-N table.
- `pub fn aggregate` — Aggregate rows by the chosen key. `top_example` is the first quote
- Clip long strings for table cells.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std

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
