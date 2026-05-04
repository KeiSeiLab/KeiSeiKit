---
title: main.rs
path: kei-fork/src/main.rs
dna_hash: sha256:88bb1fbbb2b3543d
language: rust
size_loc: 137
generated: by-keidocs
---

# kei-fork/src/main.rs

kei-fork — CLI dispatcher.

Single responsibility: parse args, dispatch to lib ops, print JSON.
Default `kit_root = std::env::current_dir()`.

## Public API

- Override kit_root (default: current dir).
- Spawn a new managed fork.
- Collect a done fork: commit, merge --no-ff, archive.
- List forks, optionally filtered by status.
- active | done | stale | merged | all
- Prune stale forks (no .DONE and age ≥ --older-than hours).
- Copy a fork's files out of band.

## Related

- parent: `kei-fork/Cargo.toml`
- imports: clap, kei_fork, std

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
