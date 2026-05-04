---
title: protocol_init.rs
path: kei-mcp/tests/protocol_init.rs
dna_hash: sha256:01ab2579cff2b688
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-mcp/tests/protocol_init.rs

Integration test — `initialize` handshake response shape.

Drives `dispatch` directly (library API) so the test does not depend on
spawning the binary. Verifies:
- `serverInfo.name` == "kei-mcp"
- `serverInfo.version` non-empty
- `capabilities.tools`, `.resources`, `.prompts` all present
- client-supplied `protocolVersion` is echoed back

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json

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
