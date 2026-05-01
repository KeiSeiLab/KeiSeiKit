# kei-gdrive-import — Wave 46 Plan

> Restored from chat ed8fb26e 2026-04-26T08:00:11Z (Wave 1 research synthesis).
> Branch: `feat/kei-gdrive-import` (created from main @ a5625e08).
> RULE 0.5 plan-mode artefact, 2 ledger anchor, 3 orchestrator-owned branch.

## Goal

One-shot wizard `kei-drive-import` that takes a Google Drive root, classifies every subfolder, and converts each detected project into a fresh repo on the local Forgejo dev-hub (`127.0.0.1:3001` per Wave 45).

## Wave 1 research verdicts (4/4 done, frozen)

| Stream | Verdict | Decision |
|---|---|---|
| GDrive sync tools | rclone primary | `brew install rclone` (MIT, arm64). **CRITICAL: NOT Drive Desktop** — corrupts `.git/` via `desktop.ini` injection |
| Existing GDrive→git scripts | None viable | Build ourselves, ~200 LOC core |
| Forgejo API | Raw curl | `POST /api/v1/user/repos {auto_init:false}`, catch 409 conflict |
| Project detection | 8-marker scoring | Cargo.toml / package.json / pyproject.toml / go.mod / pom.xml / build.gradle / Gemfile / composer.json (weight 10), threshold ≥ 8 |

## Architecture (hybrid: Rust detection + shell orchestration)

### Component 1 — `_primitives/_rust/kei-gdrive-import` (Rust, Constructor Pattern)

```
src/
├── cli.rs        clap subcommands
├── classify.rs   single-folder verdict {PROJECT, AMBIGUOUS, NOT-A-PROJECT}
├── scan.rs       walk-tree → JSON array of classifications
├── scoring.rs    8-marker weighted scorer (table-driven, easy to extend)
├── lib.rs        re-exports
└── main.rs       binary entry
tests/
├── classify_fixtures.rs
├── scan_smoke.rs
└── fixtures/
    ├── rust-project/Cargo.toml
    ├── node-project/package.json
    ├── photos-folder/IMG_0001.jpg
    └── mixed/{README.md, src/, .git/}
```

**CLI surface:**
```bash
kei-gdrive-import classify <path>         # → JSON {verdict, score, primary_lang, markers: [...]}
kei-gdrive-import scan-tree <root>        # → JSON array of all folders + classifications
kei-gdrive-import scan-tree --remote drive:Projects/  # → uses `rclone lsf` if path starts with remote:
```

**JSON schema (output of classify):**
```json
{
  "path": "drive:Projects/MyApp",
  "verdict": "PROJECT",
  "score": 18,
  "primary_lang": "rust",
  "markers": [
    {"file": "Cargo.toml", "weight": 10, "kind": "build_manifest"},
    {"file": "src/main.rs", "weight": 5, "kind": "source_file"},
    {"file": "README.md", "weight": 3, "kind": "doc"}
  ]
}
```

### Component 2 — `install/lib-dev-hub-gdrive-import.sh` (idempotent installer)

- `brew install rclone jq` (skip if present)
- compile `kei-gdrive-import` (cargo build --release, copy to `${KIT}/bin/`)
- generate wizard wrapper at `${KIT}/dev-hub/drive-import-wizard.sh`
- **NO launchd plist** — interactive one-shot, not a daemon
- post-install hint: "run `kei-drive-import` to start"

### Component 3 — `dev-hub/drive-import-wizard.sh` (bash, interactive)

