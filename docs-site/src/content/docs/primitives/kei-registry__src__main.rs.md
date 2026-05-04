---
title: main.rs
path: kei-registry/src/main.rs
dna_hash: sha256:0912beb5e5cf029c
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-registry/src/main.rs

kei-registry binary entry point.

Constructor Pattern: this file does ONE thing — parse CLI args and
dispatch to `handlers::dispatch`. All policy lives in the library.
Exit codes per spec: 0 success, 1 IO error, 2 not-found, 3 schema mismatch.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: clap, kei_registry

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
