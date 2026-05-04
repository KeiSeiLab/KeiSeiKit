---
title: config.rs
path: kei-pipe/src/config.rs
dna_hash: sha256:8786a77c0fbb5a5d
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-pipe/src/config.rs

Per-step and DAG-level cache configuration types + TOML parsers.

Kept separate from `dag.rs` so the core DAG cube stays under the 200-LOC
Constructor Pattern limit. Everything here is a pure value type or a
small string-validation helper — no I/O, no side effects.

## Public API

- Per-step or DAG-level cache opt-in. Both fields required when present.
- Atom kind as declared in the DAG. Only `Query` and `Transform` are
- Internal TOML surface for the `[pipe]` block.
- Internal TOML surface for per-step or DAG-level `cache = { ... }`.
- Flatten the TOML view into the public [`CacheConfig`] shape. `db`
- Split the optional `[pipe]` block into `(cache_config, cache_db_path)`.
- Parse a `kind = "..."` string into a typed [`StepKind`].

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
