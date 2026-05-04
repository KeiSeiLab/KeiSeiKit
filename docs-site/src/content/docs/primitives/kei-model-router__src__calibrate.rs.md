---
title: calibrate.rs
path: kei-model-router/src/calibrate.rs
dna_hash: sha256:387a4939661ca53d
language: rust
size_loc: 208
generated: by-keidocs
---

# kei-model-router/src/calibrate.rs

Offline calibration of kernel weights from observed ledger outcomes.

Goal: re-fit (α_role, α_caps, α_scope, α_body) so that predicted
posterior mean q̂(d, m) tracks the ACTUAL post-hoc success rate of
similar past task-classes.

Approach: leave-one-out on each ledger row. For row i with full DNA
d_i, model m_i, observed outcome ω_i, compute the kernel-smoothed
prediction q̂_{-i}(d_i, m_i) using all OTHER rows. The residual
(ω_i − q̂_{-i}) measures bias; the weights that minimize sum of
squared residuals are the calibrated weights.

For initial seed weights this implementation uses a coarse grid
search over weight tuples (5 levels × 4 dims = 625 configs) — small
enough to brute force on the typical ledger size (≤10k rows).

Constructor Pattern: pure-fn cube; no I/O outside passing in a
Connection. Caller (CLI subcommand) decides where to print results.

## Public API

- Leave-one-out kernel-weighted mean prediction for observation i.

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
