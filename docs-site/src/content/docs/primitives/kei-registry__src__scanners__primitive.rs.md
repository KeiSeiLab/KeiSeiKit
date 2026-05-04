---
title: primitive.rs
path: kei-registry/src/scanners/primitive.rs
dna_hash: sha256:4e6f3f56b03237e5
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-registry/src/scanners/primitive.rs

Primitive scanner — walks `<kit-root>/_primitives/_rust/*/Cargo.toml`.

Constructor Pattern: this cube knows the workspace-crate naming
convention only. The body bytes are the raw `Cargo.toml`; the name is
`[package].name`; caps are the comma-joined dependency family heuristic
(e.g. `regex,sqlite,toml`).

## Public API

- `pub struct PrimitiveScanner` — `_primitives/_rust/<crate>/Cargo.toml` adapter.
- Extract `[package].name` from a Cargo.toml byte slice. Tolerates both
- Heuristic capability codes from declared `[dependencies]` keys.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
