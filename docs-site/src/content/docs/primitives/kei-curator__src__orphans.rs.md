---
title: orphans.rs
path: kei-curator/src/orphans.rs
dna_hash: sha256:78cd6812fd367c95
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-curator/src/orphans.rs

Prune orphan URIs — those that appear in `cross_edges` but have no in-edges.
Conservative: only removes edges where the tail URI has no other incoming edge.

## Related

- parent: `kei-curator/Cargo.toml`
- imports: anyhow, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
