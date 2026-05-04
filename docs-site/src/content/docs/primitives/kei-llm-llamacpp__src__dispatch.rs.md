---
title: dispatch.rs
path: kei-llm-llamacpp/src/dispatch.rs
dna_hash: sha256:cd67bbf567e00c9f
language: rust
size_loc: 158
generated: by-keidocs
---

# kei-llm-llamacpp/src/dispatch.rs

Dispatch — map a parsed `Cmd` to its concrete behaviour.

Each handler is ≤30 LOC. They share a tiny set of helpers
(json print, error print) so the main.rs entry stays trivial.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: kei_llm_llamacpp, serde, std

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
