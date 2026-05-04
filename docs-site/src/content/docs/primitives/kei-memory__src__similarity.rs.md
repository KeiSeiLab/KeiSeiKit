---
title: similarity.rs
path: kei-memory/src/similarity.rs
dna_hash: sha256:ca30fbf86ed27fe4
language: rust
size_loc: 28
generated: by-keidocs
---

# kei-memory/src/similarity.rs

Cosine similarity over sparse term-weight maps.

Constructor Pattern: one cube, one pure-math responsibility.
Classical numerator = Σ a·b over shared keys;
classical denominator = ‖a‖₂ · ‖b‖₂. No normalize-to-Frobenius, no rank
projection — just textbook cosine on HashMap<String, f64>.

## Public API

- `pub fn cosine_tfidf` — Cosine similarity between two sparse vectors keyed by token.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: std

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
