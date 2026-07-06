# Changelog

All notable changes are tagged via `git tag v*`. Latest entries first.

## Unreleased

- **self-audit codify quality gate (RULE 0.14-Q)** — Phase-4 `codify` /
  `create hook` routes now must carry a when-NOT-to-apply clause + a
  verification criterion, and match rigidity to finding severity
  (critical→block … low→note) before the `/escalate-recurrence` handoff
  (new `skills/self-audit/codify-quality-gate.md`). Adds a "When NOT to
  use" section to the skill. Method adapted from Trail of Bits
  `skills-curated/skill-extractor` quality guide. (`ee40c43`)
- **escalate-recurrence Verify criterion + hook smoke-test (RULE 0.14-Q)**
  — the codifier that self-audit's codify route hands off to now emits a
  `## Verify` section in every generated rule and smoke-tests the hook
  against the reproducing input (correct exit code + clean on benign
  input) *before* registration — a hook not seen fire is never registered.
  Closes the loop with `codify-quality-gate.md`. Pure-click contract
  preserved (Verify derived from Phase-0 evidence, no new question).
  (`3020172`)

---

## v0.63.0 — 2026-05-30

Post-audit fix-all release (5 HIGH + 7 MED from multi-LLM audit).

- **H1 numeric drift** — `plugin.json`, README header counts, `bin/kei`
  splash now read substrate version from `plugin.json` instead of
  hardcoding. Real counts: 37 agents / 52 skills / 53 hooks / 83 blocks.
- **H2** — stripped `_primitives/_rust/target/` + `_assembler/target/`
  from on-disk scrub tree (15 GB → 23 MB).
- **H3 workspace orphans** — `kei-graph-stream` added to umbrella
  `members`; `kei-model-router` added to umbrella `exclude` (it ships
  its own `[workspace]` for faster iteration).
- **H4** — README + `_blocks/baseline.md` claim "no abstract factories"
  narrowed to user code. `Box<dyn Trait>` for backend dispatch (memory
  / git / llm pluggable stores) is canonical Rust and stays.
- **M1** — README + bootstrap.sh dropped the "private repo /
  `gh auth login`" prerequisite. Documented `--activate-hooks`.
- **M2** — `kei status` subcommand added (previously fell through to
  `claude status` silently).
- **M3** — `hooks/hooks.json` documented as curated plugin-format
  subset; `settings-snippet.json` is canonical for classic install.
  Both intentional.
- **M5** — `kei-search-core::StubFetcher` docstring clarifies that the
  user-facing `/research` skill works via Claude tools and does NOT
  depend on this crate. The Rust stub is for future Rust-side
  automation.
- **M6** — this changelog re-summarises v0.47 → v0.63 (17 untracked
  releases).
- **M7** — `install/lib-preflight.sh` Russian comments translated to
  English.

## v0.62.0 — 2026-05-30

Deep scrub of every leftover reference to the extracted frontend
cluster across ~50 files (install scripts, hook registries, MCP TS
registry, test fixtures, docs banners, block index, Forgejo CI matrix).

## v0.61.0 — 2026-05-30

Site-building cluster (17 skills + 8 primitives + frontend-validator
agent) extracted to a private sibling repo `KeiSeiLab/frontend-studio`
for productisation. Generic image / video / 3D / animation skills
(`nano-banana`, `video-gen`, `animate`, `motion-design`,
`scroll-animation`, `3d-scene`, `visual-explainer`,
`design-inspiration`, `playwright-cli`) remain in public KSK.

## v0.60.0 — 2026-05-28

Identity unified: `git filter-repo --mailmap` rewrote every commit's
author + committer to `KeiSei LLC <hello@keilab.io>`.
`--replace-message` removed remaining identity strings from commit
message bodies. Force-pushed main + all 32 tags.

## v0.59.0 — 2026-05-28

Bootstrap wizard fix: source `kei-prompt.sh` BEFORE invoking
`prompt_profile()` (the earlier order left `kei_is_interactive`
unbound, and the wizard silently defaulted to "minimal" under
`curl|bash`). Also reads from `/dev/tty` explicitly when stdin is
the curl pipe.

## v0.58.0 — 2026-05-28

`web-install.sh` auto-resets the `origin` URL if it disagrees with the
expected `KEISEI_REPO` — fixes stale checkouts from the keigit-mirror
era after the public-github migration.

## v0.57.0 — 2026-05-28

`install/lib-launchd.sh` functions guarded for Linux — every entry
point returns 0 immediately when `KEI_OS != darwin`.

## v0.56.0 — 2026-05-28

