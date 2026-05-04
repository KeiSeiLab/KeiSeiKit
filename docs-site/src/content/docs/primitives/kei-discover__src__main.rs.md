---
title: main.rs
path: kei-discover/src/main.rs
dna_hash: sha256:a9ed4b2e4bd209fd
language: rust
size_loc: 129
generated: by-keidocs
---

# kei-discover/src/main.rs

kei-discover CLI — register / list / search / install / stats.

Metadata-only: `install` flips the local `installed` flag but does
NOT fetch anything. Real federation (remote index, fetch, signature
verify) arrives in a future wave.

Exit-code contract: 2 for validation / duplicate / not-found, 1 for
storage / IO, 0 on success (matches kei-entity-store convention).

## Related

- parent: `kei-discover/Cargo.toml`
- imports: clap, kei_discover, std

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
