---
title: classifier_cli.rs
path: frustration-matrix/src/classifier_cli.rs
dna_hash: sha256:331fc010dfc9443f
language: rust
size_loc: 41
generated: by-keidocs
---

# frustration-matrix/src/classifier_cli.rs

CLI glue for the `classify` subcommand.

Constructor Pattern: main.rs stays dispatch-only; this cube owns the
print-a-ranking layer. Pure function of (dir, message, min_len,
threshold). Splits load + classify + print into three tiny helpers.

## Public API

- `pub fn run` — Entry point called from `main::dispatch`. Load bundle, classify, print.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
