---
title: cli.rs
path: kei-model/src/cli.rs
dna_hash: sha256:6508a3370454200e
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-model/src/cli.rs

clap structs for the 5 subcommands.

`Cli` is the top-level parser; `Cmd` is the subcommand enum. Concrete
arg-bags live next to their variant so each one is a self-contained struct.

## Public API

- List models matching all supplied filters.
- Pick the cheapest active model for a role + budget + caps triple.
- Estimate cost in micro-cents for a token budget.
- List distinct providers + active/deprecated counts.
- Walk a fallback chain until None or cycle.
- Comma-separated capabilities, e.g. "code,vision".
- Positional model id.

## Related

- parent: `kei-model/Cargo.toml`
- imports: clap, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
