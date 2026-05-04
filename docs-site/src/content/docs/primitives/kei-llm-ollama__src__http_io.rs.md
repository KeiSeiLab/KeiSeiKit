---
title: http_io.rs
path: kei-llm-ollama/src/http_io.rs
dna_hash: sha256:0e112192ddba5aef
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-llm-ollama/src/http_io.rs

Low-level HTTP helpers shared by `Client`.

Keeps `client.rs` under the Constructor-Pattern 200-LOC limit by hosting
the per-call decode + status-check primitives here.

## Public API

- Read body, decode JSON, otherwise translate to ApiError.
- `pub fn check_status` — Map non-2xx status to `ApiError`. 404 → `ModelNotFound` (caller path is hint).

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
