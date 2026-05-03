# Architecture

How agents, blocks, manifests, the assembler, and cross-tool bridges fit together.

---

## The build pipeline

```
  Manifest (_manifests/<name>.toml)          <-- source of truth
         |
         |   [assembler/src/*.rs]              <-- Rust binary
         v
  Generated agent (.claude/agents/<name>.md)  <-- regenerated, never hand-edited
         ^
         |                                     [hook: assemble-agents]
  Block edit (_blocks/<block>.md)             <-- triggers rebuild of ALL agents
```

12 hooks enforce the pipeline (7 pipeline + 3 session-audit + 2 capability — `agent-capability-check` / `agent-capability-verify` wire the v0.24 substrate capability layer; see `hooks/` directory for details):

- **`assemble-agents`** (PostToolUse, Write/Edit) — rebuilds the affected agent(s) whenever a manifest or a block changes. No manual rebuild needed.
- **`assemble-validate`** (PreToolUse, Bash) — blocks `git commit` inside `~/.claude` if any manifest fails validation. Keeps the repo in a buildable state at all times.
- **`no-hand-edit-agents`** (PreToolUse, Edit/Write) — refuses edits to any `.md` under `~/.claude/agents/` that starts with the `<!-- GENERATED -->` marker, pointing you at the manifest instead. Override with `AGENT_MIGRATION=1` for emergencies only.
- **`tomd-preread`** (PreToolUse, Read) — auto-converts opaque binary formats (`.docx`, `.doc`, `.xlsx`, `.pptx`, `.csv`) to markdown via the `tomd` primitive and redirects Claude to read the cached `.md` instead.
- **`agent-fork-logger`** (PreToolUse, Agent) — RULE 0.12 advisory: logs every Agent subagent invocation to the `kei-ledger` SQLite DB so the orchestrator can validate the fork bundle. Never blocks; silent no-op if `kei-ledger` is absent.
- **`orchestrator-dirty-check`** (PreToolUse, Agent) — RULE 0.13 advisory: stderr-warns when `git status --porcelain` of the current repo is non-empty before spawning a sub-agent, so orchestrators don't compound uncommitted output across parallel agents. Never blocks; bypass with `ORCHESTRATOR_DIRTY_OK=1` (per-call) or `ORCHESTRATOR_META=1` (meta-orchestrator).
- **`site-wysiwyd-check`** (PostToolUse, Edit/Write) — on frontend-source edits (`.tsx`, `.vue`, `.svelte`, `.astro`, `.css`, `.html`, `.jsx`, `.ts`) in a project with a live dev server (`.keisei/dev-server.pid`), takes a Playwright screenshot via `mock-render` and diffs against `.keisei/target.png` via `visual-diff`. Advisory-only — drift is reported to stderr, never blocks.
- **`session-end-dump`** (Stop event) — RULE 0.14 self-audit: archives the session JSONL trace and ingests it into `kei-memory`.
- **`milestone-commit-hook`** (PostToolUse, Bash) — RULE 0.14 self-audit: appends a one-line session summary to `~/.claude/memory/audit-backlog.md` on every `feat:`/`refactor:`/merge commit.
- **`error-spike-detector`** (PostToolUse, any tool) — RULE 0.14 self-audit: tags + logs the pattern when 3+ errors occur within the last 20 tool calls.

## Creating a new agent

Run the wizard in Claude Code:

```
/new-agent
```

You'll be asked (via multiple option-picker batches, not free-text) — each batch groups several click-only questions into a single `AskUserQuestion` call:

1. Project stack (Rust CLI / axum / SwiftUI / Flutter / FastAPI / Next.js / React-Vite / Vue-Nuxt / SvelteKit / Astro / Go / Embedded / Python ML)
2. Deploy target (local-only / EC2 / Cloudflare / Modal / Docker / none)
3. Uses paid APIs? (Yes / No)
4. Contains ML? (Yes / No)
5. Has credentials? (Yes / No)
6. Uses scrapers? (None / Free-tier / Paid tier)

Then one free-text prompt for slug + description + path + gotchas. The wizard composes the manifest, validates it, assembles the `.md`, and prints a two-step git-commit command you can run or edit first.

## Adding custom blocks

Blocks are plain markdown in `~/.claude/agents/_blocks/`. To add one:

