---
title: report_builders.rs
path: kei-db-contract/src/report_builders.rs
dna_hash: sha256:37b2c2a82756747c
language: rust
size_loc: 70
generated: by-keidocs
---

# kei-db-contract/src/report_builders.rs

Small builders that translate raw SQL/TS pieces into report rows.

## Public API

- `pub fn orphan_table_report` — Whole-table report for an SQL table with no matching TS type.
- `pub fn orphan_type_report` — Whole-table report for a TS type with no matching SQL table.
- `pub fn empty_report` — Vacuous report for the (None, None) pair (only triggered by an empty workspace).
- `pub fn append_orphan_ts_fields` — Append every TS field that no SQL column claimed as orphan-TS rows.

## Related

- parent: `kei-db-contract/Cargo.toml`
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
