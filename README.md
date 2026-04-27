# KeiSeiKit

A local-first substrate that installs next to any LLM-based assistant
and keeps its memory, accountability, and nightly learning persistent.
Rust core (53 crates, ≤ 2 MB each), TypeScript plugin glue. MIT.

Primary target: Claude Code. Also compatible with Cursor, Continue,
Zed, Aider, Windsurf, OpenClaw, Cline, Kimi via 11 cross-tool bridges
and a Rust-native MCP server (`kei-mcp`), and with any
OpenCode / OpenClaude / open-source Claude-Code clone that speaks
the MCP server protocol + standard hook events.

A second deployment surface lands with the v0.40 cortex stack: a local
HTTP daemon (`kei-cortex`) on `127.0.0.1:9797`, a Svelte browser app
(`cortex-ui`) with a Live2D pet, a ratatui terminal client (`kei-tty`),
and a VSCode extension (`@keisei/vscode-cortex`). Install via
`./install.sh --profile=cortex` and run `/cortex-setup` — five-minute
walkthrough in [`docs/QUICKSTART.md`](./docs/QUICKSTART.md).

## One-line install (Claude Code)

```bash
/plugin marketplace add KeiSei84/KeiSeiKit
/plugin install keisei@keisei-marketplace
```

Twelve agents, 45 skills, 12 hooks, nightly consolidation wired in
60 seconds. Install paths for MCP-only, Cortex, Cursor, Continue, Zed,
Aider, Docker, Nix — [`docs/INSTALL.md`](./docs/INSTALL.md).

## What it does (and where to verify)

**DNA — persistent identity per agent run.** Every invocation gets an
80-char deterministic string of the form:

    <role>::<caps-bitmap>::<sha8-scope>::<sha8-body>-<hex8-nonce>

Example:

    edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-e9bf468d

- `role` — which capability bundle the agent got (edit-local / read-
  only / auditor / merger / …).
- `caps-bitmap` — which policy fragments composed its prompt (no-git-
  ops, files-whitelist, constructor-pattern, cargo-check-green, …).
- `scope` — 32-bit hash of the files-whitelist + denylist. Same files
  → same scope regardless of who asks.
- `body` — 32-bit hash of the task description. Two people asking
  the same thing → same body.
- `nonce` — random per spawn. Distinguishes reruns of identical tasks.

Same scope+body → same problem space. You can ask **"has this type of
task run before, how did it go"** in one query, without embeddings,
without a vector DB, without round-tripping through an LLM. The index
is a plain 32-bit hash lookup against the SQLite ledger. `kei-dna-index
precedent --body <sha8>` returns every past run with its status
merged / failed / running — instantly. Code: `_primitives/_rust/kei-
shared/src/dna.rs` (format SSoT), `_primitives/_rust/kei-dna-index/**`
(queries), `_primitives/_rust/kei-ledger/src/schema.rs` (storage).

**Git branch per work-unit.** `kei-fork create` makes an atomic
triplet: a branch, a worktree under `_forks/<agent-id>/`, and a ledger
row. Agent writes in isolation, touches `.DONE`, `kei-fork collect`
atomically merges + archives + updates ledger. On any step failure,
everything rolls back. No more "the agent overwrote main". Code:
`_primitives/_rust/kei-fork/**`.

**Persistent memory on disk.** Three SQLite files:
`~/.claude/agents/ledger.sqlite` (who-did-what), `~/.claude/memory/
sessions.sqlite` (JSONL traces + embedding cache), `~/.claude/memory/
kei-memory.sqlite` (cross-session patterns). All backupable with
`cp`, portable with `keisei mount`, inspectable with `kei-brain-view
summary`. Nothing in RAM-only; nothing in a cloud you don't control.
Code: `_primitives/_rust/kei-{ledger,memory}/**`.

**Prompts composed from fragments.** Agent prompts aren't hand-written
— they're assembled from `_capabilities/<cat>/<slug>/text.md` files at
spawn time. Edit a fragment, every agent inherits the change. Single
source of truth for prompt engineering. Code: `_primitives/_rust/kei-
agent-runtime/src/compose.rs`.

