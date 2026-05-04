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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
