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
