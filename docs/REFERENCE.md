# Reference

Every shipped component, its real behaviour, and where to look in source. Each subsection documents the actual CLI surface as extracted from `_primitives/_rust/*/src/main.rs`, `_primitives/*.sh`, `hooks/*.sh`, and `skills/*/SKILL.md`. If a flag or subcommand is not listed here, it does not exist in the current build.

**Index:** [Rust primitives](#rust-primitives) · [Shell primitives](#shell-primitives) · [Hooks](#hooks) · [Skills](#skills-grouped) · [`keisei` CLI deep-dive](#keisei-cli--exobrain-entry-point) · [Pipelines](#pipelines)

---

## Rust primitives

All 47 crates live under `_primitives/_rust/<name>/`. After `install.sh` runs, binaries land at `~/.claude/agents/_primitives/_rust/target/release/<name>`. Exit codes: `0` success, `1` usage/IO error, `2` validation/diff-found (per-tool; see each entry).

> NOTE (v0.33): the following sections enumerate the 25 crates documented through v0.22. 22 newer crates shipped in v0.23–v0.33 (`kei-entity-store`, `kei-agent-runtime`, `kei-capability`, `kei-provision`, `kei-pipe`, `kei-cache`, `kei-spawn`, `kei-replay`, `kei-atom-discovery`, `kei-forge`, `kei-runtime`, `kei-diff`, `kei-scheduler`, `kei-watch`, `kei-prune`, `kei-discover`, `kei-brain-view`, `kei-hibernate`, `kei-ledger-sign`, `kei-dna-index`, `kei-fork`, `kei-shared`) are not yet documented here — see `CHANGELOG.md` for their shipped semantics and the crates' own `Cargo.toml` + `atoms/*.md` for their current CLI surface. A full REFERENCE.md expansion is tracked as a follow-up doc task.

### `kei-ledger` — agent-fork lifecycle ledger (RULE 0.12)

SQLite-backed append-only log of every non-trivial agent invocation. One row per fork; the orchestrator uses `validate` to check that the 6 required artefacts exist on the child branch before merging.

```
kei-ledger [--db <path>] <subcommand>
  init                                        — create ledger file + schema
  fork <id> <branch> --spec-sha <sha>         — log a new running agent
          [--parent <id>] [--worktree <path>]
  done <id> --summary <s>                     — mark running agent done
  fail <id> --reason <r>                      — mark running agent failed
  merged <id>                                 — mark done/failed agent merged into parent
  list [--status running|done|failed|merged]  — dump history
  tree <id>                                   — parent → children tree from root id
  validate <branch> [--repo-root <path>]      — check 6-artefact bundle exists
                                                (spec.md / plan.md / progress.json /
                                                 chatlog.md / handoffs.md / review.md)
```

State: `$KEI_LEDGER_DB` or `~/.claude/agents/ledger.sqlite`. `validate` exits 2 if artefacts missing.

### `kei-migrate` — universal SQL migration runner

Postgres / SQLite / MySQL via a single `DATABASE_URL`. Up/down/status/create.

```
kei-migrate --database-url <url> [--dir <path=migrations>] <subcommand>
  up                  — apply all pending migrations
  down [n=1]          — revert the last N migrations (needs <ts>_<name>.down.sql)
  status              — list applied vs pending
  create <name>       — scaffold <ts>_<name>.sql (+ .down.sql)
```

URL formats accepted: `postgres://...`, `sqlite:///abs/path.db` or `sqlite::memory:`, `mysql://...`. Reads `DATABASE_URL` env var as fallback.

### `kei-changelog` — Conventional Commits → Markdown

Git-cliff-shaped generator. Walks a commit range, groups by conventional-commit type, prepends to `CHANGELOG.md` or emits to stdout.

```
kei-changelog [--from <ref>] [--to <ref=HEAD>] [--unreleased]
              [--version <label=v0.1.0>] [--repo <path=.>]
              [--update <file>]
```

`--update` prepends under `# CHANGELOG` header (idempotent — won't duplicate an existing identical block). Without `--update`, prints to stdout.

### `ssh-check` — sshd_config linter

Reads `/etc/ssh/sshd_config` + every `sshd_config.d/*.conf`, merges via last-wins, scores against the KeiSeiKit hardened-baseline rule matrix.

```
ssh-check [--config <path=/etc/ssh/sshd_config>]
          [--drop-in <dir=/etc/ssh/sshd_config.d>]
          [--allow-user <name>]... [--json]
```

Repeatable `--allow-user` for extra accepted `AllowUsers` entries (default: `keiadmin`). Exits 2 if violations found.

### `firewall-diff` — ufw intent-vs-live diff

Defensive-only — does NOT execute `ufw` itself. Takes an intent YAML + a captured `ufw status numbered` output (file or stdin) and reports drift.

```
firewall-diff --intent <yaml> (--status-file <path> | --stdin) [--json]
```

Exit 2 on any diff. Use via `ufw status numbered | firewall-diff --intent fw.yaml --stdin`.

### `mock-render` — WYSIWYD screenshot-and-lock

Playwright-backed. Enforces the "What You See Is What's Deployed" invariant: every site section's source file is hashed, the hash is locked against a screenshot, and `verify` fails if the source drifts after lock.

```
mock-render screenshot <url> --out <png> [--viewport WxH]
mock-render lock    --project <dir> --section <src> [--screenshot <png>]
mock-render verify  --project <dir> --section <src>
mock-render status  --project <dir>
```

Exit 2 on invariant violation (source hash changed since lock).

### `visual-diff` — pixel PNG comparator

Used by `site-wysiwyd-check` hook to detect visual drift. Produces a red-overlay diff PNG when images differ beyond threshold.

```
visual-diff <a.png> <b.png> [--out <file=diff.png>] [--threshold <pct=1.0>]
```

Exit 2 if mismatch exceeds threshold. Prints percentage + diff-px count.

### `tokens-sync` — design tokens → Tailwind + CSS vars

Single JSON file → Tailwind `theme.extend` config and CSS `:root` custom properties. Either or both output targets; at least one required.

```
tokens-sync <tokens.json> [--out-tailwind <path>] [--out-css <path>]
```

JSON minimum shape: `{ colors, fonts, spacing, radius }`.

### `kei-memory` — session retrospective + pattern detector (RULE 0.14)

Offline TF-IDF / recurrence analyzer powering `/self-audit`. Ingests JSONL transcripts, surfaces cross-session patterns.

```
kei-memory [--db <path>] <subcommand>
  ingest --session-id <id> --transcript <path.jsonl> [--prompt <text>]
  analyze [--session <id>] [--last <n=1>] [--summary]
  patterns [--cross-session] [--session <id>]
  similar <prompt> [--limit <k=5>]          — top-k past sessions by TF-IDF cosine
  dump <session-id>                          — emit session events as markdown
  stats                                      — N sessions / N events / top tools
  backlog [--add <s>] [--list] [--clear]     — silent-first audit backlog
```

State: `$KEI_MEMORY_DB` or `~/.claude/memory/kei-memory.sqlite`.

### `kei-conflict-scan` — deep-sleep conflict scanner (v0.13.0)

Scans a memory-repo clone for rule conflicts, overlapping hook matchers, >70%-duplicate blocks, orphaned wikilinks, Constructor-Pattern violations.

```
kei-conflict-scan --path <root>
                  [--format json|human] [--only rules|hooks|blocks|orphans|cp]
                  [--exit-on-hit]
```

Emits JSON (default) or human table. Exit 2 only when `--exit-on-hit` AND hits found.

### `kei-refactor-engine` — refactor-plan generator (v0.13.0)

Consumes `kei-conflict-scan` JSON, emits plan markdown + an auto-resolve review file (NOT a unified diff — v0.14.1 retraction).

```
kei-refactor-engine [--input <conflicts.json>|-] [--plan-only]
                    [--apply-to-branch <branch>]
                    [--plan-out <path>] [--patch-out <path>]
```

`--patch-out` writes a markdown review (kept name for backward-compat). `requires_human_decision` items excluded from auto-resolve, listed in plain plan.

### `kei-graph-check` — post-refactor graph-integrity gate (v0.13.0)

Resolves wikilinks + handoff refs + block refs across a memory-repo clone. Used as a gate BEFORE orchestrator commits the deep-sleep fork branch.

```
kei-graph-check --path <root> [--after-diff <patch>] [--json]
```

`--after-diff` treats any `+++ /dev/null` removal or `# removed: <p>` header as a phantom-removed file. Exit 2 on any broken reference.

### `kei-store` — memory-repo backend abstraction (v0.21)

Unifies GitHub / Forgejo / Gitea / Filesystem / S3 behind one `MemoryStore` trait. Real `aws-sdk-s3` when built with `--features s3`; otherwise falls back to a local-manifest stub (still behind `KEI_STORE_ALLOW_S3_STUB=1`).

```
kei-store [--config <path>] <subcommand>
  init <backend> [--url <url>]      — write store-config.toml scaffold
  read <path>                        — fetch blob to stdout
  write <path> <file>                — upload local file
  list <dir>                         — list names under dir
  branch <name>                      — set active branch/prefix
  commit --message <msg>             — commit staged writes
  push <branch>                      — push branch to remote
  pull <branch>                      — pull branch from remote
  status                             — print backend name
```

Config: `~/.claude/agents/_primitives/store-config.toml` (override with `--config`). S3 endpoint override: `KEI_STORE_S3_ENDPOINT` env or `s3.endpoint` TOML field. AWS default credential chain applies.

### `kei-artifact` — typed artifact handoff store (v0.16)

JSON-Schema-validated blob store for inter-agent handoffs. Five built-in schemas: `spec`, `plan`, `patch`, `review`, `research`. Custom schemas registered at runtime auto-sync to the assembler via `schemas.json` export.

```
kei-artifact [--db <path>] <subcommand>
  init                                              — register 5 built-in schemas
  register-schema --name <n> --path <schema.json>
  list-schemas
  export-schemas [--path <out>]                     — refresh assembler's schemas.json
  emit --schema <n> --from <agent> --content <json> — write an artifact
       [--meta key=val]... [--parent <id>]
  get <id> [--format typed|raw]
  list [--schema <n>] [--from <agent>] [--since <N>s]
  validate <id>                                      — re-validate against schema
  chain <id>                                         — walk parent-handoff chain
```

State: `$KEI_ARTIFACT_DB` or `~/.claude/artifacts/artifacts.sqlite`.

### `kei-auth` — HMAC-signed token issuer (v0.14.1 security fix)

Issue / verify / revoke. Signing secret sourced from `KEI_AUTH_KEY` env var ONLY — the old `--key` CLI flag was removed in v0.14.1 because it leaked the secret through `/proc/<pid>/cmdline` and shell history.

```
kei-auth [--db <path>] <subcommand>
  issue  --user <u> --project <p> [--scope read|write|admin=read] [--ttl <sec=86400>]
  verify <token>
  revoke <token>
```

Key source (per RULE 0.8): `export KEI_AUTH_KEY="$(openssl rand -hex 32)"` or sourced from `~/.claude/secrets/.env`. State: `$KEI_AUTH_DB` or `~/.claude/auth/auth.sqlite`.

### `kei-router` — natural-language → tool-call JSON

Rule-based NL router. Parses a short query into a structured tool-call JSON object. Used by the compose-solution skill as a first-pass dispatcher.

```
kei-router <query> [--forward]
```

`--forward` adds `_forward=true` hinting remote-MCP forwarding on fallback. Prints pretty JSON to stdout.

### `kei-sage` — Obsidian-style knowledge vault

SQLite-backed FTS5 knowledge store with BFS-related and PageRank. Can import an Obsidian vault wholesale.

```
kei-sage [--db <path>] <subcommand>
  import <vault>                                    — import .md files with frontmatter
  search <query> [--limit <n=20>]                   — FTS5 over title+content
  related <key> [--depth <d=2>]                     — BFS from a vault path/key
  rank [--limit <n=20>]                             — PageRank over wikilinks
  add --title <t> [--content <c>] [--vault-path <p>] [--grade <g=E4>]
  edit <id> [--title <t>] [--content <c>] [--grade <g>]
  link <src> <dst> [--edge-type <t=related>]
```

State: `$KEI_VAULT_DB` or `~/.claude/sage/vault.sqlite`.

### `kei-task` — task DAG CLI

SQLite task graph with dependencies, milestones, FTS search. Dep types arbitrary (`blocks`, `relates`, ...).

```
kei-task [--db <path>] <subcommand>
  create <title> [--description <d>] [--priority low|medium|high=medium]
  update <id> [--status <s>] [--title <t>]
  add-dependency <from> <to> [--dep-type <t=blocks>]
  graph                                — list all edges
  dependency-chain <id>                — topologically walk deps
  search <query> [--limit <n=20>]
  milestone <name> [--description <d>]
  link-milestone <task-id> <milestone-id>
```

State: `$KEI_TASK_DB` or `~/.claude/task/task.sqlite`.

### `kei-chat-store` — chat session archive

Session + message CRUD with token/cost accounting, FTS search, archive flag.

```
kei-chat-store [--db <path>] <subcommand>
  start --project <p> [--title <t>] [--model <m>]
  save --session-id <id> --role <user|assistant|system> <content>
       [--tokens-in <n>] [--tokens-out <n>] [--cost <f>]
  search <query> [--limit <n=20>]
  archive <session-id>
  stats                                — JSON summary
```

State: `$KEI_CHAT_DB` or `~/.claude/chat/chat.sqlite`.

### `kei-crossdomain` — cross-domain link graph

Generic typed-edge graph for linking any URIs. Used to wire rules ↔ memory ↔ artefacts ↔ chats.

```
kei-crossdomain [--db <path>] <subcommand>
  link <from> <to> [--edge-type <t=related>] [--weight <w=1.0>] [--evidence <g=E4>]
  unlink <from> <to> [--edge-type <t=related>]
  query <node>                         — all edges touching node
  graph <start> [--depth <d=2>]        — BFS
  auto-link <node>                     — propose + add edges via heuristic
  stats                                 — count per edge type
```

State: `$KEI_CROSS_DB` or `~/.claude/cross/cross.sqlite`.

### `kei-search-core` — research pipeline scaffold

Budget-bounded research runner. Current build ships a `StubFetcher` (real web fetch pluggable); runs the research-pipeline cubes end-to-end and persists results for markdown/JSON export.

```
kei-search-core [--db <path>] <subcommand>
  run <prompt> [--budget <microusd=1_000_000>]   — default budget = 1 USD
  stop <id>
  export <id> [--format md|json=md]
```

State: `$KEI_SEARCH_DB` or `~/.claude/search/research.sqlite`.

### `kei-content-store` — creative asset + prompt registry

Register generated assets, prompts, campaigns; track prompt version history.

```
kei-content-store [--db <path>] <subcommand>
  register-asset <title> [--file-path <p>] [--media-type <m>] [--provider <n>]
  register-prompt <prompt-text> [--model <m>] [--prompt-type <t>]
  create-campaign <name> [--description <d>]
  attach-asset <campaign-id> <asset-id>
  prompt-history <prompt-id>
```

State: `$KEI_CONTENT_DB` or `~/.claude/content/content.sqlite`.

### `kei-social-store` — people + organisation CRM

Person/org registry with interaction log + relationship graph.

```
kei-social-store [--db <path>] <subcommand>
  search-people <query> [--limit <n=20>]
  add-person <name> [--email <e>] [--handle <h>] [--source <s=manual>]
  add-org <name> [--org-type <t=company>]
  log-interaction <person-id> <interaction-type> [--content <c>]
                  [--channel <ch=manual>] [--target-id <n>]
  relationship-graph
```

State: `$KEI_SOCIAL_DB` or `~/.claude/social/social.sqlite`.

### `kei-curator` — edge decay + orphan pruning

Operates on any of the `kei-sage` / `kei-crossdomain` SQLite databases. Periodic cleanup: exponential decay on edge weights, prune orphans.

```
kei-curator --db <path> <subcommand>
  decay [--default-lambda <λ=0.05>] [--threshold <θ=0.1>]
  prune-orphans
```

Requires an explicit `--db <path>` — there is no default.

### `keisei` — exobrain multi-client CLI (v0.19+)

Entry-point that mounts a portable brain directory into one or more AI clients. See the [dedicated deep-dive](#keisei-cli--exobrain-entry-point) below.

## Shell primitives

All 13 live under `_primitives/*.sh`. Installed with `chmod +x` at `~/.claude/agents/_primitives/`. Shell primitives are POSIX sh where feasible; two (`provision-hetzner`, `provision-vultr`, `harden-base`) use bash explicitly.

### `tomd.sh` — universal format → markdown

Converts PDF / DOCX / DOC / HTML / PPTX / XLSX / CSV / images / source code to markdown. Used by the `tomd-preread` hook to auto-convert binary formats before Claude reads them. Deps: `pandoc`, `python3`, `jq`. Optional: `pymupdf4llm` (better PDF), `openpyxl` (XLSX tables), `tesseract` (OCR).

### `design-scrape.sh` — Playwright site scrape

Scrapes a live URL into tokens + section map + desktop/mobile screenshots.

```
design-scrape <url> [--out <dir>]
```

Output: `<out>/desktop.png`, `<out>/mobile.png`, `<out>/tokens.json`, `<out>/structure.json`. Requires `npx` + Playwright chromium.

### `live-preview.sh` — dev-server wrapper

Detects framework from `package.json`, runs `npm run dev`, writes PID to `.keisei/dev-server.pid` for the `site-wysiwyd-check` hook to discover.

```
live-preview start <dir>
live-preview stop  [pid]       — default: reads .keisei/dev-server.pid
live-preview status
```

### `figma-tokens.sh` — Figma → tokens.json

Fetches Figma Variables + Styles via REST API, emits a `tokens.json` consumable by `tokens-sync`.

```
FIGMA_TOKEN=figd_xxx figma-tokens <file-key> [--out <path=tokens.json>]
```

Token MUST come from env (RULE 0.8). File-key is the segment after `/design/` or `/file/` in the Figma URL.

### `frontend-inspect.sh` — project fingerprint

Reports framework (Astro / Next / SvelteKit / Vite-React / static / unknown), styling (tailwind4 / tailwind3 / css-modules / plain), package manager, component count, whether tests exist.

```
frontend-inspect [<dir>] [--json]
```

### `screenshot-decode.sh` — vision-API screenshot → structured description

Posts a PNG + prompt to the Anthropic Messages API (claude-sonnet-4) and prints the text response.

```
ANTHROPIC_API_KEY=sk-ant-xxx screenshot-decode <png> [--prompt <text>]
```

Default prompt asks for token + layout + sections as JSON. API key MUST come from env (RULE 0.8).

### `metrics-scrape.sh` — Prometheus /metrics scraper

Scrape + format / filter / alert-check. POSIX sh.

```
metrics-scrape <url>                                  # table (default)
metrics-scrape <url> --format json                    # needs jq
metrics-scrape <url> --format alert-check --filter <re> --threshold <n>
metrics-scrape <url> --filter '^http_requests_total'
```

`alert-check` format exits non-zero if any filtered metric exceeds threshold.

### `log-ship.sh` — structured log tee + forward

Pipes JSON-line logs from stdin to stdout and optionally forwards to Loki / Datadog / generic HTTP. Local tee ALWAYS happens, even if forward fails — observability must degrade gracefully.

```
cat log.jsonl | log-ship --target stdout
journalctl -o json | log-ship --target loki --endpoint http://loki:3100/loki/api/v1/push --label job=api
tail -f app.log | log-ship --target datadog --endpoint <dd-url>
cat log.jsonl | log-ship --target http --endpoint <url>
cat log.jsonl | log-ship --target stdout --validate
```

Env (no CLI token leak): `LOG_SHIP_DD_API_KEY`, `LOG_SHIP_BEARER`.

### `provision-hetzner.sh` — Hetzner Cloud provisioner

Idempotent wrapper over `hcloud` CLI. Re-running `create <name>` on an existing server prints its IP and exits 0.

```
provision-hetzner create <name> [--type cx22|cax11] [--location fsn1]
                                [--image debian-12] [--ssh-key <id>]
                                [--firewall <name>] [--user-data <file>]
provision-hetzner status  <name>
provision-hetzner destroy <name> [--force]
provision-hetzner list
```

Env (RULE 0.8): `HCLOUD_TOKEN`.

### `provision-vultr.sh` — Vultr provisioner

Same shape as Hetzner. Uses `vultr-cli` v3.

```
provision-vultr create <label> [--plan vc2-1c-1gb] [--region ams]
                               [--os-id 2136] [--ssh-key <id>]
                               [--firewall <group-id>] [--user-data <file>]
provision-vultr status  <label>
provision-vultr destroy <label> [--force]
provision-vultr list
```

Env (RULE 0.8): `VULTR_API_KEY`. Idempotency key is the human `label` field.

### `harden-base.sh` — post-provision baseline hardening

Runs ON the target VPS. Generic Debian/Ubuntu hardening: apt, ssh drop-in, ufw, fail2ban, auditd, unattended-upgrades. Never reboots — surfaces `needrestart` hints only.

```
sudo bash harden-base.sh [--admin-user <name=keiadmin>] [--ssh-port <n=22>]
                          [--allow-port <n/proto>]... [--no-caddy]
                          [--no-reboot] [--skip apt|ssh|ufw|fail2ban|auditd|unattended]
```

Every step is test → configure → reload — re-running is safe.

### `kei-ci-lint.sh` — GitHub/Forgejo Actions linter

7 rule suite (R1-R7): required fields, least-privilege permissions, OIDC-vs-long-lived-token, cache-hit hygiene, SHA pinning, deprecated-action flags, pwn-request pattern.

```
kei-ci-lint <file.yml> [file2.yml ...]
kei-ci-lint --dir .github/workflows [--warn]
kei-ci-lint --dir .forgejo/workflows
```

Requires `yq` v4+ (mikefarah/yq Go impl — not the Python one).

### `kei-docs-scaffold.sh` — auto-doc generator

Detects project type (Cargo.toml / package.json / pyproject.toml / pubspec.yaml / go.mod / Package.swift / docker-compose), emits CLAUDE.md / DECISIONS.md / runbook / README.

```
kei-docs-scaffold [--type=all|claude|decisions|runbook|readme]
                  [--force] [--dry-run] [DIR]
```

Default: `--type=all`. Idempotent without `--force`.

## Hooks

All 12 kit-shipped hooks live under `hooks/*.sh`, get copied to `~/.claude/hooks/` on install. Every hook respects `KEI_DISABLED_HOOKS` and `KEI_HOOK_PROFILE` (see Runtime hook controls in [INSTALL.md](./INSTALL.md#runtime-hook-controls)). Silent fall-through on missing `jq` — never aborts a tool call system-wide. (v0.24 added `agent-capability-check` + `agent-capability-verify` for the substrate capability layer — not individually enumerated below yet.)

| Hook | Event | Severity | Bypass |
|---|---|---|---|
| `assemble-agents` | `PostToolUse:Edit\|Write` | advisory (rebuilds; never blocks) | — |
| `assemble-validate` | `PreToolUse:Bash` | **block** — exit 1 on `git commit` in `~/.claude` when manifests fail validation | — |
| `no-hand-edit-agents` | `PreToolUse:Edit\|Write` | **block** — exit 2 on generated `.md` edit attempts | `AGENT_MIGRATION=1` |
| `tomd-preread` | `PreToolUse:Read` | redirect — exit 2 with stderr pointing to cached `.md` | — |
| `agent-fork-logger` | `PreToolUse:Agent` | advisory (logs to `kei-ledger`; silent if absent) | — |
| `orchestrator-dirty-check` | `PreToolUse:Agent` | warn (stderr only) | `ORCHESTRATOR_META=1` or `ORCHESTRATOR_DIRTY_OK=1` |
| `site-wysiwyd-check` | `PostToolUse:Edit\|Write` | warn (stderr drift report) | — |
| `session-end-dump` | `Stop` | advisory (archives trace + calls `kei-memory ingest`) | — |
| `milestone-commit-hook` | `PostToolUse:Bash` | advisory (appends to audit-backlog on `feat:`/`refactor:`/merge) | — |
| `error-spike-detector` | `PostToolUse:*` | warn (stderr when ≥3 errors in 20-call window) | — |

Details beyond the table:

- **`assemble-agents`** — diff source: `_manifests/*.toml` rebuilds one agent; `_blocks/*.md` rebuilds ALL. Calls `_assembler/target/release/assemble --in-place`.
- **`assemble-validate`** — only fires when `git commit` runs inside `~/.claude`; validates every manifest; stderr-lists failures and exits 1 on any.
- **`no-hand-edit-agents`** — SSoT marker is `<!-- GENERATED by _assembler -->` on line 1. Files without the marker pass silently (legacy hand-authored agents). `AGENT_MIGRATION=1` overrides during migration only.
- **`tomd-preread`** — whitelist: `.docx`, `.doc`, `.xlsx`, `.pptx`, `.csv`. Cache key: basename + mtime + short path-hash (prevents collision between two same-basename files). Cache dir: `$KEISEI_TOMD_CACHE` or `/tmp/keisei-tomd-cache`.
- **`agent-fork-logger`** — extracts `subagent_type` + `prompt` + `isolation`; branch = `agent/<slug>-<ts>` when `isolation=worktree`, else `inline-<slug>-<ts>`. Spec-SHA = first 16 hex chars of SHA-256(prompt).
- **`orchestrator-dirty-check`** — runs `git status --porcelain` on repo root, stderr-warns with modified + untracked counts.
- **`site-wysiwyd-check`** — triggers on `.tsx` / `.vue` / `.svelte` / `.astro` / `.css` / `.html` / `.jsx` / `.ts`. Walks up to find `.keisei/dev-server.pid`; bails if no live server or no `.keisei/target.png`.
- **`session-end-dump`** — copies transcript JSONL to `~/.claude/memory/traces/<session_id>.jsonl`, calls `kei-memory ingest`, then best-effort calls `kei-sleep-sync.sh` (RULE 0.15, silent if sleep-sync not opted in).
- **`milestone-commit-hook`** — case-sensitive prefix match on `feat:`, `refactor:`, `merge` (avoids false-firing on `feature-docs.md`).
- **`error-spike-detector`** — rolling window in `~/.claude/memory/error-window.txt`; error-classifier = `is_error=true` OR message matches `/error:|failed|panic|denied/i`.

## Skills (grouped)

43 skills under `skills/<name>/SKILL.md`. Each is invoked as `/<skill-name>` inside Claude Code. Free-text is used only for intake fields; every other decision is a click via `AskUserQuestion`. (v0.23–v0.27 added 4 skills — `/spawn-agent`, `/atom-new`, and others — not yet individually enumerated below.)

<details>
<summary><b>Meta / project setup (4)</b></summary>

| Skill | One-liner |
|---|---|
| `/compose-solution` | Meta-orchestrator — converts a free-text task into the right artefact (agent / skill / hook / rule / block) by composing existing primitives. Enriches `_blocks/` over time. |
| `/new-project` | 4-phase bootstrap — intake, fork skeleton (branch + ledger row + sub-agent spawn), parallel execution with progress aggregation, per-branch merge ceremony. RULE 0.12 at project scale. |
| `/new-agent` | Interactive 6-question wizard that builds a project-specialist manifest and its `.md`. |
| `/onboard` | Scan a project (or scope) and propose agents + hooks + primitives based on detected stack. Three modes: Full auto, Step-by-step, Full manual. |

</details>

<details>
<summary><b>Design / frontend (18)</b></summary>

| Skill | One-liner |
|---|---|
| `/site-create` | End-to-end site pipeline — intake → design → sections → WYSIWYD mock-render loop → audits → preview → deploy. The verify gate HARD-BLOCKS deploy of unlocked sections. |
| `/site-builder` | Build a site from block recipes. WYSIWYD invariant via `mock-render`. |
| `/site-teardown` | Deconstruct any live site into a reusable recipe — HTML, CSS, JS, tokens, animations. |
| `/landing-page` | Orchestrates design + copy + assets + animations + SEO for a landing page. Supports recipes (apple-product, saas, portfolio, ecommerce). |
| `/design-system` | Build a design system — tokens, base components, Tailwind config, dark mode, docs. |
| `/ui-component` | Build a UI component — API design, variants, a11y, animations, tests. |
| `/form-builder` | Multi-step forms — Zod validation, Turnstile anti-spam, serverless backends, upload, progressive enhancement. |
| `/scroll-animation` | Scroll-driven animation — GSAP ScrollTrigger, CSS scroll-timeline, parallax, pin/scrub. |
| `/motion-design` | Motion design — page transitions, element animations, View Transitions API, Rive/Lottie, a11y. |
| `/web-effects` | Visual web effects — WebGL shaders, particles, noise/grain, displacement maps, CSS-only. |
| `/web-assets` | Image / font / video optimization — AVIF, responsive srcset, font subsetting, Sharp.js. |
| `/figma-to-code` | Figma design → code — screenshot, context, tokens, responsive implementation. |
| `/frontend-design` | Anti-AI-slop aesthetic philosophy — typography pairing, color theory, spatial composition, motion guidelines, design archetypes. |
| `/responsive-audit` | 6-breakpoint audit — layout, touch targets, overflow, images. |
| `/a11y-audit` | WCAG 2.2 AA compliance — contrast, keyboard nav, screen reader, prefers-reduced-motion. |
| `/perf-audit` | Perf baseline → profile → top-3 bottlenecks → fix → remeasure. |
| `/seo-audit` | Technical + content SEO via WebFetch + code inspection. |
| `/web-deploy` | Cloudflare Pages / Vercel / edge functions / caching / Core Web Vitals / CI/CD / DNS. |

</details>

<details>
<summary><b>Infra / ops (4)</b></summary>

| Skill | One-liner |
|---|---|
| `/vm-provision` | End-to-end VPS — provider → plan → provision → harden → `ssh-check` + `firewall-diff` hard-gate → handoff. Stops if either verification fails. |
| `/ci-scaffold` | Production CI/CD plan — platform (GitHub vs Forgejo), build matrix, OIDC-vs-token, release automation, security gate. Emits workflow YAML + runs `kei-ci-lint`. |
| `/observability-setup` | Logs + metrics + traces triad on an existing service — instrumentation → scrape+ship → dashboard → alerts. |
| `/auth-setup` | Production auth/IAM plan — user flows, IdPs, session strategy, authorization model, threat mitigations. Never writes secret values. |

</details>

<details>
<summary><b>API / schema / docs / tests (5)</b></summary>

| Skill | One-liner |
|---|---|
| `/api-design` | API design — style (REST / GraphQL / tRPC / gRPC), resource model, OpenAPI 3.1 or GraphQL SDL, versioning, rate-limit + auth handoff, codegen. |
| `/schema-design` | Relational schema → migrations → `kei-migrate` apply. PG / SQLite / MySQL autodetect. |
| `/docs-scaffold` | 5-phase — detect project type, audit existing docs, generate CLAUDE.md / DECISIONS.md / runbook / README / diagrams / CHANGELOG. |
| `/test-matrix` | Beyond-unit test stack — fuzzing, property-based, load, E2E, mutation. Composes right mix per language × critical path × CI target. |
| `/test-gen` | Generate tests for untested code — happy path, edge cases, error handling. |

</details>

<details>
<summary><b>Retro / audit / research (5)</b></summary>

| Skill | One-liner |
|---|---|
| `/self-audit` | RULE 0.14 session retrospective triage — runs `kei-memory analyze + patterns`, routes findings to `/escalate-recurrence` / `/debug-deep` / audit-backlog. |
| `/pr-review` | PR review — Constructor Pattern awareness, security, SSoT check. |
| `/refactor` | Refactor with behavior preservation — checkpoint, extract, test, audit. |
| `/debug-deep` | 5-phase RCA using multi-agent analysis + error pattern matching. |
| `/research` | Deep research via parallel agents + web search + cross-referencing. |

</details>

<details>
<summary><b>Sleep layer + runtime (3)</b></summary>

| Skill | One-liner |
|---|---|
| `/sleep-setup` | RULE 0.15 one-time wizard — local-only / remote-only / hybrid, trigger time, memory-repo init, SSH deploy key, `/schedule create` + cron snippet. |
| `/sleep-on-it` | v0.12 incubation — defer a question to the nightly remote agent. Up to 5 tasks per night (15 min each). Priority maps to budget. |
| `/hooks-control` | v0.15.1 click-only runtime enable/disable — emits shell `export` / `unset` for user to paste. Does NOT execute anything itself. |

</details>

## `keisei` CLI — exobrain entry point

`keisei` is the only Rust primitive that mounts state into OTHER tools' configs. It reads a portable brain directory (see brain layout in [INSTALL.md](./INSTALL.md#the-keisei-cli--multi-client-exobrain-mount-v019)) and writes `mcpServers.keisei` entries into each detected AI client's config file.

**Five subcommands** (every flag listed is the actual clap arg surface in `_primitives/_rust/keisei/src/main.rs`):

```
keisei attach <brain-path> [--scope user|project]
keisei mount  <brain-path>
keisei detach
keisei status
keisei list-adapters
```

**Flag matrix:**

| Subcommand | Required args | Flags | Notes |
|---|---|---|---|
| `attach` | `<brain-path>` (dir with `manifest.toml`) | `--scope user\|project` (default `user`) | Attaches to the **first detected** client. Adapters that don't support requested scope error out cleanly. |
| `mount` | `<brain-path>` | — | Auto-attach to EVERY detected AI client. Always user-scope (host-wide fan-out by design). |
| `detach` | — | — | Removes `mcpServers.keisei` from every client in the marker; preserves the user's other MCP entries. Deletes marker. |
| `status` | — | — | Brain name + path + attach timestamp + per-client config path + health (brain root is a dir? `mcp_server` binary exists?). |
| `list-adapters` | — | — | Tabular view: `name / detected / config_path / scopes`. |

**Supported adapters (v0.21):** Claude Code, Cursor, Continue, Zed. Claude Code and Cursor advertise both `user` + `project` scope. Continue and Zed are user-only.

**Exit codes:**
- `0` — success
- `1` — error (brain validation fail, no adapter detected, NameConflict, scope unsupported, IO)

No separate exit code for "config invalid" — all errors funnel through exit 1 with the specific message printed to stderr.

**Env vars:**

| Var | Purpose |
|---|---|
| `KEISEI_HOME` | Test hook — overrides `$HOME` for marker-file resolution and adapter config-path lookup |
| `KEI_STORE_S3_ENDPOINT` | Custom S3-compatible endpoint (R2 / MinIO / Wasabi) — consumed by `kei-store` when built with `--features s3` |
| `KEI_STORE_ALLOW_S3_STUB` | Set to `1` to permit the local-manifest S3 stub when the real `s3` feature isn't built |
| `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` / `AWS_REGION` | Standard AWS credential chain — consumed via `aws-sdk-s3` default resolver |
| `KEI_AUTH_KEY` | HMAC secret for `kei-auth` (NOT a CLI flag since v0.14.1) |

**Marker SSoT:** `~/.keisei/attached.toml` (v0.21+). Schema v3 entries have a `scope` field. Legacy `~/.claude/keisei-attached.toml` auto-migrates on first read (one-shot: legacy deleted, new location written, stderr notice printed).

**Security hardening (v0.19):**
- Brain `mcp_server` path MUST be relative + inside the brain root (rejects `/usr/bin/curl`, `../../etc/shadow`, Windows-style `..\..\`)
- Brain `name` matches `^[a-z][a-z0-9_-]{0,63}$`
- Brain root rejected if it's a symlink (blocks USB → `$HOME` pivot)
- Adapters refuse to clobber existing `mcpServers.<name>` entries — explicit `NameConflict` error, no silent overwrite
- All config writes go through `fsx::write_atomic_json` (Windows-safe via `tempfile::NamedTempFile::persist`)

## Pipelines

Hub-and-spoke skills that combine primitives into end-to-end flows. Each one is an option-picker-first, free-text-last wizard; every phase has a verify-criterion.

| Skill | One-line purpose |
|---|---|
| `/compose-solution` | Meta-composer: decompose any task, grep prior art, propose math-first architecture, assemble the right artefact (agent / skill / hook / block) |
| `/new-project` | Bootstrap a project specialist agent + repo skeleton + bridges + ledger row |
| `/new-agent` | Interactive 6-question wizard that builds a project-specialist manifest and its `.md` |
| `/site-create` | Frontend stack pick → design tokens → scaffold → WYSIWYD loop (mock-render, visual-diff, tokens-sync) |
| `/schema-design` | DB schema design → migrations → `kei-migrate` apply (PG/SQLite/MySQL autodetect) |
| `/observability-setup` | Pick metrics + logs stack → scrape + ship config (`metrics-scrape`, `log-ship`) |
| `/auth-setup` | Pick auth model (session / JWT / OAuth2) → emit routes + middleware + token rotation |
| `/api-design` | Contract-first: pick REST vs GraphQL vs gRPC, emit types + handlers + tests |
| `/ci-scaffold` | GitHub Actions / Forgejo Actions workflow skeleton + `kei-ci-lint` pre-commit |
| `/test-matrix` | Test stack matrix: unit / integration / e2e / visual; pick stack, emit skeleton |
| `/docs-scaffold` | Doc site skeleton (mdbook / docusaurus / astro-starlight) + `kei-changelog` generator |
| `/vm-provision` | VM provider pick → `provision-*` primitive → `harden-base` + `ssh-check` + `firewall-diff` verification |

All pipelines share a single discovery layer: `/compose-solution` Phase 3's prior-art grep covers `_blocks/`, `_manifests/`, `_primitives/` (shell + Rust), `skills/`, `_bridges/`, `hooks/` — so any pipeline can reuse primitives without re-inventing them.
