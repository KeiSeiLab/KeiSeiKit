---
title: stats.rs
path: kei-memory/src/stats.rs
dna_hash: sha256:eefeecf769ccbf5e
language: rust
size_loc: 33
generated: by-keidocs
---

# kei-memory/src/stats.rs

DB-wide statistics renderer.

Constructor Pattern: extracted from commands.rs (was `print_stats`).
Pure formatter: takes a Connection, returns a String. Sessions, events,
patterns counts plus the top-10 most-invoked tools.

## Public API

- `pub fn render_stats` — Render DB-wide statistics as a multi-line string.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: rusqlite

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
