# KeiSeiKit — Anthropic Claude Code plugin format

This document describes the plugin-format install path (v0.16+) and how it relates to the classic `./install.sh` path. Both paths are supported; use whichever fits.

## TL;DR

```bash
# One-time
/plugin marketplace add KeiSei84/KeiSeiKit
# Install
/plugin install keisei@keisei-marketplace
```

The plugin auto-registers: agents, skills, hooks, and the MCP server. No manual `~/.claude/settings.json` edits. No `install.sh` needed for the core (non-primitive) experience.

## Layout

The repo follows the [Anthropic Claude Code plugin spec](https://code.claude.com/docs/en/plugins):

```
.claude-plugin/
  plugin.json          # plugin manifest (name, version, author, license, keywords)
  marketplace.json     # marketplace manifest — lets this repo serve as a marketplace source
  mcp-template.json    # template for .mcp.json (copy to repo root; see "MCP prerequisite" below)
agents/                # auto-discovered by Claude Code at plugin-install time
skills/<name>/SKILL.md # auto-discovered
hooks/hooks.json       # PreToolUse / PostToolUse / Stop hooks with ${CLAUDE_PLUGIN_ROOT} paths
.mcp.json              # MCP server registration (see prerequisite note)
```

Paths inside `hooks/hooks.json` use `${CLAUDE_PLUGIN_ROOT}` (expanded by Claude Code at runtime to the plugin install directory) rather than absolute `$HOME/.claude/hooks/...` paths. This lets the same hooks ship unchanged whether the plugin is installed from GitHub, npm, or a local path.

## Plugin install vs classic install — what differs

| Feature | Plugin install | Classic `./install.sh` |
|---|---|---|
| Agents registered | yes, automatic | yes, copied to `~/.claude/agents/` |
| Skills registered | yes, automatic | yes, copied to `~/.claude/skills/` |
| Hooks wired | yes, via `hooks/hooks.json` | requires `--activate-hooks` (jq-merge of `settings-snippet.json`) |
| MCP server | yes, via `.mcp.json` (once `@keisei/mcp-server` is published) | same |
| 47 Rust primitives | **no** — plugin ships manifest sources only; no cargo build | yes, `--profile=<name>` builds the selected set |
| 13 shell primitives | **no** | yes, copied to `~/.claude/agents/_primitives/` |
| Disk footprint | ~2 MB (plugin cache) | ~2 MB minimal up to ~200 MB full |
| Update path | `/plugin update keisei` | `git pull && ./install.sh` |
| Update visibility | Claude Code shows version change | silent |

**Bottom line:** plugin install is the right default for the agent-kit experience (agents + skills + hooks). For the Rust primitives (`tomd`, `kei-ledger`, `provision-hetzner`, `kei-migrate`, etc.), fall back to the classic installer or run it alongside the plugin — the two don't collide because the plugin namespaces into its own install dir and the classic installer writes to `~/.claude/`.

## Prerequisites

**For plugin install:**
- Claude Code 2.1+ (check with `claude --version`)
- Network access to `github.com/KeiSei84/KeiSeiKit` on `/plugin marketplace add`

**For the MCP server subset:**
- `@keisei/mcp-server` published to npm — **STATUS: not yet published as of v0.16.0.** The `.mcp.json` entry is structurally correct and will activate automatically once the package is published. Until then, the `keisei` MCP server simply won't appear in your tool list — the agents, skills, and hooks all work without it.
- Node.js 18+ (for `npx` to fetch the server on demand)

**For the Rust primitives (classic install only):**
- Rust stable, `jq`, plus the soft-deps listed in the main README per-profile table.

## Known limitations

1. **Rust primitives not auto-installed.** The plugin format doesn't currently express "also run `cargo build` at install time". We ship the manifest sources in-repo so that users who want the primitives can run `./install.sh --profile=full` alongside the plugin. A future version may add pre-built release binaries for common platforms (macOS arm64/x86_64, Linux x86_64) into `bin/` so the plugin can ship primitives without a cargo step.
2. **`@keisei/mcp-server` not yet on npm.** The `.mcp.json` entry is the canonical intent, but the package needs publishing first. See `_ts_packages/packages/mcp-server/README.md` for the publish pipeline.
3. **Hooks use `${CLAUDE_PLUGIN_ROOT}`.** This is the official Claude Code plugin variable. Older Claude Code versions (<2.1) that predate plugin support will not expand this variable — stick with classic install on those versions.
4. **No version-pinning yet.** `/plugin install keisei@keisei-marketplace` installs the default branch HEAD. For reproducible team installs, add the `--ref=<tag>` flag once it lands in Claude Code (currently in the spec per the extension schema `ref` field).

## Feedback & bugs

Open an issue at [github.com/KeiSei84/KeiSeiKit/issues](https://github.com/KeiSei84/KeiSeiKit/issues). A well-formed problem description is already half the solution.

## References

- [Anthropic Claude Code plugins docs](https://code.claude.com/docs/en/plugins)
- `README.md` — main install guide (plugin section is the new default)
- `settings-snippet.json` — retained for classic install; the plugin path does not use it
- `install.sh --help` — classic installer options, now with a plugin-first banner
