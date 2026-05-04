---
title: health_down.rs
path: kei-llm-ollama/tests/health_down.rs
dna_hash: sha256:ad59e0d9add6f3ef
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-llm-ollama/tests/health_down.rs

`is_running` returns `false` when nothing listens on the given URL.

We bind a TCP listener on an ephemeral loopback port, drop it (releasing the
port), and point the client at that now-unbound URL. The connection refuses
immediately on every modern OS — we don't have to wait for any timeout.

## Related

- parent: `kei-llm-ollama/tests`
- imports: kei_llm_ollama, std

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
