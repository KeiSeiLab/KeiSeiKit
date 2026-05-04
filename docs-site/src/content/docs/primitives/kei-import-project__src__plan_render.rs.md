---
title: plan_render.rs
path: kei-import-project/src/plan_render.rs
dna_hash: sha256:96df84224c493922
language: rust
size_loc: 135
generated: by-keidocs
---

# kei-import-project/src/plan_render.rs

plan_render — markdown renderer for MigrationPlan.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- `pub fn render_markdown` — Render a `MigrationPlan` to the HERMES-MIGRATION-PLAN.md format.
- Convert Unix epoch seconds to an ISO-8601 UTC string.

## Related

- parent: `kei-import-project/Cargo.toml`
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
