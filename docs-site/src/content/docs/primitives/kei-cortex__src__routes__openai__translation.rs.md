---
title: translation.rs
path: kei-cortex/src/routes/openai/translation.rs
dna_hash: sha256:4be0889004544028
language: rust
size_loc: 133
generated: by-keidocs
---

# kei-cortex/src/routes/openai/translation.rs

OpenAI tool-call ⇄ kei-cortex tool-name translation.

kei-cortex exposes 8 internal tools (read, write, edit, bash, glob,
grep, webfetch, agent). OpenAI clients send a `tools[]` array whose
`function.name` we accept verbatim if it matches one of those 8 —
otherwise we drop the entry and surface a `Warning` header for the
frontend to display. We do NOT attempt to alias arbitrary OpenAI
function names to our tools; that would mask client bugs.

## Public API

- `pub const KEI_TOOLS` — The 8 tools kei-cortex's `ToolRegistry::with_project_root` registers.
- `pub fn filter_supported_tools` — Filter the client-supplied `tools[]` array, keeping only entries whose
- `pub fn build_tool_call` — Build a single `ToolCall` describing a kei-cortex tool invocation,
- `pub fn flatten_user_prompt` — Extract the user-visible prompt from an OpenAI `messages[]` array.
- `pub fn build_assistant_message` — Compose the assistant's reply message. Keeps `content` if non-empty

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
