---
title: cli.rs
path: kei-machine-probe/src/cli.rs
dna_hash: sha256:f61a8ea7fcd4bb13
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-machine-probe/src/cli.rs

clap CLI shapes — three subcommands.

Constructor Pattern: this cube holds parser structs only. Dispatch
happens in `main.rs`; per-subcommand handlers live in this module
and call into the library.

## Public API

- Run all detectors, emit JSON Machine struct.
- Probe + recommend, emit JSON Recommendations struct.
- Probe + recommend, emit human-readable summary.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: clap, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
