---
title: commit.rs
path: kei-changelog/src/commit.rs
dna_hash: sha256:3d23b28ce4232f0a
language: rust
size_loc: 77
generated: by-keidocs
---

# kei-changelog/src/commit.rs

Commit model — parsed conventional-commit record.

## Public API

- Conventional-commit kind.
- Anything we do not recognise as conventional.
- Stable ordering for grouping in CHANGELOG.md (lower = earlier).
- Human-facing section heading used in `render::render_markdown`.
- Parsed commit record used by the walker and renderer.

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: std

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
