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
| **Community npm registry** | Publish your agents / skills / hooks as scoped packages on [`keigit.com`](https://keigit.com) (public Forgejo + npm registry, OAuth login, per-user PAT). `npm publish` to your own scope, `npm install` from anyone else's. See [`docs/PUBLISHING.md`](./docs/PUBLISHING.md) |

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
  **beta**. Local HTTP daemon + ratatui TUI + MCP stdio JSON-RPC
  build clean. Browser app and VSCode-extension frontends are concept.
- **`@keisei/mcp-server` npm package** — local `dist/` builds work;
  not yet published to npm registry.
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
