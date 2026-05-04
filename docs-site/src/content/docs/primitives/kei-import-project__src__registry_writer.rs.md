---
title: registry_writer.rs
path: kei-import-project/src/registry_writer.rs
dna_hash: sha256:6900f407c0963398
language: rust
size_loc: 132
generated: by-keidocs
---

# kei-import-project/src/registry_writer.rs

registry_writer — register identified modules in kei-registry.

Each module becomes a `BlockType::Primitive` row. Idempotent: matching
body_sha → no-op (unchanged). Differing body_sha → supersede chain.

Constructor Pattern: one responsibility (write modules → registry). I/O
lives here; DNA composition delegates to kei_registry::register.

## Public API

- `pub struct RegisterResult` — Summary returned after a register call.
- `pub fn register_modules` — Register every module in `kei-registry` under `BlockType::Primitive`.
- Concatenate source file contents in sorted-path order for deterministic SHA.
- 8-hex SHA-256 prefix over raw bytes. Mirrors kei_registry::dna_block usage.
- `pub fn project_slug` — Last path component of project root, defaulting to "unknown".
- Resolve the registry DB path: explicit → env → default.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, kei_registry, sha2, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
