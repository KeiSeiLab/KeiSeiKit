---
title: tools_list.rs
path: kei-mcp/tests/tools_list.rs
dna_hash: sha256:fadde79ee355d1b2
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-mcp/tests/tools_list.rs

Integration test — `tools/list` walks a mock atom registry and emits
MCP-shape tool descriptors.

Builds a minimal `<root>/<crate>/atoms/<verb>.md` layout in tempdir and
verifies that every atom round-trips into a `{name, description,
inputSchema}` triple.

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
