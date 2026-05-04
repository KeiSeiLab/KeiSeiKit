---
title: related.rs
path: kei-registry/src/related.rs
dna_hash: sha256:b802601ae323b83e
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-registry/src/related.rs

Related-block discovery via naive substring match.

Constructor Pattern: this cube owns the dependency-graph approximation
ONLY. It re-reads body bytes from disk for each candidate (cheap on a
few-thousand-block kit) and looks for the root block's name as a
substring. Depth > 1 unrolls iteratively over the active set.

Limitations: substring match is intentionally lossy — false positives
(one block's name appears in unrelated prose) cost the user nothing
beyond an extra row in JSON output. False negatives (a block referred
to by alias) are an open extension, not a bug.

## Public API

- One related-hit row: the block plus its BFS distance from the root.
- `pub fn find_related` — Find blocks whose body references `root` by name. BFS bounded by `depth`.
- One BFS step. Read each frontier block's body and find blocks in

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, rusqlite, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
