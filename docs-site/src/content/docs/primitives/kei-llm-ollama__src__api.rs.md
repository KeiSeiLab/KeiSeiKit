---
title: api.rs
path: kei-llm-ollama/src/api.rs
dna_hash: sha256:67dd8d1ab7611369
language: rust
size_loc: 141
generated: by-keidocs
---

# kei-llm-ollama/src/api.rs

Ollama HTTP API wire types.

Schema source: <https://github.com/ollama/ollama/blob/main/docs/api.md>
Pinned against Ollama v0.x — schema is stable across patch releases.
Verified live against running daemon v0.21.2 at 127.0.0.1:11434.

## Public API

- Chat message — matches `/api/chat` and `/api/generate` schema.
- `/api/tags` GET response.
- One installed model in the tags list.
- Optional details block emitted by Ollama for installed models.
- `/api/generate` POST request.
- `/api/generate` non-stream response.
- `/api/chat` POST request.
- `/api/chat` non-stream response.
- `/api/pull` POST progress line.
- `/api/version` GET response (used by health probe).
- `pub fn build_options` — Build options object for generate/chat from CLI flags.

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: serde

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
