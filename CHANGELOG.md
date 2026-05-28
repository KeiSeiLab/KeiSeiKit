# Changelog

All notable changes are tagged via `git tag v*`. This file tracks unreleased work + release pointers.

## Unreleased

- `chore(docs)`: regenerate DNA-INDEX (reduced scope) (`302ca661`)
- `fix(kei-conflict-scan)`: close 3 backlog bugs + Phase C draft emission (`f354aacc`)
- `feat(kei-buddy)`: conversational LLM-driven flow + kei-sage retrieval graph-RAG (`b61b17ea`)
- `feat(contacts)`: glue sync + Google pagination + Apple discovery + folding (`06bcce99`)
- `fix(kei-conflict-scan)`: wikilink path-norm + drop handoff false-positives (`6cd99982`)
- `feat(kei-buddy fleet)`: 5 atomics — google/apple contacts + classifier + tick + slash-commands (`450156a4`)

## Released

Release notes per tag are kept in the GitHub Releases UI:
https://github.com/KeiSeiLab/KeiSeiKit/releases

Highlights below; full notes in each tag's GitHub Release page.

### v0.45.0 — post-install onboarding wizard + 5 prod-install bug fixes (2026-05-26)

User feedback from real curl|bash with `profile=full`: "нет выбора провайдера, нахуй не понятно что делать после установки". Closed.

- **NEW** `kei onboard` — 4-step wizard auto-triggered at end of install (TTY only). Walks user through: pick primary CLI → kei mcp-wire → MOONSHOT_API_KEY hint → kei-doctor health check. Re-runnable any time.
- **NEW** `bin/kei onboard|setup|wizard` arm.
- **FIX** `act_runner: command not found` — resolver tries `act_runner` → `gitea-runner`; brew install switched to `gitea-runner` (functionally equivalent for Forgejo).
- **FIX** Forgejo `no such table: user` — added `forgejo migrate` before `admin user create` (idempotent).
- **FIX** `zoekt: No formulae or casks found` — graceful fallback: brew taps → `go install` → clean skip with warning.
- **DEFERRED** `kei-shared missing` + launchd `Input/output error` → v0.46.

### v0.44.0 — pre-release audit: 1 CRITICAL + 4 HIGH + 4 MEDIUM (2026-05-26)

Four-CLI parallel pre-release audit (Claude+Grok+Gemini+Copilot, each reviewing different angle) surfaced 9 real issues in v0.43. All patched.

- **CRITICAL** Walk-up canonicalize for non-existent leaf paths (defeats v0.42 fix #1 when parent didn't exist either).
- **HIGH** O_NOFOLLOW open + fd-write closes TOCTOU window during hook chain await.
- **HIGH** Sanitize MOONSHOT_API_KEY pre-curl (config injection blocked).
- **HIGH** `env_clear` + whitelist on subprocess spawn (no secret leak via kei_bash).
- **HIGH** `Path::starts_with` + canonical KEI_ALLOWED_ROOTS (no prefix-bypass).
- **MED** macOS $TMPDIR carve-out (allowed_roots check FIRST; narrowed /var/ blanket).
- **MED** Timeout doc honesty (per-step not aggregate).
- **MED** cwd in hook input.
- **MED** Failure-fallback cache has full schema.

### v0.43.0 — kei limits + 4 audit fixes (2026-05-26)

- **NEW** `kei limits` — honest subscription-quota report. Research-grounded: 4 of 5 CLIs have no public quota API. Only Kimi balance via Moonshot `/v1/users/me/balance` (requires MOONSHOT_API_KEY).
- **NEW** Pet integration — reads cache, shows Kimi balance segment if live.
- **FIX** Atomic cache write (mktemp + atomic mv).
- **FIX** `tonumber?` swallows parse errors; `_safe_json` wrapper.
- **FIX** Token off argv (curl `--config -` via stdin).
- **FIX** `jq` runtime guard.

### v0.42.0 — re-audit fixes: 1 CRITICAL + 5 HIGH+MED (2026-05-26)

Re-audit found v0.41 fixes were incomplete. All patched.

- **CRITICAL** Symlink leaf bypass — canonicalize full path + reject is_symlink leaf for new files (3-of-4 reviewers convergent).
- **HIGH** $HOME removed from default allowed_roots (was self-neuter vector — agent could overwrite `~/.claude/hooks/*`).
- **HIGH** Empty section `[bash]/[edit]/[write]` now also FAIL-CLOSED.
- **MED** `tokio::fs` in load_chain.
- **MED** process_group + killpg applied to hook subprocess too.

### v0.41.0 — security hardening from Phase C dogfooding (2026-05-26)

- **HIGH** Fail-CLOSED on missing config + hook (was: silent pass-through).
- **HIGH** Path-traversal guard (denylist + canonicalize).
- **MED** `tokio::fs` async I/O (was: blocking std::fs on tokio thread).
- **MED** Process-group kill on Unix.

### v0.40.0 — Phase C: cross-CLI hook enforcement (2026-05-26)

- **NEW** `kei_bash` / `kei_edit` / `kei_write` MCP tools in `kei-mcp`.
- **NEW** `policy-chain.toml` SSoT for which hooks gate which tool.
- **NEW** 3-tier enforcement model (Claude+Grok TIER 1, Copilot TIER 2, Agy+Kimi TIER 3).
- **NEW** `kei mcp-wire` orchestrator + 5 per-CLI wire scripts.

### v0.39.x — multi-LLM DNA (2026-05-26)

- **NEW** `kei pick` interactive picker.
- **NEW** `kei agent <name>` with DNA-driven provider resolution.
- **NEW** `kei primary` get/set default backend.
- **NEW** `spawn_agent` MCP tool — any MCP-capable CLI can spawn KeiSeiKit agents on any backend.

### v0.38.0 — opt-in hook packs + stack profiles (2026-05-26)

- **NEW** Hook packs (safety / evidence / observability / epistemic / orchestration / git-guard / stack-rust).
- **NEW** Stack profiles (minimal / web / ml / systems / mobile).
- **NEW** `kei configure` re-runnable.
