---
title: diff.rs
path: kei-registry/src/diff.rs
dna_hash: sha256:4fb08a51f1db0ac1
language: rust
size_loc: 61
generated: by-keidocs
---

# kei-registry/src/diff.rs

Facet-by-facet diff between two blocks.

Constructor Pattern: pure data, no I/O. The four compared facets are
`block_type`, `caps`, `scope_sha`, `body_sha` — i.e. the four
identity-bearing inputs to the DNA wire format. `name` is intentionally
NOT diffed (it is a derived label) and `path` is reflected via
`scope_sha` (which is `SHA256(path)`).

## Public API

- Diff result. `differs` lists facet names whose values disagree;
- `pub fn diff_blocks` — Compute the diff. Order of facets is canonical so output is stable

## Related

- parent: `kei-registry/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
