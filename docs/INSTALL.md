# Installation

Complete install guide. Quick-start lives in the main [README](../README.md#install).

---

## Two install paths

| Path | Command | Best for |
|---|---|---|
| **Plugin** (v0.16+, recommended on Claude Code 2.1+) | `/plugin marketplace add KeiSei84/KeiSeiKit` then `/plugin install keisei@keisei-marketplace` | Agents + skills + hooks + MCP. Zero cargo build. See [PLUGIN.md](../PLUGIN.md). |
| **Classic** `./install.sh` | Below | Full kit incl. 47 Rust primitives + 13 shell primitives. Required for `ops` / `dev` / `full` profiles. |

## Prerequisites

**Hard** (needed for every install, regardless of profile):

- **Rust** (stable toolchain) — the assembler Cargo workspace is always built
- **jq** — used by the shell hooks for JSON parsing (`brew install jq` / `apt install jq`)
- **Claude Code** — the agents, hooks, and skills target Claude Code's agent / skill / hook surface

**Soft** (only needed if the chosen profile pulls the primitive in):

- **pandoc** — `tomd` uses it for `.docx` / `.pptx` / `.html` (needed for `core` / `full` profile)
- **Node + Playwright** — for the 3 browser-driven frontend primitives `design-scrape`, `live-preview`, `mock-render` (`frontend` / `full` profile); install with `npm i -g playwright && playwright install chromium`
- **sqlite3 CLI** — optional for manual DB inspection of `kei-ledger` / `kei-migrate` (their binaries embed SQLite via `rusqlite`; `ops` / `dev` profile)
- **hcloud / vultr-cli** — wrapped by `provision-hetzner` / `provision-vultr` (`ops` profile)
- **yq v4** (mikefarah/yq Go impl) — required by `kei-ci-lint` (`dev` profile)

`install.sh` checks only the deps relevant to the selected profile and soft-warns once per missing tool.

## Classic install

```bash
git clone <your-fork-of-this-repo> KeiSeiKit
cd KeiSeiKit
./install.sh                      # profile=minimal (default, no primitives)
```

`install.sh` is idempotent. It:

1. Creates `~/.claude/agents/{_blocks,_manifests,_primitives,_bridges,_templates,_assembler,_generated}`, `~/.claude/hooks`, `~/.claude/skills`
2. Copies all blocks + bridges (overwrites — these are SSoT from the kit)
3. Copies primitives ONLY for the selected profile (default: `minimal` = none). Tracks installed set in `~/.claude/agents/_primitives/.installed`.
4. Copies generic manifests (skips if you already have a manifest with that name)
5. Builds the Rust assembler (`cargo build --release` in `_assembler/`)
6. If any Rust primitive is in the selected profile: writes a scoped workspace `Cargo.toml` listing ONLY the installed crates, then `cargo build --release`
7. Generates agent `.md` files in-place with `AGENT_ROOT=~/.claude/agents assemble --in-place`
8. Copies the 12 hooks and 43 skills

After install, the only remaining step is merging `settings-snippet.json` into your `~/.claude/settings.json` to activate the hooks. You can do this automatically with `./install.sh --activate-hooks` or answer `y` at the end-of-install TTY prompt.

### Interactive install

Run `./install.sh` with no profile flag on a TTY and you get a menu:

- `whiptail` or `dialog` detected → curses-style TUI (radiolist for profile, checklist for custom)
- neither available → plain-text numbered picker (`1-7` + a `custom` option)

After the profile is chosen, an **Install Plan** screen summarizes what will be copied, which soft-deps are present (`jq`, `pandoc`, `playwright`, `cargo`, `hcloud`, `vultr-cli`, `yq`, `sqlite3`, `curl`), and the rough time + disk footprint — then asks `Proceed? [Y/n]`. Pass `--yes` to skip the confirm screen (the menu still runs). Pass `--no-execute` to parse menu + confirm and exit without copying anything (useful for dry-run). The menu is **skipped automatically** when any selection flag is passed (`--profile`, `--add`, `--remove`, `--list`) or when stdin/stdout is not a TTY (CI runs default to `minimal` exactly as before).

## Install profiles

By default `./install.sh` is **minimal** — agents + hooks + skills + bridges, no primitives. Fastest (~5s) and zero Rust compile for primitives. You opt into primitives via `--profile=<name>` or one-at-a-time via `--add=<name>`.

| Profile | Primitives added | Install time | Disk (approx) |
|---|---|---|---|
| `minimal` (default) | none | ~5s | ~2 MB |
| `core` | `tomd` | ~5s | ~2 MB |
| `frontend` | 8 site tools: `mock-render`, `visual-diff`, `tokens-sync`, `design-scrape`, `live-preview`, `figma-tokens`, `frontend-inspect`, `screenshot-decode` | ~60s | ~80 MB |
| `ops` | 9 infra tools: `kei-ledger`, `ssh-check`, `firewall-diff`, `provision-hetzner`, `provision-vultr`, `harden-base`, `metrics-scrape`, `log-ship`, `kei-provision` | ~90s | ~55 MB |
| `dev` | 17 dev tools: `kei-migrate`, `kei-changelog`, `kei-ci-lint`, `kei-docs-scaffold`, `kei-memory`, `kei-conflict-scan`, `kei-refactor-engine`, `kei-graph-check`, `kei-store`, `kei-artifact`, `kei-agent-runtime`, `kei-capability`, `kei-entity-store`, `kei-pipe`, `kei-cache`, `kei-spawn`, `kei-replay` | ~90s | ~60 MB |
| `full` | everything in `MANIFEST.toml` `full` profile (46 primitives — see manifest for exact list; the v0.29 → v0.33 additions `kei-diff`, `kei-scheduler`, `kei-watch`, `kei-prune`, `kei-discover`, `kei-brain-view`, `kei-hibernate`, `kei-ledger-sign`, `kei-dna-index`, `kei-fork`, `kei-shared` ship as sources only, not in any profile yet) | ~6 min | ~220 MB |

Examples:

```bash
./install.sh                        # minimal (no primitives)
./install.sh --profile=frontend     # minimal + 8 site tools
./install.sh --profile=full         # everything (old default behaviour)
./install.sh --add=kei-ledger       # add a single primitive on top of current install
./install.sh --add=kei-ledger,ssh-check
./install.sh --add=ops              # a profile name works too — unions its members in
./install.sh --list                 # show each primitive: name | kind | installed? | description
./install.sh --remove=kei-migrate   # remove one (rebuilds scoped rust workspace if needed)
```

Profile resolution lives in `_primitives/MANIFEST.toml` — one `[primitive.<name>]` entry per primitive plus a `[profile]` block. Edit the manifest to define new profiles without touching `install.sh`.

> **Migrating from a full install:** if you're re-running `install.sh` after an earlier version that installed all primitives unconditionally, the new default (`minimal`) will REMOVE them. To preserve the old behaviour explicitly, pass `--profile=full`.

> **Re-install disclaimer:** `install.sh` is idempotent for clean state but **overwrites kit-owned `_blocks/`, `_primitives/`, `_bridges/`, `_templates/`, `_assembler/`, `hooks/`, and `skills/` on re-run** — local modifications under those directories are backed up to `<dir>.bak-TIMESTAMP/` (or, for shared hook files, to `<file>.bak-TIMESTAMP`). User-owned `_manifests/*.toml` are never overwritten.

## MCP server binary (zero-install path, v0.18)

From v0.18 each GitHub release ships a **single static binary** of the `@keisei84/mcp-server` package for five platforms — no Node, no `npm install`. Drop the binary anywhere (USB stick, S3 bucket, Downloads folder) and run it. This is Phase 1 of the "exobrain" distribution goal: any MCP-capable client can mount KeiSeiKit from read-only media.

| Platform | Asset name |
|---|---|
| Linux x64 | `kei-mcp-server-linux-x64` |
| Linux arm64 | `kei-mcp-server-linux-arm64` |
| macOS x64 | `kei-mcp-server-darwin-x64` |
| macOS arm64 | `kei-mcp-server-darwin-arm64` |
| Windows x64 | `kei-mcp-server-windows-x64.exe` |

```bash
# Linux / macOS
curl -L -o kei-mcp-server \
  https://github.com/<your-fork>/KeiSeiKit/releases/latest/download/kei-mcp-server-darwin-arm64
chmod +x kei-mcp-server
# macOS only — clear Gatekeeper quarantine on the downloaded binary:
xattr -d com.apple.quarantine ./kei-mcp-server 2>/dev/null || true
./kei-mcp-server --stdio
```

Every asset has a matching `<name>.sha256` for integrity verification. Build details and local cross-compile recipes: `_ts_packages/packages/mcp-server/BUILD.md`.

If you drop the binary at `~/.claude/agents/_primitives/_rust/target/release/kei-mcp-server-<os>-<arch>[.exe]` (the same layout `install.sh` uses for Rust primitives), re-running `install.sh` will detect it and skip any bun/npm build step. Set `KEI_SKIP_MCP_BUILD=1` to force-skip that step regardless of detection.

## The `keisei` CLI — multi-client exobrain mount (v0.19+)

The `keisei` Rust crate is the entry-point that turns a **brain directory** (portable, filesystem-backed AI state — memory + artifacts + manifests + MCP server binaries) into an attachment on one or more AI clients in a single command. Brain layout:

```
<brain-root>/
├── manifest.toml          # schema_version, brain name, path pointers
├── memory/                # kei-memory git store (session traces, audit backlog)
├── artifacts/             # kei-artifact SQLite (typed handoff bundles)
├── manifests/             # user persona TOML library
└── bin/
    ├── kei-mcp-server-darwin-arm64
    ├── kei-mcp-server-darwin-x64
    ├── kei-mcp-server-linux-x64
    ├── kei-mcp-server-linux-arm64
    └── kei-mcp-server-windows-x64.exe
```

**`manifest.toml` — schema v2 (recommended, v0.20+)** dispatches to the right binary for the host at attach time:

```toml
[brain]
schema_version = 2
name = "my-brain"
created = "2026-04-22T00:00:00Z"

[paths.mcp_server]
darwin-arm64  = "bin/kei-mcp-server-darwin-arm64"
darwin-x64    = "bin/kei-mcp-server-darwin-x64"
linux-x64     = "bin/kei-mcp-server-linux-x64"
linux-arm64   = "bin/kei-mcp-server-linux-arm64"
windows-x64   = "bin/kei-mcp-server-windows-x64.exe"
```

A single brain on USB/iCloud now serves every host automatically. Schema v1 (single-string `mcp_server = "bin/..."`) is still accepted for backward-compat.

Four CLI commands:

| Command | What it does |
|---|---|
| `keisei attach <brain-path> [--scope user\|project]` | Mount brain into a single detected client (default: first detected) |
| `keisei mount <brain-path>` | Auto-attach to **every** detected client (Claude Code, Cursor, Continue, Zed) in one call |
| `keisei detach` | Unmount — strips `mcpServers.keisei` from every client's config, deletes marker |
| `keisei list-adapters` | Tabular status of every adapter: name / detected / config path |
| `keisei status` | Show currently-attached brain, client list, health (does `mcp_server` binary still exist?) |

Use cases:

- **Laptop travel.** Brain lives on USB / iCloud Drive. Plug in at home → `keisei mount /Volumes/MyBrain` attaches to Claude Code + Cursor simultaneously. Unplug → `keisei detach` clears everything.
- **Team shared persona library.** Commit a brain repo to your private Forgejo/GitHub. Every developer clones it, runs `keisei mount ./team-brain`, same 30-agent persona library active in their Claude Code.
- **Cloud brain.** Point `keisei attach s3://my-bucket/brain/` at an S3-backed brain synced via `kei-store` (v0.21 — real S3 / R2 / MinIO backend behind the `s3` feature flag). Memory follows you to any machine with network.
- **Experimental personas in isolation.** Spin up a test brain via `cp -r ~/production-brain ~/experimental-brain`, `keisei attach ~/experimental-brain`. Iterate without touching production state.

For full flag matrix + env vars + security hardening details → [REFERENCE.md § `keisei` CLI](./REFERENCE.md#keisei-cli--exobrain-entry-point).

## Runtime hook controls

Every kit-shipped hook (v0.14.2+) honours two env vars so you can silence noise or isolate a failure without editing `~/.claude/settings.json`:

- `KEI_DISABLED_HOOKS` — comma- or space-list of hook base names (no `.sh`), e.g. `KEI_DISABLED_HOOKS=site-wysiwyd-check,milestone-commit-hook`. The literal `all` disables every hook.
- `KEI_HOOK_PROFILE` — one of `full` (default), `advisory-off`, `minimal`, `off`.

| Profile | What stays on |
|---|---|
| `full` (default) | Every hook |
| `advisory-off` | Disables pure-stderr advisories (`recurrence-suggest`, `citation-verify`, `error-spike-detector`, `milestone-commit-hook`). Safety gates stay on. |
| `minimal` | Only safety-critical: `no-hand-edit-agents`, `secrets-guard`, `assemble-validate`. Everything else off. |
| `off` | Every hook off — escape hatch for debugging hook interactions. |

```bash
# One-session disable of a single noisy hook:
export KEI_DISABLED_HOOKS=site-wysiwyd-check

# Permanent quieter profile (paste into ~/.zshrc / ~/.bashrc):
export KEI_HOOK_PROFILE=advisory-off

# Full re-enable:
unset KEI_DISABLED_HOOKS KEI_HOOK_PROFILE
```

Interactive wizard: run `/hooks-control` — click-only picker that shows current state and emits the `export` / `unset` command for you to paste. The skill never executes anything itself.

## What you get

| Category | Count | Examples |
|---|---:|---|
| Behavioral blocks | 82 | `baseline`, `evidence-grading`, `rule-math-first`, `stack-rust-axum`, `stack-react-vite`, `stack-sveltekit`, `stack-astro`, `deploy-modal`, `api-fal-ai`, ... |
| Generic agents (manifests) | 12 | `kei-code-implementer`, `kei-critic`, `kei-validator`, `kei-security-auditor`, `kei-architect`, `kei-researcher`, `kei-ml-implementer`, `kei-cost-guardian`, `kei-modal-runner`, ... |
| Hooks (PreToolUse / PostToolUse) | 12 | `assemble-agents`, `assemble-validate`, `no-hand-edit-agents`, `tomd-preread`, `agent-fork-logger`, `orchestrator-dirty-check`, `site-wysiwyd-check`, `session-end-dump`, `milestone-commit-hook`, `error-spike-detector`, `agent-capability-check`, `agent-capability-verify` |
| Portable skills | 43 | `compose-solution`, `new-agent`, `new-project`, `site-create`, `schema-design`, `observability-setup`, `auth-setup`, `api-design`, `ci-scaffold`, `test-matrix`, `docs-scaffold`, `vm-provision`, ... |
| Primitives (Rust crates, opt-in) | 47 | `kei-ledger`, `kei-migrate`, `kei-changelog`, `ssh-check`, `firewall-diff`, `mock-render`, `visual-diff`, `tokens-sync`, `kei-memory`, `kei-conflict-scan`, `kei-refactor-engine`, `kei-graph-check`, `kei-store`, `kei-router`, `kei-sage`, `kei-task`, `kei-chat-store`, `kei-crossdomain`, `kei-search-core`, `kei-content-store`, `kei-social-store`, `kei-curator`, `kei-auth`, `kei-artifact`, `keisei`, `kei-agent-runtime`, `kei-capability`, `kei-provision`, `kei-entity-store`, `kei-pipe`, `kei-cache`, `kei-spawn`, `kei-replay`, `kei-atom-discovery`, `kei-forge`, `kei-runtime`, `kei-diff`, `kei-scheduler`, `kei-watch`, `kei-prune`, `kei-discover`, `kei-brain-view`, `kei-hibernate`, `kei-ledger-sign`, `kei-dna-index`, `kei-fork`, `kei-shared` |
| Primitives (shell, opt-in via profile) | 13 | `tomd`, `design-scrape`, `live-preview`, `figma-tokens`, `frontend-inspect`, `screenshot-decode`, `metrics-scrape`, `log-ship`, `provision-hetzner`, `provision-vultr`, `harden-base`, `kei-ci-lint`, `kei-docs-scaffold` |
| Shell helpers (always copied) | 3 | `kei-sleep-setup`, `kei-sleep-sync`, `kei-sleep-queue` (dormant until you run `/sleep-setup`) |
| Cross-tool bridges | 11 | Cursor legacy/MDC, Codex, Copilot, Windsurf, Junie, Continue, Gemini, Aider, Replit |

Of the 82 blocks, the **8 base blocks** (`baseline`, `evidence-grading`, `memory-protocol`, `rule-pre-dev-gate`, `rule-test-first`, `rule-error-budget`, `rule-double-audit`, `rule-math-first`) are referenced directly by the 12 shipped manifests. The remaining blocks (`stack-*`, `deploy-*`, `api-*`, `scraper-*`, `domain-*`) are a library consumed by the `/new-agent` wizard and the hub-and-spoke pipeline skills.

**Cognitive mode blocks** (`_blocks/mode-*.md`) are composable behavioural skews — `mode-skeptic`, `mode-devils-advocate`, `mode-minimalist`, `mode-maximalist`, `mode-first-principles`. Add any combination to an agent's manifest `blocks = [...]` list to stack the mode. See `_blocks/README.md` for the full list.

## Next steps

- [ARCHITECTURE.md](./ARCHITECTURE.md) — how agents, blocks, manifests, and the assembler fit together
- [REFERENCE.md](./REFERENCE.md) — every primitive / hook / skill with its actual CLI surface
- [SLEEP-LAYER.md](./SLEEP-LAYER.md) — nightly cycle + self-audit + cloud REM
- [SECURITY.md](./SECURITY.md) — threat model + mitigations
