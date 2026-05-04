---
title: main.rs
path: kei-decision/src/main.rs
dna_hash: sha256:05c2c5d2ad490e15
language: rust
size_loc: 163
generated: by-keidocs
---

# kei-decision/src/main.rs

kei-decision CLI entry point — clap parse + dispatch only.

Each subcommand has a thin `run_*` function declared inline below. Heavy
logic lives in the library modules (`parser`, `classifier`, `ranker`,
`emitter`, `executor`, `ledger`, `sleep_link`, `graph`).

## Related

- parent: `kei-decision/Cargo.toml`
- imports: clap, kei_decision, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
