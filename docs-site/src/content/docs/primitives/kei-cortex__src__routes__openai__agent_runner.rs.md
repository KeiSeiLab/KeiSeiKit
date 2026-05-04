---
title: agent_runner.rs
path: kei-cortex/src/routes/openai/agent_runner.rs
dna_hash: sha256:09794c6e8fb8f84e
language: rust
size_loc: 274
generated: by-keidocs
---

# kei-cortex/src/routes/openai/agent_runner.rs

Agent runner — bridges the OpenAI-compat surface (`/v1/*`) to the
real kei-cortex agent loop in `tool::run_with_tools`.

Replaces the Phase-1.1 `stub_agent_reply` with two adaptors:

* `collect_reply`  — sync (non-stream) drain. Runs the loop to
completion and concatenates every `LoopEvent::AssistantText`
into a single string. Used by the JSON handlers.
* `stream_events`  — async streaming. Spawns the loop on a
tokio task and returns the raw `LoopEvent` receiver so the
stream-forwarder can translate per-event into SSE.

Constructor Pattern: this cube owns ONE responsibility — wiring
the agent loop. SSE serialisation lives in `stream_forwarder.rs`;
the loop itself lives in `tool::loop_driver`.

## Public API

- `pub const EVENT_CHANNEL_CAPACITY` — Channel capacity for the loop → forwarder pipe.
- Synchronous (non-stream) drain: run the agent loop to completion
- Translate kei-cortex's TokenUsage into OpenAI Usage shape.
- `pub fn snapshot_usage` — Snapshot the streaming TokenUsage accumulator and translate to the
- `pub fn stream_events_with_tracking` — Streaming variant with token-event recording. Spawns the agent loop on a
- Construct the agent-loop event stream with caller-provided usage
- Drain the loop stream into the bounded mpsc. Stops on first send
- `pub fn record_post_turn_token_event` — Fire one [`TokenWrite`] into the AppState's token-tracker after a
- Local `conversation_id` for one-shot chat-completions calls. The
- H-2 contract: TokenUsage with input/output_tokens translates to
- H-2 contract: snapshot_usage round-trips through Arc<Mutex<TokenUsage>>.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, futures, std, tokio, tokio_util

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
