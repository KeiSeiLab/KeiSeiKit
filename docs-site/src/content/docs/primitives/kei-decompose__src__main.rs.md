---
title: main.rs
path: kei-decompose/src/main.rs
dna_hash: sha256:a6b51168bafadfd3
language: rust
size_loc: 181
generated: by-keidocs
---

# kei-decompose/src/main.rs

kei-decompose CLI entry — clap dispatch only.

All real work lives in module entrypoints. This file's only job is to
convert clap matches → module call → exit code.

Exit codes:
0  success
1  file/IO error
2  no parser detected / parse error
3  kei-spawn invocation failed

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: clap, kei_decompose, std

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