**Sleep — three phases of overnight consolidation.** Your assistant keeps working while you're asleep, but on your rules, on your machine. Phase A (incubation) runs daytime-queued `/sleep-on-it` tasks — up to 5 tasks ≤ 480 min total, with checkpointing every N min. Phase B (REM) reads the day's JSONL traces, finds cross-session patterns, and writes a morning report. Phase C (NREM, every 7 days) runs a substrate-wide conflict-scan + refactor-engine and proposes a clean-up branch in plan-only or plan+fork mode. Nothing auto-injects into your next session — decisions stay with you. Full spec: [`docs/SLEEP-LAYER.md`](./docs/SLEEP-LAYER.md).

**Self-audit, silent-first.** Three hooks — `session-end-dump`,
`milestone-commit-hook`, `error-spike-detector` — feed a retrospective.
Sessions 1–10 log without prompting. From session 11 on, if the same
mistake reappears twice, `/escalate-recurrence` offers to codify it
as rule + wiki entry + hook. Code: `hooks/*.sh`, `skills/self-audit/`,
`skills/escalate-recurrence/`.

## Drive→Forgejo Import Pipeline (Wave 46–47)

End-to-end pipeline that takes one or more Google Drive folders and lands
them as private repos on the local Forgejo dev-hub (Wave 45). Zero manual
auth/Keychain/.env work — installer auto-bootstraps; user only does the
one-time rclone OAuth click. Each function below has file:line proof.

### `kei-gdrive-import classify` — single-folder verdict

Walks the marker table against a folder, returns a JSON `Classification`
with verdict (PROJECT / AMBIGUOUS / NOT-A-PROJECT / ALREADY-REPO), score,
primary language, and matched markers. Local FS path by default; remote
rclone path with `--remote`.

**Code proof** (`_primitives/_rust/kei-gdrive-import/src/classify.rs:120`):
```rust
pub fn classify(path: &Path) -> Classification {
    let names: Vec<String> = MARKERS
        .iter()
        .filter(|m| path.join(m.file).exists())
        .map(|m| m.file.to_string())
        .collect();
    verdict_from_names(path.display().to_string(), names)
}
```

**Try it**: `kei-gdrive-import classify ~/Projects/KeiSeiKit`

### `classify_remote` — verdict via `rclone lsf`, no download

For Drive folders. Shells to `rclone lsf <path> --max-depth 1`, applies
the same scoring as local. Zero bytes downloaded — only filename
listing.

**Code proof** (`_primitives/_rust/kei-gdrive-import/src/classify.rs:98`):
```rust
pub fn classify_remote(remote_path: &str) -> anyhow::Result<Classification> {
    use std::process::Command;
    let output = Command::new("rclone")
        .args(["lsf", remote_path, "--max-depth", "1"])
        .output()
```

**Try it**: `kei-gdrive-import classify --remote "gdrive:projects/MyApp"`

### `scan-tree` — walk one level under a root

Lists immediate subfolders + classifies each. Local (`std::fs::read_dir`)
or remote (`rclone lsjson`).

**Code proof** (`_primitives/_rust/kei-gdrive-import/src/scan.rs:19`):
```rust
pub fn scan_tree(root: &Path) -> Result<Vec<Classification>> {
    let mut out: Vec<Classification> = Vec::new();
    let entries = std::fs::read_dir(root)
        .with_context(|| format!("read_dir {}", root.display()))?;
```

**Try it**: `kei-gdrive-import scan-tree --remote "gdrive:projects/" | jq '.[].verdict'`

### 8-marker scoring table — single SSoT

The const array that drives every verdict. Add a row → entire pipeline
recognises a new build system. No second copy anywhere.

**Code proof** (`_primitives/_rust/kei-gdrive-import/src/scoring.rs:26`):
```rust
pub const MARKERS: &[Marker] = &[
    Marker { file: "Cargo.toml",      weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("rust")    },
    Marker { file: "package.json",    weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("node")    },
    Marker { file: "pyproject.toml",  weight: 10, kind: MarkerKind::BuildManifest, primary_lang: Some("python")  },
```

### `kei-drive-import` wizard — explicit-paths mode (default)

