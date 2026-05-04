---
title: main.rs
path: kei-arch-derive/src/main.rs
dna_hash: sha256:d5d3c831ae716bd8
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-arch-derive/src/main.rs

kei-arch-derive CLI — emit / coverage / infer.

Constructor Pattern: each subcommand is a thin wrapper around one
library entry point. Heavy lifting lives in `lib.rs` modules.

## Public API

- Path to the kei-registry SQLite database. Read-only in v0.1 (the

## Related

- parent: `kei-arch-derive/Cargo.toml`
- imports: anyhow, clap, kei_arch_derive, kei_registry, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