1. `touch ~/.claude/agents/_blocks/stack-mystack.md` and write the block.
2. Reference it in a manifest's `blocks = [...]` list.
3. The PostToolUse hook rebuilds the affected agent(s) automatically.

Blocks should be 10-50 lines, single-concern, and readable in isolation. If a block exceeds ~60 lines, split it into two.

## Adding custom manifests

Copy `_templates/specialist.toml.template` and fill the placeholders, OR run `/new-agent` and answer the wizard. Either way, the assembler validates the manifest and generates the `.md` on write.

## Agents overview

All kit agents are namespaced under `kei-*` so they won't collide with your own agents (e.g. your personal `validator` or `critic`) living in `~/.claude/agents/`.

| Agent | Role |
|---|---|
| `kei-code-implementer` | Write production code, Constructor Pattern enforced, Test-First discipline |
| `kei-infra-implementer` | Deploy scripts, CI/CD, secrets management, cost-aware paid infra |
| `kei-ml-implementer` | Training scripts, inference code, Modal jobs, exact param counts |
| `kei-critic` | Read-only anti-pattern / bug / security / perf / debt finder |
| `kei-validator` | Fact-checker; verifies API existence, version compat, citations, doc claims |
| `kei-security-auditor` | Risk-classified security audit with variant analysis + supply chain check |
| `kei-architect` | Read-only structural analysis; dep graph, patterns, coupling |
| `kei-researcher` | Generic web + codebase research, evidence-graded findings |
| `kei-ml-researcher` | ML literature, benchmarks, reproducibility, tooling-reuse search |
| `kei-cost-guardian` | Pre-launch GO/NO-GO for paid compute (Modal, AWS, fal.ai, Apify, etc.) |
| `kei-modal-runner` | Modal compute orchestrator with anti-stop guard (never stops running jobs) |
| `kei-fal-ai-runner` | fal.ai image/video/3D generation expert |

## Cross-tool bridges

KeiSeiKit ships 11 verified tool-bridge templates under `_bridges/`. Render them into any project and the same Constructor-Pattern ruleset is visible to every AI coding tool you use — no drift, one source of truth.

**Tools covered:**

| Tool | Output file |
|---|---|
| Cursor (legacy) | `.cursorrules` |
| Cursor (modern MDC) | `.cursor/rules/main.mdc` |
| Codex CLI / Warp / Zed / Antigravity fallback | `AGENTS.md` |
| GitHub Copilot | `.github/copilot-instructions.md` |
| Windsurf | `.windsurf/rules/main.md` |
| JetBrains Junie | `.junie/guidelines.md` |
| Continue.dev | `.continue/rules/main.md` |
| Google Antigravity / Gemini CLI | `GEMINI.md` |
| Aider | `CONVENTIONS.md` + `.aider.conf.yml` |
| Replit Agent | `replit.md` |

**Three ways to generate:**

1. **At install time** — `./install.sh --with-bridges` renders all 11 into `$PWD` after the normal install completes. Skipped if `$PWD` is the KeiSeiKit repo itself.
2. **From the `/new-agent` wizard** — Phase 8 asks click-only whether to generate all 11, just `AGENTS.md`, or skip.
3. **Manually, any time** — `~/.claude/agents/_bridges/emit.sh <project-dir>` (the install copies `_bridges/` into your agent fleet dir). Add `--only <output-path>` to restrict to a single file.

All paths are idempotent: existing bridge files in the project are skipped, never overwritten. See `_bridges/README.md` for the full template→output-path table.

## Meta-composer

`/compose-solution` is the meta-creator: tell it what you want to solve in one free-text paragraph, it decomposes the task, greps existing blocks / skills / manifests / primitives / bridges for prior art, proposes a minimal math-first architecture, and assembles the right artefact — agent, skill, hook, rule, block, or pipeline invocation. Every decision except the intake is a click (option-picker), never free-text.

Example: "I want a hook that blocks `rm -rf ~/` in any Bash call" → Phase 2 decomposes into (pattern-match, severity, event, wiki entry) → Phase 3 greps `hooks/`, `_blocks/`, `_primitives/` for prior art → Phase 5 proposes `hook = PreToolUse:Bash + pattern + exit 2` → Phase 7 hands off to `/escalate-recurrence` with severity and event pre-filled.

