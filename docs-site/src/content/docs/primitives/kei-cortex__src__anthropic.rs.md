---
title: anthropic.rs
path: kei-cortex/src/anthropic.rs
dna_hash: sha256:5a32e29e317bc5ea
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-cortex/src/anthropic.rs

Thin reqwest client for Anthropic Messages API (streaming mode).

`open_stream` performs the HTTP handshake synchronously then returns a
`Stream<Item = Result<String, Error>>` of text deltas extracted from
`content_block_delta` frames. Non-text events are skipped.
API key is read from `ANTHROPIC_API_KEY` at call time (env rotation-friendly).

Reliability envelope:
- `BUDGET` (120 s) caps the HTTP handshake (`open_stream`).
- `IDLE` (30 s) caps the gap between individual SSE chunks; silent
streams are surfaced as `Error::Timeout` so the handler can emit
an SSE error event rather than hanging the client.

## Public API

- `pub const BUDGET` — Overall HTTP-handshake budget. Past this point we give up even if the
- `pub const IDLE` — Per-chunk idle budget. If the stream goes silent for this long we bail
- Cap on upstream error bodies we propagate. Prevents Anthropic echoing a
- Cap on upstream error body reads via `read_capped` (16 KiB).
- A single turn in the conversation.
- Client errors surfaced to the caller.
- Open a streaming Messages request. Returns the async stream of text deltas
- Turn a validated streaming response into a text-delta stream.
- Build the JSON request body.
- Fire the POST request with the right headers; map HTTP errors to `Error`.
- Turn non-2xx responses into structured `Error` values. 429 → RateLimit,
- Cap a string at `max` bytes on a char boundary. Used for error previews

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: async_stream, crate, futures, serde, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
