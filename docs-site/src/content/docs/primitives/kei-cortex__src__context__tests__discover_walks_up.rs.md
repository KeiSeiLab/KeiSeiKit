---
title: discover_walks_up.rs
path: kei-cortex/src/context/tests/discover_walks_up.rs
dna_hash: sha256:7e98e562293b2abf
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-cortex/src/context/tests/discover_walks_up.rs

Discover walks from `cwd` upward, returning every CLAUDE.md / AGENTS.md
it finds in nearest-first order.

Setup: tempdir -> a/b/c/d/e (5 levels). Plant CLAUDE.md at e (innermost),
at c (mid), at a (outermost). Run discover from e. Expect three hits in
the order [e, c, a].

Symlink-safety: also plant a symlink CLAUDE.md and assert it's skipped.

## Related

- parent: `kei-cortex/Cargo.toml`
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
