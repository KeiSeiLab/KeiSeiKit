---
title: task.rs
path: kei-scheduler/src/task.rs
dna_hash: sha256:3746890ab5867553
language: rust
size_loc: 61
generated: by-keidocs
---

# kei-scheduler/src/task.rs

`Task` — in-memory snapshot of a `scheduler_tasks` row.

Serializable for the CLI (`list-due` prints JSON). Status is a plain
String so callers can introduce new sentinels without a type bump.

## Public API

- `pub mod status` — Canonical task status sentinels. Schema default is `pending`;
- `pub fn from_row` — Column order MUST match the SELECT in `query.rs::SELECT_ALL`.
- `pub const SELECT_COLS` — SELECT column list used by `query.rs` and `run.rs`. Exported so

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: rusqlite, serde

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
