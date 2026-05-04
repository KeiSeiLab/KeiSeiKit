---
title: schemas.rs
path: kei-artifact/src/schemas.rs
dna_hash: sha256:953b1af2ac17db01
language: rust
size_loc: 27
generated: by-keidocs
---

# kei-artifact/src/schemas.rs

Built-in schemas — 5 shipped schemas, embedded at compile time.

Chain: architect(spec) → code-implementer(plan → patch) →
critic/security(review) → researcher(research) feeds back.
Each file lives in `kei-artifact/schemas/*.json` and is embedded via
`include_str!` so the CLI `--self-register` path needs no filesystem.

## Public API

- `pub const BUILTIN` — (name, schema JSON text). Keep in sync with `schemas/*.json`.
- `pub fn register_builtins` — Register all 5 built-in schemas. Idempotent.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: anyhow, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
