---
title: matching.rs
path: kei-db-contract/src/matching.rs
dna_hash: sha256:8562ecd488632119
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-db-contract/src/matching.rs

Pair SQL tables with TS types using name heuristics:
`users` ≈ `User`, `magic_tokens` ≈ `MagicToken`, `auth_sessions` ≈ `AuthSession`.

## Public API

- One paired (or orphan) result from the matching step.
- `pub fn pair_tables_with_types` — Pair every SQL table with at most one TS type and vice versa.
- `pub fn canonicalize_table` — Strip plural -s, normalize separators, lowercase. `magic_tokens` → `magictoken`.
- `pub fn canonicalize_type` — Lowercase + strip trailing -s. `MagicToken` → `magictoken`.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
