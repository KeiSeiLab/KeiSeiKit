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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
