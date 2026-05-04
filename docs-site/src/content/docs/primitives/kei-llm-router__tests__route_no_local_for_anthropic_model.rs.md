---
title: route_no_local_for_anthropic_model.rs
path: kei-llm-router/tests/route_no_local_for_anthropic_model.rs
dna_hash: sha256:ed59cc35933e9720
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-llm-router/tests/route_no_local_for_anthropic_model.rs

Test 3 — `claude-opus-4-7` + `--require-local` → NoBackendAvailable.

Even though the machine could run local models, NO local backend has
a remote-only model id. With `require_local=true`, route MUST refuse
to walk the registry fallback chain (which would yield Anthropic).

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
