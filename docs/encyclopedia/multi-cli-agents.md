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
| glm      | `claude`  | `-p`         | ✅    | Z.ai GLM-5.2 via Anthropic-compatible endpoint; wraps the `claude` binary + `ZAI_API_KEY` (smoke 2026-06-30). See [GLM (Z.ai) backend](#glm-zai-backend) below. |

Run `kei run-via list` to see installed backends, current primary, and agent names.

## GLM (Z.ai) backend

`glm` is special: it does **not** ship its own CLI. Z.ai's GLM Coding Plan
exposes an **Anthropic-compatible** endpoint, so the backend reuses the
`claude` binary and just points it at Z.ai via env vars. It is the actual
Claude Code process (not a foreign CLI) — only the model behind the API
differs.

**Setup** — drop your key in `~/.claude/secrets/.env` (RULE 0.8), the launcher
sources it automatically:

```bash
echo 'ZAI_API_KEY=...' >> ~/.claude/secrets/.env && chmod 600 ~/.claude/secrets/.env
```

**Usage:**

```bash
kei agent --on=glm critic "review src/auth.rs"   # one agent on GLM
kei primary glm                                   # all agents + bare `kei` default to GLM
kei primary claude                                # back to Anthropic
```

**Env knobs** (all optional):

| Var | Default | Purpose |
|---|---|---|
| `ZAI_API_KEY` | — (required) | Z.ai key → `ANTHROPIC_AUTH_TOKEN` |
| `ZAI_MODEL` | `glm-5.2` | maps to Opus + Sonnet slots |
| `ZAI_SMALL_MODEL` | `glm-5-turbo` | maps to the Haiku slot |
| `ZAI_BASE_URL` | `https://api.z.ai/api/anthropic` | endpoint override |

The env is injected only into the exec'd subprocess — your real Anthropic
`claude` backend is untouched. The glm arm forces `claude --strict-mcp-config`
because loading the full MCP fleet in a `-p` subprocess otherwise hangs the
launch (~60 s); the trade-off is that the GLM sub-session does not see
`kei-mcp` / `spawn_agent` (the standard agent roster declares no MCP tools, so
this is harmless in practice).

**Default routing (DNA) — benchmark-grounded (2026-06).** Two tiers, not a
matrix. GLM-5.2 is near-Opus on SWE-bench-class coding at ~1/6 the cost with a
1M context, but weakest on the hardest long-horizon multi-step work and complex
multi-tool orchestration. So:

- **→ GLM** (bulk / coding / read-only scan, cheap): `code-implementer*`,
  `critic*`, `researcher-code`, and the mechanical security scanners
  `security-auditor-variant` / `security-auditor-supply-chain` (GLM beat Opus 4.8
  on IDOR detection F1 39 vs 28 at ~1/6 cost — good for breadth scanning).
- **→ claude** (judgement / deep reasoning / expensive failure): `architect`
  (deep structural reasoning is Opus's edge), `validator*`, `security-auditor` +
  `security-auditor-differential` (final risk verdict), `cost-guardian`, `ml-*`,
  `modal-runner`, `infra-*`, `researcher` / `researcher-web` / `researcher-hybrid`.

Override per call with `--on=`. Escalate `code-implementer*` to `--on=claude` for
marathon / cross-cutting refactors (GLM's weak spot).

**Cross-model verification — `kei second-opinion`.** Different models surface
different issues (the v0.62 audit showed this empirically). For critical
*read-only* checks, run the same agent on two backends in parallel and diff:

```bash
kei second-opinion security-auditor "audit src/auth.rs"   # glm vs claude
kei second-opinion critic "review src/lock.rs" --on=glm --vs=grok
```

It refuses mutating agents (Edit/Write) — running a writer twice would
double-apply edits. Cheap because GLM is cheap; strictly better than picking one
model for high-stakes read-only passes.

**Invocation path matters.** `provider` resolution fires only on the
`kei agent <name>` CLI path and `spawn_agent` (MCP). Claude Code's native `Agent`
tool runs sub-agents in-process on the *current* backend and ignores `provider`.
To actually route bulk work to GLM, dispatch via `kei agent --on=glm …` from the
shell, or set `kei primary glm` to move the whole session. The shipped default is
`kei primary claude` (orchestrator on Claude, bulk dispatched to GLM).

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

## First-run wizard (`kei onboard`, v0.45+)

After install, `bootstrap.sh` auto-triggers `kei onboard` if stdin is a TTY.
The wizard walks through:

1. Pick primary LLM orchestrator (claude / grok / agy / copilot / kimi)
2. Run `kei mcp-wire` to wire kei-mcp into all detected CLIs
3. Optional MOONSHOT_API_KEY hint for `kei limits` live polling
4. Run `kei-doctor` health check

Re-run any time: `kei onboard`. Skip auto-trigger on install: `KEI_NO_ONBOARD=1`.

## Orchestrator picker — `kei` no longer hardcodes claude

Without args, `kei` reads `~/.claude/config/primary.toml` and execs that CLI.
The picker lets you change it interactively:

```bash
kei pick               # interactive menu → set primary → launch it
kei                    # splash → exec the configured primary
kei --on=grok          # one-shot launch of grok (does NOT change primary)
kei primary grok       # set default to grok (no launch)
kei primary            # show current primary
```

The splash shows `primary CLI: <backend>` so you always know which orchestrator
will start. If the chosen primary isn't installed, `kei` prints the install
command and offers `kei pick` as recovery.

## Subscription quotas — `kei limits` (v0.43+)

```bash
kei limits             # human-readable report
kei limits --json      # machine-readable
kei limits --quiet     # cache-refresh only, no output
```

Research-grounded honest delivery: 4 of 5 CLIs have **no public programmatic
API for quota**. The command shows status markers + dashboard URLs for those.
Only Kimi exposes a balance API via Moonshot `/v1/users/me/balance` —
requires `MOONSHOT_API_KEY` env. The cache lives at
`~/.claude/pet/limits-cache.json`; the pet statusline reads it (does NOT
poll itself) and displays the Kimi balance segment when live.

## Cross-CLI sub-agent spawn via MCP — `spawn_agent`

`kei-mcp` exposes a built-in `spawn_agent` MCP tool. Any CLI that connects
to it as an MCP client can invoke KeiSeiKit agents on any backend, no matter
what the orchestrator is:

```jsonrpc
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "spawn_agent",
    "arguments": {
      "name": "critic",
      "task": "review src/auth.rs for race conditions",
      "on": "grok"
    }
  }
}
```

Internally `spawn_agent` shells out to `kei-agent-cli.sh` with the same DNA
resolution as `kei agent`. The `on` argument is optional — without it, the
backend is picked from the agent's manifest, then `primary.toml`, then claude.

**Why this matters:** Claude Code has a native `Agent` tool for sub-agent
spawning. Grok / Antigravity / Copilot / Kimi do NOT have that surface
natively — but they all support MCP. With `spawn_agent` exposed via kei-mcp,
**every backend that speaks MCP gets KeiSeiKit's sub-agent capability**. So
when Grok is your orchestrator, it can still spawn `critic` on Claude (or
`code-implementer` on Antigravity, or anything else) — the orchestrator
choice no longer caps your sub-agent surface.

Wire kei-mcp into the orchestrator's MCP config (each CLI has its own):

| CLI | MCP config |
|---|---|
| claude | `~/.claude/settings.json` `mcpServers` block |
| grok | `~/.grok/config.json` (or check `grok --help`) |
| agy | `~/.antigravity/mcp.json` (check `agy plugin list`) |
| copilot | `~/.copilot/mcp.json` (check `copilot --help`) |
| kimi | `kimi mcp add` subcommand |

Point each at `<kit>/_primitives/_rust/target/release/kei-mcp` (built via
`cargo build -p kei-mcp --release`).

## Rule enforcement — see also: cross-CLI policy

**Phase C delivered**: KeiSeiKit's safety hooks now have a 3-tier enforcement
model across CLIs. See [cross-cli-policy.md](./cross-cli-policy.md) for the
full matrix and `kei mcp-wire` setup. Short version: TIER 1 (full native)
on claude+grok, TIER 2 (MCP-wrapped) on copilot, TIER 3 (advisory) on agy+kimi.

## Rule enforcement caveat (READ THIS — pre-Phase-C view)

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
