# KeiSeiKit

A **multi-LLM substrate** for agentic coding. Same agent definition,
any LLM backend — Claude Code, Grok, Antigravity (Gemini), GitHub
Copilot, or Kimi. Pick your orchestrator with `kei pick`; agents
spawn sub-agents on other LLMs via MCP `spawn_agent`; safety hooks
enforce on every backend through a 3-tier model. Three-phase nightly
sleep consolidates 30-session windows into morning markdown reports.

**Apache 2.0** — explicit patent grant + retaliation clause.

## Highlights

- **5 LLM CLIs unified.** Claude Code (native hooks), Grok (port to
  `~/.grok/settings.json`), Antigravity/Gemini, GitHub Copilot, Kimi.
  DNA-routed: each agent's manifest declares a `provider`;
  `kei agent <name>` resolves DNA → primary → claude fallback.
- **Sub-agents on any backend.** Agents call `spawn_agent` (built-in MCP
  tool in `kei-mcp`) to dispatch other agents to whichever LLM fits
  the task. Cross-CLI orchestration without lock-in — Grok can spawn
  critic@Claude, then ml-implementer@Gemini, all from one session.
- **3-tier policy enforcement.** Claude + Grok TIER 1 (full native
  PreToolUse), Copilot TIER 2 (MCP-wrapped + `--excluded-tools=shell`),
  Agy + Kimi TIER 3 (advisory). `no-github-push`, `safety-guard`,
  `destructive-guard`, `citation-verify`, `numeric-claims-guard`
  surface on every backend that supports tool-call gating.
- **Three-phase nightly sleep.** Phase A (incubation — queued tasks
  via `/sleep-on-it`), Phase B (REM consolidation — analyzes last 30
  sessions, writes morning markdown), Phase C (NREM deep-sleep, every
  7 days — conflict scan + refactor proposals). Outputs are markdown;
  you decide what merges.
- **Native token streaming.** Each backend streams in its own print
  mode (`claude -p`, `grok --print`, `agy --print`, `copilot --prompt`);
  KeiSeiKit composes the agent prompt + task and passes through. No
  buffering layer.
- **Persistent memory.** SQLite ledger + content-addressable store,
  session-spanning context, cross-machine sync via memory-repo.
- **Agent DNA.** Deterministic variable-length identity per
  invocation: `<role>::<caps>::<scope-sha8>::<body-sha8>-<nonce8>`.
  Same task → same prefix → "did this run before?" via SQL, no
  embeddings.
- **Constructor Pattern.** Substrate, not framework. You compose; it
  doesn't dictate workflow. File >200 LOC → decompose. No mixins, no
  DI containers, no abstract factories.
- **Self-maintaining.** Every substrate edit cascades: registry
  updates, agent regeneration, DNA index refresh, keimd graph
  reindex. Auto-self-indexing via kei-registry SQLite.

## By the numbers (v0.51.1)

110 Rust crates · 69 skills · 54 hooks · 38 agent manifests ·
86 substrate blocks · 18 capability atoms · 7 substrate roles ·
565 indexed DNAs · 6 install profiles (minimal → full).

## Platforms

- **macOS** (arm64 + x64) — fully supported, primary dev target.
- **Linux** (Ubuntu, Debian, Fedora, Arch — x64 + arm64) — fully supported.
- **Windows** — substrate itself is Bash-only, but the **MCP server binary**
  ships as `kei-mcp-server-windows-x64.exe` in every release. Three
  recommended paths:
  - **WSL2** (recommended) — install Windows Subsystem for Linux,
    then run `bootstrap.sh` inside Ubuntu/Debian as normal. Full
    substrate works. `bootstrap.sh` auto-detects WSL2 and Git Bash;
    on bare Windows it prints a one-time WSL setup guide and copies
    the `wsl --install` command to your clipboard.
  - **MCP-only** — drop `kei-mcp-server-windows-x64.exe` into your
    Claude Desktop / VS Code MCP config to get `spawn_agent` +
    `kei_bash`/`kei_edit`/`kei_write` tools, without the full
    Bash-based substrate. Skills, hooks, and `kei` CLI not available
    in this mode.
  - **Native PowerShell port** — demand-driven. WSL gives 100%
    coverage today with 0 code duplication, so a native `.ps1`
    substrate isn't built yet. If you want it, open an issue with
    a thumbs-up on the existing Windows-native tracker (or file
    one) — once demand is real, we'll build it. The MCP-server
    binary path already covers the common "just want spawn_agent
    in Claude Desktop" case.

## Maturity matrix

The substrate ships as a layered set of components at different
maturity levels. Read this before relying on any single piece for
production work.

