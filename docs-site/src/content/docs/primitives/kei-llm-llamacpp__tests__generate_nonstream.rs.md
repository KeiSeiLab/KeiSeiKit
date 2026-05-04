---
title: generate_nonstream.rs
path: kei-llm-llamacpp/tests/generate_nonstream.rs
dna_hash: sha256:ad6c25165221ad6d
language: rust
size_loc: 38
generated: by-keidocs
---

# kei-llm-llamacpp/tests/generate_nonstream.rs

MockRunner returns fake llama-cli stdout (token answer + timing
footer in stderr). Asserts Response.text + tokens_per_sec parse out
of the combined buffer.

## Related

- parent: `kei-llm-llamacpp/tests`
- imports: kei_llm_llamacpp

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
