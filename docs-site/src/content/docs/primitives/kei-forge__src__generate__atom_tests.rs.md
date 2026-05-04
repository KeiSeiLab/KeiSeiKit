---
title: atom_tests.rs
path: kei-forge/src/generate/atom_tests.rs
dna_hash: sha256:82c5c0b019bf9912
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-forge/src/generate/atom_tests.rs

Integration-flavoured tests for the pure-Rust atom generator.

Uses `tempfile::TempDir` to stand up a miniature copy of the repo
layout and exercises `generate_atom` end-to-end without touching the
real filesystem. Kept in its own file so `generate.rs` stays within
the Constructor-Pattern 200-LOC cap.

## Related

- parent: `kei-forge/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
