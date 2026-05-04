---
title: cli_cmds.rs
path: kei-artifact/src/cli_cmds.rs
dna_hash: sha256:472c48b6ad388617
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-artifact/src/cli_cmds.rs

CLI command bodies for artifact CRUD (emit / get / list / chain).

Constructor Pattern: one file for the read/write-artifact commands,
kept separate from main.rs so the binary file stays <200 LOC.
Each public `cmd_*` fn < 30 LOC.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: kei_artifact, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
