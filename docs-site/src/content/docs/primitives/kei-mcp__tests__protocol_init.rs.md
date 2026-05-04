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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
