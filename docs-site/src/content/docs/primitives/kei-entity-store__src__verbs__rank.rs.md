---
title: rank.rs
path: kei-entity-store/src/verbs/rank.rs
dna_hash: sha256:6b00b87ffdf36f4e
language: rust
size_loc: 170
generated: by-keidocs
---

# kei-entity-store/src/verbs/rank.rs

`rank` verb — PageRank (power iteration, d=0.85, 50 iter) over the
schema's `edge_table`. Returns `{ results: [{id, score}, ...] }`
sorted by score descending.

Dispatches on `schema.edge_key_kind`: `IntegerPair` emits
`{id: i64, score: f64}` rows; `TextPair` and `TextPairWithMetadata`
emit `{id: String, score}`. For `TextPairWithMetadata` with
`has_weight: true` the rank propagation is proportional to edge
weight (weighted PageRank); otherwise each edge contributes equally.

## Public API

- Adjacency extracted from an integer-keyed edge table:
- Adjacency extracted from a text-keyed edge table:
- Generic weighted PageRank — each edge entry is `(target, weight)`.

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json, std

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
