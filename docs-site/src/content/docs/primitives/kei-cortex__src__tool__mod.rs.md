---
title: mod.rs
path: kei-cortex/src/tool/mod.rs
dna_hash: sha256:01f1e593fbcf0680
language: rust
size_loc: 48
generated: by-keidocs
---

# kei-cortex/src/tool/mod.rs

Tool registry + agentic loop substrate.

Constructor Pattern: each tool is one cube (`read.rs`, `write.rs`,
`edit.rs`, `bash.rs`, `glob_tool.rs`, `grep.rs`, `webfetch.rs`,
`agent.rs`); the `registry` cube wires them by name; `schemas.rs`
holds the Anthropic JSON-Schema definitions; `loop_driver` runs the
turn-by-turn loop.

Wave 44a security cubes:
- `path_sandbox.rs` — chroot + basename + dotfile deny
- `atomic_io.rs`    — shared atomic_write (used by write/edit)
- `ip_filter.rs`    — SSRF deny-list (used by webfetch)
- `bash_denylist.rs` — argv0 + substring deny + allow-list

Public API: a daemon-side caller composes
`ToolRegistry::with_project_root(...)` + `tool_definitions()` and
hands them to `loop_driver::run_with_tools` along with a
`ModelInvoker` closure. See `INTEGRATION.md` for the exact patch the
orchestrator applies to `handlers/chat.rs`.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
