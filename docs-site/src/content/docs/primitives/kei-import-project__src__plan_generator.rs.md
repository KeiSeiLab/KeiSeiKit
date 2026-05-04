---
title: plan_generator.rs
path: kei-import-project/src/plan_generator.rs
dna_hash: sha256:8402fe6569c3e707
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-import-project/src/plan_generator.rs

plan_generator — cluster map entries into numbered migration phases.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.
Rendering lives in plan_render.rs.

## Public API

- Status of a generated migration phase.
- Matcher confident; orchestrator + agent can implement.
- Low confidence / ambiguous; needs human triage before porting.
- One cluster of modules sharing the same trait family.
- Full output of the plan generator.
- `pub fn build_plan` — Cluster `map_entries` into numbered migration phases.
- `pub fn render_markdown` — Render a `MigrationPlan` to the HERMES-MIGRATION-PLAN.md format.

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
