---
title: dag.rs
path: kei-pipe/src/dag.rs
dna_hash: sha256:966d5c7ef25e50b1
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-pipe/src/dag.rs

DAG spec parsing + topological sort.

TOML shape — `[[steps]]` array with fields `id`, `atom`, optional
`depends-on = [ids...]`, optional `input = { ... }`. Optional per-step
`kind = "query|transform|command|stream"` and `cache = { enabled, ttl_sec }`.
Optional DAG-level `[pipe] cache = { enabled, ttl_sec, db = "..." }`.

Invariants:
- `id` and `atom` must be non-empty strings
- `id` must be unique across the DAG
- every `depends-on` entry must reference a known step id
- the dependency graph must be acyclic

## Public API

- Error cases raised while parsing or sorting a DAG.
- One atom invocation in a DAG. `input` is retained as `serde_json::Value`
- Parsed DAG. `steps` preserves declaration order so error messages line
- Internal TOML surface — kept private so callers only see the cleaned
- `pub fn parse_dag` — Parse TOML text into a cleaned `DagSpec` with per-step validation.

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: crate, serde, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
