---
title: apply.rs
path: kei-diff/src/apply.rs
dna_hash: sha256:a862913cddef548b
language: rust
size_loc: 169
generated: by-keidocs
---

# kei-diff/src/apply.rs

Apply an RFC 6902 patch (add/remove/replace subset) to a JSON document.

Root-path `""` replace swaps the entire document. Array `add` with
index == len (or `-`) appends; in-range index inserts and shifts.
Array `remove` deletes and shifts. Object ops insert/delete/replace keys.

## Public API

- `pub fn apply` — Apply `patch` to `root` and return a new `Value`. `root` is cloned;

## Related

- parent: `kei-diff/Cargo.toml`
- imports: crate, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
