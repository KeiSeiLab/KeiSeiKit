# Multi-CLI agent invocation

> *Cross-LLM agent execution. Same agent definition, different backend.*

KeiSeiKit agents are markdown files. Any LLM CLI that takes a prompt can
host them — `kei run-via` is the launcher that bridges them.

## Backends

Registered in `_primitives/cli-backends.toml` (SSoT). Installed locally
via your own subscription / package manager:

| Backend  | CLI binary | Non-interactive flag | Native `--agent` | Notes |
|----------|-----------|----------------------|------------------|-------|
| claude   | `claude`  | `-p`                 | yes              | Claude Code (Anthropic) |
| grok     | `grok`    | `--print`            | yes              | xAI Grok Build TUI |
| agy      | `agy`     | `--print`            | no               | Google Antigravity (alias: `antigravity`) |
| copilot  | `copilot` | `--prompt`           | no               | GitHub Copilot CLI (`@github/copilot`) |
| kimi     | `kimi`    | stdin                | no               | Moonshot Kimi (primarily TUI/ACP) |
| codex    | `codex`   | `-p`                 | no               | OpenAI Codex (register-only) |

Run `kei run-via list` to see which are installed on the current machine
and to list available agent names.

## Usage

```bash
# Invoke the 'critic' agent through Grok with a task:
kei run-via grok critic "review src/auth.rs for variant analysis"

# Same agent, different backend:
kei run-via agy critic "review src/auth.rs"
kei run-via copilot critic "review src/auth.rs"
kei run-via claude critic "review src/auth.rs"

# Point at an arbitrary agent .md (not in ~/.claude/agents/):
kei run-via grok --file=/tmp/my-agent.md "do the thing"

# Backend's native --agent flag (grok/claude only):
KEI_NATIVE_AGENT=1 kei run-via grok critic "review src/auth.rs"
```

## How it works

1. Reads `~/.claude/agents/<agent-name>.md` (assembler-generated prompt).
2. Strips YAML frontmatter.
3. Composes with task as: `<agent prompt>\n\n---\n\nTASK FOR THIS RUN:\n<task>`.
4. Execs the backend's non-interactive CLI with the composed prompt.

No agent file is modified. No new tokens are issued. Subscription
authentication is whatever each CLI uses (its own login / config dir).

## When to use each

This is a tool, not a recommendation. Each backend has different
strengths; the substrate is agnostic about which you pick. Pick by:

- **Familiarity** — the CLI you already use day-to-day.
- **Subscription cost** — burn the one with cheaper marginal cost first.
- **Specific feature** — e.g. `grok --agent` for native sub-agent
  switching mid-conversation; `agy --sandbox` for terminal restriction.
- **Independent second opinion** — same agent, different model, see if
  conclusions diverge.

## Adding a new backend

1. Add a `[backend.<name>]` table to `_primitives/cli-backends.toml`.
2. Add a case arm in `scripts/kei-agent-cli.sh` `backend_bin()` and
   `backend_invoke()` for the new CLI's print-flag.
3. Add a row to the table above.

## What it is NOT

- Not a router — picks no backend for you; you ask, it dispatches.
- Not a federation — each backend runs independently with its own
  context; there is no cross-backend state.
- Not a wrapper around the backend's tool surface — what the CLI can
  do (Bash, file edits, MCP, etc.) is determined by that CLI, not
  KeiSeiKit. The substrate only ships the prompt.

## Related

- `_primitives/_rust/kei-llm-router/` — Beta-posterior router for
  *programmatic* model selection inside Rust code (a different layer).
- `_primitives/_rust/kei-mcp/` — MCP server that exposes KeiSeiKit
  primitives to ANY MCP-compatible client (Cursor / Continue / Zed /
  Aider / Cline / Windsurf / OpenClaw).
