---
title: cursor.rs
path: keisei/src/adapters/cursor.rs
dna_hash: sha256:ba3034596c642abe
language: rust
size_loc: 125
generated: by-keidocs
---

# keisei/src/adapters/cursor.rs

Cursor adapter — writes MCP server entry to Cursor's MCP config.

Scope:
- `Scope::User`    → `~/.cursor/mcp.json`
- `Scope::Project` → `$CWD/.cursor/mcp.json`

Detection fires if either the user-scope dir or a project-scope dir
exists. Schema [UNVERIFIED — matches Claude Desktop MCP convention]:
`{ "mcpServers": { "keisei": { "command": "...", "args": [] } } }`.

Security (v0.19 audit): collision-safe — if `mcpServers["keisei"]`
already exists with different content, attach fails with
`NameConflict` rather than silently clobbering.

v0.21.1: JSON merge/remove/persist logic lives in `jsonmcp` (shared
with Claude Code + Zed).

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, std

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
