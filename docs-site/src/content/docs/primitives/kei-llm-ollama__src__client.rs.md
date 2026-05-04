---
title: client.rs
path: kei-llm-ollama/src/client.rs
dna_hash: sha256:4c1cbcf1748a8a4c
language: rust
size_loc: 187
generated: by-keidocs
---

# kei-llm-ollama/src/client.rs

HTTP client wrapping the Ollama daemon.

Default base URL is `http://127.0.0.1:11434` — local-only by design.
See `<https://github.com/ollama/ollama/blob/main/docs/api.md>` for schema.

## Public API

- Thin wrapper around `reqwest::Client` aimed at the Ollama daemon.
- `pub fn new` — New client. `base_url` should be `http://host:port` (no trailing slash).
- `GET /api/tags` — list installed models.
- `GET /api/tags` with explicit per-call timeout.
- `GET /api/version` with explicit per-call timeout (used by health probe).
- `POST /api/generate` — non-streaming.
- `POST /api/chat` — non-streaming.
- `POST /api/generate` — streaming. No timeout (Ollama generation can be slow).
- `POST /api/chat` — streaming. No timeout.
- `POST /api/pull` — model download. Returns raw NDJSON bytes-stream.
- `DELETE /api/delete` — remove an installed model.
- `POST /api/show` — model details (raw JSON value).

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: bytes, crate, futures, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
