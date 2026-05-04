---
title: group.rs
path: kei-changelog/src/group.rs
dna_hash: sha256:4c914b537e680696
language: rust
size_loc: 39
generated: by-keidocs
---

# kei-changelog/src/group.rs

Group commits by kind, preserving insertion order within each bucket.

## Public API

- Commits grouped by `CommitKind`, sorted by `CommitKind::sort_key`.
- Build a `Grouped` from an ordered slice of commits.

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: crate, std

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
