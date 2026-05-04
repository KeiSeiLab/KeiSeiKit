---
title: validate.rs
path: kei-entity-store/src/verbs/validate.rs
dna_hash: sha256:24a5faa9ca71faac
language: rust
size_loc: 112
generated: by-keidocs
---

# kei-entity-store/src/verbs/validate.rs

Shared input-type validator for create / update.

Strict typed validation: integer fields require JSON numbers that
fit i64; text fields require JSON strings; real fields require JSON
numbers convertible to f64. Wrong-type input returns
`VerbError::InvalidType` instead of silent coercion to `0` / `""`.

TEXT size cap: any text value longer than `MAX_TEXT_BYTES` is
rejected to prevent OOM from hostile input. Per-field override is
planned (TODO B5: add `max_bytes: Option<usize>` to `FieldDef`).

## Public API

- `pub const MAX_TEXT_BYTES` — Default TEXT size cap — 64 KiB. Enforced for every TextNotNull /
- `pub fn coerce` — Convert an input JSON value to a typed `SqlValue` for `f`.
- `pub fn check_text_len` — Reject text values that exceed the configured cap. Used by create

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
