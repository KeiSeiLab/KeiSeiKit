---
title: lib.rs
path: kei-projects-watcher/src/lib.rs
dna_hash: sha256:5bdc4bc72bf11fd2
language: rust
size_loc: 19
generated: by-keidocs
---

# kei-projects-watcher/src/lib.rs

kei-projects-watcher — fsevents daemon that keeps `kei-projects-index`
fresh by watching `~/Projects/` and debouncing 2 s before re-indexing
each touched project root.

The library surface (this file) exposes the watcher + debounce types
plus CLI subcommand bodies so the binary stays under the
Constructor-Pattern file budget.

Constructor Pattern: every concrete type lives in its own module
(`watcher`, `debounce`, `cli`); this file ONLY declares modules and
re-exports the public API.

## Related

- parent: `kei-projects-watcher/Cargo.toml`

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
