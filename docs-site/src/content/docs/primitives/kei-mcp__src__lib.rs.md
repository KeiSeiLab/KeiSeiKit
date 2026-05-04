---
title: lib.rs
path: kei-mcp/src/lib.rs
dna_hash: sha256:27975d474cfe7969
language: rust
size_loc: 17
generated: by-keidocs
---

# kei-mcp/src/lib.rs

kei-mcp — Model Context Protocol server.

Exposes the atom registry (and skills as resources) over stdio JSON-RPC,
per the MCP spec at <https://modelcontextprotocol.io/>. JSON-RPC 2.0,
line-delimited, one request → one response. Stateless beyond the
`initialize` handshake.

Library shape exists so integration tests can drive `dispatch` directly
without spawning the binary.

## Related

- parent: `kei-mcp/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