Bash orchestrator. Default mode: pass paths directly (no scan ceremony,
"obvious path optimised" per Wave 47). `--scan` for auto-discover. `--paths-from FILE` for batch.

**Code proof** (`_templates/drive-import-wizard.sh.tmpl:539`):
```bash
main_paths() {
    local paths=()
    if [ -n "$PATHS_FILE" ]; then
        while IFS= read -r line; do
            line="${line%%#*}"   # strip comments
            line="$(printf '%s' "$line" | awk '{$1=$1};1')"  # trim
            [ -n "$line" ] && paths+=("$line")
        done < "$PATHS_FILE"
    fi
```

**Try it**: `kei-drive-import "gdrive:projects/A" "gdrive:Work/clientB"`

### Allowlist guard derived from `$FORGEJO_URL` (Wave 46 audit HIGH-3 fix)

The remote-allowlist check used to hardcode `^http://127.0.0.1:3001/`,
silently breaking any user who overrode `KEI_FORGEJO_URL`. Now derived
from the same env var that builds the repo URL.

**Code proof** (`_templates/drive-import-wizard.sh.tmpl:420`):
```bash
case "$origin_url" in
    "${FORGEJO_URL}/"*) ;;
    *)
        printf 'kei-drive-import: ERROR: REJECTED: remote not allowlisted (expected %s/...): %s\n' \
            "$FORGEJO_URL" "$origin_url" >&2
        exit 1
        ;;
esac
```

### `_dhf_bootstrap_admin_user` — Forgejo admin + token + Keychain auto-create

On first install, detects empty Forgejo, creates admin `$USER` with random
24-char password + 40-hex access token, stashes both in macOS Keychain
(`forgejo-admin-password` + `forgejo-api-token`), stamps `.env` with
`KEI_FORGEJO_USER` + `KEI_FORGEJO_URL`. Idempotent — re-runs are no-op.

**Code proof** (`install/lib-dev-hub-forgejo.sh:95`):
```bash
_dhf_bootstrap_admin_user() {
  local config username user_count password output token kc env_file
  local kc_token_svc kc_pass_svc
  config="$(_dhf_app_ini)"
  username="${KEI_FORGEJO_ADMIN_USER:-${USER:-denis}}"
```

**Try it**: included in `./install.sh --profile=full-hub --yes`. Retrieve later: `security find-generic-password -s 'forgejo-admin-password' -w`.

### `install_primitives` clean-stamp guard (audit HIGH-1 fix)

Pre-fix: `.installed` file recorded ALL primitives regardless of whether
their `install_*()` function succeeded. Re-runs skipped silently-broken
installs. Now: per-primitive exit-code tracking, only clean installs stamp.

**Code proof** (`install/lib-primitives.sh:122`):
```bash
install_primitives() {
  local names existing combined kind p any_rust=0 any_external=0
  local installed_clean install_ok
  names="$(cat)"
  existing="$(read_installed)"
  installed_clean=""
```

### `tools/sync-public.sh` — public-mirror pipeline

4-phase: rsync (with IP-aware excludes) → sed substitutions → leak-grep
(hard banlist + soft warnlist) → diff stat. Push is NEVER automatic
(RULE 0.1) — script prints exact manual command at the end.

**Code proof** (`tools/sync-public.sh:190`):
```bash
main() {
    say "MODE: $MODE"
    say "private: $PRIVATE_ROOT"
    say "public:  $PUBLIC_ROOT (remote $PUBLIC_REMOTE/$PUBLIC_BRANCH)"
    echo
    phase_rsync
```

**Try it**: `./tools/sync-public.sh --dry-run` (preview), then `--confirm` (apply).

### MCP exposure (Wave 47)

One line in the registry exposes `kei-gdrive-import` to every MCP-aware
client (Cursor, Continue, Zed, Cline, Kimi, OpenClaw) the moment they
mount `kei-mcp-server`. No per-client glue.

