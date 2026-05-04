---
title: discovery_no_bins.rs
path: kei-llm-llamacpp/tests/discovery_no_bins.rs
dna_hash: sha256:3cde9b91db424669
language: rust
size_loc: 43
generated: by-keidocs
---

# kei-llm-llamacpp/tests/discovery_no_bins.rs

With both binaries absent, `discover` returns an all-None BinPaths
and `any_found` is false. The Runner is never called because
`locate()` fails first via PATH lookup.

## Related

- parent: `kei-llm-llamacpp/tests`
- imports: kei_llm_llamacpp, std

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
