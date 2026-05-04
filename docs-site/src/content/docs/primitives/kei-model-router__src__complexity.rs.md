---
title: complexity.rs
path: kei-model-router/src/complexity.rs
dna_hash: sha256:c9cdc160cbf14d9f
language: rust
size_loc: 194
generated: by-keidocs
---

# kei-model-router/src/complexity.rs

Task-complexity heuristic.

Maps (prompt, role) → τ ∈ [0, 1] via additive feature scoring. Pure
function, no LLM call. Fast classifier so router itself has near-zero
overhead.

Calibration: weights are seeded from session observation; the
`calibrate` subcommand can re-fit them against ledger outcomes.

Design: every signal contributes a clamped weight; total weight
divided by maximum-possible-weight gives τ. Returns matched feature
list for transparency / debugging.

Constructor Pattern: pure-fn cube. No state, no I/O.

## Public API

- Tier mapping for human consumption: τ ∈ [0, 0.30] = lookup,
- High-complexity signals — bump τ up. Weight 0.20 each.
- Mid-complexity signals — bump τ up. Weight 0.10 each.
- Low-complexity signals — bump τ DOWN. Weight 0.10 each (negative).
- Roles known to require architectural reasoning. Add 0.20 to τ if matched.
- Roles known to be read-only / lookup. Subtract 0.20 from τ.
- Empirical thresholds — prompt length signals.

## Related

- parent: `kei-model-router/Cargo.toml`

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
