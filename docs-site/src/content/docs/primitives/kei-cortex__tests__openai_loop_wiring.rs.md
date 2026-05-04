---
title: openai_loop_wiring.rs
path: kei-cortex/tests/openai_loop_wiring.rs
dna_hash: sha256:a67fb178c72ad7ed
language: rust
size_loc: 402
generated: by-keidocs
---

# kei-cortex/tests/openai_loop_wiring.rs

Hermes Phase 1.1.b wiring witnesses for `/v1/chat/completions`.

These tests prove the OpenAI-compat surface drives the REAL agent
loop (`tool::run_with_tools` via `agent_runner::collect_reply` /
`agent_runner::stream_events`) and NOT the legacy `stub_agent_reply`
placeholder kept around as deprecated dead code in `chat_helpers.rs`.

Approach: divert Anthropic upstream traffic to `shared_mock_anthropic`
which always returns the canned text `"hi"`. If the response carries
`[kei-cortex stub] echo:` then the loop was bypassed; if it carries
`hi` then the loop ran end-to-end.

Constructor Pattern: a sibling test cube to `openai_compat.rs` so each
file stays focused (router shape there, loop wiring here).

## Public API

- Sync /v1/chat/completions — response carries the mock's "hi" and NOT
- Streaming /v1/chat/completions — SSE body carries `delta` chunks fed
- Streaming /v1/chat/completions — body MUST be chunked across multiple
- /v1/responses — sync mode — response.output[0].text carries the
- /v1/responses — stream mode — SSE body carries `response.output_text.delta`
- /v1/runs — POST returns 202 + run id, GET events SSE carries the
- /v1/runs/{id}/events second subscriber — once the SSE stream has been

## Related

- parent: `kei-cortex/tests`
- imports: axum, kei_cortex, std, tower

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
