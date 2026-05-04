---
title: list.rs
path: kei-fork/src/list.rs
dna_hash: sha256:e8f85545ac4482da
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-fork/src/list.rs

`list(kit_root, status_filter)` — enumerate known forks.

Walks two roots:
- `_forks/<id>/` — live worktrees (Active, Done, Stale)
- `_archive/forks/<date>/<id>/` — post-collect (Merged)

For each discovered directory, reads `.KEI_FORK_META.toml` to build
a `ForkHandle`, classifies status, and filters. Returns `Vec` sorted
by `started_ts` ascending so oldest forks list first.

## Public API

- Helper reused by `gc` — enumerate live worktrees with their

## Related

- parent: `kei-fork/Cargo.toml`
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
