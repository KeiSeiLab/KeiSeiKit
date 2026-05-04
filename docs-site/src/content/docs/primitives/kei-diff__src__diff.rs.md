---
title: diff.rs
path: kei-diff/src/diff.rs
dna_hash: sha256:64bf19f8bf3e68bf
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-diff/src/diff.rs

Structural JSON diff.

Algorithm:
* Both objects → recurse per-key across the union (add/remove/recurse).
* Both arrays → index-based (recurse on overlap; add-tail or remove-tail
for length delta). NOT LCS — simpler, idempotent enough for drift
detection, and cheap (O(n)).
* Otherwise, if values differ → `replace`.
* Equal values → no-op.

Rationale for skipping LCS: the consumer (kei-replay drift check) cares
about "does anything differ" and "at which logical coordinate", not
minimum-edit-distance. Index-based gives stable paths; LCS would produce
a smaller patch on shuffled arrays but with ambiguous paths.

## Public API

- `pub fn diff` — Compute an RFC 6902 subset patch that transforms `old` into `new`.

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
