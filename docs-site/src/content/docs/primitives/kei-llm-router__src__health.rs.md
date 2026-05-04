---
title: health.rs
path: kei-llm-router/src/health.rs
dna_hash: sha256:573ac64c0fdadef5
language: rust
size_loc: 115
generated: by-keidocs
---

# kei-llm-router/src/health.rs

Per-backend health check.

Constructor Pattern: ONE responsibility — answer "is this backend up
enough to take a request right now?" for each of the three local
backends. Every check delegates to the underlying W57/W58/W59 crate;
the router never spawns processes itself.

- **Ollama** — HTTP probe (`kei_llm_ollama::is_running`).
- **llama.cpp** — discovery via `kei_llm_llamacpp::discover` (binary present?).
- **MLX** — platform gate first, then discovery.

## Public API

- Outcome of a single backend health probe.
- Short reason — populated whether `available` is true or false.
- Probe Ollama via the W57 HTTP client (1s default timeout).
- Probe llama.cpp via the W58 discovery (binary present on PATH or
- `pub fn check_mlx` — Probe MLX — combined platform gate + binary discovery.
- Health-check ALL three backends in parallel. Returned in the

## Related

- parent: `kei-llm-router/Cargo.toml`
- imports: crate, serde

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
