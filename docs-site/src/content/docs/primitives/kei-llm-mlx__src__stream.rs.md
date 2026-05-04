---
title: stream.rs
path: kei-llm-mlx/src/stream.rs
dna_hash: sha256:bf8176191e3cb921
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-llm-mlx/src/stream.rs

Streaming generate — line-delimited NDJSON chunks.

mlx_lm `--stream` (or its `--stream-format json` flavour) prints one
JSON object per generated token: `{"delta": "X", "tokens_so_far": N}`,
optionally followed by a final `{"done": true, ...}` marker.

Constructor Pattern: this cube parses chunks from already-captured
stdout. Live streaming over a pipe is the consumer's job (or a
follow-up cube). The parser tolerates non-JSON warning lines (skipped
with no error) so mlx_lm logs interleaved with NDJSON do not break us.

## Public API

- Token text appended in this step (empty on a `done`-only marker).
- True only on the terminal marker.
- Cumulative token count up to and including this chunk.
- `pub fn parse_stream` — Parse a multi-line NDJSON stdout into ordered `Chunk`s. Non-JSON
- `pub fn concat_chunks` — Concatenate every chunk's `delta` into the final text. Convenience

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