Phase 6 is the feedback loop: when a component has no prior art, the skill drafts a new `_blocks/<slug>.md` and — on your click — persists it. Next time `/compose-solution` (or `/new-agent`) runs, that block is discoverable. Every session leaves the kit a little smarter; the report prints `_blocks/` count before → after so the growth is visible.

See `skills/compose-solution/SKILL.md` and its phase files (`phase-1-intake.md` through `phase-7-assemble.md`) for the full 7-phase pipeline.

## Regenerating counts

Every number in the README / INSTALL.md (crates / skills / hooks / blocks / primitives / profile sizes) is wrapped in an HTML-comment marker — `<!-- count:NAME -->24<!-- /count:NAME -->` — and regenerated from sources of truth (`_primitives/MANIFEST.toml`, `_primitives/_rust/Cargo.toml`, filesystem walks). No more drift when a primitive or skill is added.

```bash
./scripts/regen-counts.sh            # rewrite README.md in place
./scripts/regen-counts.sh --check    # exit 1 if drift detected (no writes)
```

Pre-commit gate: `scripts/precommit-counts-check.sh` — wire it into your hook manager (or symlink into `.git/hooks/pre-commit`) to block commits when README counts drift from the sources.

## Workflow-file editing protocol

Every `.github/workflows/*.yml` edit is defended by three gates. The v0.20.1 incident (a real-but-wrong-semantic SHA pin on `dtolnay/rust-toolchain` broke CI for 30 minutes before discovery) motivated formalising them.

