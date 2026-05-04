---
title: lib.rs
path: kei-hibernate/src/lib.rs
dna_hash: sha256:712edf7875d98c9a
language: rust
size_loc: 23
generated: by-keidocs
---

# kei-hibernate/src/lib.rs

kei-hibernate — whole-brain export/import for KeiSeiKit state.

Wave 14 primitive. Serialises an entire KeiSei installation
(sqlite stores + capabilities / roles / blocks / agents /
skills / hooks) into a single tar.zst bundle with a SHA-256
manifest, then restores it on another machine.

Public surface kept deliberately small: `export`, `import`,
`inspect`. Each dispatches to a Constructor-Pattern cube.

## Related

- parent: `kei-hibernate/Cargo.toml`

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
