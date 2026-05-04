---
title: index.rs
path: kei-projects-index/src/index.rs
dna_hash: sha256:914ba24b31f57aa4
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-projects-index/src/index.rs

Index orchestrator: walk → git_state → docs → sqlite_scan → upsert.

Constructor Pattern: one cube = the "rebuild" pipeline. Pure glue —
all data extraction lives in sibling modules. Idempotent: rebuilding
against the same filesystem yields the same DB state (rows are
upserted by primary key `path`).

## Public API

- Build a `ProjectRow` from the four data sources for one project.
- Upsert one row keyed on PRIMARY KEY (`path`).
- Open `db_path` (creating parent dir) and apply the schema.
- `pub fn rebuild_index` — Rebuild the index from `projects_root` into `db_path`.
- `pub fn rebuild_index_with_conn` — Same as `rebuild_index` but uses a caller-supplied connection. Used by
- `pub fn reindex_one` — Re-index a single project (one row). Used by the fsevents watcher

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: anyhow, chrono, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
