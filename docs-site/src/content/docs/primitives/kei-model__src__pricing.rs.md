---
title: pricing.rs
path: kei-model/src/pricing.rs
dna_hash: sha256:adf625f1e6e9be89
language: rust
size_loc: 96
generated: by-keidocs
---

# kei-model/src/pricing.rs

`Pricing` struct + `estimate` cost helper.

Every pricing row in the SSoT TOML ships with `status = "placeholder"` and
a `source_url` per RULE 0.4. Real micro-cents/Mtok come from a follow-up
verification commit. Callers should inspect `status` before quoting cost.

## Public API

- Micro-cents per million input tokens.
- Micro-cents per million output tokens.
- ISO-8601 date the row was last verified, e.g. "2026-04-27".
- `pub fn estimate` — Estimate total micro-cents for a (`in_tokens`, `out_tokens`) call.
- `(a * b) / d` with saturating multiply, integer floor.

## Related

- parent: `kei-model/Cargo.toml`
- imports: serde

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
