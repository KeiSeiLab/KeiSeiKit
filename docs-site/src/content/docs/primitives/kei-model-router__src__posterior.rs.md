---
title: posterior.rs
path: kei-model-router/src/posterior.rs
dna_hash: sha256:9cad1ff878e95bdc
language: rust
size_loc: 232
generated: by-keidocs
---

# kei-model-router/src/posterior.rs

Beta posterior over per-(task-class, model) success rate.

For each (task_class_dna, model) pair in the ledger we count:
n+ = rows with outcome='functional' AND escalation_depth=0 (clean wins)
n- = rows with anything else (partial, scaffolding, fail, retry)

Posterior on success probability q ∼ Beta(α₀ + n+, β₀ + n-) with
uniform prior α₀ = β₀ = 1. Confidence-bounded lower estimate
`q_lower(δ)` returned via the inverse-Beta CDF approximation
(Wilson-style normal approx — adequate for our regime where n is
typically small but δ ≈ 0.10).

Constructor Pattern: SQL is one query, math is pure-fn,
`Posterior::from_ledger` is the only DB-touching surface.

## Public API

- `pub fn mean` — Posterior mean q̄ = α / (α + β).
- `pub fn variance` — Variance Var[q] = αβ / ((α+β)² (α+β+1))
- `pub fn quality_lower_bound` — Wilson-style normal-approx lower confidence bound:
- `pub fn observe` — Bayesian update with new observation (success ⇒ α+1, failure ⇒ β+1).
- `pub fn from_ledger` — Build posterior from ledger rows for a given (task_class_dna, model).
- One-sided z-score for confidence (1−δ). Approximates inverse normal

## Related

- parent: `kei-model-router/Cargo.toml`
- imports: crate, rusqlite

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
