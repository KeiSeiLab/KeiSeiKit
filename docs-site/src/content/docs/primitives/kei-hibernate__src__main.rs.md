---
title: main.rs
path: kei-hibernate/src/main.rs
dna_hash: sha256:9d97c2374ab6e972
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-hibernate/src/main.rs

kei-hibernate CLI.

Subcommands: `export`, `import`, `inspect`. Thin dispatcher over
the library surface; each arm is <30 LOC.

## Public API

- Bundle kit_root into a tar.zst archive.
- Output bundle path (e.g. bundle.tar.zst).
- Kit root (defaults to current directory).
- Extract a bundle into kit_root (pass --dry-run to preview).
- Print manifest contents without extracting.

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: clap, kei_hibernate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