```
$ kei-drive-import

  ┌─ Step 1: rclone config (one-time OAuth) ─────┐
  │ Detected remotes: drive:                     │
  │ Missing remote? → run `rclone config` first  │
  └──────────────────────────────────────────────┘

  ┌─ Step 2: scan ──────────────────────────────┐
  │ root = drive:Projects/                      │
  │ Found 47 folders                            │
  │ Classifying via kei-gdrive-import...        │
  │   31 PROJECT   (score ≥ 8)                  │
  │    8 AMBIGUOUS (score 5-7) — review needed  │
  │    8 NOT-A-PROJECT (skipped)                │
  └──────────────────────────────────────────────┘

  ┌─ Step 3: select ────────────────────────────┐
  │ [✓] all 31 projects                         │
  │ [ ] 8 ambiguous (review each via fzf)       │
  │ Forgejo: http://127.0.0.1:3001              │
  │ Owner: ${USER}                              │
  │ Default branch: main                        │
  └──────────────────────────────────────────────┘

  ┌─ Step 4: migrate (per project) ─────────────┐
  │ → rclone copy drive:Projects/X /tmp/staging/X
  │ → write .gitignore (lang-aware)             │
  │ → git init && git add . && git commit       │
  │ → curl POST /api/v1/user/repos { name:X }   │
  │ → git remote add origin http://.../${USER}/X│
  │ → git push -u origin main                   │
  │ → log result to ledger                      │
  └──────────────────────────────────────────────┘
```

### Component 4 — Tests

