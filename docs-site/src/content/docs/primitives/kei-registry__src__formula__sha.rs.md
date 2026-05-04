---
title: sha.rs
path: kei-registry/src/formula/sha.rs
dna_hash: sha256:0a4b54146101a65b
language: rust
size_loc: 42
generated: by-keidocs
---

# kei-registry/src/formula/sha.rs

Canonical short-sha8 of a `BlockFormula` (excluding `block_id`).

Constructor Pattern: hashing is one cube. The canonical serialisation
uses `serde_json` over the four formula facets in the canonical order
`(type, invariants, effects, deps)`. `effects` and `deps` are
`BTreeSet`, so their iteration order is sorted; `invariants` order is
the author's chosen order — predicates are positional by design.

## Public API

- `pub fn formula_sha` — Compute the deterministic 8-hex-char digest of the formula's contents.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: sha2

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
