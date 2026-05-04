---
title: tools_call_dispatches.rs
path: kei-mcp/tests/tools_call_dispatches.rs
dna_hash: sha256:7a0bc389c2328a61
language: rust
size_loc: 108
generated: by-keidocs
---

# kei-mcp/tests/tools_call_dispatches.rs

Integration test — `tools/call` dispatches to `<crate> run-atom <verb>`.

Strategy: write a tiny shell-script "fake binary" into a tempdir, point
`KEI_RUNTIME_BIN_DIR` at that dir, and verify the handler's response
contains the JSON the script printed. This proves:
- tool name is parsed into (crate, verb)
- resolve_binary picks up KEI_RUNTIME_BIN_DIR
- stdout JSON is captured into `content[0].text`

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
