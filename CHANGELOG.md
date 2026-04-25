# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Entries are generated from the git history via
`_primitives/_rust/kei-changelog` (a conventional-commits walker).
Regenerate a single version block with, e.g.:

```bash
_primitives/_rust/target/release/kei-changelog \
  --from v0.14.2 --to v0.15.0 --version v0.15.0 --update CHANGELOG.md
```

## [Unreleased]

> Work in flight on `feat/v0.16-changelog-gen` and follow-up branches.
> Only placeholders — no corresponding commits exist yet. Any line that
> ships must be replaced with the real commit summary before release.

### Changed
- **primitives (v0.22 — `keisei` schema v4, BREAKING marker shape):**
  - Marker schema bumped from v3 to v4. The top-level `brain_path` / `brain_name` / `attached_at` fields are gone; every `[[attachments]]` entry now owns its own `brain_path`, `brain_name`, and `attached_at`. Consequence: one marker can track multiple brains wired to different clients at the same time (e.g. brain-A on Claude Code at user scope + brain-B on Cursor at project scope). `config::AttachRecord::new(attachments)` is the fresh constructor; raw struct literals no longer compile.
  - `ClientAdapter::post_attach_hint` signature widened from `fn (&self) -> &'static str` to `fn (&self, brain: &Brain, scope: Scope) -> String` so adapters can interpolate the brain's name and the resolved scope into the reload instruction. Implementations of the trait outside this crate need to update.
  - Adapter enumeration centralised in `adapters/_registry.rs::all_adapters`. `adapter::all()` now delegates; the "add a 5th adapter" touchpoint drops from three files to one. Public API of `adapter::all()` unchanged.
- **primitives (v0.22 — `config.rs` decomposition):**
  - Schema-migration logic extracted from `config.rs` into `config_migrate.rs` (pure functions on `WireRecord` → `AttachRecord`). Time helpers extracted into `time.rs` with a 5-test anchor suite covering epoch-0, leap day 2020-02-29, century non-leap 2100-03-01, an arbitrary recent timestamp, and `civil_from_days` direct invariants. `config.rs` drops from 224 LOC to 197 LOC, below the 200-LOC Constructor Pattern ceiling.

### Added
- **primitives/keisei (v0.22 Track A — schema v4 multi-brain marker):** `AttachRecord` inverted so every `Attachment` carries its own `brain_path` + `brain_name` + `scope` + `attached_at`. Enables brain-A attached to Claude Code (user scope) + brain-B attached to Cursor (project scope) simultaneously in ONE marker. v1/v2/v3 readers transparent via `#[serde(untagged)]` — auto-migrate silently on first v0.22 `config::read()` with one-line stderr notice.
- **primitives (v0.22 — `Scope::Auto` CLI default):**
  - `keisei attach <brain>` without `--scope` now defaults to `auto`. Each adapter exposes `auto_scope()`; Claude Code returns `Scope::Project` when CWD has `.claude/` (dir or `settings.json`), Cursor when CWD has `.cursor/`. Continue + Zed stay on user-scope default. Team workflow `cd team-repo; keisei attach brain` now picks project-scope without an extra flag.
  - `Scope::Auto` is a CLI-level intent only — `attach.rs` resolves it to a concrete `User` / `Project` before writing the marker. The persisted marker never contains `auto`.
- **primitives (v0.22 — `keisei mount` per-adapter scope resolution):**
  - `mount` now resolves scope per-adapter via `auto_scope()` instead of forcing `Scope::User` across the fan-out. A single `keisei mount brain` inside `team-repo/` can wire Cursor at project scope and Claude Code at user scope (or both at project, depending on CWD).
- **primitives (v0.22 — templated `post_attach_hint`):** `fn post_attach_hint(&self, brain: &Brain, scope: Scope) -> String` interpolates the brain name + resolved scope into the per-adapter reload instruction. No more Claude-Code-specific string in the orchestrator.
- **primitives (v0.22 — adapter registry):** new `adapters/_registry.rs` (32 LOC) is the single canonical adapter list. `adapter::all()` delegates. Adding a 5th adapter is one line, one place.
- **primitives (v0.22 — `config.rs` decomposition):** `config.rs` 224 → 197 LOC. Extracted `time.rs` (90 LOC — `now_utc_string` + `format_epoch_utc` + `civil_from_days` + 5 unit tests covering epoch-0, leap-day 2020-02-29, century-non-leap 2100-03-01, arbitrary 2026-04-22, RFC3339 shape) and `config_migrate.rs` (114 LOC — `WireRecord` v1→v4 migration).
- **docs (v0.22 — README `## Reference` section):** major new section appended before `## Runtime hook controls`. Documents the actual CLI surface of every shipped component: 25 Rust primitives (clap subcommands + flags + state paths + exit codes extracted from `_primitives/_rust/*/src/main.rs`), 13 shell primitives (header docstrings + flags), all 10 hooks (event + severity + bypass table), 39 skills grouped into 6 collapsible categories, and a deep-dive on the `keisei` exobrain CLI (real flag matrix, exit codes, env vars, marker SSoT, v0.19 security hardening). Every claim is source-verified — flags not present in code are marked absent, not fabricated. Adds ~670 LOC to README.md; collapsible `<details>` used for the 39-skill table to keep scroll length reasonable.
- **primitives/keisei (v0.22 Track C — filesystem-type advisory):** new `fs_type.rs` cube (`<110 LOC`) classifies the brain root via `statfs(2)` on macOS + Linux and returns `FsWarning::{None,ExFat,Fat32,Unknown}`. Windows support deferred (returns `Unknown` until `GetVolumeInformationW` lands). `Brain::load` now prints a stderr advisory when exFAT / FAT32 is detected — SQLite WAL shared-mmap is unreliable there and `keisei mount` (multi-client) WILL corrupt `kei-memory` / `kei-artifact` / `kei-social-store` DBs. Warning is non-blocking — single-client `keisei attach` on exFAT stays supported. New runtime dep `libc = "0.2"` (unix-only). Two new integration tests (`brain_load_on_typical_filesystem_no_warn`, `fs_type_detection_returns_none_on_standard_fs`).
- **tests/battle (v0.22 Track C — distro matrix):** two new Dockerfiles alongside the existing `ubuntu:24.04` image. `Dockerfile.install-test-alpine` (Alpine 3.19 — musl libc, exposes musl-static-link quirks in `rusqlite` / `git2` / `aws-sdk-s3`). `Dockerfile.install-test-debian` (Debian 12 bookworm — glibc, different apt structure from Ubuntu). `README.md` documents the 3-image matrix and documents known musl-static-link failures as matrix signal rather than regression.
- **docs (v0.22 Track C — USB guide platform split):** `USB-BRAIN-GUIDE.md` restructured into a TOC + platform-agnostic preamble (prerequisites, exFAT/FAT32 warning, invariants, troubleshooting). Three new platform-specific walkthroughs: `USB-BRAIN-GUIDE-macos.md` (Gatekeeper `xattr`, `/Volumes/`, `diskutil`), `USB-BRAIN-GUIDE-linux.md` (`/media/$USER/`, `umount`, ext4, optional systemd-udev auto-attach), `USB-BRAIN-GUIDE-windows.md` (PowerShell, drive letter, NTFS, `Dismount-Volume`, FS-advisory returns `Unknown` caveat).

