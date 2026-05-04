---
title: since.rs
path: frustration-matrix/src/since.rs
dna_hash: sha256:63867fe02ebfbcb9
language: rust
size_loc: 71
generated: by-keidocs
---

# frustration-matrix/src/since.rs

`--since` parser + mtime filter.

Accepts `30d`, `7d`, `1d`, `all`, or any `<N>d` (positive integer days).
Returns a `SystemTime` cut-off; files strictly older than the cut-off
are excluded from the scan.

Rationale: we use filesystem mtime rather than in-document timestamps
because chatlogs have heterogeneous timestamp formats (ISO, human,
none). mtime is reliable, cheap, and matches user intent of "files I
edited in the last 30 days".

## Public API

- `pub fn parse` — Cut-off time from a `--since` string. `None` means "scan everything".
- `pub fn passes` — True iff `path`'s mtime is at or after `cutoff`. Missing mtime → true

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, std

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
