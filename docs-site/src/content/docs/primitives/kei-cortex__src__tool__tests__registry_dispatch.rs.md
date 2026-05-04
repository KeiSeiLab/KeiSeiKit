---
title: registry_dispatch.rs
path: kei-cortex/src/tool/tests/registry_dispatch.rs
dna_hash: sha256:37ff4c3bff0bebd9
language: rust
size_loc: 113
generated: by-keidocs
---

# kei-cortex/src/tool/tests/registry_dispatch.rs

Validates the registry's dispatch table:
- default registry has all 8 named tools
- dispatch on unknown name yields `is_error = true` (no panic)
- dispatch on registered tool returns a successful `ToolResult`
- registry survives concurrent dispatches (Send + Sync via Arc)

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
