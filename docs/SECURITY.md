# Security model

What the kit touches, what it never touches, and the mitigations baked in.

---

## Threat surface overview

| Risk | Where it lives | Mitigation |
|---|---|---|
| Memory-repo leaks session content | Sleep-sync pushes trace JSONL off-machine | Private repo enforced by wizard; `[PATENT-IP]` sessions skip push entirely |
| Hardcoded tokens in source | Edits by agents / humans | `secrets-guard` Rust hook (PreToolUse Edit\|Write) blocks known token shapes |
| GitHub push of patent-sensitive content | `git push` command | `no-github-push.sh` hook + `genesis-leak-guard.sh` pre-commit symlink |
| Malicious GitHub Action tag re-point | `.github/workflows/*.yml` | SHA-pinning + `validate-workflow-shas.sh` + `actionlint` in CI |
| S3 SSRF / IMDS credential exfil | `kei-store` with custom endpoint | `validate_endpoint` rejects loopback / link-local / metadata hosts |
| Escape-sequence injection via brain name | `keisei status` / `attach` output | Control-byte sanitiser on every manifest-sourced string |
| Brain → `$HOME` pivot via symlink | `keisei attach <USB>` | Brain root rejected if symlink; `mcp_server` path must be relative + inside brain |
| SQLite WAL corruption on USB mount | `keisei mount <exFAT drive>` | Runtime advisory; exFAT/FAT32 warning in USB guide |

## Key mitigations in detail

### Memory-repo must be private

Sleep-sync pushes your session traces (prompts, tool calls, file paths, code snippets) to a git repo you control. `/sleep-setup` Phase 1 warns loudly on PUBLIC visibility. A public memory-repo leaks everything your agents have seen.

If the session is marked `[PATENT-IP]` in the prompt or runs in a banned-project CWD, `session-end-dump.sh` skips the push entirely — local trace is kept, never leaves the machine.

### No GitHub push for kit-internal state

RULE 0.1 forbids `git push` to github.com for any repo containing unfiled-patent IP. Kit ships `genesis-leak-guard.sh` as a pre-commit hook symlink template to keep patent-sensitive terms off any remote.

Override for legitimate public push: set env `GENESIS_LEAK_BYPASS=1` for the single commit + document the bypass reason in the commit body. The hook logs every bypass to `~/.claude/memory/genesis-bypass-log.md`.

### Secrets by reference only

`secrets-guard` Rust hook blocks hardcoded tokens at `PreToolUse(Edit|Write)`. Every SSH key, API key, deploy token lives in `~/.claude/secrets/.env` (chmod 600, gitignored) or per-project `secrets/*.env`.

Hook detects these token shapes:

| Pattern | Source |
|---|---|
| `sk-[A-Za-z0-9]{20+}` | OpenAI/Anthropic legacy |
| `sk-ant-[A-Za-z0-9_-]{40+}` | Anthropic current |
| `ghp_[A-Za-z0-9]{36}` | GitHub classic PAT |
| `github_pat_[A-Za-z0-9_]{82}` | GitHub fine-grained |
| `xoxb-[0-9]+-[0-9]+-[A-Za-z0-9]+` | Slack bot |
| `[0-9]{8,10}:[A-Za-z0-9_-]{35}` | Telegram bot |
| `AKIA[A-Z0-9]{16}` | AWS access key |
| `-----BEGIN (RSA \|EC \|OPENSSH )?PRIVATE KEY-----` | PEM private keys |
| `Bearer [A-Za-z0-9._-]{20+}` | generic bearer |

Allowlist (no false-positives): env references (`$VAR`, `os.environ[...]`, `std::env::var(...)`), placeholders (`YOUR_TOKEN_HERE`, `<redacted>`), safe paths (`*/secrets/**`, `*.env.example`).

Bypass for emergency: set env `SECRETS_GUARD_BYPASS=1` on the single call.

### Supply-chain defences

All GitHub Actions in `.github/workflows/` are pinned by full commit SHA (defends against CVE-2025-30066-class mutable-tag attacks).

- `scripts/validate-workflow-shas.sh` verifies every pin exists upstream via `git ls-remote`
- `scripts/install-actionlint.sh` checks SHA-256 of the downloaded tarball before extraction
- `scripts/lint-workflows.sh` runs `actionlint` over every workflow file
- CI job `workflow-lint` runs all three on every push + PR (< 30 s)
- `dependabot.yml` raises weekly PRs for SHA updates across github-actions, npm, and cargo ecosystems

### S3 / R2 / MinIO hardening

`kei-store::s3_cloud::validate_endpoint` rejects loopback, link-local, and cloud-metadata hosts by default to close the SSRF / IMDS-credential-leak surface:

- `127.0.0.0/8`, `::1` (loopback)
- `169.254.0.0/16`, `fe80::/10` (link-local)
- `metadata.google.internal`, `metadata.aws.internal` (cloud metadata)