### Removed
- **primitives (v0.22 — dead `Error` variants):**
  - `Error::NotAttached` — never surfaced; `detach` prints "nothing to detach" and returns `Ok(())`.
  - `Error::AdapterFailed { client, reason }` — never constructed; `mount` / `detach` orchestration carried `(client, reason)` tuples instead. Downstream matches on these variants won't compile against v0.22.

### Added (pre-v0.22)
- **primitives (v0.21 — keisei SSoT relocation + `Scope` enum):**
  - Marker file relocated from `~/.claude/keisei-attached.toml` to `~/.keisei/attached.toml`. `~/.claude/` is Claude-Code-specific territory and should not host cross-adapter keisei state. `config::read()` performs a one-shot migration the first time it runs under v0.21: if the legacy file exists and the new location is empty, the marker moves over (new file written, legacy file deleted) and a stderr notice is emitted.
  - `Scope` enum (`user` / `project`) on the `ClientAdapter` trait. Adapters declare `supported_scopes()`; `config_path(scope)`, `attach(brain, scope)`, `detach(brain_name, scope)` are scope-aware. Claude Code and Cursor support both scopes; Continue and Zed are user-only. `keisei attach` gains `--scope=<user|project>` (default `user`); `keisei mount` stays host-wide (`Scope::User` fan-out by design).
  - Marker schema v3: each `[[attachments]]` entry carries `scope = "user" | "project"`. Pre-v0.21 markers without the field default to `Scope::User` silently. New error variant `Error::ScopeUnsupported { client, scope, supported }` fires when a caller asks for a scope the adapter doesn't advertise.
- **primitives (v0.21 — `kei-store` real S3 backend):**
  - `S3CloudStore` — functional S3 / R2 / MinIO / Wasabi backend via `aws-sdk-s3` v1. GetObject / PutObject / ListObjectsV2 (paginated) / DeleteObject wired behind the existing `MemoryStore` trait (sync-over-async via a single-thread tokio runtime). Enables `keisei attach s3://my-bucket/brain/` as a real cloud-mount path, not just a local stub.
  - Opt-in feature flag `s3` on the `kei-store` crate — off by default so users who don't need cloud pay zero binary weight. Enabling adds tokio + hyper + rustls + aws-sdk-s3 (~5 MB release binary growth [estimate, E5 — not yet measured; would require `cargo build --release` before/after feature flag]).
  - AWS default credential chain honoured (env vars → `~/.aws/credentials` → IMDS). No new credential format; RULE 0.8 secrets-single-source unchanged.
  - Endpoint override for non-AWS S3-compat providers via `KEI_STORE_S3_ENDPOINT` env var (runtime) or `s3.endpoint` in `store-config.toml` (persistent). Path-style addressing auto-enabled when a custom endpoint is set (MinIO / some R2 configs).
  - "Branch" semantics: S3 has no native branching, so a branch is modelled as a key prefix (`<branch>/<path>`). `branch()` sets the active prefix in-memory; default `main`.
  - Factory auto-routes: `backend = "s3"` + feature `s3` + `s3.bucket` set → real cloud; otherwise falls back to the v0.14 local-manifest stub (still behind `KEI_STORE_ALLOW_S3_STUB=1`).
  - Path-traversal guard parity with `FilesystemStore`: absolute and `..`-component paths rejected before keys are spliced.
