# Cross-CLI policy enforcement

> *Same safety rules. Any LLM CLI. Three honesty tiers.*

KeiSeiKit's safety hooks (`no-github-push`, `safety-guard`, `destructive-guard`,
`citation-verify`, `numeric-claims-guard`) originally fired only inside Claude
Code's `PreToolUse` pipeline. Phase C extends enforcement to other CLIs —
but the strength of enforcement depends on what each CLI permits.

## The 3-tier honesty model

| Tier | What it means | CLIs |
|---|---|---|
| **TIER 1 — full native** | Tool-call enforcement at the CLI's own hook layer. Same as Claude. | claude, **grok** |
| **TIER 2 — MCP-wrapped** | Native shell disabled at launch; agent forced to use our policy-gated `kei_bash`/`kei_edit`/`kei_write` MCP tools. | **copilot** |
| **TIER 3 — advisory** | CLI can't disable native shell; we register kei-mcp and instruct the agent to prefer `kei_*` tools, but enforcement is prompt-level only. | **agy, kimi** |

For patent-sensitive or production-PR work — stick to TIER 1 (claude or grok).

## How to wire

One command sets up enforcement for whichever CLIs you have installed:

```bash
kei mcp-wire                    # detect + wire all installed CLIs
kei mcp-wire grok               # wire one CLI
kei mcp-wire --dry-run          # preview config changes without writing
kei mcp-wire --list             # show enforcement tier per CLI
```

The orchestrator is idempotent — running twice produces the same config.

## What `kei mcp-wire` writes

### claude (TIER 1 — already enforced)
No-op. Native PreToolUse hooks already gate every tool call. `kei mcp-wire claude`
prints the optional `mcpServers` snippet you can add to
`~/.claude/settings.json` if you want claude to also see `spawn_agent` for
sub-agent dispatch.

### grok (TIER 1 — port our hooks)
Writes `~/.grok/settings.json` `hooks.PreToolUse` block:

- `Bash` matcher → `no-github-push.sh` + `safety-guard.sh` + `destructive-guard.sh`
- `Edit` matcher → `citation-verify.sh` + `numeric-claims-guard.sh`
- `Write` matcher → `citation-verify.sh` + `numeric-claims-guard.sh`

Plus registers kei-mcp with `GROKCODE=1` env (so kei-mcp's policy chain skips
duplicate enforcement when invoked via Grok — your native hooks already fired).

xAI's Grok uses the same JSON input contract as Claude Code's PreToolUse, so
our hook scripts run unchanged. Identical enforcement to claude.

### copilot (TIER 2 — disable native shell, force MCP)
Writes `~/.copilot/mcp-config.json` registering kei-mcp. To activate enforcement,
launch copilot with `--excluded-tools='shell'`:

```bash
alias copilot='copilot --excluded-tools=shell'
```

The agent will have NO native shell tool, only kei-mcp's `kei_bash` —
which runs the policy chain before execution. `kei_edit` / `kei_write`
similarly gate file mutations.

### agy / kimi (TIER 3 — advisory)
Writes their MCP config (`~/.gemini/config/mcp_config.json` for agy,
`~/.kimi/mcp.json` for kimi) registering kei-mcp.

**The honest part:** these CLIs do NOT have a way to disable their native
shell. The agent CAN reach for native bash regardless of what we tell it.
The system prompt nudges it toward `kei_bash`, but a determined or careless
agent can bypass.

For patent-sensitive work — **don't use agy or kimi as orchestrator**.
Use them for analysis / brainstorming / no-side-effect tasks only.

## Internals

### policy-chain.toml (SSoT)

One file declares which hooks gate which tool, for all CLIs that go through
the MCP layer:

```toml
# ~/.claude/hooks/_lib/policy-chain.toml
[bash]
chain = ["no-github-push.sh", "safety-guard.sh", "destructive-guard.sh"]

[edit]
chain = ["citation-verify.sh", "numeric-claims-guard.sh"]

[write]
chain = ["citation-verify.sh", "numeric-claims-guard.sh"]
```

To add a hook: append its basename. The hook script must already exist in
`~/.claude/hooks/` and follow the standard PreToolUse contract (read JSON
on stdin with `.tool_name` + `.tool_input`, return exit 0 = pass / 2 = block).

### kei-mcp built-in tools

`kei-mcp` (Rust MCP server at `_primitives/_rust/kei-mcp/`) exposes four
built-in tools that bypass atom discovery:

- `spawn_agent(name, task, on?)` — invokes a KeiSeiKit agent on any backend
- `kei_bash(command, cwd?)` — runs `[bash]` chain → executes
- `kei_edit(file_path, old_string, new_string)` — runs `[edit]` chain → edits
- `kei_write(file_path, content)` — runs `[write]` chain → writes

The chain runs against the same hook scripts Claude uses; identical input
shape, identical decisions. On block, the hook's stderr surfaces as the MCP
error message so the calling agent sees exactly why.

### Double-enforcement guard

If kei-mcp is invoked from a process where `$CLAUDECODE=1` or `$GROKCODE=1`,
it SKIPS its hook chain — the CLI's native hooks already fired. This is set
automatically by `kei mcp-wire claude` / `kei mcp-wire grok`. On copilot /
agy / kimi the env is unset → chain runs.

## Verification

```bash
# All 4 built-ins must list:
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  | kei-mcp | jq -r '.result.capabilities'

# Block test (kei_bash refuses forbidden command):
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"kei_bash","arguments":{"command":"git push https://github.com/x/y.git main"}}}' \
  | kei-mcp 2>&1 | grep "RULE 0.1"   # expects: BLOCK — RULE 0.1 NO GITHUB PUSH

# Pass test:
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"kei_bash","arguments":{"command":"echo OK"}}}' \
  | kei-mcp | tail -1 | jq -r '.result.content[0].text'   # expects: OK
```

## Related

- [Multi-CLI agent invocation](./multi-cli-agents.md) — DNA-resolved agent dispatch
- `kei-mcp` source: `_primitives/_rust/kei-mcp/src/handlers/safe_tools.rs`
- Policy SSoT: `hooks/_lib/policy-chain.toml`
- Wire scripts: `scripts/kei-mcp-wire*.sh`
