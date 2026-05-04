---
title: protocol.rs
path: kei-mcp/src/protocol.rs
dna_hash: sha256:64346d1ff3c11db4
language: rust
size_loc: 129
generated: by-keidocs
---

# kei-mcp/src/protocol.rs

JSON-RPC 2.0 envelope + MCP method enum + per-request server context.

MCP method names are dotted/slash-delimited per spec
(`tools/list`, `tools/call`, `resources/list`, `resources/read`,
`prompts/list`, `prompts/get`, `initialize`).

## Public API

- JSON-RPC 2.0 request envelope. `id` is `Value` (number or string per spec).
- JSON-RPC 2.0 response envelope. Either `result` or `error`, never both.
- JSON-RPC 2.0 error object — `code` + `message` + optional `data`.
- MCP method enum — strings normalised to spec wire names.
- `pub fn parse` — Map a wire string to the typed enum. Unknown methods become `Other(s)`.
- `pub struct ServerContext` — Per-server context shared across requests. Holds the atom-registry root
- `pub const METHOD_NOT_FOUND` — Standard JSON-RPC error codes per spec.
- `pub fn ok` — Build a `result`-shaped response for a known id.
- `pub fn err` — Build an `error`-shaped response.

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: kei_skills, serde, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
