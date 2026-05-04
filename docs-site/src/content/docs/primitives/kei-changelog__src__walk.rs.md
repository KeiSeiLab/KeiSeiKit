---
title: walk.rs
path: kei-changelog/src/walk.rs
dna_hash: sha256:76c9b34bcd35f8e8
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-changelog/src/walk.rs

git2 walker — collect commits between two refs.

## Public API

- Range specification passed in from CLI.
- `pub fn walk_range` — Walk commits in topological order (newest first) from `to` back to `from`.

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: anyhow, crate, git2

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
