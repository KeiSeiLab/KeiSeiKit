---
title: render.rs
path: kei-changelog/src/render.rs
dna_hash: sha256:08cc775097e80ea7
language: rust
size_loc: 74
generated: by-keidocs
---

# kei-changelog/src/render.rs

Render a `Grouped` set of commits as a CHANGELOG.md section.

## Public API

- Options governing the rendered section.
- Heading for the version block, e.g. "v0.7.0" or "Unreleased".
- Optional release date. If `None`, uses today (UTC).
- If true, include short (7-char) SHA suffix on each line.
- Render the grouped commits into markdown. Returns an empty string if the

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: chrono, crate

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
