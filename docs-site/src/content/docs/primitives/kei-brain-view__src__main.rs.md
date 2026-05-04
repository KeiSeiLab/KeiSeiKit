---
title: main.rs
path: kei-brain-view/src/main.rs
dna_hash: sha256:a314aef825f90078
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-brain-view/src/main.rs

kei-brain-view CLI entrypoint.

Constructor Pattern: main.rs = argument parsing + dispatch. Each
subcommand calls into a library fn. No business logic inline.

## Public API

- Path to ledger.sqlite. Defaults to ~/.claude/agents/ledger.sqlite.
- Print the full fork-tree as indented text (roots first, BFS).
- Bucket counts by status + has-dna.
- Print ancestors + descendants of the node matching a DNA prefix.
- Group DNAs by scope / body / role+caps and print the cluster tree.
- One of: scope | body | role
- One-shot aggregate summary over the ledger DNAs.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: clap, kei_brain_view, kei_dna_index, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
