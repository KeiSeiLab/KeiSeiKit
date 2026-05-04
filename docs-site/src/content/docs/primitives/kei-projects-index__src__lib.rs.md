---
title: lib.rs
path: kei-projects-index/src/lib.rs
dna_hash: sha256:ccbe918cfbbbfe40
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-projects-index/src/lib.rs

kei-projects-index — public library surface.

Constructor Pattern: each module is one cube with one responsibility.
`kei-projects-watcher` (sibling daemon) and `kei-cortex` (HTTP daemon)
both depend on this crate's library API to read / write the project
state DB at `~/.claude/agents/projects-index.sqlite`.

## Related

- parent: `kei-projects-index/Cargo.toml`

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
