---
title: error.rs
path: kei-llm-ollama/src/error.rs
dna_hash: sha256:e01ad8b67b5a7de3
language: rust
size_loc: 66
generated: by-keidocs
---

# kei-llm-ollama/src/error.rs

Error type for the Ollama HTTP adapter.

Maps to deterministic exit codes:
- 0 success
- 1 IO/decode/transport error
- 2 daemon-not-running OR model-not-found
- 3 timeout

## Public API

- Connection refused on localhost:11434 — Ollama daemon is not up.
- 404 from /api/generate or /api/chat — model not pulled.
- Non-2xx status that is not a 404.
- JSON decode failure on response body.
- Network or library-level transport error (not a connection refused).
- Request exceeded timeout budget.
- `pub fn exit_code` — Map error to process exit code.
- `pub fn classify_reqwest_error` — Classify a reqwest error: connection-refused → DaemonNotRunning, timeout → Timeout, else Transport.

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: thiserror

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
