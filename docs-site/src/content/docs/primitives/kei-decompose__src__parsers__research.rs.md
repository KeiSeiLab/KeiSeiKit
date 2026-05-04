---
title: research.rs
path: kei-decompose/src/parsers/research.rs
dna_hash: sha256:b0a2bb816d3f8238
language: rust
size_loc: 193
generated: by-keidocs
---

# kei-decompose/src/parsers/research.rs

/research MASTER-REPORT.md adapter.

Mirrors the kei-decision parser shape (Wave 51): scans for an
`## Actionable plan` / `## Backlog` / `## Action items` section, then
parses the markdown table that follows. The trait wrapper lets the
registry treat it as one adapter among many.

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