**Code proof** (`_ts_packages/packages/mcp-server/src/tool-registry.ts:31`):
```typescript
{ binary: "kei-gdrive-import", desc: "Classify a Google Drive folder as PROJECT/AMBIGUOUS/NOT-A-PROJECT/ALREADY-REPO via 8-marker scoring (Cargo.toml, package.json, pyproject.toml, go.mod, pom.xml, build.gradle, Gemfile, composer.json). Subcommands: classify <path> [--remote], scan-tree <root> [--remote]. Remote mode shells to rclone lsf — no download." },
```

### `/drive-import` skill (Wave 47, Claude Code)

Claude Code slash-command bridging the wizard. Pre-flight checklist,
pipeline diagram (10 steps), failure-mode triage table.

**Code proof** (`skills/drive-import/SKILL.md:1`):
```yaml
---
name: drive-import
description: Import one or more Google Drive folders as private repos into the local Forgejo dev-hub. Wraps the kei-drive-import bash wizard. Click-only path-picker, optional --scan auto-discover. Requires rclone OAuth (one-time) + Wave 45 dev-hub Forgejo running.
argument-hint: (optional) "drive:path/to/folder"
---
```

**Try it**: `/drive-import` in Claude Code.

### Bridge documentation — generic AGENTS.md (Wave 47)

The `_bridges/agents-md.tmpl` is the convention read by Codex CLI / Copilot
/ Aider / Windsurf / Warp / Zed / Antigravity / Junie. Tool catalogue
appears here once, not per-IDE.

**Code proof** (`_bridges/agents-md.tmpl:27`):
```markdown
## Available KeiSeiKit Tools (MCP-exposed)

When the kit's MCP server is mounted (`keisei attach` or `keisei mount`), these
Rust primitives are callable from any MCP-aware client (Cursor, Continue,
Zed, Cline, Kimi, OpenClaw):
```

### Integration tests — 7/8 PASS (i4)

