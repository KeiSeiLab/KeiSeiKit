---
title: kw_tables.rs
path: kei-router/src/kw_tables.rs
dna_hash: sha256:6c2e2cefe413b971
language: rust
size_loc: 197
generated: by-keidocs
---

# kei-router/src/kw_tables.rs

Per-domain keyword rule tables. Split from `keywords.rs` for Constructor
Pattern <200 LOC compliance. Each table is a `const` slice so the whole
router is built at compile time — zero allocation hot-path.

## Related

- parent: `kei-router/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
