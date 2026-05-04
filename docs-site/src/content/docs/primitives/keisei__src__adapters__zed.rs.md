---
title: zed.rs
path: keisei/src/adapters/zed.rs
dna_hash: sha256:675b562d3f54a9c5
language: rust
size_loc: 117
generated: by-keidocs
---

# keisei/src/adapters/zed.rs

Zed adapter — writes MCP/context-server entry into Zed settings.

Config path [UNVERIFIED for exact schema key-name]:
- macOS:   `~/Library/Application Support/Zed/settings.json`
- Linux:   `~/.config/zed/settings.json`
- Windows: not supported in this adapter (Zed Windows is preview)

Schema (under a top-level `context_servers` object):
```json
{
"context_servers": {
"keisei": {
"command": "/path/to/kei-mcp-server",
"args": [],
"env": { "KEISEI_BRAIN_ROOT": "..." }
}
}
}
```

NOTE: Zed's `context_servers` key is the documented extension point for
MCP at time of writing — but the full schema (arg handling,
environment) is [UNVERIFIED] in this session. If a future Zed release
diverges, update this module.

Security (v0.19 audit): collision-safe — if `context_servers["keisei"]`
already exists with different content, attach fails with
`NameConflict` rather than silently clobbering.

v0.21.1: JSON merge/remove/persist logic lives in `jsonmcp` (shared
with Claude Code + Cursor).

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
