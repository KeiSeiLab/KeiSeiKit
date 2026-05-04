---
title: architecture.rs
path: kei-decompose/src/parsers/architecture.rs
dna_hash: sha256:9102377cfd7d7923
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-decompose/src/parsers/architecture.rs

/architecture decisions adapter.

Detects:
- `## Decision` heading (single architectural decision file)
- `## Recommendation` / `## Recommendations` section

Extracts:
- Numbered recommendations under the recommendations heading
(e.g. `1. Adopt X`, `2. Refactor Y`) — one Action per item.

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