- **tests/battle:** Docker-based clean-Ubuntu install test — `tests/battle/Dockerfile.install-test` + `verify.sh` + `battle-entry.sh` + README. Builds a fresh `ubuntu:24.04` image, runs `install.sh --profile=<minimal|core|dev|full>` under `--yes`, then asserts post-install counts (blocks ≥ 79, skills ≥ 39, top hooks ≥ 10, `_lib` hooks ≥ 2), runs `hooks/_lib/test-gate.sh`, and validates `settings.json`. First real-world "does it work on a fresh machine?" signal — CI previously only ran `--no-execute` dry-runs. v0.21 ship-blocker for any profile that regresses.
- **primitives (v0.20 — brain schema v2 + per-client hint):**
  - Brain schema v2 with per-platform `mcp_server` dispatch — a single brain directory can now host binaries for darwin-arm64/darwin-x64/linux-x64/linux-arm64/windows-x64 and `keisei attach` picks the right one automatically. Schema v1 (single string) still accepted for backward-compat.
  - `ClientAdapter::post_attach_hint()` — per-client reload instruction, no more hardcoded Claude-Code string in the orchestrator.
- **primitives:** `keisei` CLI MVP — `attach <brain-path>` + `status` subcommands for mounting a portable exobrain directory into Claude Code. First step of the v0.18 exobrain architecture (multi-client adapter surface prepared; only `claude-code` adapter ships in MVP).
- **primitives (v0.19 — multi-client exobrain):**
  - `keisei mount <brain-path>` — attach a brain to EVERY detected AI client in one shot (Claude Code + Cursor + Continue + Zed).
  - `keisei detach` — remove the brain from every client recorded in the marker, preserving user's other MCP/context-server entries.
  - `keisei list-adapters` — tabular dump of every registered adapter and whether it's detected on this host.
  - 3 new `ClientAdapter` implementations: `cursor` (`.cursor/mcp.json` project-local or `~/.cursor/mcp.json` global), `continue` (`~/.continue/config.{yaml,json}` — YAML preferred, JSON fallback), `zed` (`~/Library/Application Support/Zed/settings.json` on macOS or `~/.config/zed/settings.json` on Linux, under `context_servers`).
  - `keisei-attached.toml` schema **v2** — carries a list of `[[attachments]]` (client_type + config_path) instead of a single `client_type`. v1 markers read transparently (auto-migrated in memory).
  - New error variants: `AdapterFailed { client, reason }` and `ConfigParseError { path, reason }`.
- Placeholder: CHANGELOG.md generation wired through `kei-changelog` (this file).
- Placeholder: `.github/workflows/release.yml` — tag-driven multi-platform release.
- Placeholder: pre-built-binary install path in `install.sh` (`KEI_SKIP_RUST_BUILD=1`).
- added: `kei-mcp-server` single-binary compile for 5 platforms (linux/darwin/windows × x64/arm64 where available) via `bun build --compile` — v0.18 Phase 1 of the exobrain distribution architecture. Ships as bare binaries + `.sha256` sums on every GitHub release; `install.sh` detects a dropped binary at `_primitives/_rust/target/release/kei-mcp-server-<os>-<arch>` and skips bun/npm build. Opt-out via `KEI_SKIP_MCP_BUILD=1`. See `_ts_packages/packages/mcp-server/BUILD.md`.

### Changed
- **primitives (v0.22 Track B — `kei-store` AsyncBackend trait + shared tokio runtime):**
  - New `async_backend` module (gated behind `s3` feature) — introduces an `AsyncBackend` sub-trait (4 async methods: `get`/`put`/`list`/`list_recursive` + `label`) and a generic `AsyncBackendStore<B: AsyncBackend>` wrapper that implements `MemoryStore` on top of any backend. Adding a new cloud backend (GCS, Azure Blob, Bunny Storage, …) is now 6 methods, not a re-invention of the sync-over-async bridge.
  - Shared process-global multi-thread tokio runtime via `OnceLock<Runtime>` — 2 worker threads, `enable_io + enable_time`. Replaces the previous per-instance `current_thread` runtime inside `S3CloudStore`, which caused a `block_on` panic when two `S3CloudStore` instances in one process interacted across threads (N=2-Store footgun).
  - `S3CloudStore` is now `pub type S3CloudStore = AsyncBackendStore<S3AsyncBackend>`. Public API (`S3CloudStore::new(cfg)`, `.branch()`, `.current_branch()`, `.key()`, `.backend_name()`) preserved. New `S3AsyncBackend` struct in `s3_cloud/backend.rs` holds the `aws-sdk-s3::Client` and the bucket name; the sync wrapper handles branch-prefix + path-validation + commit-manifest.
  - `validate_rel`, `short_hash`, `is_manifest_key` helpers moved from `s3_cloud/keys.rs` into `async_backend` (single source of truth for every future cloud backend). `s3_cloud/keys.rs` kept as a thin re-export shim so external callers and its unit tests keep working unchanged.
  - New deps under `s3` feature: `async-trait 0.1` + `tokio` feature `rt-multi-thread`. No change to the default-feature dep graph.
  - +7 tests (5 async_backend units + `two_store_instances` + `runtime_is_multi_thread`). Existing 46 tests (31 unit + 9 integration + 6 smoke) unchanged and green.
- Placeholder: plugin / block format refresh targeted for v0.16.0.

### Security
- **primitives/keisei (v0.19.2 audit polish — M1):** `keisei-attached.toml` marker is now `chmod 0o600` on unix (Windows unchanged — no equivalent bit). The marker carries the resolved `brain_path` and every attached client's config path; restricting it to owner-only closes the residual "other local user can enumerate attached brains" surface.
- **primitives/keisei (v0.19.2 audit polish — L9):** every manifest-sourced string printed by `status` and `attach` (brain name, brain path, client/config paths) is now scrubbed through `display::sanitize_display`, which replaces every ASCII control byte (`< 0x20` or `== 0x7F`) with `?`. Closes the escape-sequence injection surface from a malicious `brain.name` like `"evil\x1b[2Jpayload"` that would otherwise clear the user's terminal or rewrite already-printed lines.
- **primitives/keisei (v0.19.2 audit polish — L12):** `manifest.toml` is now capped at 64 KiB (`Error::ManifestTooLarge { size, max }`). The check runs off `fs::metadata` before `read_to_string` so an attacker-supplied 1 GB file can't exhaust memory inside the toml parser. Legit manifests are ~1 KB; the cap is three orders of magnitude of headroom.