8 assertions over mocked rclone + mocked Forgejo + real `kei-gdrive-import`
binary. Test 8 gracefully skips when port 3001 is held by real Forgejo
(can't fake auth against unknown live daemon).

**Code proof** (`tests/gdrive_import_integration.sh:181`):
```bash
main() {
    setup
    test1_classify_rust_app
    test2_classify_already_imported
    test3_classify_photo_folder
    test4_scan_tree
    test5_classify_remote
    test6_wizard_secret_block
    test7_wizard_remote_allowlist
    test8_wizard_ledger_ok
```

**Try it**: `bash tests/gdrive_import_integration.sh`

## Deployment modes

| Where | How | When you want it |
|---|---|---|
| **Local laptop (plugin)** | `/plugin install keisei` | Daily dev / research / writing inside Claude Code |
| **Local laptop (cortex stack)** | `./install.sh --profile=cortex` then `/cortex-setup` | Browser chat UI + Live2D pet + voice + VSCode panel |
| **Terminal** | `./install.sh --add=kei-tty` then `kei-tty chat` | Same daemon, ratatui TUI, pipe-friendly `send` mode |
| **USB stick** | `keisei mount /Volumes/usb/brain` | Move between machines |
| **iCloud / Dropbox** | Same `keisei mount` on a cloud-synced folder | Auto-backup + multi-device |
| **Cloud VPS** | `ssh user@host && ./install.sh && keisei serve` | Always-on agent, tablet client, shared team brain |
| **Docker** | `docker run -v ./brain:/home/kei/.claude keiseikit` | Ephemeral CI runs, reproducible test envs |
| **MCP-only** | `./install.sh --profile=mcp` | Plug the memory layer into a different assistant, skip the plugin |
| **MCP server (Rust)** | `cargo build -p kei-mcp --release` then register in `mcp_servers.json` | Expose every atom + skill to Claude Code / Cline / OpenClaw as MCP tools |

Rust binaries are ≤ 2 MB each, statically linked where possible, no
Python runtime. Any x86_64 / arm64 host with git + SQLite works.
Bring your own API key (Anthropic / OpenAI / OpenRouter / local
Ollama) via `~/.claude/secrets/.env` — the kit never ships keys.

## Cortex stack (v0.40 — local HTTP daemon + UIs)

The plugin path puts the substrate inside Claude Code. The cortex stack puts it in your browser, terminal, and editor. Same memory, same ledger, same DNA — different frontends.

| Piece | What it is | Path |
|---|---|---|
| `kei-cortex` | axum HTTP daemon on `127.0.0.1:9797`. Routes (all under `/api/v1/cortex/` except `/healthz`): `/healthz` (no-auth), `/summary`, `/pet/:user_id` GET, `/pet/:user_id/interaction` POST, `/pet/:user_id/chat` POST (Anthropic SSE — agentic loop with 8 tools `read`, `write`, `edit`, `bash`, `glob`, `grep`, `webfetch`, `agent` from `src/tool/registry.rs`), `/pet/:user_id/portrait/stylize` POST (fal.ai Flux), `/pet/:user_id/tts` POST (ElevenLabs), `/stt` POST (faster-whisper subprocess — RULE 0.2 #6), `/ledger/recent`, `/memory/search`, `/usage`, `/fs/list`, `/term` WS (PTY via `portable-pty` + `tokio-tungstenite`). Bearer-token auth from `~/.keisei/cortex.token`; per-route concurrency caps in `src/routes.rs`. | `_primitives/_rust/kei-cortex/` |
| `cortex-ui` | Svelte 5 + Vite 5 web app. Routes: `Setup`, `Dashboard`, `PetEditor`, `LedgerStream`, `MemorySearch`. Components: chat panel with PTT voice + auto-TTS, `BudgetStrip` (today's API spend), `FileTree`, `TerminalPane` (xterm.js → `WS /term`), Live2D pet (`pixi-live2d-display`) with idle animation + lip-sync. | `_ts_packages/packages/cortex-ui/` |
| `kei-tty` | ratatui terminal client. `kei-tty chat` interactive crossterm TUI; `kei-tty send --message "…"` one-shot pipe-friendly mode. | `_primitives/_rust/kei-tty/` |
| `@keisei/vscode-cortex` | VSCode extension. Activitybar entry "KeiSei Cortex", webview chat view, `Cmd+Shift+K` opens panel, right-click → "Ask About Selection" sends highlighted text. | `_ts_packages/packages/vscode-cortex/` |
| `kei-mcp` | Rust MCP server (stdio JSON-RPC 2.0). Atoms → MCP tools (`<crate>::<verb>`); skills → MCP resources (`skill://<name>`). Methods: `initialize`, `tools/{list,call}`, `resources/{list,read}`, `prompts/{list,get}`. | `_primitives/_rust/kei-mcp/` |

Setup is one wizard — `/cortex-setup`. Seven phases. Five-minute walkthrough in [`docs/QUICKSTART.md`](./docs/QUICKSTART.md).

## Why Rust + TypeScript (no Python)

**Rust** for the engine — 53 crates covering ledger, memory, DNA,
fork, scheduler, watcher, assembler, CLI, plus the v0.40 cortex stack
(daemon, MCP server, terminal client). The class of mistakes LLMs
make most often — `None` vs empty array, missing `.await`, unhandled
`Result`, mutex misuse — are compile-time errors in Rust. The LLM
literally cannot ship them. The type system is the fence.

**TypeScript** for adapters that talk to JS-only ecosystems — Claude
Code's Node plugin API, the cortex-ui Svelte 5 frontend, the VSCode
extension webview, and five external-API adapter packages
(`gmail-adapter`, `grok-adapter`, `recall-adapter`, `telegram-adapter`,
`youtube-adapter`). The substrate core never assumes Node is running.

**Why not Python — it's transformer math, not taste.** Writing Rust
costs more tokens up-front (explicit types, explicit errors, explicit
ownership) but debugging costs ≈ 0 — the compiler is the fence.
Writing Python costs ~5 minutes (no types, no lifetimes, no compile
step), then debugging loops for hours: each failed run produces new
tokens the assistant must read, reason about, correct — often
triggering the next failure, another iteration, another token burn.
This isn't empirical. Python's tokenization (indentation-as-syntax,
runtime dynamic dispatch, late-binding names) leaves each token with
low semantic commitment, so the transformer must track more
conditional paths forward. Rust pins each token to a compile-time
shape: fewer open paths, fewer silent surprises, fewer debug loops.
Net cost of a shipped Rust primitive: predictable. Net cost of a
Python primitive: unbounded. RULE 0.2 enforces this with a hook that
blocks `python` / `python3` / `uv run python` without an explicit
opt-in prefix. `install.sh` runs on a fresh host with zero Python
installed — if Python slips into this repo, it's a bug.

## Probably fits — it's a way of working with AI as much as a tool

You'll like this if you prefer:

- **Seeing what your AI did**, rather than a dashboard that hides the
  mistakes. The ledger shows every agent run, every fork, every
  rollback. Honesty by construction.
- **Your own machine over someone else's cloud.** Brain lives on your
  disk, backs up with `cp`, moves on a USB.
- **Two install commands once**, instead of a SaaS onboarding flow.
- **One of**: macOS, Linux, or Windows-via-WSL2. Native Windows is on
  the backlog.
- **Owning your API keys** — bring your own Anthropic / OpenAI /
  OpenRouter / local Ollama, never ours.
- **Working across multiple AI assistants** — Claude Code today,
  Cursor on the plane, an OpenCode fork next month; the substrate
  moves with you.
- **A workflow that learns while you sleep** rather than a tool you
  restart every morning.

If you'd rather have a hosted multi-tenant SaaS with a polished web
UI and no CLI — this isn't that, and that's OK. Plenty of great tools
in that space. KeiSeiKit is for the other camp.

## What ships (run the command, get the number)

| Thing | Count | Verify |
|---|---|---|
| Rust crates (workspace) | 53 | `ls -d _primitives/_rust/*/ \| wc -l` |
| Agents | 12 | `ls kei-*.md 2>/dev/null \| wc -l` |
| Skills | 45 | `ls -d skills/*/ \| wc -l` |
| Hooks | 12 | `ls hooks/*.sh \| wc -l` |
| Behavioural blocks | 82 | `ls _blocks/*.md \| wc -l` |
| Capabilities | 16 | `find _capabilities -name text.md \| wc -l` |
| Roles | 7 | `ls _roles/*.toml \| wc -l` |
| Cross-tool bridges | 11 | `ls _bridges/*.tmpl \| wc -l` |
| TS packages | 7 | `ls -d _ts_packages/packages/*/ \| wc -l` |
| Profiles (`MANIFEST.toml`) | 8 | minimal / core / frontend / ops / dev / mcp / cortex / full |

Counts regenerate from source via `scripts/regen-counts.sh`. Test totals
fluctuate per-feature; run `cd _primitives/_rust && cargo test --workspace`
to get the current number on your machine.

## Architecture in one paragraph

One hub crate (`kei-entity-store`) owns SQLite migrations, FTS
indexes, verb dispatch. Sibling store crates (chat, content, social,
crossdomain, task, scheduler) are thin shims over the hub — star
topology, zero sibling-to-sibling edges. DNA format is SSoT'd in
`kei-shared` (grep-verify: `grep -l "kei_shared::dna" _primitives/
_rust/*/src/*.rs`). Constructor Pattern is enforced pre-commit:
every file ≤ 200 LOC, every function ≤ 30 LOC. See
[`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md).

## Verify everything works (60 seconds)

```bash
git clone <your-private-remote-url>/KeiSeiKit && cd KeiSeiKit
cd _primitives/_rust && cargo test --workspace
# Expect: test result: ok. <N> passed; 0 failed   (N grows with feature work)
cd .. && tests/substrate_integration.sh && tests/hook_wiring_integration.sh
# Expect: two PASS lines
```

If `cargo test --workspace` exits 0 and the two integration scripts
print PASS, the repo you're reading matches the repo I'm describing.

The `<repo-url>` placeholder is intentional — RULE 0.1 forbids pushing
this code to `github.com`. Use a private remote (Forgejo, Gitea, or
self-hosted git) instead.

## Compared to similar tools

| | KeiSeiKit | LangChain | AutoGen | MCP alone |
|---|---|---|---|---|
| Local-first, no cloud required | ✓ | partial | ✓ | ✓ |
| Rust core, ≤ 2 MB binaries | ✓ | ✗ | ✗ | N/A |
| Per-run deterministic DNA | ✓ | ✗ | ✗ | ✗ |
| Git-fork work isolation | ✓ | ✗ | ✗ | ✗ |
| Three-phase nightly cycle | ✓ | ✗ | ✗ | ✗ |
| Capability-fragment prompts | ✓ | partial | partial | ✗ |
| Cross-tool bridges (Cursor, Zed, Aider, …) | 11 | ✗ | ✗ | ✗ |
| Works with OpenCode / OpenClaude clones | ✓ | ✗ | ✗ | ✓ |
| Enforced Constructor Pattern | ✓ | ✗ | ✗ | N/A |

Sits alongside these tools, not a replacement. Uses MCP where MCP
fits; LangChain stays useful for its retriever zoo.

## Docs

| | |
|---|---|
| [QUICKSTART.md](./docs/QUICKSTART.md) | Five-minute install-to-browser walkthrough for the cortex stack |
| [PHILOSOPHY.md](./docs/PHILOSOPHY.md) | The biological principles, in depth |
| [INSTALL.md](./docs/INSTALL.md) | All install paths, profiles, `keisei` CLI, hook controls |
| [ARCHITECTURE.md](./docs/ARCHITECTURE.md) | Build pipeline, atoms, recipes, frontends, bridges, MCP |
| [REFERENCE.md](./docs/REFERENCE.md) | Every primitive, hook, skill with flags and exit codes |
| [SLEEP-LAYER.md](./docs/SLEEP-LAYER.md) | Phase A / B / C nightly cycle + self-audit |
| [TAXONOMY.md](./docs/TAXONOMY.md) | The seven-facet vocabulary |
| [SUBSTRATE-SCHEMA.md](./docs/SUBSTRATE-SCHEMA.md) | Atom contract |
| [SECURITY.md](./docs/SECURITY.md) | Threat model + mitigations |
| [USB-BRAIN-GUIDE.md](./docs/USB-BRAIN-GUIDE.md) | Portable brain — macOS / Linux / Windows |
| [WHY.md](./docs/WHY.md) | The full story of why this exists |
| [CHANGELOG.md](./CHANGELOG.md) | What changed, version by version |
| [PLUGIN.md](./PLUGIN.md) | Anthropic plugin-format details |

## Patents and IP

This repository contains code adjacent to patent-pending technologies
in the wider KeiTech portfolio. Public push is forbidden per **RULE 0.1
(no GitHub push)** — `git push` to `github.com` is hard-blocked by the
`~/.claude/hooks/no-github-push.sh` hook. Use a private remote
(Forgejo, Gitea, self-hosted) for any work that builds on this kit.
Read-only github (clone, fetch, view) remains allowed.

The kit itself is MIT-licensed (this repo). Sister projects in the
KeiTech portfolio that touch ML weights, guidance laws, kernel-level
code, or offensive security are governed by `~/.claude/rules/security.md`
and never deploy publicly without double-confirmation.

## Origin and an honest note to whoever reads this

I'm not a developer. I picked up Claude Code in January 2026 and had
never written a line of code before that. This project is a seven-day
run that started from wanting to beat [Andrej Karpathy's CLAUDE.md](https://github.com/karpathy)
on methods for working with AI agents — huge thanks to him; that file
showed what was possible. The kit went a bit further than a rules
sheet, and now it's too big for one person to hold.

If you're an engineer reading this — be kind. Constructive criticism
is welcome. Any architectural decision that genuinely optimises
something is welcome. I will learn from the diff.

**On the bus factor.** Bus factor is a real concept — but it doesn't
apply the same way in our generation. Everyone has AI tools that can
read, develop, and support a codebase like this. There is no single
human dependency here: you, your Claude, your Cursor, your Aider can
pick up this code and continue it. Anyone can contribute something —
human or agent. The substrate is designed to be readable that way.

If your contribution lands and the work feels worth it to you — I
have another version of this, more mathematical and more neural-
identical, that I'd be glad to share access to.

## Author

Built by Denis Parfionovich at KeiLab (`parfionovich@keilab.io`) while
running 4–8 parallel Claude Code terminals per day. Forks welcome.

MIT License. See [LICENSE](./LICENSE).
