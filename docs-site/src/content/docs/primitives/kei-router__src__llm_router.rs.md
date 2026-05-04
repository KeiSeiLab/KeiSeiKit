---
title: llm_router.rs
path: kei-router/src/llm_router.rs
dna_hash: sha256:cb9f38bec5cb6aad
language: rust
size_loc: 131
generated: by-keidocs
---

# kei-router/src/llm_router.rs

LLM provider router — multi-provider abstraction.

Wave 32 v0.40: holds a registry `name → Box<dyn Provider>` and selects
either by explicit name or by cost given a token estimate.

Stateless per request: the router holds provider configs (api keys + model
+ endpoint), but no conversation state.

## Public API

- `pub fn new` — Empty router — register providers manually.
- `pub fn from_env` — Register all providers whose API key is present in env.
- `pub fn register` — Register one provider; returns the name registered.
- `pub fn pick` — Lookup by stable name. Errors with `UnknownProvider` if unregistered.
- `pub fn names` — Names of registered providers (sorted for stable iteration).
- `pub fn cheapest_for_estimated_tokens` — Cheapest provider for an estimated workload. Cost is computed per-MTok
- Estimate request cost in cents (rounded up to whole cents — we never

## Related

- parent: `kei-router/Cargo.toml`
- imports: async_trait, crate, futures, std

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
