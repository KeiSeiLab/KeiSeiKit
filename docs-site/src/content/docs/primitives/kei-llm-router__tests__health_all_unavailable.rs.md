---
title: health_all_unavailable.rs
path: kei-llm-router/tests/health_all_unavailable.rs
dna_hash: sha256:e512d9937089afc0
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-llm-router/tests/health_all_unavailable.rs

Test 6 — empty candidate list (every backend down) →
* `decide` returns `Error::NoBackendAvailable`,
* the tried-list mirrors the discovery output (here: `<none>`).

## Related

- parent: `kei-llm-router/tests`
- imports: kei_llm_router, kei_machine_probe

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
