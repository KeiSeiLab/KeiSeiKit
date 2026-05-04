---
title: chat_stream.rs
path: kei-cortex/src/handlers/chat_stream.rs
dna_hash: sha256:026ba2bd7e4f5d6e
language: rust
size_loc: 219
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_stream.rs

Chat SSE-stream wiring — extracted from `chat.rs` so each cube stays
under the 200-LOC ceiling.

Responsibilities: `run_loop_stream` (assemble invoker + ctxs);
`build_event_stream` (translate LoopEvents to SSE + fire post-Done
cost + memory-nudge tasks); `loop_event_to_sse` (single-event
mapper, `pub(super)` so `chat_test.rs` can drive it directly).

Wave 44c (F-HIGH-5): cancel via `CancellationToken` + `CancelOnDrop`
so SSE-client disconnect cancels the agent loop.

Hermes P2.2.b: post-`Done` fires `chat_memory_nudge::spawn_nudge` to
register the (user, assistant) turn pair with the scheduler.

## Public API

- Spawn the agent loop and translate events into SSE frames.
- Compose the post-loop cost-recording context. Extracted from
- RAII guard that fires `cancel.cancel()` on Drop. Replaces the
- Translate `LoopEvent`s into axum SSE events. Client disconnect is
- Side-effects fired after the trailing `Done` event: cost record +
- Fire-and-forget cost record. Errors logged inside `record_chat_cost`;
- Sibling of `spawn_cost_record` — populates the token-event row for
- Map one `LoopEvent` to ≥0 SSE events. `Done` emits sentiment + done.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: async_stream, axum, crate, futures, std, tokio_util, tool

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
