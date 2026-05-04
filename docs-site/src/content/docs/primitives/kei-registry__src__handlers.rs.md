---
title: handlers.rs
path: kei-registry/src/handlers.rs
dna_hash: sha256:5715907ca2eb41f6
language: rust
size_loc: 328
generated: by-keidocs
---

# kei-registry/src/handlers.rs

CLI command handlers.

Constructor Pattern: this cube wires CLI args to library calls. Each
handler is a thin adapter. The common parts (db path resolve,
id-or-DNA lookup) live in sibling cubes (`paths.rs`, `lookup.rs`).
Schema-version mismatch surfaces as exit 3, not-found 2, IO 1.

## Public API

- Exit-code outcome. `Ok` for success, plus typed not-found variant.
- `pub fn dispatch` — Dispatch one parsed Command. Returns Outcome → main maps to exit code.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
