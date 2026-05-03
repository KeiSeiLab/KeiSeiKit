# KeiSeiKit

A **multi-LLM substrate** that gives any agentic coding tool persistent
memory, deterministic agent identity, and self-maintaining orchestration.
Works first-class with Claude Code; MCP-compatible bridges generate
context for Cursor / Continue / Zed / Aider / Windsurf / Cline /
OpenClaw / Kimi from the same source-of-truth.

**Apache 2.0** — explicit patent grant + retaliation clause. 102 Rust
crates (~132K LOC), 67 skills, 35 hooks, 37 agent manifests, 82
substrate blocks, 18 capability bundles, 7 substrate roles. Self-
indexing via kei-registry SQLite (currently 495 active DNAs across the
public substrate). Three-phase nightly consolidation. Foreign-project
ingestion runtime (`kei-import <repo-url>`).

## Maturity matrix

The substrate ships as a layered set of components at different
maturity levels. Read this before relying on any single piece for
production work.

| Component | Status | Notes |
|---|---|---|
| 24+ Rust primitives | varies (alpha → beta → concept) | Inspect each crate's `Cargo.toml` `package.metadata.keisei.maturity` if declared; otherwise treat as **alpha** unless you've personally exercised it. Most primitives are alpha — they build, type-check, and have unit tests, but have not been hardened against adversarial input or run at scale. |
| Cortex daemon (`kei-cortex` HTTP + WS) | alpha | CLI-driven daemon works in author's daily use; HTTP REST + WS endpoints + 8-tool `/chat` agentic loop build clean. **Browser app (`cortex-ui`) and VSCode extension (`@keisei/vscode-cortex`) are concept-level** — scaffolds present, not production paths. |
| MCP server (`@keisei84/mcp-server`) | alpha | Published to **GitHub Packages** (`https://npm.pkg.github.com/`) under the `@keisei84` scope (matches the github org `KeiSei84`). Configure your `~/.npmrc` per [`docs/PUBLISHING.md`](./docs/PUBLISHING.md), then `npm install @keisei84/mcp-server`. Local dist build still works for development (see Quick start). |
| Sleep layer (Phase A / B / C) | alpha | Phase A queue (`/sleep-on-it` → cloud agent) + Phase B markdown morning report work. **Auto-codification of rules from sleep insights is not yet wired** — codification path is manual via `/escalate-recurrence`. Phase C deep-sleep refactor proposals run on a 7-day cadence and write plan-only markdown by default. |
| Hooks (35 shipped) | beta | Tested in author's daily use (4–8 parallel Claude Code terminals). Pipeline hooks (`assemble-agents`, `no-hand-edit-agents`) are load-bearing; advisory hooks (RULE 0.12 / 0.13 / 0.14) are non-blocking. |
| Skills + manifests + assembler | beta | Structured + `assembler-validate` gate runs on every `git commit` inside `~/.claude`. Schema is locked (see [`docs/AGENT-SCHEMA-LOCKED.md`](./docs/AGENT-SCHEMA-LOCKED.md)). |

## What it does

| | |
|---|---|
| **Persistent memory** | SQLite ledger + content-addressable memory store, session-spanning context, cross-machine sync via memory-repo |
| **Agent DNA** | Deterministic 80-char identity per invocation: `<role>::<caps>::<scope-sha8>::<body-sha8>-<nonce>`. Same task → same prefix → "did this run before?" via SQL, no embeddings |
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

```bash
# Claude Code (primary target — full hook + agent integration)
/plugin marketplace add KeiSei84/KeiSeiKit
/plugin install keisei@keisei-marketplace

# Any MCP-compatible client (Cursor / Continue / Zed / Aider / etc)
git clone https://github.com/KeiSei84/KeiSeiKit-1.0
cd KeiSeiKit-1.0
./install.sh --profile=minimal
```

37 agents + 67 skills + 35 hooks + nightly consolidation wired in
60 seconds. Eleven install profiles (`minimal` → `core` → `full` +
MCP-only / Cortex / Cursor / Continue / Zed / Aider / Docker / Nix)
documented in [`docs/INSTALL.md`](./docs/INSTALL.md).

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
  Bayesian posterior router activates per-task-class when ≥100
  outcome rows accumulate (currently 3). Until then, routing happens
  via orchestrator discipline plus advisor-hook stderr nudges.
- **Cortex stack** (`kei-cortex` / `kei-tty` / `kei-mcp`) ships as
  **alpha** (CLI/daemon track) — downgraded from "beta" because two
  of the three intended frontends are not yet shipping. Local HTTP
  daemon + ratatui TUI + MCP stdio JSON-RPC build clean and run in
  the author's daily use. **Browser app (`cortex-ui`) and VSCode
  extension (`@keisei/vscode-cortex`) are concept-level only** —
  scaffolds exist, no production wiring. Treat the daemon + CLI as
  the supported surface; treat the GUI frontends as roadmap.
- **`@keisei84/mcp-server` npm package** — published via **GitHub
  Packages**. To install from the registry:
  ```bash
  # ~/.npmrc — one-time setup
  echo "@keisei84:registry=https://npm.pkg.github.com/" >> ~/.npmrc
  echo "//npm.pkg.github.com/:_authToken=<your-github-pat>" >> ~/.npmrc
  # PAT scope: read:packages (write:packages only if you publish)

  npm install @keisei84/mcp-server
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
  `publishConfig.registry` pinned to `npm.pkg.github.com` so an
  accidental `npm publish` from this repo cannot route to npm.org.
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

Stack: **Rust core** (102 crates, ≤2 MB each, 12-trait runtime + plugin
registry) + **TypeScript glue** (6 adapters: gmail / grok / recall /
telegram / youtube / mcp-server). Backend impls cover:

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

## Author & collaboration

Built by Denis Parfionovich (`info@greendragon.info`) running
4–8 parallel Claude Code terminals per day. Solo-maintained.
Apache 2.0 makes the bus factor manageable: any AI-assisted
developer (you, your Claude, your Cursor, your Aider) can read
this codebase and continue it.

**Forks welcome. PRs welcome. Issues welcome.**

**Open to collaboration.** If you have:
- a use-case this substrate would solve and you can't see how — open
  a discussion
- ideas for the SaaS roadmap (cross-machine memory sync, hosted
  nightly consolidation, encyclopedia-as-API) — email or open an issue
- a related project you're building (agent infra, MCP servers,
  cross-tool bridges, prompt-engineering substrates) and want to
  cross-pollinate — reach out
- want to integrate KeiSeiKit primitives into your product or
  research — Apache 2.0 already permits it; happy to help you wire it

Email reaches the author directly. No marketing list, no funnel.
