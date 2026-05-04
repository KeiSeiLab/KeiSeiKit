---
title: genealogy.rs
path: kei-runtime-core/src/genealogy.rs
dna_hash: sha256:9d0b7a4f670ea15f
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-runtime-core/src/genealogy.rs

[`HasGenealogy`] — trait for entities that can walk their parent
chain. Combined with [`HasDna`], gives every entity a queryable
genealogy from itself back to the root user signup.

## Public API

- An entity whose ancestors can be looked up. Implementations typically
- Walk parents from immediate up to root. Empty when self is root.
- Convenience: ultimate ancestor. Returns self's DNA when self is
- Return depth from root. 0 = self IS root.

## Related

- parent: `kei-runtime-core/Cargo.toml`
- imports: crate

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
