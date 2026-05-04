---
title: extract.rs
path: kei-memory/src/extract.rs
dna_hash: sha256:a7dde65355050248
language: rust
size_loc: 179
generated: by-keidocs
---

# kei-memory/src/extract.rs

Pull tool_use / tool_result blocks out of a real Claude Code trace line.

Constructor Pattern: this cube only walks the JSON shape; classification +
persistence live elsewhere. Real trace shape (see ingest.rs::TraceLine):

message.content : array
element {type: "tool_use",   name: <T>, id: <id>, input: {...}}
element {type: "tool_result", tool_use_id: <id>, is_error: bool}
element {type: "text",       text: "..."}

Old `tool: <name>` flat field is GONE — it was the schema-mismatch root
cause that dropped ~50% of trace lines silently before Wave A.

## Public API

- One `tool_use` block extracted from a Claude Code assistant message.
- One `tool_result` block — the user-side counterpart of `ToolUse`.
- `pub fn extract_tool_uses` — Walk `message.content[]`, return every `tool_use` element.
- `pub fn extract_tool_result` — Walk `message.content[]`, return the FIRST `tool_result` element if any.
- Best-effort: grab `input.file_path` if present (Edit/Read/Write tools).
- `pub fn parse_timestamp_to_epoch` — Parse an ISO-8601 / RFC-3339 timestamp string to Unix epoch seconds.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: chrono, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
