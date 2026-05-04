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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