| Component | Status | Notes |
|---|---|---|
| 24+ Rust primitives | varies (alpha → beta → concept) | Inspect each crate's `Cargo.toml` `package.metadata.keisei.maturity` if declared; otherwise treat as **alpha** unless you've personally exercised it. Most primitives are alpha — they build, type-check, and have unit tests, but have not been hardened against adversarial input or run at scale. |
| Cortex daemon (`kei-cortex` HTTP + WS) | alpha | CLI-driven daemon works in author's daily use; HTTP REST + WS endpoints + 8-tool `/chat` agentic loop build clean. **Browser app (`cortex-ui`) and VSCode extension (`@keisei/vscode-cortex`) are concept-level** — scaffolds present, not production paths. |
| MCP server (`@keisei/mcp-server`) | alpha | Published to **keigit.com** (`https://keigit.com/api/packages/keisei/npm/`) — author-operated Forgejo npm registry on a public DNS. Configure your `~/.npmrc` per [`docs/PUBLISHING.md`](./docs/PUBLISHING.md), then `npm install @keisei/mcp-server`. Local dist build still works for development (see Quick start). |
| Sleep layer (Phase A / B / C) | alpha | Phase A queue (`/sleep-on-it` → cloud agent) + Phase B markdown morning report work. **Auto-codification of rules from sleep insights is not yet wired** — codification path is manual via `/escalate-recurrence`. Phase C deep-sleep refactor proposals run on a 7-day cadence and write plan-only markdown by default. |
| Hooks (35 shipped) | beta | Tested in author's daily use (4–8 parallel Claude Code terminals). Pipeline hooks (`assemble-agents`, `no-hand-edit-agents`) are load-bearing; advisory hooks (RULE 0.12 / 0.13 / 0.14) are non-blocking. |
| Skills + manifests + assembler | beta | Structured + `assembler-validate` gate runs on every `git commit` inside `~/.claude`. Schema is locked (see [`docs/AGENT-SCHEMA-LOCKED.md`](./docs/AGENT-SCHEMA-LOCKED.md)). |

## What it does

