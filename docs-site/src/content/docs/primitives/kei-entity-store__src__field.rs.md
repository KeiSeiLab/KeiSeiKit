---
title: field.rs
path: kei-entity-store/src/field.rs
dna_hash: sha256:99729beb13b3077e
language: rust
size_loc: 94
generated: by-keidocs
---

# kei-entity-store/src/field.rs

`FieldDef` — one column in an `EntitySchema`. Split out of
`schema.rs` to keep both files under the Constructor-Pattern
200-LOC cap.

## Public API

- One column in an EntitySchema.
- Default literal for TextDefault / IntegerNotNull (as SQL literal
- Emit a single-column index `idx_<table>_<name>`.
- Default for `Real` / `RealDefault` columns. `None` means 0.0.
- Sentinel pair for `TextArchiveEnum` — `(active, archived)`.
- Internal base constructor — zeroes optional fields so the
- `pub fn is_pk` — True if this FieldDef is a primary key (either integer or text).

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
