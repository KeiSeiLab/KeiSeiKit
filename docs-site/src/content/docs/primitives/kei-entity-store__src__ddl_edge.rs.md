---
title: ddl_edge.rs
path: kei-entity-store/src/ddl_edge.rs
dna_hash: sha256:b30ca907f7485211
language: rust
size_loc: 132
generated: by-keidocs
---

# kei-entity-store/src/ddl_edge.rs

Edge-table DDL generators. Split out of `ddl.rs` to keep each file
inside the Constructor Pattern 200-LOC cap. `ddl.rs` retains the
entity-table, index, and FTS DDL; this module owns edge-table DDL
in all three variants (`IntegerPair`, `TextPair`,
`TextPairWithMetadata`).

## Public API

- `pub fn edge_table_for` — Dispatcher — picks edge-table DDL for a given `EdgeKeyKind`. Added
- `pub fn try_edge_table_for` — Fallible dispatcher — same as `edge_table_for` but returns
- Text-keyed edge DDL: `(src_path TEXT, dst_path TEXT, edge_type TEXT)`.
- Text-keyed edge DDL with optional metadata columns + caller-chosen
- DDL for one extra edge column. Limited subset of `FieldKind` — edge

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate

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
