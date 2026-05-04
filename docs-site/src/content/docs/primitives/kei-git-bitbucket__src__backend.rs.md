---
title: backend.rs
path: kei-git-bitbucket/src/backend.rs
dna_hash: sha256:39478c494da7ebb2
language: rust
size_loc: 154
generated: by-keidocs
---

# kei-git-bitbucket/src/backend.rs

[`BitbucketBackend`] — DNA-bearing [`GitBackend`] impl over [`BitbucketClient`].

`ensure_repo` parses `workspace/slug` from the remote URL path. `clone` and
`push` shell out to the `git` CLI (no libgit2 dep). `mirror` is a
clone-then-push composition.

## Public API

- Parse `workspace/slug` from a Bitbucket remote URL.

## Related

- parent: `kei-git-bitbucket/Cargo.toml`
- imports: crate, kei_runtime_core, std

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
