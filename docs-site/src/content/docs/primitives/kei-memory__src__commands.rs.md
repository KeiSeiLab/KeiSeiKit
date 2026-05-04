---
title: commands.rs
path: kei-memory/src/commands.rs
dna_hash: sha256:738f88c0db5df472
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-memory/src/commands.rs

Command handlers — one function per CLI subcommand.

Constructor Pattern: each handler <30 LOC, single responsibility.
Pulled out of main.rs to keep the dispatcher under the 200 LOC limit.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
