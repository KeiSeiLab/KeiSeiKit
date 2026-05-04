---
title: diff.rs
path: kei-db-contract/src/diff.rs
dna_hash: sha256:8cb25b02d969afa9
language: rust
size_loc: 158
generated: by-keidocs
---

# kei-db-contract/src/diff.rs

Diff cube: compare SqlTable vs TsType, produce per-field statuses.

## Public API

- Status of a single field after pairing one SQL column with one TS field.
- Status of a paired (or orphan) table↔type unit.
- One field-level row in the report.
- One table-level row in the report.
- Top-level report shape.
- `pub fn diff_project` — Run the full diff over already-parsed inputs.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: crate, serde

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
