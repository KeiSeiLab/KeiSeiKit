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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
