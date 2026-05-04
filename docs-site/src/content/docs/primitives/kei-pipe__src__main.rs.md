---
title: main.rs
path: kei-pipe/src/main.rs
dna_hash: sha256:1814081fd9a2ca9d
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-pipe/src/main.rs

`kei-pipe` CLI — `run <dag.toml>` and `validate <dag.toml>`.

Exit codes:
- 0 — ok (run or validate)
- 1 — CLI / IO / parse failure (DAG couldn't be loaded)
- 2 — one or more steps failed during `run`

## Public API

- Run a DAG — execute every step in topo order, print final report.
- Parse + topo-sort without executing; prints the resolved order.

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: clap, kei_pipe, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
