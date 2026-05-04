---
title: lib.rs
path: kei-graph-check/src/lib.rs
dna_hash: sha256:636d662fe52cd969
language: rust
size_loc: 10
generated: by-keidocs
---

# kei-graph-check/src/lib.rs

kei-graph-check — post-refactor reference-integrity gate.

Inputs: a directory root + an optional patch file (advisory only — we
detect file deletions/renames declared in the patch header and warn).
Output: list of broken references with file:line.

## Related

- parent: `kei-graph-check/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
