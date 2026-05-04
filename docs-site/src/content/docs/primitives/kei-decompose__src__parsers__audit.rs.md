---
title: audit.rs
path: kei-decompose/src/parsers/audit.rs
dna_hash: sha256:555e7714cd00dc30
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-decompose/src/parsers/audit.rs

/wave-audit adapter.

Detects the Wave-audit MD shape:
- `## Wave N` heading (or "Audit Report" / similar)
- `## Priority Matrix` section (table of findings)
- `## Apply Plan` section (actionable next steps)

Extracts each Priority Matrix row as one `Action`. Header columns:
`| # | Severity | Finding | Fix | Complexity | Blast | Score | [E] |`
plus tolerated header variants.

## Public API

- Locate the `## Priority Matrix` heading, then return the index of the

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, regex, std

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
