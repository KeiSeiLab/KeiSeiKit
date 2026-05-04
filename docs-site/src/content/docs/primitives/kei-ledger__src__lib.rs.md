---
title: lib.rs
path: kei-ledger/src/lib.rs
dna_hash: sha256:1c36a6f52629a53e
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-ledger/src/lib.rs

kei-ledger — public library surface.

Constructor Pattern: the binary (`main.rs`) and the library share the
same module tree via `mod` declarations here. External crates depend
on `kei_ledger::record_cost` directly without re-exposing the CLI's
clap-driven dispatch surface.

Wave 40 (2026-04-24): added so `kei-cortex` can plumb cost recording
through `kei_ledger::record_cost(conn, id, cents, provider, model)`
after each chat turn. Prior to v6, the only consumer was the CLI
binary itself, so a `[lib]` target was unnecessary.

## Related

- parent: `kei-ledger/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
