---
title: main.rs
path: kei-task/src/main.rs
dna_hash: sha256:bc25f961a114d689
language: rust
size_loc: 180
generated: by-keidocs
---

# kei-task/src/main.rs

kei-task CLI — create / update / add-dep / graph / dependency-chain.

Pilot refactor (Stream B): `create`, `search`, `add-dependency` now
dispatch through `kei_task::atoms::*`. Remaining subcommands call
legacy module functions directly — they migrate in a later pass.

## Public API

- Typed error used by the CLI to carry both a message and an exit code.
- Machine-facing atom invocation — `run-atom <verb>` reads JSON from
- Classify any kei-task atom error via the shared `run_atom` exit-code table.

## Related

- parent: `kei-task/Cargo.toml`
- imports: clap, kei_task, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
