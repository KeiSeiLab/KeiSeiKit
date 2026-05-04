---
title: lib.rs
path: kei-pet/src/lib.rs
dna_hash: sha256:e1c376e5476545f9
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-pet/src/lib.rs

kei-pet — pet persona manifest parse/validate/overlay.

Scope boundaries: this crate implements a standard TOML-backed persona
manifest. Identity is Ed25519 (RFC 8032). Cache/projection patterns are
standard CQRS. NO imports, references, or conceptual mentions of sibling
research-grade projects are permitted in this crate.

## Public API

- `pub const SCHEMA_VERSION` — Current schema version written by this crate.
- `pub fn parse` — Parse TOML text → `PetManifest`, running full validation.
- `pub fn to_toml` — Serialize a validated manifest back to TOML text.

## Related

- parent: `kei-pet/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
