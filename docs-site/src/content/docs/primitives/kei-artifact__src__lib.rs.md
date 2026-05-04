---
title: lib.rs
path: kei-artifact/src/lib.rs
dna_hash: sha256:d21a1f7a9766946f
language: rust
size_loc: 26
generated: by-keidocs
---

# kei-artifact/src/lib.rs

kei-artifact — typed artifact handoff store.

Constructor Pattern: one concern per file.
- `schema`    — SQL DDL + schema registry table.
- `store`     — `Store` cube (Connection wrapper).
- `hash`      — sha256 artifact id helper.
- `schemas`   — built-in schema registration (spec/plan/patch/review/research).
- `validate`  — minimal JSON Schema (strict subset of draft 2020-12).
- `artifact`  — CRUD on `artifacts` table (emit / get / list / chain).
- `export`    — v0.16 schema-registry export for the assembler.

Storage path (CLI default): `~/.claude/artifacts/artifacts.sqlite` or
`$KEI_ARTIFACT_DB`.

## Related

- parent: `kei-artifact/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
