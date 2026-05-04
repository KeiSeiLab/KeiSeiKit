---
title: lib.rs
path: kei-projects-watcher/src/lib.rs
dna_hash: sha256:5bdc4bc72bf11fd2
language: rust
size_loc: 19
generated: by-keidocs
---

# kei-projects-watcher/src/lib.rs

kei-projects-watcher — fsevents daemon that keeps `kei-projects-index`
fresh by watching `~/Projects/` and debouncing 2 s before re-indexing
each touched project root.

The library surface (this file) exposes the watcher + debounce types
plus CLI subcommand bodies so the binary stays under the
Constructor-Pattern file budget.

Constructor Pattern: every concrete type lives in its own module
(`watcher`, `debounce`, `cli`); this file ONLY declares modules and
re-exports the public API.

## Related

- parent: `kei-projects-watcher/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