- **`scripts/lint-workflows.sh`** — runs [`actionlint`](https://github.com/rhysd/actionlint) over every workflow file. Catches syntax errors, expression typos, dead `if:` branches, and shell-injection risks. If the binary isn't on PATH, the script prints an install hint and exits 0 (advisory). Install with `bash scripts/install-actionlint.sh` or `brew install actionlint`.
- **`scripts/validate-workflow-shas.sh`** — extracts every `uses: <repo>@<sha40>` pin from `.github/workflows/*.yml` + `.github/dependabot.yml` and runs `git ls-remote https://github.com/<repo>.git <sha>`. A fabricated or force-pushed-out-of-existence SHA exits 1 with `SHA MISSING:`. Network errors are soft (`[UNVERIFIED]`). Tag refs like `@v4` or `@stable` are skipped (policy decision). Add trailing comment `# validate-workflow-shas: skip=<reason>` on a line to intentionally skip it.
- **CI job `workflow-lint`** — runs both scripts on every push and PR. Finishes in well under 30 s.
- **Optional pre-commit hook:** `ln -sf ../../scripts/pre-commit-workflow-lint.sh .git/hooks/pre-commit` — runs the two scripts only when a workflow file is staged.

SHA-pinning third-party actions defeats tag re-point attacks (CVE-2025-30066 class), but only if the SHA you wrote is real AND means what you think it means. `actionlint` catches the first class of mistake; `validate-workflow-shas.sh` catches the second. Together they close the window between local edit and CI-fail.

---

## Layer model (atoms, recipes, skills, frontends)

The substrate is layered. Each layer has a single contract; layers below are stable foundations for layers above.

```
┌──────────────────────────────────────────────────────────────────────────┐
│ Layer 4 — Frontends (where the user types)                               │
│   Claude Code plugin · cortex-ui (Svelte) · kei-tty (ratatui) ·          │
│   @keisei/vscode-cortex · MCP clients (Cline, OpenClaw, Cursor MCP)      │
├──────────────────────────────────────────────────────────────────────────┤
│ Layer 3 — Skills (markdown wizards)                                      │
│   45 `/commands` under skills/<name>/SKILL.md                            │
│   Each is an AskUserQuestion-driven phase-pipeline (5-9 phases typical)  │
├──────────────────────────────────────────────────────────────────────────┤
│ Layer 2 — Recipes (TOML DAGs)                                            │
│   Topo-sorted atom graphs with explicit JSON I/O between steps           │
│   Runtime: kei-pipe                                                      │
├──────────────────────────────────────────────────────────────────────────┤
│ Layer 1 — Primitives (cubes)                                             │
│   53 Rust crates + 13 shell primitives                                   │
│   Each ≤ 200 LOC per file, ≤ 30 LOC per function (Constructor Pattern)   │
├──────────────────────────────────────────────────────────────────────────┤
│ Layer 0 — Atoms (locked vocabulary)                                      │
│   13 verbs: INGEST PARSE TRANSFORM ENRICH VALIDATE DECIDE DISPATCH       │
│             PERSIST RETRIEVE SEQUENCE EMIT OBSERVE EXECUTE               │
│   Discovery: kei-atom-discovery walks _primitives/_rust/*/atoms/*.md     │
└──────────────────────────────────────────────────────────────────────────┘
```

The atom set is the substrate's locked vocabulary. New verbs require an explicit schema change (see [`SUBSTRATE-SCHEMA.md`](./SUBSTRATE-SCHEMA.md) and [`AGENT-SUBSTRATE-SCHEMA.md`](./AGENT-SUBSTRATE-SCHEMA.md)). New primitives, recipes, skills, and frontends compose without touching the lower layers.

### Atom → MCP tool

`kei-mcp` walks `_primitives/_rust/<crate>/atoms/<verb>.md` files and emits one MCP tool per atom — `<crate>::<verb>` is the tool name; the atom's first paragraph is the description; the atom's frontmatter `input_schema` is the JSON-Schema. The same atoms become callable from any MCP client (Claude Code, Cline, OpenClaw, Cursor with MCP) without rewiring. See [`_primitives/_rust/kei-mcp/README.md`](../_primitives/_rust/kei-mcp/README.md) for the wire format and the three client-config examples.

### Skill → MCP resource

The same `kei-mcp` server walks `skills/<name>/SKILL.md` and emits one MCP resource per skill at `skill://<name>`. MCP-aware clients can read the skill text without invoking it — useful for "explain this command before running it" UX flows.

### Frontend → daemon

The cortex stack inverts the flow for browser / terminal / editor usage. Instead of an LLM client calling MCP tools, a Rust daemon (`kei-cortex`) hosts the LLM loop and exposes a REST + WS surface to thin frontends. The token in `~/.keisei/cortex.token` is the only auth boundary.

```
  Browser (cortex-ui)        Terminal (kei-tty)      VSCode (vscode-cortex)
       │                            │                          │
       │      Authorization: Bearer <cortex.token>             │
       └────────────┬───────────────┴──────────────────────────┘
                    │
              http://127.0.0.1:9797
                    │
                kei-cortex (axum)
                    │
       ┌────────────┼────────────┬─────────────┐
       │            │            │             │
   Anthropic    ElevenLabs    fal.ai     local Python
    (chat)        (TTS)      (portrait)    (faster-whisper STT)
```

Every endpoint streams when streaming makes sense (`/chat` SSE, `/tts` audio chunks, `WS /term` PTY frames). The 8 built-in tools that the `/chat` agentic loop wires — `read`, `write`, `edit`, `bash`, `glob`, `grep`, `webfetch`, `agent` (see `_primitives/_rust/kei-cortex/src/tool/registry.rs`) — all execute locally; only the LLM call leaves the machine.

---

## Memory architecture (3-layer)

Three markdown layers — `~/.claude/CLAUDE.md` (algorithms, depends on nothing) → `~/.claude/memory/<project>.md` (per-project data, self-contained) → `~/.claude/memory/MEMORY.md` (≤ 200-line index, links only). Three SQLite files underlie them: `agents/ledger.sqlite` (RULE 0.12), `memory/sessions.sqlite` (JSONL + embeddings), `memory/kei-memory.sqlite` (RULE 0.14 pattern detector). All inspectable with `sqlite3` and `kei-brain-view summary`. Nothing in RAM-only.

---

## Sleep architecture (Phase A / B / C)

Three nightly phases, each addressing a different consolidation problem. Full spec in [`SLEEP-LAYER.md`](./SLEEP-LAYER.md).

```
03:00 local
  │
  ├─ Phase A — Incubation (the daytime task queue)
  │    Up to 5 user-submitted tasks from /sleep-on-it, ≤ 480 min total.
  │    Marathon mode: 1 task gets the whole night.
  │    Per-task budgets: quick 15 / standard 60 / deep 240 / marathon 480.
  │    Checkpointing every N min so a cut-short run leaves an artefact.
  │    Output: sleep-results/<uuid>.md, sleep-results/<uuid>.partial.md
  │
  ├─ Phase B — REM consolidation (cross-session pattern report)
  │    Agent reads the day's traces, finds recurrences, writes
  │    sleep-reports/YYYY-MM-DD.md. User reads in the morning.
  │    Skipped on marathon nights.
  │
  └─ Phase C — NREM deep-sleep (system consolidation, every 7 days)
       kei-conflict-scan + kei-refactor-engine + kei-graph-check.
       Output: deep-sleep/YYYY-MM-DD branch (plan + optional patch).
       Zero-conflict guarantee: any conflict requiring human decision
       is excluded from the patch and listed explicitly in the plan.
```

The user always reads. Nothing the cloud agent writes is auto-injected into the next session — codification goes through `/escalate-recurrence`.

---

## Substrate dogfood (kei-fork → kei-ledger → kei-spawn)

Every non-trivial agent invocation is itself a substrate transaction. The lifecycle:

```
kei-fork create <agent-slug>
       │
       │  - branch: agent/<slug>-<timestamp>
       │  - worktree: _forks/<slug>-<ts>/
       │  - ledger row: (agent_id, branch, parent, status=running)
       │  - DNA: <role>::<caps>::<scope8>::<body8>-<nonce8>
       ▼
agent writes 6 artefacts inside the worktree:
  spec.md plan.md progress.json chatlog.md handoffs.md review.md
       │
       │  - touches .DONE marker on completion
       │  - kei-watch hook auto-collects
       ▼
kei-fork collect
       │
       │  - merge or squash decision via AskUserQuestion (RULE 0.12)
       │  - ledger row → status=merged|rejected|deferred
       │  - worktree archived: _forks/_archive/...
       ▼
kei-ledger validate         ← optional gate before merge
kei-replay <agent_id>       ← reconstruct the spawn from DNA + ledger
```

Per RULE 0.13, the *orchestrator* (parent agent or main session) creates the branch, commits, and pushes. The child agent only writes files inside the worktree — it cannot invoke git from there because the sandbox denies Bash inside `.claude/worktrees/<agent>/` by design. The 6-file artefact bundle is the agent's full audit trail.

---

## Model router — current state (2026-05-03)

The `kei-model-router` crate implements a Bayesian-posterior tier
selector (Haiku / Sonnet / Opus) keyed on task-class DNA + a Beta
posterior over per-(task-class, model) success rates. The companion
`kei-token-tracker` crate logs `TokenEvent` rows per LLM call to
SQLite.

What this is and is not, today:

- **It is** a long-running learning loop. The active-learning consumer
  reads outcome rows from `kei-token-tracker` and updates Beta-posterior
  parameters per (task-class, model) pair. As more outcomes accumulate,
  the ranking deviates from the manifest-declared default.
- **It is not** "smart routing on day one". A fresh install has **0**
  outcome rows. Until at least N≈100 outcomes per task-class accumulate
  in production, the router falls back to the model declared in the
  agent manifest's `model:` frontmatter. With 37 agent manifests
  currently declaring `model: opus`, the practical effect on a fresh
  install is "always Opus" — the router's posterior has no data to
  override the default with.
- **Outcome-row count for a fresh install: 0.** Plan to run for some
  weeks under realistic load before the router meaningfully reorders
  tier selection. Until then, route by orchestrator discipline +
  advisor-hook stderr nudges, exactly as the README's "Honest limits"
  section calls out.

The Beta-posterior + cost-minimisation math is in
`_primitives/_rust/kei-model-router/src/`. The aggregation surface
(per-model cost / day, sleep-report markdown emitter) is in
`kei-token-tracker`.

---

## Git model summary

Cross-references to the rules that govern git state:

- **RULE 0.12** — agent-fork lifecycle (6-file bundle + ledger row + merge ceremony).
- **RULE 0.13** — orchestrator owns branch + commits + pushes; agents write files only.

The `checkpoint:`, `feat:`, `refactor:`, `audit:`, and `fix:` commit prefixes are SSoT'd in [`~/.claude/rules/git-conventions.md`](https://example.invalid/git-conventions). `kei-changelog` walks Conventional Commits to regenerate `CHANGELOG.md` blocks; `kei-fork validate` checks the 6-file bundle on every merge.
