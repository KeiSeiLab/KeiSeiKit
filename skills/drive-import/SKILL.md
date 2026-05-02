---
name: drive-import
description: Import one or more Google Drive folders as private repos into the local Forgejo dev-hub. Wraps the kei-drive-import bash wizard. Click-only path-picker, optional --scan auto-discover. Requires rclone OAuth (one-time) + Wave 45 dev-hub Forgejo running.
argument-hint: (optional) "drive:path/to/folder"
---

# /drive-import — Google Drive → Forgejo project import

## When to use

- Importing one or more Google Drive folders as private repos into a local Forgejo dev-hub.
- Auto-discovering and classifying Drive project folders via `--scan` before import.
- Migrating Drive-based project archives to version-controlled Forgejo repos.

Bridge to the kei-drive-import wizard. Two modes:

| Mode | When | How |
|---|---|---|
| **explicit** (default) | user knows the folder paths | one or more `drive:path/...` strings |
| **scan** | user wants auto-discover | `--scan drive:Projects/` walks + classifies + asks |

## Pre-flight (run once, idempotent)

1. `./install.sh --profile=full-hub --yes` — installs Forgejo + rclone + gitleaks, auto-creates admin user `$USER` with random password (Keychain `forgejo-admin-password`) and access token (Keychain `forgejo-api-token`), stamps `~/.claude/secrets/.env` with `KEI_FORGEJO_USER`, `KEI_FORGEJO_URL`, `KEI_DRIVE_REMOTE`, `RCLONE_CONFIG`. Implementation: [`install/lib-dev-hub-forgejo.sh::_dhf_bootstrap_admin_user`](../install/lib-dev-hub-forgejo.sh).
2. `rclone --config ~/.config/rclone/rclone.conf config` — interactive OAuth, scope `drive.readonly`, remote name `gdrive`. One-time browser click on "Allow".

## Pipeline — what happens per folder

```
classify (kei-gdrive-import classify --remote) → MUST not be ALREADY-REPO
   ↓
rclone copy (frozen flag block: --drive-skip-gdocs --drive-skip-shortcuts +
             7 OS-junk excludes + --tpslimit 10 --checksum) → /tmp/staging/
   ↓
size + extension histogram (>50% .pdf or >30% media → user prompt)
   ↓
gitleaks dir --no-banner --redact → exit non-zero = BLOCKED-SECRETS
   ↓
language-aware .gitignore (SHA-pinned to github/gitignore@5763345…)
   ↓
git init -b main + commit "Import from Drive: <name>"
   ↓
POST /api/v1/user/repos {auto_init:false, private:true} (Basic auth, token from Keychain)
   ↓
git remote add origin "${FORGEJO_URL}/${FORGEJO_USER}/<name>.git"
   ↓
allowlist guard: case "$origin_url" in "${FORGEJO_URL}/"*) — derived from env
   ↓
git -c http.extraHeader="Authorization: Basic <b64>" push -u origin main
   ↓
ledger CSV row: timestamp,name,status,forgejo_url,staging_path
```

Status enum: `OK / SKIPPED-ALREADY-REPO / BLOCKED-SECRETS / BLOCKED-FORGEJO / FAILED-RCLONE / FAILED-PUSH`.

## Invocation

When user says "import these folders from drive" / "пилотную папку из Google Drive в форджео":

1. Ask user for one or more `drive:path` strings (use AskUserQuestion if more than one).
2. Run: `kei-drive-import "drive:path1" "drive:path2"`
3. Report: how many OK, how many SKIPPED, how many BLOCKED, ledger path.

For batch import, write a paths file:
```
drive:projects/A
drive:Work/clientB
# comments OK, blank lines OK
drive:foo/bar
```
Then: `kei-drive-import --paths-from list.txt`.

## Failure modes — diagnose

| Symptom | Cause | Fix |
|---|---|---|
| `RCLONE_CONFIG not set` | first install never ran preflight | `cat ~/.claude/secrets/.env.example | grep RCLONE_CONFIG` → copy to live `.env` |
| `remote 'gdrive:' missing` | OAuth not done | `rclone config` (one-time browser click) |
| `Forgejo not reachable` | dev-hub daemon down | `launchctl load ~/Library/LaunchAgents/com.keisei.forgejo.plist` |
| `BLOCKED-SECRETS` | gitleaks found AKIA / GitHub PAT / PEM key | edit fixture or `git rm` the leak in staging, then re-run |
| `REJECTED: remote not allowlisted` | `KEI_FORGEJO_URL` mismatch between env and actual URL | sync `.env` |

## Don't

- Don't try `--scan` on a 10k-folder root — preflight enumerates gdocs first which is slow
- Don't pass paths with trailing slash — wizard normalises but use `drive:foo` not `drive:foo/`
- Don't manually push to github after import — `tools/sync-public.sh` is the public-mirror channel

## See also

- Wizard source: `_templates/drive-import-wizard.sh.tmpl`
- Rust scorer: `_primitives/_rust/kei-gdrive-import/src/`
- Install lib: `install/lib-dev-hub-gdrive-import.sh`
- Integration tests: `tests/gdrive_import_integration.sh` (7/8 PASS, 1 graceful skip)
- Plan + audit history: `tasks/kei-gdrive-import/PLAN.md`
