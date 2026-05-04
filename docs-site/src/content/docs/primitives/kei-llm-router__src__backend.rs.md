---
title: backend.rs
path: kei-llm-router/src/backend.rs
dna_hash: sha256:9e67fd4465e3a2b8
language: rust
size_loc: 85
generated: by-keidocs
---

# kei-llm-router/src/backend.rs

`Backend` enum + `BackendKind` tag.

Constructor Pattern: ONE responsibility — represent the three concrete
local-LLM execution targets the router can pick.

- `Mlx` — Apple Silicon native (mlx_lm shell-out).
- `LlamaCpp` — local llama.cpp binary + .gguf file on disk.
- `Ollama` — Ollama daemon at `127.0.0.1:11434`.

## Public API

- Concrete backend selection — each variant carries the data the
- `pub fn kind` — Strip variant data, return just the kind tag.
- `pub fn identifier` — Identifier used by the caller (model id, tag, or filename).
- Variant tag without payload — used in discovery output, health
- `pub fn from_tier` — Map a `kei_machine_probe::BackendTier` to the router's `BackendKind`.

## Related

- parent: `kei-llm-router/Cargo.toml`
- imports: serde, std

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
