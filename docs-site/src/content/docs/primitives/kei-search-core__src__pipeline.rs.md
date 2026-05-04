---
title: pipeline.rs
path: kei-search-core/src/pipeline.rs
dna_hash: sha256:7ab5cf3ad8e5619c
language: rust
size_loc: 90
generated: by-keidocs
---

# kei-search-core/src/pipeline.rs

3-wave research runner.

Wave 0: split prompt into claims (naive split on `.`; real NLU later).
Wave 1: for each claim, fetch sources via [`SourceFetcher`].
Wave 2: score consensus per claim from sources (majority = higher grade).

## Related

- parent: `kei-search-core/Cargo.toml`
- imports: anyhow, crate

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
