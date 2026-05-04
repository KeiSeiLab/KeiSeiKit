---
title: support.rs
path: kei-arch-map/tests/support.rs
dna_hash: sha256:7f97223abed61094
language: rust
size_loc: 33
generated: by-keidocs
---

# kei-arch-map/tests/support.rs

Thin wrappers around the library's evidence::* checkers, kept here so
`tests/evidence.rs` reads cleanly. Each wrapper accepts string-ish args
and forwards to the matching `kei_arch_map::evidence::*::check`.

## Related

- parent: `kei-arch-map/tests`
- imports: std

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
