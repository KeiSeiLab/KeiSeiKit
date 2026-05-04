---
title: initialize.rs
path: kei-mcp/src/handlers/initialize.rs
dna_hash: sha256:f703e1b7126bc4e2
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-mcp/src/handlers/initialize.rs

`initialize` method — MCP handshake.

Returns server info + capability descriptor. Capabilities advertised:
`tools` (list+call), `resources` (list+read), `prompts` (list+get,
placeholder for now). Per MCP spec the protocol version is echoed
back to the client.

## Public API

- Default protocol version we advertise. Clients may negotiate a different
- Capability matrix advertised in the handshake. Each sub-object is an

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: crate, serde_json

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
