---
title: types.rs
path: kei-cortex/src/tool/types.rs
dna_hash: sha256:84f09c3b7fbb370f
language: rust
size_loc: 137
generated: by-keidocs
---

# kei-cortex/src/tool/types.rs

Tool primitive types — `ToolCall`, `ToolResult`, `ToolError`.

These mirror the Anthropic Messages API `tool_use` / `tool_result`
content-block shape. A `ToolCall` is what the model emits and what we
dispatch on; a `ToolResult` is what we hand back in the next turn.

Constructor Pattern: type-only module, no I/O, no state. The actual
executor functions live in sibling cubes (`read.rs`, `write.rs`, etc.).

## Public API

- One tool invocation requested by the model. Mirrors the
- Anthropic-side block id, echoed back verbatim in `tool_result`.
- The tool name (matches a registry key).
- Tool-specific input arguments (validated per tool).
- One tool result we hand back to the model. Goes into the next
- Echo of the originating `ToolCall.id`.
- Plain text content. Anthropic accepts a string OR a content-block
- True when the executor errored. Tells the model "your call failed,
- `pub fn ok` — Build a successful result.
- `pub fn err` — Build an error result.
- Errors produced by tool executors. Distinguished from `AppError` so
- `pub fn as_message` — Render as the `content` of a `ToolResult { is_error: true }`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json

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