`install/lib-prereqs.sh` detects missing C toolchain (`cc`/`gcc`) on
Linux before invoking cargo build — emits actionable install hint per
distro instead of failing inside `libc` build script.

## v0.55.0 — 2026-05-28

KeiSei LLC named as project owner in LICENSE / NOTICE / plugin.json.
Linux compatibility guards added to all 7 dev-hub-* libs +
`lib-launchd.sh`. `lib-os.sh` introduced as OS-detection cube.

## v0.54.0 — 2026-05-28

Hierarchical multi-LLM orchestration trial — used claude/grok/agy/
kimi/copilot/codex CLIs as parallel sub-orchestrators for the v0.51
audit-fix batch. Validated the "team-of-CLIs" pattern that landed as
Phase C.

## v0.53.0 — 2026-05-28

Author identity unified across all `Cargo.toml`s (50+ files). All
v0.51 audit findings addressed.

## v0.52.x — 2026-05-22 → 2026-05-27

Rules-as-cubes design + TTY-interactivity-gate rule (after 7-instance
install incident — `[ -t 1 ]` falsely false under `curl|bash` because
of tee'd stdout). v0.52.1 converted Russian user-facing strings to
English.

## v0.51.x — 2026-05-21

9 HIGH/MED audit findings fixed in parallel agents (double-audit
protocol). `kei-cortex/src/handlers/webfetch.rs` decomposed
(448 LOC → policy cube).

## v0.50.0 — 2026-05-20

UX: complete-stack profiles (`full`, `cortex`, `full-hub`, `dashboard`,
`local-mirror`) imply `--yes` to skip all 3 wizards.

## v0.49.x — 2026-05-19

Refactored installer interactivity into `scripts/kei-prompt.sh` cube
(SSoT). v0.49.1 added a visible-reason banner when the onboarding
wizard skips; v0.49.2 guards `/dev/tty` open() with a subshell probe
to survive ENXIO in sandboxes.

## v0.48.0 — 2026-05-18

`bootstrap.sh` reattaches stdin to `/dev/tty` ONCE in the entry-point
so `curl|bash` prompts actually wait for the user.

## v0.47.0 — 2026-05-17

Splash polish: yellow drop-shadow on the KeiSei wordmark; post-install
launch prompt offers to run `kei` immediately; Windows guidance text
added.

---

## Earlier (v0.38 → v0.45)

### v0.45.0 — post-install onboarding wizard + 5 prod-install bug fixes (2026-05-26)

`kei onboard` wizard auto-triggers at end of install (TTY only).
4 fixes: act_runner → gitea-runner fallback; forgejo migrate before
admin user create; zoekt graceful skip; kei-shared / launchd
deferred to v0.46.

### v0.44.0 — pre-release audit: 1 CRITICAL + 4 HIGH + 4 MEDIUM (2026-05-26)

Four-CLI parallel pre-release audit (Claude+Grok+Gemini+Copilot)
surfaced 9 real issues; all patched.

### v0.43.0 — `kei limits` + 4 audit fixes (2026-05-26)

Honest subscription-quota report. Research-grounded: only Kimi has a
public balance API (`/v1/users/me/balance`); others have none.

### v0.42.0 — re-audit fixes: 1 CRITICAL + 5 HIGH+MED (2026-05-26)

Re-audit found v0.41 fixes were incomplete. Symlink leaf bypass closed
(canonicalize + reject is_symlink); $HOME removed from default
allowed_roots; fail-closed on empty config sections.

### v0.41.0 — security hardening from Phase C dogfooding (2026-05-26)

Fail-CLOSED on missing config; path-traversal guard;
`tokio::fs` async I/O; process-group kill on Unix.

### v0.40.0 — Phase C: cross-CLI hook enforcement (2026-05-26)

`kei_bash` / `kei_edit` / `kei_write` MCP tools in `kei-mcp`.
`policy-chain.toml` SSoT for which hooks gate which tool. 3-tier
enforcement model. `kei mcp-wire` orchestrator + 5 per-CLI wire
scripts.

### v0.39.x — multi-LLM DNA (2026-05-26)

`kei pick`, `kei agent <name>` with DNA-driven provider resolution,
`kei primary` get/set, `spawn_agent` MCP tool.

### v0.38.0 — opt-in hook packs + stack profiles (2026-05-26)

Hook packs (safety / evidence / observability / epistemic /
orchestration / git-guard / stack-rust). Stack profiles (minimal /
web / ml / systems / mobile). `kei configure` re-runnable.

---

Older release notes are kept on the GitHub Releases page:
https://github.com/KeiSeiLab/KeiSeiKit/releases
