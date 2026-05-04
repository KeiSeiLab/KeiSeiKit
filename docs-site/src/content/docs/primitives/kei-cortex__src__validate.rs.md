---
title: validate.rs
path: kei-cortex/src/validate.rs
dna_hash: sha256:96a5a7f4637b9e1b
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-cortex/src/validate.rs

Shared input validators — pure functions, no I/O, no state.

Centralised so every handler touching `:user_id` goes through the SAME
whitelist. Rejects anything we would not want substituted into a path,
a SQL LIKE clause, or a TOML filename.

## Public API

- `pub const MAX_USER_ID_LEN` — Upper bound on `user_id` length. Keeps path construction trivial and
- `pub fn user_id` — Strict whitelist validator for the `:user_id` path parameter.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
