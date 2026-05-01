# kei-mcp — Model Context Protocol server

`kei-mcp` exposes the KeiSeiKit atom registry over the [Model Context
Protocol](https://modelcontextprotocol.io/) so MCP-aware clients (Claude
Code, Cline, OpenClaw, etc.) can discover and call our 13 atoms + N
primitives as MCP tools, and read our skills as MCP resources.

## What you get

- **Tools** — every atom in `_primitives/_rust/*/atoms/*.md` becomes one
  MCP tool. Tool name is the atom's full id (`<crate>::<verb>`), the
  description is the first paragraph of the atom's body, and the input
  schema is the JSON-Schema referenced by the atom's frontmatter.
- **Resources** — every `skills/<name>/SKILL.md` becomes one MCP resource
  at `skill://<name>` returning the SKILL.md text on read.
- **Prompts** — placeholder list (empty) for now.

## Wire format

JSON-RPC 2.0 over stdio, line-delimited (one request per line, one
response per line). stdout carries protocol frames ONLY; everything else
(diagnostics, warnings) goes to stderr.

Supported methods: `initialize`, `tools/list`, `tools/call`,
`resources/list`, `resources/read`, `prompts/list`, `prompts/get`.

## Build

```sh
cargo build -p kei-mcp --release
# binary: target/release/kei-mcp
```

## Configuration (env)

| Variable | Default | What it does |
|---|---|---|
| `KEI_MCP_ATOMS_ROOT` | `_primitives/_rust` | Where to walk for `<crate>/atoms/*.md` |
| `KEI_MCP_SKILLS_ROOT` | `skills` | Where to walk for `<name>/SKILL.md` |
| `KEI_RUNTIME_BIN_DIR` | (unset) | Resolve `<crate>` binaries here before falling back to `$PATH` |

## Register with Claude Code

Add to `~/.claude/mcp_servers.json`:

```json
{
  "kei": {
    "command": "/absolute/path/to/kei-mcp",
    "args": [],
    "env": {
      "KEI_MCP_ATOMS_ROOT": "/absolute/path/to/KeiSeiKit/_primitives/_rust",
      "KEI_MCP_SKILLS_ROOT": "/absolute/path/to/KeiSeiKit/skills",
      "KEI_RUNTIME_BIN_DIR": "/absolute/path/to/KeiSeiKit/_primitives/_rust/target/release"
    }
  }
}
```

## Register with Cline

Edit Cline's `cline_mcp_settings.json` (open via `Cline: Edit MCP
Settings` from the command palette):

```json
{
  "mcpServers": {
    "kei": {
      "command": "/absolute/path/to/kei-mcp",
      "args": [],
      "env": {
        "KEI_MCP_ATOMS_ROOT": "/absolute/path/to/KeiSeiKit/_primitives/_rust",
        "KEI_MCP_SKILLS_ROOT": "/absolute/path/to/KeiSeiKit/skills"
      }
    }
  }
}
```

## Register with OpenClaw

Add to `~/.openclaw/mcp.json`:

```json
{
  "servers": {
    "kei": {
      "command": "/absolute/path/to/kei-mcp",
      "args": [],
      "env": {
        "KEI_MCP_ATOMS_ROOT": "/absolute/path/to/KeiSeiKit/_primitives/_rust",
        "KEI_MCP_SKILLS_ROOT": "/absolute/path/to/KeiSeiKit/skills"
      }
    }
  }
}
```

## Manual smoke test

```sh
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}' \
  | ./kei-mcp
```

You should see one line of JSON on stdout containing `serverInfo.name: "kei-mcp"`.
