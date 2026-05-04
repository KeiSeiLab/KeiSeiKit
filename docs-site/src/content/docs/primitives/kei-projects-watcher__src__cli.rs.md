---
title: cli.rs
path: kei-projects-watcher/src/cli.rs
dna_hash: sha256:13f3ea09bab6268f
language: rust
size_loc: 77
generated: by-keidocs
---

# kei-projects-watcher/src/cli.rs

CLI subcommand dispatch — kept out of `main.rs` to honour the
Constructor-Pattern file-size budget.

## Public API

- `pub fn open_db` — Open the index DB and ensure its schema is migrated.
- Daemon entry: initial rebuild, then watch loop until SIGINT/SIGTERM.
- Receive debounced project paths and re-index until a signal arrives.
- `pub fn cmd_status` — Print last-indexed-ts of each project as pretty JSON to stdout.

## Related

- parent: `kei-projects-watcher/Cargo.toml`
- imports: anyhow, crate, rusqlite, std, tokio, tracing

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
