---
title: run_agent.rs
path: kei-cortex/src/routes/openai/run_agent.rs
dna_hash: sha256:c566dc195a465671
language: rust
size_loc: 212
generated: by-keidocs
---

# kei-cortex/src/routes/openai/run_agent.rs

Real agent loop driving `/v1/runs` via `agent_runner::stream_events`.

P1.1.d (2026-04-28): replaces the Phase-1.1 `run_stub` with a wired
loop that drains `tool::LoopEvent`s into `AgentChunk`s the SSE
handler streams to the client.

Lifecycle marks on the `RunRegistry`:
* `in_progress` on the FIRST `LoopEvent::AssistantText` (or first
`Delta` chunk that escapes the translator).
* `cancelled` if the cancel token fires before the loop completes.
* `completed` when `LoopEvent::Done` lands (loop's natural exit).

Cancel is honoured by passing the same `CancellationToken` into
`agent_runner::stream_events` — the loop's own `select!` against
the in-flight invoker (Wave 44c) terminates the run within
milliseconds of `/v1/runs/{id}/stop`.

## Public API

- Default system prompt used when the request does not include a
- Mutable per-run drain bookkeeping. Tracks whether the loop already
- The real agent. Spawned by `runs::create_run` immediately after the
- Drain the loop's `LoopEvent` stream into `AgentChunk` SSE frames,
- Update the registry / drain bookkeeping based on the next loop event.
- Forward translated chunks to the SSE channel. Returns `Err` if the
- Mark the registry final state. If the loop never reached `Done`
- Pull the first `role: system` message out, falling back to the
- Concatenate `system` + `user` content into a single prompt for the

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, tokio, tokio_util

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
