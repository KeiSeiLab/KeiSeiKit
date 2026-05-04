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
