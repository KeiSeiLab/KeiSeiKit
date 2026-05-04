---
title: backlog.rs
path: kei-memory/src/backlog.rs
dna_hash: sha256:a79f11fea150f941
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-memory/src/backlog.rs

Backlog — silent-first audit item CRUD.

Constructor Pattern: one cube, one CLI subcommand's worth of logic.
Wire-point #3 of the injection guard (RULE 0.14 audit-CRUD path):
malicious content in audit items would be rendered into self-audit
reports verbatim, so we scan before persistence the same as
`ingest::insert_event` does for transcript messages.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: chrono, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
