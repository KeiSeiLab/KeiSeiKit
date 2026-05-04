---
title: dump.rs
path: kei-memory/src/dump.rs
dna_hash: sha256:927cc7792959cc96
language: rust
size_loc: 43
generated: by-keidocs
---

# kei-memory/src/dump.rs

Event dump renderer — print events for a session as markdown.

Constructor Pattern: extracted from commands.rs (was `dump_events`).
Pure formatter: takes a Connection + session_id, returns a String.
The CLI wrapper in commands.rs prints it; library callers can capture.

## Public API

- `pub fn render_events` — Render a session's events as a markdown bullet list.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
