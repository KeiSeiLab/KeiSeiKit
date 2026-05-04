---
title: continue_adapter.rs
path: keisei/src/adapters/continue_adapter.rs
dna_hash: sha256:0cef0da365fbd305
language: rust
size_loc: 206
generated: by-keidocs
---

# keisei/src/adapters/continue_adapter.rs

Continue.dev adapter — writes MCP server entry into `~/.continue/`.

Config path strategy [UNVERIFIED — see note]:
1. If `~/.continue/config.yaml` exists → YAML mode
2. Else if `~/.continue/config.json` exists → JSON mode
3. Else if `~/.continue/` exists → create `config.yaml` fresh
4. Else `detect()` returns false (graceful)

Schema (both forms), under top-level `mcpServers`:
```yaml
mcpServers:
- name: keisei
command: /path/to/kei-mcp-server
args: []
env:
KEISEI_BRAIN_ROOT: /Volumes/Brain1
```

NOTE: Continue's exact MCP/plugin schema is [UNVERIFIED] in this
session. Adapter uses list-form `mcpServers` from v0.18 prototypes +
public Continue `config.yaml` docs. Detach preserves unrelated keys.

Security (v0.19 audit): collision-safe — existing `name: keisei` with
different content → `NameConflict`, no silent overwrite.

## Public API

- Load doc as a generic `serde_json::Value`. YAML → Value via serde_yaml,

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, serde_json, std

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