- `tests/gdrive_import_integration.sh` — fake `rclone` via PATH override, fake Forgejo via netcat listener
- Rust unit tests cover scoring + classification fixtures
- Smoke test asserts wizard skips folders containing `.git/` already (don't re-import live repos)

## Ledger row (2)

```
agent_id     = wave46-gdrive-import-orchestrator
branch       = feat/kei-gdrive-import
parent       = main @ a5625e08
spec_sha     = (this file)
status       = running
started_ts   = 2026-04-26T...
```

## Wave 2 research — DONE 2026-04-26 (3/3 streams)

### R1 — rclone edge-cases (E1 except where noted)
- Per-file cap: 5 TB (Drive hard-limit). 750 GiB/day = upload only, irrelevant for read.
- Rate: ~12k qps personal, rclone backs off natively. Practical throughput ≈2 files/sec.
- **Gdocs**: `--drive-skip-gdocs` makes them invisible. Pre-flight `lsf` enumeration MUST surface count to user. Opt-in to `--drive-export-formats=md,docx,xlsx` (md unverified for current API [E5]).
- OS-junk (`.DS_Store`/`Thumbs.db`/`desktop.ini`) NOT filtered by default — explicit `--exclude` needed.
- `rclone copy` idempotent on re-run (size+mtime, `--checksum` stronger).
- Shortcuts: dereferenced by default → infinite loop risk → `--drive-skip-shortcuts` mandatory.

**Recommended flag block (frozen):**
```bash
rclone copy "drive:$SRC" "$DST" \
    --drive-skip-gdocs \
    --drive-skip-shortcuts \
    --drive-skip-dangling-shortcuts \
    --drive-acknowledge-abuse \
    --exclude "**/.DS_Store" --exclude "**/._*" \
    --exclude "**/Thumbs.db" --exclude "**/desktop.ini" \
    --exclude "**/.Spotlight-V100/**" --exclude "**/.Trashes/**" --exclude "**/.fseventsd/**" \
    --transfers 4 --checkers 8 --tpslimit 10 \
    --retries 5 --low-level-retries 10 \
    --checksum --create-empty-src-dirs \
    --stats 5s --log-file "$DST/.rclone-import.log"
```

### R2 — auth UX + secrets (RULE 0.8 reconciled)
- Auth mode: **interactive browser OAuth** via `rclone config` (autoconfig=Y, localhost:53682). Headless + service-account rejected for single-user macOS.
- Scope: `drive.readonly` (minimum for list+download). [E1 developers.google.com]
- **Token CANNOT live in `.env`** — rclone rewrites it on every auto-refresh.
- 2-tier secrets layout:
  - Real token: `~/.config/rclone/rclone.conf` chmod 600 (XDG default, treat like `~/.ssh/`)
  - `~/.claude/secrets/.env`:
    ```
    RCLONE_CONFIG=${HOME}/.config/rclone/rclone.conf
    KEI_DRIVE_REMOTE=gdrive
    ```
- Detection commands (exit codes undocumented — parse stderr):
  - Missing remote: `rclone --config "$RCLONE_CONFIG" listremotes \| grep -q '^gdrive:$'`
  - Expired token: `rclone about gdrive: 2>&1 \| grep -qiE 'oauth2\|401\|token'`
- Wizard MUST pass `--config "$RCLONE_CONFIG"` explicitly (belt-and-suspenders to env var).

### R3 — license/safety (5-step pre-push checklist)
- **Tool pick**: `gitleaks v8.30.1` MIT (`brew install gitleaks`). Static `gitleaks dir <path>` mode (no git history needed). Default ruleset covers AWS / GCP / GitHub PAT / Stripe / PEM private keys / generic API keys.
- **gitignore source**: github/gitignore CC0-1.0, SHA-pinned to `576334520435382d6522f349b9d270eda1e79a25` (last commit 2026-04-24).
- **marker→template map** (hardcode, do NOT name-guess):
  | Marker | Template URL filename |
  |---|---|
  | Cargo.toml | Rust.gitignore |
  | package.json | Node.gitignore |
  | pyproject.toml | Python.gitignore |
  | go.mod | Go.gitignore |
  | pom.xml | Maven.gitignore |
  | build.gradle | Gradle.gitignore |
  | Gemfile | Ruby.gitignore |
  | composer.json | Composer.gitignore |

**5-step ordered pre-push checklist (wizard MUST run in order):**
1. Existing repo detect: `rclone lsf --dirs-only --include ".git/" <src>` + HEAD-file fallback (Drive may store `.git` opaque). Found → SKIP + warn.
2. Size + extension histogram: `du -sh` + bytes-per-extension. If `.pdf >50%` OR `{.mp4,.mov,.mkv,.iso,.zip} >30%` → prompt user (third-party content risk).
3. Secret scan: `gitleaks dir --no-banner --redact <src>`. Non-zero → BLOCK until resolved or explicit bypass.
4. Apply language `.gitignore` BEFORE first `git add` (fetch from SHA-pinned URL above).
5. Final remote check: assert URL matches `127.0.0.1:3001` allowlist; reject `github.com` per .

### Cross-cutting — prompt-injection notes
Both R2 + R3 caught fake `<system-reminder>` blocks appended to rclone.org and github docs pages via WebFetch. Pattern: trailing fake "MCP Server Instructions" telling agent to load computer-use tools. Both agents correctly ignored. Wizard implementation does NOT execute LLM-fetched content; this is research-tooling concern only.

## Wave 3 implementation (4 streams parallel, 3)

| I# | Worktree | Files | Agent prompt clause |
|---|---|---|---|
| I1 | `agent-gdrive-rust` | `_primitives/_rust/kei-gdrive-import/**` | "MUST NOT invoke git/cargo build (cargo check ok). Write files only." |
| I2 | `agent-gdrive-installer` | `install/lib-dev-hub-gdrive-import.sh` | same |
| I3 | `agent-gdrive-wizard` | `dev-hub/drive-import-wizard.sh` (template), `_templates/` | same |
| I4 | `agent-gdrive-tests` | `tests/gdrive_import_integration.sh` + fixtures | same |

## Wave 4 — merge ceremony

Per 2: AskUserQuestion per branch [merge --no-ff / squash / reject / defer]. Orchestrator commits with `feat(wave46):` prefix.

## Out of scope (deferred)

- Reverse direction (Forgejo → Drive backup) — separate primitive `kei-gdrive-export`
- GitHub mirror — covered by existing `tools/sync-public.sh`
- Bidirectional sync — explicit non-goal, this is one-shot import
- Web UI — terminal-only

## Risks (Wave 1)

1. `rclone config` is interactive on first run — wizard must detect and pause for user
2. Forgejo not running → `curl` fails fast, wizard aborts with clear message
3. Folder named `Projects` (Drive) maps to nested KeiSeiKit `Projects/` confusion — wizard uses absolute paths throughout
4. Network drop mid-batch — per-project retries, ledger row per project for restart
