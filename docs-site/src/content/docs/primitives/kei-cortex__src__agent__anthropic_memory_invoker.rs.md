---
title: anthropic_memory_invoker.rs
path: kei-cortex/src/agent/anthropic_memory_invoker.rs
dna_hash: sha256:4d9889678799e251
language: rust
size_loc: 157
generated: by-keidocs
---

# kei-cortex/src/agent/anthropic_memory_invoker.rs

Anthropic-backed implementation of the memory-review `Invoker` trait.

Constructor Pattern: this cube owns ONE responsibility — execute the
review prompt against Anthropic Messages and return the assistant
reply as a single `String`. No tool-use, no streaming. Tools are
intentionally absent: the review prompt forbids further actions and
either emits `Nothing to save.` or a short paragraph. Adding tool
support here would re-introduce the loop driver and break the
≤200-LOC budget. The persistence step happens in `memory_persist`.

Wiring contract:
* Caller provides a `system` string (typically `REVIEW_PROMPT`).
* Each `invoke()` call snapshot-renders the conversation as
`Message` rows and POSTs once to `/v1/messages`.
* Errors (network, 4xx/5xx, missing API key) collapse to a stable
short string so the scheduler can log without panicking.

Tradeoff: one HTTP call per review (~500ms tail). The scheduler
cool-down (60s) caps cost; reviews fire ≤1/min/session.

## Public API

- HTTP budget for a single review call. Mirrors the streaming-handshake
- `pub struct AnthropicMemoryInvoker` — Concrete Invoker that POSTs to Anthropic Messages. Captures the
- `pub fn new` — Build a new invoker bound to a fixed system/review prompt.
- Single round-trip to Anthropic. Errors collapse to a stable
- JSON body for the review request. Snapshot turns are mapped to
- POST and extract the first text block from the response. Any HTTP
- Walk `content` looking for the first `{"type":"text","text":...}`
- `pub const ERROR_PREFIX` — Stable prefix the persist step can detect via `is_error_reply` so a
- `pub fn is_error_reply` — True when the reply is a transport-error placeholder rather than

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, serde_json, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
