---
title: sleep.rs
path: kei-decompose/src/parsers/sleep.rs
dna_hash: sha256:d8bfbd90cfd8d479
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-decompose/src/parsers/sleep.rs

Sleep-layer report adapter (RULE 0.15 Phase B/C output).

Detects:
- Frontmatter / commit refs containing `REM:` or `NREM:`
- `## Patterns` section (cross-session pattern listing)
- `## Backlog` section (open follow-ups)

Extracts:
- `- [ ] action` checklist items as Actions
- Pattern rows under `## Patterns` (one Action per pattern)

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
