---
title: escalate.rs
path: kei-model-router/src/escalate.rs
dna_hash: sha256:bcf044c8d245b755
language: rust
size_loc: 96
generated: by-keidocs
---

# kei-model-router/src/escalate.rs

Retry-ladder bookkeeping for the router.

When a model returns `outcome != functional` on first pass, we may
want to retry on the next-tier model (Haiku → Sonnet → Opus). The
escalation depth is recorded in the ledger row so future posterior
aggregation discounts retries.

Constructor Pattern: pure-fn cube, no I/O. Side effects (writing the
depth back to ledger) happen in caller / hook.

## Public API

- `pub const MAX_ESCALATION_DEPTH` — Hard ceiling on escalation depth. Two retries (depth 1 and 2) gives
- Retry on the next-tier model.
- No more tiers above OR depth ceiling reached. Caller should
- `pub fn next_after_failure` — Decide whether to retry given (current_model, current_depth, outcome).

## Related

- parent: `kei-model-router/Cargo.toml`
- imports: crate

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
