---
title: create_defaults.rs
path: kei-entity-store/src/verbs/create_defaults.rs
dna_hash: sha256:146744d032194973
language: rust
size_loc: 109
generated: by-keidocs
---

# kei-entity-store/src/verbs/create_defaults.rs

Per-kind value-for-insert helpers split out of `create.rs` to keep
that file under the Constructor-Pattern 200-LOC cap. Each function
handles one FieldKind's default / coerce logic.

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
