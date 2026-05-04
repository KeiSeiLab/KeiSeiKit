---
title: loop_driver.rs
path: kei-cortex/src/tool/loop_driver.rs
dna_hash: sha256:aa4ce7d74e30cfb5
language: rust
size_loc: 196
generated: by-keidocs
---

# kei-cortex/src/tool/loop_driver.rs

Agentic loop — orchestrates model ↔ tool turns until termination.

Each turn: invoke model → emit text events → dispatch tool calls →
loop. Terminates on `stop_reason != "tool_use"`, on `MAX_TURNS`, or
when the cancel token fires.

Wave 44c (2026-04-24, F-HIGH-5): cancel moved from
`oneshot::Receiver<()>` to `CancellationToken` and is `select!`'d
against the in-flight invoker so long-running tool turns cancel
within milliseconds, not after the turn completes.

## Public API

- `pub const MAX_TURNS` — Hard cap on turns. Past this we abort with an error event so the user
- One block in a model response.
- Token usage reported by the provider alongside the model response.
- One model turn (its content + the reason it stopped).
- Tokens reported by the provider. `None` when the provider didn't
- `pub type ModelInvoker` — Boxed async invoker. The orchestrator wires the real Anthropic call;
- One entry in our local conversation. `Tool` rows carry the `tool_result`
- Events streamed to the SSE client.
- One turn's outcome. Visible to the `dispatch` cube.
- Text-only; loop terminates after emitting it.
- Tool calls present; loop dispatches and continues.
- Invoker errored; loop terminates.
- Bundle of state captured by the loop coroutine. Exists only to keep
- Drives the turn loop, emits per-turn events, terminates on
- Invoke the model and classify the response into a `TurnOutcome`.
- Pull `ToolUse` blocks out of a content list, cloning so the caller

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: async_stream, futures, serde_json, std, tokio_util

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
