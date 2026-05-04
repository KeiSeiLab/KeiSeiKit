---
title: rules.rs
path: kei-router/src/rules.rs
dna_hash: sha256:3993c0c8fc9a5cd3
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-router/src/rules.rs

Keyword rule type + `require` predicate model.

## Public API

- A dispatch rule: any matching keyword routes to `tool` if `require(extracted)` is true.
- A dynamic (runtime-added) rule — owned strings so caller can build at startup.

## Related

- parent: `kei-router/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