### Fixed
- Placeholder: hook-bypass edge case follow-up to v0.15.1.
- **primitives/kei-store (v0.21.1 audit wave, HIGH-1):** `S3CloudStore::commit()` now calls a new `list_recursive(prefix)` helper (ListObjectsV2 without `delimiter`) so every nested key under the branch — e.g. `write("traces/x.jsonl", ...)` — contributes to the manifest hash. The previous implementation called `list("")` which under the hood used `delimiter="/"` and hid all sub-directory writes from the commit, silently breaking hash-stability. `commit()` ALSO strips any existing `manifest-*.json` entries from the input so the hash is stable across repeated commits on unchanged data.
- **primitives/kei-store (v0.21.1 audit wave, HIGH-2):** `S3Cfg::access_key_env` + `S3Cfg::secret_key_env` are now wired through to the aws-sdk-s3 builder. When both are set, we resolve the named env vars into an explicit `Credentials` provider and overlay it on the SDK config. Partial configuration (only one of the two set) now returns an error rather than silently ignoring it. Previously both fields were dead — configured users were getting the ambient AWS default chain instead of the named pair.
- **primitives/kei-store (v0.21.1 audit wave, HIGH-5):** all tests that mutate process env on `KEI_STORE_*` vars now take a shared `test_env::ENV_LOCK` mutex (exposed under `cfg(any(test, feature = "s3"))`). Prevents cargo-test parallelism from racing multiple tests on the same env state. `github.rs` dedups onto the shared lock; `s3_cloud/tests.rs` + `tests/s3_smoke.rs` now use it.
- **primitives/keisei (v0.21.1 audit wave, HIGH-3):** `detach.rs` + `mount.rs` now scrub every manifest-sourced string (brain name, brain path, config path, client type, error reason) through `display::sanitize_display` before `println!` / `eprintln!`. `status.rs` + `attach.rs` were already compliant; this closes the L9 regression gap for the other two print sites. Two new integration tests (`detach_sanitizes_control_chars_in_marker_fields`, `mount_sanitizes_control_chars_in_error_reason`) assert source-level guard presence.
- **primitives/keisei (v0.21.1 audit wave, HIGH-4):** extracted `adapters/jsonmcp.rs` (~107 LOC) as the shared JSON merge/remove/persist helper used by the `claude-code`, `cursor`, and `zed` adapters. All three adapters drop from ~170 LOC to ~105 LOC each and share a uniform error-surfacing contract (`Error::ConfigParseError { path }` rather than raw serde_json on parse failure). `continue_adapter.rs` is YAML-based and is unaffected.
- **security (v0.21.1 audit wave, H1):** `scripts/install-actionlint.sh` now verifies SHA-256 of the downloaded tarball before extraction. Per (OS, ARCH) hashes are pinned at the top of the script and documented as the output of `checksums.txt` on the upstream release page. If a hash is marked `SKIP` (documented as `[UNVERIFIED]` pending live fetch), the installer prints a WARNING. Missing `shasum` / `sha256sum` is a hard exit 2 — refuses to install an unverified binary. Env override `ACTIONLINT_SHA256_OVERRIDE=<hex>` lets CI inject the hash at runtime.
- **security (v0.21.1 audit wave, H2):** `kei-store::s3_cloud::client::validate_endpoint` rejects loopback / link-local / metadata hosts (`127.0.0.0/8`, `::1`, `169.254.0.0/16`, `fe80::/10`, `metadata.google.internal`, etc.) and plain-HTTP URLs by default. Closes the SSRF / IMDS-leak surface where an attacker-controlled `KEI_STORE_S3_ENDPOINT` pointed at `http://169.254.169.254` would cause the AWS default credential chain to sign requests against the instance metadata endpoint and leak IMDS creds. Env overrides: `KEI_STORE_S3_ALLOW_INTERNAL=1` (local MinIO / tests), `KEI_STORE_S3_ALLOW_INSECURE=1` (plain-HTTP). When a custom endpoint is set, explicit `access_key_env` + `secret_key_env` are REQUIRED — the default credential chain is no longer consulted for non-AWS endpoints.
- **docs (v0.21.1 audit wave, D1):** `docs/USB-BRAIN-GUIDE.md` now warns that **exFAT / FAT32 are NOT safe for multi-client attach** — SQLite WAL shared-memory mmap doesn't work reliably on those filesystems. Recommends APFS / ext4 / NTFS for `keisei mount`. Troubleshooting entry "SQLite corruption on mount-attach" added with recovery steps.
- **docs (v0.21.1 audit wave, D2):** the "~5 MB release binary growth" claim for the `s3` feature is now labelled `[estimate, E5 — not yet measured]` in both CHANGELOG.md and the `s3_cloud` module doc-comment. Prevents over-claim until a real `cargo build --release` before/after comparison is landed.
- **scripts (v0.21.1 audit wave, D3):** `scripts/validate-workflow-shas.sh` now exits 2 when UNVERIFIED pins exist AND no `GITHUB_TOKEN` was provided (rate-limit path). Previously silently returned 0 which masked incomplete verification in CI.
- **primitives/keisei (v0.19 audit hardening):** close 3 Security HIGH + 3 Critic HIGH + 2 Critic MEDIUM findings. Path-escape guard on `mcp_server` + `memory/artifacts/manifests` (absolute / `..` / canonical-mismatch → `PathEscape`); brain-name regex `^[a-z][a-z0-9_-]{0,63}$` (`InvalidName`); symlink-rooted brain inputs rejected (`BrainIsSymlink` — closes USB → `$HOME` pivot); MCP-entry collision check across all 4 adapters (`NameConflict` instead of silent clobber); dropped unused `rusqlite` dep (no C toolchain tail); `BrainPaths.{memory,artifacts,manifests}` relaxed to `Option<String>`; `$KEISEI_HOME`/`$HOME` resolver deduped into `paths.rs` SSoT; `fsx::write_atomic` rewritten on `tempfile::NamedTempFile` for Windows + cross-fs correctness; 5 adversarial integration tests added (16 total pass).
- **primitives/keisei (v0.19.2 polish):** dropped unused `ClientAdapter` imports from `mount.rs` + `detach.rs`; `Error::NotAttached` and `AttachRecord::has_client` now carry explicit `#[allow(dead_code)]` markers documenting that they're reserved for future callers / test-only respectively. `cargo check -p keisei` is warning-clean; integration suite is 19/19 pass (3 new: `marker_file_has_0600_perms_on_unix`, `status_sanitizes_control_chars_in_brain_name`, `manifest_too_large_rejected`). `brain.rs` module-level doc-comment now lists the v0.19 invariants (path confinement / symlink reject / name regex / manifest size cap) and flags schema v2 as v0.20.