| | |
|---|---|
| **Persistent memory** | SQLite ledger + content-addressable memory store, session-spanning context, cross-machine sync via memory-repo |
| **Agent DNA** | Deterministic variable-length identity per invocation: `<role>::<caps>::<scope-sha8>::<body-sha8>-<nonce8>` (≥33 chars; role + caps slugs are variable). Same task → same prefix → "did this run before?" via SQL, no embeddings. See [`docs/DNA-FORMAT.md`](./docs/DNA-FORMAT.md) for the wire spec. |
| **Constructor Pattern for prompts** | Agent `.md` files composed from manifests + blocks + capability bundles + rule fragments. Edit a block → all agents using it recompose. Single source of truth |
| **kei-fork** | Atomic git triplet (branch + worktree + ledger row) for parallel agent runs. Atomic rollback. No main-branch collisions across 4-8 simultaneous Claude sessions |
| **Three-phase sleep** | Phase A incubation (queued tasks) → Phase B REM consolidation (analyzes last 30 sessions, writes morning markdown report) → Phase C NREM deep-sleep (every 7 days, conflict scan + refactor proposals). No feedback loop — outputs are markdown, you decide what to keep |
| **Auto self-indexing** | Every substrate file edit triggers registry update + agent regeneration + DNA-INDEX.md refresh + keimd graph reindex |
| **Foreign-project ingestion** | `kei-import <repo>` walks → matches against 12 runtime traits → extracts skills from README/docs → generates migration plan → produces per-phase agent prompts |
| **Cross-tool bridges** | One rule-set, 11 target formats (`.cursorrules`, `.windsurf/rules/main.md`, `.github/copilot-instructions.md`, `AGENTS.md`, `GEMINI.md`, etc) |
| **npm-style publishing path** | Publish your agents / skills / hooks as scoped packages. The author runs an opt-in mirror at [`keigit.com`](https://keigit.com) (public Forgejo + npm registry, OAuth, per-user PAT) — this is an **author-operated mirror (KeiSei84 / private Forgejo)**, not a neutral community service. The substrate is remote-agnostic; use any git remote and any npm registry you trust. See [`docs/PUBLISHING.md`](./docs/PUBLISHING.md) |

## Why it exists

The author runs 4-8 parallel Claude Code terminals daily. Without
substrate, every session loses context, every parallel agent collides
on `main`, every "did we already solve this?" requires manual grep.
With substrate, identity carries — agents know what ran before,
results converge through the ledger, fork-as-triplet prevents
collisions, three-phase sleep produces overnight consolidation.

This is a tool first, not a product. If it solves your problem,
fork it.

## Quick start

**Pick ONE** of the four entry points below — they all do the same thing.

### 1. Web installer (anonymous-friendly, ONE line)

```bash
curl -fsSL https://install.keisei.app | bash
```

CI / non-interactive variant:

```bash
curl -fsSL https://install.keisei.app | bash -s -- --profile=full --yes
```

### 2. Claude Code plugin

```
/plugin marketplace add KeiSeiLab/KeiSeiKit-1.0
/plugin install keisei@keisei-marketplace
```

### 3. GitHub clone (private repo — needs `gh auth login` first)

```bash
gh auth login
gh repo clone KeiSeiLab/KeiSeiKit-1.0 ~/keisei
cd ~/keisei && ./bootstrap.sh --profile=full
```

### 4. Manual (any MCP-compatible client — Cursor / Continue / Zed / Aider)

```bash
git clone https://github.com/KeiSeiLab/KeiSeiKit-1.0.git
cd KeiSeiKit-1.0 && ./bootstrap.sh
```

`web-install.sh` (served at `install.keisei.app`) is a thin curl-pipeable
wrapper that clones the repo and delegates to `bootstrap.sh` — single
source of truth, no duplicated install logic.

38 agents + 69 skills + 54 hooks + nightly consolidation wired in
~60 seconds. Twelve install profiles (`outcome-only`, `minimal`,
`core`, `frontend`, `ops`, `dev`, `mcp`, `cortex`, `local-mirror`,
`dashboard`, `full-hub`, `full`) defined in
`_primitives/MANIFEST.toml` and documented in
[`docs/INSTALL.md`](./docs/INSTALL.md). For non-Claude-Code clients
(Cursor / Continue / Zed / Aider) the bridges format the same source
into client-native config — those are bridge targets, not separate
profiles.

## Post-install — the `kei` CLI

After install, `kei` is the substrate entrypoint. On first interactive
run an onboarding wizard walks you through picking a primary LLM
orchestrator and wiring kei-mcp into the CLIs you have installed:

```bash
kei                              # launch primary CLI (default: claude)
kei onboard                      # post-install wizard (re-runnable)
kei pick                         # interactive primary picker
kei primary [<backend>]          # get/set primary LLM provider

kei agent <name> "<task>"        # invoke agent: backend from DNA → primary
kei agent --on=grok <name> "..." # invoke agent on a specific backend
kei run-via <backend> <name> "<task>"   # explicit-backend dispatch

kei mcp-wire                     # wire kei-mcp into all installed CLIs
kei mcp-wire --list              # show enforcement tier per CLI

kei limits                       # honest subscription-quota report
                                 # (4 of 5 CLIs have no public API)

kei configure                    # re-pick hook packs + stack profile
kei message ...                  # cross-session mailbox
kei --status                     # splash with substrate health
```

### Multi-LLM agent dispatch

Agents are markdown prompts that can be served by ANY of 5 supported
CLIs (Claude Code, Grok, Antigravity-Gemini, GitHub Copilot, Kimi).
Each agent's manifest may declare a `provider` field that becomes its
DNA; `kei agent <name>` then routes to that provider automatically.
See [`docs/encyclopedia/multi-cli-agents.md`](./docs/encyclopedia/multi-cli-agents.md).

### Cross-CLI policy enforcement

KeiSeiKit's safety hooks (`no-github-push`, `safety-guard`,
`destructive-guard`, `citation-verify`, `numeric-claims-guard`) extend
to non-Claude CLIs through a 3-tier enforcement model:

- **TIER 1 — full native**: Claude (existing) + Grok (ports our hooks to `~/.grok/settings.json`)
- **TIER 2 — MCP-wrapped**: Copilot (`--excluded-tools=shell` + force `kei_bash` via MCP)
- **TIER 3 — advisory**: Agy + Kimi (cannot disable native shell; prompt-level only)

See [`docs/encyclopedia/cross-cli-policy.md`](./docs/encyclopedia/cross-cli-policy.md)
for the full matrix + setup.

### Outcome-only — try just the outcome loop (5 files, ~200 LOC)

If you want to try only the outcome-tracking primitive without
committing to the full kit (no daemon, no Forgejo, no launchd, no 100
crates), run `./install.sh --profile=outcome-only`. Installs 2 hooks +
a SQLite ledger + one line in `~/.claude/CLAUDE.md`; uninstalls in
four lines. See [`docs/PROFILE-OUTCOME-ONLY.md`](./docs/PROFILE-OUTCOME-ONLY.md).

## Self-maintaining

