---
title: lib.rs
path: kei-projects-index/src/lib.rs
dna_hash: sha256:ccbe918cfbbbfe40
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-projects-index/src/lib.rs

kei-projects-index — public library surface.

Constructor Pattern: each module is one cube with one responsibility.
`kei-projects-watcher` (sibling daemon) and `kei-cortex` (HTTP daemon)
both depend on this crate's library API to read / write the project
state DB at `~/.claude/agents/projects-index.sqlite`.

## Related

- parent: `kei-projects-index/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