### Security
- Pinned all GitHub Actions (`ci.yml`, `release.yml`) by full commit SHA to defend against CVE-2025-30066-class supply-chain attacks via mutable tag re-pointing.
- Removed `|| bun install` fallback from `release.yml` build-mcp-binary job — lockfile is now strictly REQUIRED (H4 audit finding).
- Added `.github/dependabot.yml` for weekly SHA update PRs on github-actions, npm, and cargo ecosystems.
- **v0.20.1 — workflow validation defense-in-depth:** motivated by the 2026-04-22 incident where `dtolnay/rust-toolchain@3c5f7ea...` SHA-pinned a specific Rust version (1.94.1 branch tip) instead of "install current stable", breaking CI for 4 jobs. Added three gates against the incident class: `scripts/install-actionlint.sh` (pinned v1.7.12 installer, macOS-arm64 + linux-x64), `scripts/lint-workflows.sh` (actionlint runner, advisory if binary missing), `scripts/validate-workflow-shas.sh` (git-ls-remote every `uses: <repo>@<sha40>` pin; exits 1 on `SHA MISSING`, soft-continues on network errors with `[UNVERIFIED]`), `scripts/pre-commit-workflow-lint.sh` (symlink-to-install pre-commit hook, fires only when workflow files are staged), and new `workflow-lint` CI job running the two validators on every push + PR.

## [0.40.0] — 2026-04-24

Waves 17-43 — cortex stack, MCP server, terminal client, VSCode extension, agentic loop, atom registry, frustration-matrix, external-skill importer, full-integration plumbing.

This release adds a second deployment surface (HTTP daemon + Svelte UI + ratatui TUI + VSCode webview) on top of the existing plugin path. The substrate core (ledger, memory, fork, atoms) is unchanged; the stack composes on top.

### Added