Plain HTTP requires opt-in via `KEI_STORE_S3_ALLOW_INSECURE=1`. When a custom (non-AWS) endpoint is set, explicit `access_key_env` + `secret_key_env` are REQUIRED — the AWS default credential chain is not consulted for non-AWS endpoints (closes the "IMDS credentials leaked to unrelated endpoint" path).

### Brain attach-marker is owner-only

`~/.keisei/attached.toml` is `chmod 0o600` on unix (Windows unchanged — no equivalent bit). Every manifest-sourced string printed by `keisei status` / `attach` / `mount` / `detach` is scrubbed through `display::sanitize_display`, which replaces every ASCII control byte (`< 0x20` or `== 0x7F`) with `?`. Closes the escape-sequence-injection surface from a malicious `brain.name` like `"evil\x1b[2Jpayload"` that would otherwise clear the user's terminal or rewrite already-printed lines.

`manifest.toml` is capped at 64 KiB — `fs::metadata` check runs before `read_to_string` so an attacker-supplied 1 GB file can't exhaust memory inside the TOML parser.

### Brain path & name validation

- Brain `mcp_server` path MUST be relative + inside the brain root (rejects `/usr/bin/curl`, `../../etc/shadow`, Windows-style `..\..\`)
- Brain `name` matches `^[a-z][a-z0-9_-]{0,63}$`
- Brain root rejected if it's a symlink (blocks USB → `$HOME` pivot)
- Adapters refuse to clobber existing `mcpServers.<name>` entries — explicit `NameConflict` error, no silent overwrite
- All config writes go through `fsx::write_atomic_json` (Windows-safe via `tempfile::NamedTempFile::persist`)

### exFAT / FAT32 warning

SQLite WAL shared-memory mmap is unreliable on those filesystems; `keisei mount` (multi-client) WILL corrupt `kei-memory` / `kei-artifact` / `kei-social-store` DBs. Brain load prints an advisory when exFAT/FAT32 is detected via `statfs(2)`. Single-client `keisei attach` on exFAT stays supported.

See [USB-BRAIN-GUIDE-macos.md](./USB-BRAIN-GUIDE-macos.md) / [-linux.md](./USB-BRAIN-GUIDE-linux.md) / [-windows.md](./USB-BRAIN-GUIDE-windows.md) for APFS / ext4 / NTFS-native walkthroughs.

## Battle-test matrix

Install-test battle matrix runs every profile against three base images before each release (`tests/battle/`):

| Image | Libc | Known quirks |
|---|---|---|
| `ubuntu:24.04` | glibc | baseline; most widely deployed |
| `alpine:3.19` | musl | exposes musl-static-link issues in `rusqlite`, `git2`, `aws-sdk-s3` |
| `debian:12` bookworm | glibc | different apt structure from Ubuntu |

Assertions per run: blocks ≥ 82, skills ≥ 43, top hooks ≥ 12, `_lib` hooks ≥ 2; `hooks/_lib/test-gate.sh` runs; `settings.json` validates. "Does it work on a fresh machine?" signal before every version ships.

See `tests/battle/README.md` for running locally.

## Rule references

For the underlying discipline: these mitigations are driven by rules in the user's Claude Code CLAUDE.md. The relevant ones:

- **RULE 0.1** — NO GITHUB PUSH (unless patent-IP review clears)
- **RULE 0.4** — NO HALLUCINATION / CITATION VERIFY
- **RULE 0.8** — SECRETS SINGLE SOURCE
- **RULE 0.10** — RECURRENCE ESCALATE (same mistake ≥2× → codify via `/escalate-recurrence`)
- **RULE 0.13** — ORCHESTRATOR BRANCH FIRST (agents write files; orchestrator owns git)
- **RULE 0.14** — SESSION SELF-AUDIT
- **RULE 0.15** — SLEEP LAYER (three-phase nightly consolidation)

## Secret hygiene

This repository is public. The `.gitignore` actively blocks commits of:

- `.env` / `.env.*` (except `.env.example`, `.env.template`)
- `secrets/`, `**/secrets/`
- Key files: `*.pem`, `*.key`, `id_rsa*`, `id_ed25519*`

If you accidentally stage a secret:
1. **Do not push.** Drop it from the working tree immediately.
2. **Revoke** the leaked credential at its provider dashboard.
3. **Rotate** any adjacent credentials that may share the leak context.
4. If already pushed to remote: rewrite history (`git filter-repo`) and force-push is NOT safe on a widely-cloned repo; prefer revoke + rotate + new-commit-atop.

The canonical secret store for Claude Code is `~/.claude/secrets/.env`
(chmod 600, RULE 0.8 in your personal umbrella). Project-specific
tokens live at `<repo>/secrets/<name>.env` — both are `.gitignore`'d.
