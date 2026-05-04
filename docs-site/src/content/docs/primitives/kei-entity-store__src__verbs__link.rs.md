---
title: link.rs
path: kei-entity-store/src/verbs/link.rs
dna_hash: sha256:2ab6e706f62de6bb
language: rust
size_loc: 191
generated: by-keidocs
---

# kei-entity-store/src/verbs/link.rs

`link` verb — INSERT edge into `<edge_table>` (idempotent via
INSERT OR IGNORE). Caller is responsible for higher-level semantic
checks (cycle detection, self-loop) — those live in the sibling
crate (e.g. kei-task::deps).

Dispatches on `schema.edge_key_kind`:
- `IntegerPair`             — input `{from: i64, to: i64, edge_type?}`
- `TextPair`                — input `{from: str, to: str, edge_type?}`
- `TextPairWithMetadata {…}` — same text keys plus optional
`weight: f64` input; `edge_id` / `created_at` are engine-managed
and NEVER taken from the caller.

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

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
