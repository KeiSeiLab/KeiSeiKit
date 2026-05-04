---
title: sqlite_scan.rs
path: kei-projects-index/src/sqlite_scan.rs
dna_hash: sha256:33155e8890fef081
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-projects-index/src/sqlite_scan.rs

SQLite-file presence scanner.

Constructor Pattern: one cube = "how many `*.sqlite` files live at
depth ≤ 2 under this project?". The dashboard uses this to decide
whether to expose a project to Datasette. Depth is bounded so we
never run away into vendored dependencies.

## Public API

- Returns true if `path` ends in `.sqlite` (case-insensitive). Files
- `pub fn count_sqlite_files` — Count `*.sqlite` files under `project_root` to depth ≤ 2.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
