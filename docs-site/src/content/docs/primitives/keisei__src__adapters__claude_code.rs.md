---
title: claude_code.rs
path: keisei/src/adapters/claude_code.rs
dna_hash: sha256:eeea192369d0206f
language: rust
size_loc: 127
generated: by-keidocs
---

# keisei/src/adapters/claude_code.rs

Claude Code adapter — writes MCP server entry into
`~/.claude/settings.json` (user scope) or `./.claude/settings.json`
(project scope). Config shape merges under `mcpServers.keisei` so we
never clobber unrelated entries.

Detection: `$CWD/.claude/settings.json` exists OR
`$KEISEI_HOME/.claude` (or `$HOME/.claude`) is a directory.
`$KEISEI_HOME` overrides `$HOME` for tests.

Security (v0.19 audit): if an entry at `mcpServers["keisei"]` already
exists and doesn't match what keisei would write, attach fails with
`NameConflict` instead of silently clobbering the user's config.

v0.21.1: the JSON merge/remove/persist logic lives in `jsonmcp`
(shared with Cursor + Zed); this file is now just the client-specific
path resolution + scope table.

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
