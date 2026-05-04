---
title: substituter.rs
path: kei-leak-matrix/src/substituter.rs
dna_hash: sha256:819d49ee4f289aab
language: rust
size_loc: 30
generated: by-keidocs
---

# kei-leak-matrix/src/substituter.rs

Substituter — applies substitute-severity rules to a string.

Used by `kei-leak-matrix substitute --scope <s>` and by upstream hooks
(e.g. sync-public.sh) before any block check is run.

## Public API

- `pub fn substitute` — Apply every substitute-severity rule whose scope matches `requested`.
- `pub fn cmd_substitute` — Handler: read stdin, write substituted content to stdout. Exit 0.

## Related

- parent: `kei-leak-matrix/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