- **primitives (Wave 20 — `kei-cortex` HTTP daemon):** `axum`-based local daemon on `127.0.0.1:9797`. Routes under `/api/v1/cortex/` (auth-gated by bearer token): `/summary`, `/pet/:user_id` (GET + POST `/interaction`, `/chat`, `/portrait/stylize`, `/tts`), `/stt`, `/ledger/recent`, `/memory/search`, `/usage`, `/fs/list`, `/term` (WS). `/healthz` mounted outside auth for monitors. `/chat` runs the Anthropic tool-use SSE loop; `/stt` shells out to `faster-whisper` (RULE 0.2 exception #6 — Python in subprocess, daemon core stays Rust); `/portrait/stylize` proxies fal.ai Flux; `/tts` streams ElevenLabs audio; `/term` carries PTY frames via `portable-pty` + `tokio-tungstenite`. Per-route concurrency caps wired in `src/routes.rs`. Bearer-token auth via `~/.keisei/cortex.token` (32 hex bytes, `chmod 600`). See `_primitives/_rust/kei-cortex/INSTALL.md`.
- **ts-packages (Wave 21+22 — `cortex-ui` Svelte 5 + Vite 5 frontend):** routes `Setup` / `Dashboard` / `PetEditor` / `LedgerStream` / `MemorySearch`. Components: chat panel with PTT voice input, `BudgetStrip` (today's API spend pulled from `/usage`), `FileTree`, `TerminalPane` (xterm.js wired to `WS /term`), Live2D pet renderer (`pixi-live2d-display` with idle animation + lip-sync to TTS stream). Auto-TTS toggle pipes `/chat` reply text through `/tts` and back into the Live2D mouth shape. Path: `_ts_packages/packages/cortex-ui/`.
- **primitives (Wave 24 — `/cortex-setup` skill):** seven-phase wizard for daemon + UI provisioning. Phase 0 preflight (OS detect, binary path), Phase 1 keys (verify ANTHROPIC / ELEVENLABS / FAL in `~/.claude/secrets/.env`), Phase 2 config (port / UI host / whisper model click-pickers), Phase 3 token generation, Phase 4 install (pip + whisper-pull + ui-build), Phase 5 supervisor (launchd plist on macOS, systemd user unit on Linux, manual fallback), Phase 6 done (composes setup URL, copies to clipboard). 5-9 AskUserQuestion calls. Pure-click contract except optional API-key paste / port / UI host. Idempotent. `skills/cortex-setup/SKILL.md` + 7 phase files.
- **primitives (Wave 25 — `frustration-matrix`):** longitudinal user-frustration scanner. Regex categories + byte n-gram likelihood classifier. Reads chatlogs / JSONL traces, emits CSV / JSONL frustration timelines. Path: `_primitives/_rust/frustration-matrix/`.
- **primitives (Wave 26.5 — `kei-skill-importer`):** universal parser/canonicalizer/emitter for external AI-coding-tool skills. Imports OpenClaw / Cline / Cursor / Claude Code / Kimi skill formats and emits canonical atoms / recipes / proposed primitives. Depends on `kei-atom-discovery`. Path: `_primitives/_rust/kei-skill-importer/`.
- **primitives (Wave 31 — `kei-tty` ratatui terminal client):** crossterm full-screen TUI for kei-cortex. Two subcommands — `chat` (interactive) and `send --message "…"` (one-shot pipe-friendly). Same daemon URL (`KEI_DAEMON_URL`, default `http://127.0.0.1:9797`), same bearer token. Path: `_primitives/_rust/kei-tty/`.
- **ts-packages (Wave 35 — `@keisei/vscode-cortex` extension):** activitybar entry "KeiSei Cortex", webview-backed chat view embedding the cortex-ui bundle. Keybinding `Cmd+Shift+K` (macOS) / `Ctrl+Shift+K` (other) for `keisei-cortex.openChat`. Right-click `keisei-cortex.chatAboutSelection` sends highlighted text to chat as context. VSCode engine `^1.93.0`, node `>=18`. Path: `_ts_packages/packages/vscode-cortex/`.
- **primitives (Wave 36 — `kei-mcp` Model Context Protocol server):** stdio JSON-RPC 2.0 server. Walks `_primitives/_rust/<crate>/atoms/<verb>.md` and exposes each atom as one MCP tool (`<crate>::<verb>` name, atom body as description, frontmatter `input_schema` as JSON-Schema). Walks `skills/<name>/SKILL.md` and exposes each skill as one MCP resource at `skill://<name>`. Methods: `initialize`, `tools/list`, `tools/call`, `resources/list`, `resources/read`, `prompts/list`, `prompts/get`. stdout = protocol frames only; diagnostics → stderr. Configs for Claude Code / Cline / OpenClaw in `_primitives/_rust/kei-mcp/README.md`. Env vars: `KEI_MCP_ATOMS_ROOT`, `KEI_MCP_SKILLS_ROOT`, `KEI_RUNTIME_BIN_DIR`. Path: `_primitives/_rust/kei-mcp/`.
- **primitives (Wave 38 — agentic loop wired into `kei-cortex`):** `/chat` endpoint runs the Anthropic tool-use loop with 8 built-in tools (`read`, `write`, `edit`, `bash`, `glob`, `grep`, `webfetch`, `agent`) — registered in `src/tool/registry.rs`. Tool results stream back through SSE; `WS /term` carries PTY frames for the cortex-ui terminal pane.
- **profiles (Wave 24 — `cortex` install profile):** new `MANIFEST.toml` profile resolving to 8 primitives — `kei-cortex`, `cortex-ui`, `kei-pet`, `kei-shared`, `kei-ledger`, `kei-memory`, `frustration-matrix`, `kei-skill-importer`. `./install.sh --profile=cortex` provisions the full cortex stack in one shot.
- **ts-packages (Waves 32-37 — external-API adapters):** five new adapter packages under `_ts_packages/packages/` — `gmail-adapter`, `grok-adapter`, `recall-adapter`, `telegram-adapter`, `youtube-adapter`. Each is a thin TS shim around the upstream API; daemon exposes them through `kei-router` request shapes.
- **docs (Wave 43 — `docs/QUICKSTART.md`):** five-minute install-to-browser walkthrough for the cortex stack. Prerequisites, `--profile=cortex`, `/cortex-setup` walkthrough, daemon healthz verification, optional `kei-tty` and `vscode-cortex` add-ons, optional `kei-mcp` registration. Path: `docs/QUICKSTART.md`.
- **docs (Wave 43 — `docs/ARCHITECTURE.md` layer model):** layer 0-4 ascii diagram (atoms / primitives / recipes / skills / frontends), atom→MCP-tool flow, skill→MCP-resource flow, frontend→daemon flow, memory architecture (3-layer + 3 SQLite files), sleep architecture (Phase A/B/C), substrate dogfood (kei-fork → kei-ledger → kei-spawn), git-model rule cross-refs.
- **docs (Wave 43 — README.md cortex stack section):** new section between deployment-modes and Why-Rust covering daemon endpoints, cortex-ui routes/components, kei-tty subcommands, vscode-cortex commands+keybindings, kei-mcp method list. Counts table refreshed (47→53 crates, 43→45 skills, 11 capabilities → 16, +TS packages, +profiles).

### Changed

- **README.md (Wave 43):** intro paragraph updated — Rust crate count 47→53, agent-tool list extended (Cursor / Continue / Zed / Aider / Windsurf / OpenClaw / Cline / Kimi), cortex stack mention added. Deployment-modes table extended with cortex profile + kei-tty + Rust MCP server rows. Why-Rust paragraph crate count refreshed; TypeScript paragraph rewritten to cover cortex-ui + vscode-cortex + 5 external-API adapters. Verify-block hardcoded test count replaced with placeholder + note that totals fluctuate per-feature. Patents/IP section added before author block.

### Security

- **Cortex daemon bearer-token auth (Wave 24):** every endpoint requires `Authorization: Bearer $(cat ~/.keisei/cortex.token)`. The token is generated by `/cortex-setup` Phase 3 (32 hex bytes from a CSPRNG), persisted at `~/.keisei/cortex.token`, and `chmod 0o600`'d. Daemon refuses requests without a matching token. Token rotation: delete the file and re-run `/cortex-setup` (Phase 3 only).
- **`/cortex-setup` Phase 1 (Wave 24, RULE 0.8):** API keys (`ANTHROPIC_API_KEY`, `ELEVENLABS_API_KEY`, `FAL_KEY`) are read from `~/.claude/secrets/.env` only. The wizard surfaces presence as `<present>` / `<missing>` / `<pasted>` and never echoes a key value to chat output. Optional paste field writes directly to `~/.claude/secrets/.env` and refuses to log the value.
- **`kei-mcp` stdio isolation (Wave 36):** stdout carries MCP protocol frames ONLY (one JSON object per line). Every diagnostic, warning, and error goes to stderr. Prevents the well-known "logger dirties the wire" failure mode that breaks JSON-RPC clients.

### Snapshot

- 53 Rust crates (`_primitives/_rust/Cargo.toml` workspace members)
- 45 skills (`skills/*/SKILL.md`)
- 12 hooks (`hooks/*.sh`)
- 82 behavioural blocks (`_blocks/*.md`)
- 16 capability fragments (`_capabilities/*/*/text.md`)
- 7 roles (`_roles/*.toml`)
- 11 cross-tool bridges (`_bridges/*.tmpl`)
- 7 TS packages (`_ts_packages/packages/`)
- 8 install profiles (minimal / core / frontend / ops / dev / mcp / cortex / full)

## [0.33.0] — 2026-04-23

Wave 16 — parallel consumer wiring. Ledger v6 + cluster-aware prune + DNA-driven fork precedent + three-role pipeline. Consolidates v0.28 → v0.33.

### Added
- **primitives:** `kei-fork` watch-hook auto-collects on `.DONE` marker
- **primitives:** `kei-prune` cluster-based retirement via `kei-dna-index` clusters
- **primitives:** `kei-brain-view` cluster + summary visualization
- **pipeline:** three-role pipeline (Writer → Auditor → Merger) with precedent pre-check

### Changed
- **kei-ledger:** schema v6 — 3 performance indexes + `fork_transactional` library API

### Snapshot
- 47 crates workspace
- 800+ tests green total

## [0.32.0] — 2026-04-23

Wave 15 Option D — DNA adjacency + managed fork primitive.

### Added
- **primitives:** `kei-dna-index` — read-only adjacency / cluster / precedent view over the ledger
- **primitives:** `kei-fork` — managed git-worktree + ledger lifecycle

### Changed
- **kei-fork:** root path moved to `_forks/` (sandbox-writable, kit convention)

## [0.31.0] — 2026-04-23

Wave 15 foundation — spawn hardening + entity-store medium fixes.

### Added
- **kei-spawn:** `HttpDriver` feature-flag behind `http-driver`

### Fixed
- **security:** `agent_id` path-traversal validator + `safe_join` hardening
- **kei-entity-store:** medium audit fixes (ddl panic, search empty-token, WAL logging)

## [0.30.0] — 2026-04-23

Wave 14 — bio-inspired primitives.

### Added
- **primitives:** `kei-prune` — bio-inspired retirement of idle agents
- **primitives:** `kei-discover` — federated marketplace discovery stub
- **primitives:** `kei-brain-view` — brain-state visualizer
- **primitives:** `kei-hibernate` — agent hibernation / reawaken lifecycle
- **primitives:** `kei-ledger-sign` — signing + verification for ledger rows

### Snapshot
- 44 crates workspace
- 713 tests green

## [0.29.0] — 2026-04-22

Wave 13 — structural diff, scheduler, watcher + HIGH audit fixes.

### Added
- **primitives:** `kei-diff` — RFC 6902 JSON Patch subset (add / remove / replace)
- **primitives:** `kei-scheduler` — cron / at / interval metadata primitive
- **primitives:** `kei-watch` — filesystem watcher (thin `notify` wrapper, sync API)

### Fixed
- **fts:** delete-transaction + archive FTS desync fixed
- **kei-dna-index:** `UNIQUE` constraint (v5 migration)

## [0.28.0] — 2026-04-23

Wave 12 — count refresh + content-store engine promotion.

### Changed
- **kei-content-store:** `CAMPAIGNS_SCHEMA` promoted to engine
- **docs:** counts refreshed across README / INSTALL / REFERENCE after v0.23 → v0.27 cluster

## [0.15.0] — 2026-04-22

### Added
- **primitives:** `kei-artifact` typed handoff pipeline (BMAD-style doc passthrough) (`3f303b7`)
- **blocks:** 5 cognitive mode blocks + 2 manifest wirings (`fdfc690`)

## [0.14.2] — 2026-04-22

### Added
- **hooks:** runtime controls via `KEI_DISABLED_HOOKS` + `KEI_HOOK_PROFILE` (v0.14.2) (`1a448e8`)

### Removed
- genesis-scan from public kit (internal tool, Bundle-only) (`268226b`)

## [0.14.1] — 2026-04-22

### Added
- **ci:** GitHub Actions workflows + `.claude/worktrees` gitignore (`407e8b7`)

### Changed
- **readme + install:** reconcile all count drift (F4 RELEASE BLOCKER) (`0199fd4`)
- **rust:** misc schema/main refactor in 8 crates (assorted CP splits) (`61448b9`)
- **mock-render:** split `main.rs` 227 LOC into 4 cubes (F5a Constructor Pattern) (`ad5977d`)

### Fixed
- **kei-auth:** remove `--key` CLI flag (F12 HIGH — `/proc/cmdline` leak) (`b449587`)
- **kei-refactor-engine:** retract 'git apply-ready' claim (F1 RELEASE BLOCKER) (`f50ef43`)
- **kei-store:** path-traversal guard (F2 RELEASE BLOCKER) + S3 stub gate (F7) + GitHub RULE 0.1 guard (F8) (`ad9c53f`)

## [0.14.0] — 2026-04-22

### Added
- **primitives:** 10 Rust crates extracted from LBM (Genesis-scrubbed) (`a5e6649`)
- **ts-packages:** 6 TS packages — MCP server + 5 external-API adapters (`7b647d5`)

### Changed
- **rust-core:** Constructor-Pattern splits in `kei-router` + `kei-auth` (`afed921`)

## [0.13.0] — 2026-04-22

### Added
- **integration:** deep-sleep wired into MANIFEST + sleep-setup Phase 3b + README (`bcd80f6`)
- **primitives:** 4 Rust crates for deep-sleep — `conflict-scan`, `refactor-engine`, `graph-check`, `store` (`0f75493`)
- **skills:** `/onboard` auto-project-analyze with 3-mode apply (full-auto / step-by-step / full-manual) (`1396139`)

### Changed
- **readme:** "Why Rust, not Python" paragraph in author note (`92c918a`)
- **readme:** clarify "my sample, not claim of originality" in author note (`47d2448`)
- **readme:** add "double sorry" disclaimer in author note (`3d5d768`)
- **readme:** move "From the author" to opening, expand with transformer-error context (`fd67315`)
- **readme:** add "From the author" note (`b103c3d`)

## [0.12.0] — 2026-04-22

### Added
- **integration:** Phase A incubation wired into trigger + install + README (`d72de64`)
- **skills:** `/sleep-on-it` 6-phase wizard + `kei-sleep-queue` CRUD + incubation prompt (`30df6cb`)

## [0.11.0] — 2026-04-22

### Added
- **integration:** `--with-sleep-sync` flag + README Cloud REM sync section (`1dd05c6`)
- **skills:** `/sleep-setup` 5-phase wizard (click + 1 free-text URL) (`b658f81`)
- **hooks:** `session-end-dump` calls `kei-sleep-sync` after ingest (`1ab39d5`)
- **primitives:** `kei-sleep-setup` wizard + `kei-sleep-sync` helper + trigger template (`4fdaab6`)

## [0.10.0] — 2026-04-22

### Added
- **integration:** register `genesis-scan` in MANIFEST core+full + README + `install.sh` sizing (`93ba0a0`)
- **hooks:** `git-pre-commit-genesis` — template for repo symlink into `.git/hooks/pre-commit` (`670af3f`)
- **primitives:** `genesis-scan` Rust — patent-IP leak detector (CI / pre-commit) (`5db8548`)
- **integration:** wire `kei-memory` into MANIFEST + settings-snippet + README for v0.10 (`0b5da5a`)
- **skills:** `/self-audit` 5-phase triage pipeline (`334a867`)
- **hooks:** 3 self-audit triggers — stop / milestone / error-spike (`a5c3896`)
- **primitives:** `kei-memory` Rust crate — offline session analyzer (Genesis-clean) (`448fc07`)

## [0.9.1] — 2026-04-21

### Added
- **install:** interactive menu (whiptail / dialog / plain) + confirm screen + `--yes` / `--no-execute` (`4809269`)

## [0.9.0] — 2026-04-21

### Added
- **install:** modular profiles + `--add` / `--remove` / `--list` incremental install (`b1b8de0`)
- **primitives:** `MANIFEST.toml` — SSoT for 21 primitives + 6 profiles (`764a999`)

### Changed
- **readme:** install profiles table + migration note for v0.9.0 (`47931a3`)

> BREAKING: default install profile is now `minimal` (was `full`).
> Re-run with `--profile=full` to preserve prior behaviour.

## [0.8.0] — 2026-04-21

### Added
- **install:** copy `_primitives/` + build Rust workspace; register `agent-fork-logger` + `site-wysiwyd` hooks (`b0d9389`)
- **hooks:** `site-wysiwyd-check` PostToolUse(Edit | Write) drift advisory (`c2041b4`)
- **skills:** `/site-create` pipeline (phases 0–4 — phases 5–6 deferred) (`839ae57`)

### Changed
- **compose-solution:** prior-art grep paths + phase-5 cross-refs for 10 pipelines + 21 primitives (`f664cbc`)
- **readme:** v0.8.0 — 73 blocks / 34 skills / 21 primitives / 6 hooks / 11 bridges + pipelines section (`ed7d566`)

[Unreleased]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.40.0...HEAD
[0.40.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.33.0...v0.40.0
[0.33.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.32.0...v0.33.0
[0.15.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.14.2...v0.15.0
[0.14.2]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.14.1...v0.14.2
[0.14.1]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/KeiSei84/KeiSeiKit/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/KeiSei84/KeiSeiKit/releases/tag/v0.8.0
