---
title: main.rs
path: kei-machine-probe/src/main.rs
dna_hash: sha256:9ee87c528c2e3a26
language: rust
size_loc: 12
generated: by-keidocs
---

# kei-machine-probe/src/main.rs

kei-machine-probe — CLI entry.

Constructor Pattern: `main` does parse + dispatch only. All subcommand
bodies live in `cli.rs`; all detection lives in the library. ≤30 LOC.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: clap, kei_machine_probe, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
