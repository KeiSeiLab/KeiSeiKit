---
title: types.rs
path: kei-tty/src/types.rs
dna_hash: sha256:5700ea00ebd75e5f
language: rust
size_loc: 132
generated: by-keidocs
---

# kei-tty/src/types.rs

Wire types shared between the client and the UI.

`ChatEvent` mirrors the SSE frames emitted by
`kei-cortex::handlers::chat`. The cortex daemon currently emits four kinds
of events: `token`, `sentiment`, `error`, `done`. Two additional variants
(`ToolUseStart`, `ToolResult`) are reserved here for forward-compat with
the upcoming tool-use streaming pipeline; an unknown event tag yields
`ChatEvent::Other` rather than a parse error so the UI keeps draining.

## Public API

- One parsed SSE event from `/api/v1/cortex/pet/:user_id/chat`.
- Incremental text delta from the model.
- Final sentiment classification (post-stream).
- Server-side tool invocation has started (forward-compat).
- Server-side tool invocation completed (forward-compat).
- Mid-stream error frame; UI surfaces it red and clears `in_flight`.
- Stream finished cleanly. Carries the conversation_id for resume.
- Unknown event tag (forward-compat); UI logs but does not panic.
- `pub fn parse_event` — Parse a single `data: { ... }` payload (without the `data:` prefix and
- Outgoing chat request body (`POST .../chat`).

## Related

- parent: `kei-tty/Cargo.toml`
- imports: serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
