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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
