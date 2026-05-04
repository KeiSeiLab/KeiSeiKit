---
title: server_handle_drop_kills.rs
path: kei-llm-llamacpp/tests/server_handle_drop_kills.rs
dna_hash: sha256:e83f77dbad0a8b04
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-llm-llamacpp/tests/server_handle_drop_kills.rs

Spawn a mock-backed ServerHandle, drop it, assert the kill flag flips.
Mock-backed handles flip an `Arc<Mutex<bool>>` instead of sending a
signal; that lets us prove Drop fired without running a real child.

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
