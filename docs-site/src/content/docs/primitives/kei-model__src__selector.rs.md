---
title: selector.rs
path: kei-model/src/selector.rs
dna_hash: sha256:c3fb487980ad21b6
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-model/src/selector.rs

`resolve` — pick cheapest active model for a (role, budget, caps) triple.

Algorithm:
1. Filter to `Status::Active`.
2. Filter to models declaring all required `caps`.
3. Filter to models matching the role tag (or, if no model carries the
tag, fall back to `selectors.toml [defaults]` to pick a target id).
4. Filter by budget (1k input + 1k output baseline cost ≤ budget_micro).
5. Sort by input rate ASC, then output rate ASC.
6. Return the cheapest.

## Public API

- Outcome of `resolve`.
- `pub fn resolve` — Pick the cheapest active model that satisfies role + caps + budget.
- `pub fn resolve_selectors_path` — Resolve selectors.toml: arg → env → compiled-in default.

## Related

- parent: `kei-model/Cargo.toml`
- imports: anyhow, crate, serde, std

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