After install, the substrate maintains itself. Every edit cascades:

```
edit any rule .md       → kei-decompose registers fragments
edit any manifest .toml → assembler regenerates one agent .md
edit any block .md      → assembler regenerates ALL agents
edit any skill SKILL.md → kei-registry updates
edit any hook .sh       → kei-registry updates
edit any primitive src/ → kei-import-project register updates
ANY substrate edit      → DNA-INDEX.md auto-refreshes
ANY substrate edit      → keimd graph auto-reindexes

nightly:
  Phase A (incubation)         → process queued tasks
  Phase B (REM consolidation)  → analyze last 30 sessions → morning report
  Phase C (NREM, every 7d)     → conflict scan + refactor proposals
```

**No automatic feedback loop into agent state.** All consolidation
outputs are human-readable markdown. You read, you decide what merges.

## Honest limits

- **Phase 5 executor (`kei-import-project`)** generates per-phase
  agent prompts as JSON; the actual `Agent({...})` spawn happens
  orchestrator-side (Claude Code Agent tool, MCP wrapper, or a thin
  shell loop). A first-class JS/TS wrapper that auto-spawns + tracks
  is future work.
- **Phase 9 Path A (model-router assembler-time rebake)** —
  37 agent manifests currently declare `model: opus` in frontmatter.
  The router uses a Beta posterior with Wilson-style lower confidence
  bound (`δ=0.10`, `q*=0.70`); it falls back to the manifest-declared
  default until the per-(task-class, model) lower-bound clears the
  quality bar — typically tens of successful observations per pair,
  not a discrete 100-row threshold (see
  `_primitives/_rust/kei-model-router/src/select.rs:74-124`). 3 outcome
  rows total today, posterior dominated by uniform prior `Beta(1,1)`.
- **Cortex stack** (`kei-cortex` / `kei-tty` / `kei-mcp`) ships as
  **alpha** (CLI/daemon track) — downgraded from "beta" because two
  of the three intended frontends are not yet shipping. Local HTTP
  daemon + ratatui TUI + MCP stdio JSON-RPC build clean and run in
  the author's daily use. **Browser app (`cortex-ui`) and VSCode
  extension (`@keisei/vscode-cortex`) are concept-level only** —
  scaffolds exist, no production wiring. Treat the daemon + CLI as
  the supported surface; treat the GUI frontends as roadmap.
