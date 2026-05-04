---
title: store.rs
path: kei-token-tracker/src/store.rs
dna_hash: sha256:81ad0e8bd7e38936
language: rust
size_loc: 168
generated: by-keidocs
---

# kei-token-tracker/src/store.rs

SQLite-backed [`Store`]. Schema migrates on every `open` / `open_in_memory`.

## Public API

- `pub struct Store` — Token-event SQLite store. Holds an owned [`Connection`]; clone the
- `pub fn open` — Open or create a SQLite database at `path`, applying pending
- `pub fn open_in_memory` — In-memory variant — same migrations applied. For tests + ephemeral
- `pub fn record_event` — Insert one [`TokenEvent`]. Returns the freshly-allocated row id so
- `pub fn list_recent` — Most recent `limit` events, newest first.
- `pub fn aggregate_by_model` — Aggregate by model since a unix-epoch lower bound (`ts >= since`).
- `pub fn count` — Total event count. Used by the CLI `count` subcommand.

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: crate, rusqlite, std

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
