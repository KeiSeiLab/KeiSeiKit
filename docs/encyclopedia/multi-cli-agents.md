# Multi-CLI agent invocation

> *Cross-LLM agent execution. Same agent definition, different backend.*
> *Same DNA, swap the brain. KeiSeiKit is no longer Claude-Code-only.*

KeiSeiKit agents are markdown files. Any LLM CLI that takes a prompt can
host them. Three call shapes:

```bash
kei agent <name> "<task>"                # DNA-resolved (manifest → primary → claude)
kei agent --on=<backend> <name> "<task>" # override DNA
kei run-via <backend> <name> "<task>"    # explicit backend (no DNA lookup)
```

## Backends — smoke-tested 2026-05-26

| Backend  | CLI       | Flag         | Smoke | Notes |
|----------|-----------|--------------|-------|-------|
| claude   | `claude`  | `-p`         | ✅    | Claude Code, native `--agent` flag |
| grok     | `grok`    | `--print`    | ✅    | xAI Grok Build TUI, native `--agent` flag |
| agy      | `agy`     | `--print`    | ✅    | Google Antigravity (Gemini models). Alias: `antigravity` |
| copilot  | `copilot` | `--prompt`   | ✅    | GitHub Copilot CLI (`@github/copilot`) |
| kimi     | `kimi`    | TUI-only     | ⚠     | No print mode — launcher saves prompt to tmpfile + opens TUI for paste. `kimi acp` JSON-RPC integration is future work. |
| codex    | `codex`   | `-p`         | —     | OpenAI Codex (register-only; not installed locally) |

Run `kei run-via list` to see installed backends, current primary, and agent names.

## DNA — agent prefers a provider

Add `provider` to the agent manifest:

```toml
# _manifests/my-agent.toml
name = "my-agent"
provider = "grok"     # preferred backend; optional
model = "grok-2"      # advisory; informs choice but not yet sent through
```

The assembler emits it into frontmatter:

```yaml
---
name: my-agent
provider: grok
---
```

Resolution order (each falls through if previous returns nothing):
1. `--on=<backend>` flag on the command line
2. `provider:` field in agent manifest
3. `~/.claude/config/primary.toml` (set via `kei primary <backend>`)
4. Default: `claude`

## Primary — your default LLM

```bash
kei primary                # show current primary (and fallback)
kei primary grok           # set default to Grok
kei primary claude         # back to Claude Code
```

`kei primary` writes `~/.claude/config/primary.toml`. Any agent without
its own `provider:` field will resolve to this. This is the lever to
"swap out Claude Code as the primary shell" — set primary to grok, and
every `kei agent <name>` runs on Grok.

## Usage examples

```bash
# DNA mode (manifest's provider, or primary, or claude):
kei agent critic "review src/auth.rs"

# Override DNA — try the same agent on a different model for a second opinion:
kei agent --on=grok critic "review src/auth.rs"
kei agent --on=agy  critic "review src/auth.rs"
kei agent --on=copilot critic "review src/auth.rs"

# Explicit backend, no DNA lookup (legacy):
kei run-via grok critic "review src/auth.rs"

# Point at an arbitrary agent file:
kei agent --on=grok --file=/tmp/my-agent.md "do the thing"

# Native --agent flag (grok/claude only):
KEI_NATIVE_AGENT=1 kei agent critic "review src/auth.rs"
```

## How it works

1. Resolves backend from DNA (see above).
2. Reads `~/.claude/agents/<agent-name>.md` (assembler-generated prompt).
3. Strips YAML frontmatter.
4. Composes with task: `<agent prompt>\n\n---\n\nTASK FOR THIS RUN:\n<task>`.
5. Execs the backend's non-interactive CLI with the composed prompt.

No agent file is modified. No new tokens are issued — subscription
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

## Rule enforcement caveat (READ THIS)

KeiSeiKit hooks (`numeric-claims-guard`, `citation-verify`, `no-github-push`,
`safety-guard`, `push-to-main`, etc.) are **Claude Code-side**:
`PreToolUse:Bash` / `:Edit` / `:Write` events that fire inside Claude Code's
process. They do **not** propagate to grok / agy / copilot / kimi.

That means:
- **Prompt-level rules** (the agent's instructions inside the `.md`) DO
  carry through — the agent reads Constructor Pattern, Evidence Grading,
  No Hallucination, etc. as part of its system prompt on any backend.
- **Tool-level enforcement** (hard-deny on `git push github.com`,
  citation guard, etc.) only applies on the **claude** backend. Other
  backends' tool surfaces are governed by THEIR own hooks/policies.

If you need true rule-enforcement on a non-claude backend, the path is
the **MCP server** (`_primitives/_rust/kei-mcp/`): registers KeiSeiKit
primitives as MCP tools that the other CLI invokes. Tool-side policies
travel with the MCP wrapper, not with the CLI.

## Adding a new backend

1. Add a `[backend.<name>]` table to `_primitives/cli-backends.toml`.
2. Add a case arm in `scripts/kei-agent-cli.sh` `backend_bin()` and
   `backend_invoke()` for the new CLI's print-flag.
3. Add a row to the smoke-test table above (state PASS/FAIL/PARTIAL).

## What it is NOT

- Not a router — picks no backend for you; you (or DNA) ask, it dispatches.
- Not a federation — each backend runs independently with its own
  context; there is no cross-backend state.
- Not a rule-enforcement layer — hooks only fire on the claude backend
  (see caveat above). For non-claude rule enforcement use MCP server.
- Not a wrapper around the backend's tool surface — what the CLI can
  do (Bash, file edits, MCP, etc.) is determined by that CLI, not
  KeiSeiKit. The substrate only ships the prompt.

## Related

- `_primitives/_rust/kei-llm-router/` — Beta-posterior router for
  *programmatic* model selection inside Rust code (a different layer).
- `_primitives/_rust/kei-mcp/` — MCP server that exposes KeiSeiKit
  primitives to ANY MCP-compatible client (Cursor / Continue / Zed /
  Aider / Cline / Windsurf / OpenClaw).
