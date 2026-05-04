---
title: tools.rs
path: kei-mcp/src/handlers/tools.rs
dna_hash: sha256:276c857b88645a15
language: rust
size_loc: 190
generated: by-keidocs
---

# kei-mcp/src/handlers/tools.rs

`tools/list` and `tools/call` — atom registry as MCP tools.

Atom→tool mapping:
name        = `<crate>::<verb>` (atom full id)
description = first paragraph of the atom's body
inputSchema = JSON loaded from `meta.input_schema`, or `{}` if missing

`tools/call` resolves the binary at `<crate>` (via PATH or
`KEI_RUNTIME_BIN_DIR`) and shells out as `<crate> run-atom <verb>`,
piping the JSON arguments on stdin. Stdout is parsed back as JSON.

MISS-4: the spawn is wrapped in a `tokio::time::timeout` so a hung atom
cannot block the JSON-RPC channel. Hard cap is `ATOM_TIMEOUT_SECS` (60s).
On timeout the child is killed and a `-32603` error is returned with
message `atom timeout`.

## Public API

- Hard cap on how long a single `tools/call` invocation may take. A
- Convert one atom's metadata into the MCP tool-descriptor shape.
- Extract the first non-empty paragraph from a markdown body. Headings
- Read a schema file as JSON. Returns `{}` on any failure (missing file,
- Resolve an atom by full id, then shell out to `<crate> run-atom <verb>`
- Spawn `<bin> run-atom <verb>` via `tokio::process::Command`, write the
- Pipe JSON-encoded `args` into the child's stdin and close the write half
- Resolve `<crate>` as an executable: prefer `KEI_RUNTIME_BIN_DIR/<crate>`,

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: crate, kei_atom_discovery, serde_json, std, tokio

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
