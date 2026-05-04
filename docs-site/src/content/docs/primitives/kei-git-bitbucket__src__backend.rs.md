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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
