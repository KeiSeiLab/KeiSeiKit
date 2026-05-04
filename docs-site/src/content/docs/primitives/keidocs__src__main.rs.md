---
title: main.rs
path: keidocs/src/main.rs
dna_hash: sha256:07319d46a4d40202
language: rust
size_loc: 165
generated: by-keidocs
---

# keidocs/src/main.rs

keidocs CLI — extract / validate per-file markdown docs.

## Public API

- Walk source tree under --root and emit one .md per source file in --out.
- Verify each .md in --out has dna_hash + parent backlink.

## Related

- parent: `keidocs/Cargo.toml`
- imports: anyhow, clap, keidocs, std, walkdir

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