- **`@keisei/mcp-server` npm package** — published to **keigit.com**
  (the author-operated Forgejo npm registry, public DNS at
  [`keigit.com`](https://keigit.com)). To install from the registry:
  ```bash
  # ~/.npmrc — one-time setup
  echo "@keisei:registry=https://keigit.com/api/packages/keisei/npm/" >> ~/.npmrc
  echo "//keigit.com/:_authToken=<your-keigit-PAT>" >> ~/.npmrc
  # PAT scope: read:package (write:package only if you publish)

  npm install @keisei/mcp-server
  ```
  For local development without the registry round-trip:
  ```bash
  cd _ts_packages
  bun install && bun run -r build
  # output: _ts_packages/packages/mcp-server/dist/index.js
  ```
  Single-binary builds via `bun build --compile` are documented in
  [`_ts_packages/packages/mcp-server/BUILD.md`](./_ts_packages/packages/mcp-server/BUILD.md)
  (5-target matrix, ~85–95 MB per binary). `package.json` has
  `publishConfig.registry` pinned to `keigit.com` so an accidental
  `npm publish` from this repo cannot route to npm.org.
- **Non-Claude clients** integrate via MCP + bridges, not native hooks.
  PreToolUse / PostToolUse / UserPromptSubmit / Stop semantics are
  Claude Code primitives. Other clients get capability exposure but
  not the hook wire-up.

## What it's NOT

- **Not a Claude Code replacement** — runs alongside, not instead-of
- **Not a SaaS** — local-first by default; hosted offering under
  consideration if community demand emerges (see [Roadmap](#roadmap))
- **Not enterprise** — solo-maintained, no SLA, no dedicated support
- **Not a framework** — substrate. You compose; it doesn't dictate
  workflow

## Roadmap

The substrate is functionally complete for solo-developer use. What
*might* be valuable as a hosted service if there's demand:

- **Cross-machine memory sync** — DNA-indexed memory available across
  laptop + desktop + cloud Claude session
- **Hosted Phase B/C nightly** — traces consolidated by a remote agent,
  morning report delivered to inbox
- **Encyclopedia search-as-API** — query team substrate by DNA / role
  / capability across multiple agents

These are **considered, not committed**. Open an issue with your
use-case if any of these would solve real pain. Until then: fork,
run locally, file PRs.

## Hermes — proof of foreign-architecture ingest

Ten phases of [Nous Research's Hermes](https://github.com/NousResearch/hermes-agent)
(MIT, Python agent framework) ingested into KeiSeiKit substrate
through April 2026. Each Hermes concept lives as a KeiSeiKit primitive:

| Hermes phase | KeiSeiKit landing |
|---|---|
| ShareGPT trajectory export | `kei-export-trajectories` crate |
| OpenAI-compat HTTP server | `kei-llm-router` providers + chat handler |
| Daytona sandbox backend | `kei-backend-daytona` (with toolbox proxy URL split) |
| Injection-guard on memory writes | wired through `kei-memory::ingest` + `kei-pet::memory` |
| Memory-nudge invoker | `Invoker` trait + `MemoryStore` Arc plumbed |
| `SKILL.md` skill format | `kei-skills::SkillRegistry`, consumed by `kei-mcp` |
| Skill-invocation aggregation | `kei-ledger` schema v8 + `aggregate-skills` CLI |
| Multi-platform gateway | `kei-gateway` (Telegram / Discord / Slack / CLI) |
| Cron / scheduler | `kei-cron-scheduler` parser+job+runner |

The `kei-import` umbrella runs the same pipeline (decompose → match
→ extract-skills → plan → execute) on any Rust / TS / Python / Go
repo. Hermes was the validation case; the runtime works on others.

## Frontend design — anti-AI-slop philosophy

The `frontend-design` skill is a deliberate counter-position to the
same-shape output of v0 / Lovable / Bolt:

- **10 archetypes** — Editorial / Swiss / Brutalist / Minimal /
  Maximalist / Retro-Futuristic / Organic / Industrial / Art Deco /
  Lo-Fi. Each declares typography pairing + color palette + layout
  language + motion style.
- **OKLCH color system** — one `--brand-hue` controls the full palette,
  perceptually uniform.
- **Phase Gate (mandatory before any code):** purpose, archetype, the
  one differentiator, three anti-references, design tokens. Skip the
  gate = skip the skill.
- **Hard bans:** Inter / Roboto / Space Grotesk, purple gradients on
  white, centered card grids as default, hero → cards → testimonials
  template, `linear` easing on UI transitions.
- **Diverge-Kill-Mutate** loop when output feels generic.
- **The Blur Test:** at 20% visibility, layout silhouette must be
  distinguishable from anti-references.

Orchestrator skill `landing-page` composes 11 skills across 6 recipes
(apple-product / saas / portfolio / ecommerce / agency / startup).

## Architecture

Stack: **Rust core** (105 workspace crates, ≤2 MB each, 12-trait runtime
+ plugin registry) + **TypeScript glue** (6 adapters: gmail / grok /
recall / telegram / youtube / mcp-server). Backend impls cover:

| Trait | Impls |
|---|---|
| ComputeProvider | bare-metal SSH, DigitalOcean, Linode, Vultr |
| GitProvider | Forgejo, Gitea, GitLab, Bitbucket |
| MemoryBackend | SQLite, Sled, Postgres, Redis |
| AuthProvider | Google OIDC, Apple Sign-In, WebAuthn passkeys, magic-link |
| NotifyChannel | Telegram, Discord, Slack, SMS (Twilio) |
| NetworkMode | WireGuard, OpenVPN, IPsec |
| LlmBackend | Anthropic, OpenAI, Kimi (Moonshot), MLX, llama.cpp, Ollama |
| ServiceManager | systemd |

Declare which impl to use in `~/.keisei/config.toml`; runtime resolves
at startup. See [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md),
[`docs/PHILOSOPHY.md`](./docs/PHILOSOPHY.md),
[`docs/SUBSTRATE-SCHEMA.md`](./docs/SUBSTRATE-SCHEMA.md),
[`docs/IMPORT-RUNTIME.md`](./docs/IMPORT-RUNTIME.md),
[`docs/PUBLISHING.md`](./docs/PUBLISHING.md),
[`docs/RULES-AS-BLOCKS.md`](./docs/RULES-AS-BLOCKS.md),
[`docs/DNA-INDEX.md`](./docs/DNA-INDEX.md).

## License

Apache 2.0. Use, fork, ship, modify. Explicit patent grant +
retaliation clause: contributors who sue any user over patents
covered by their contributions lose their license to the work.
Pre-2026-04-30 versions remain available under their original MIT
terms (irrevocable). See [LICENSE](./LICENSE) and [NOTICE](./NOTICE).

<!--
Author / collaboration section removed — to be written by hand.
TODO: replace this comment with the section you want.
-->

