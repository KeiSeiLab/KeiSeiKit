---
name: sleep-setup
description: One-time wizard (5) that configures KeiSeiKit sleep layer. Phase 0 picks mode (local-only / remote-only / hybrid); Phase 0b picks local trigger time (6 options incl. Custom HH:MM). Remote/hybrid generate an SSH deploy key, init the memory-repo, write env refs (RULE 0.8), and emit a `/schedule create` command. Local-only / hybrid emit a CronCreate snippet. Pure-click except 2 free-text fields (repo URL in Phase 2, Custom time in Phase 0b).
argument-hint: (no arguments)
---

# Sleep Setup — Cloud REM Sync Wizard (index)

You are running the one-time configuration wizard for the KeiSeiKit v0.11
sleep layer. Each session-end dump pushes to a private git repo; a cloud
Claude Code agent on `/schedule` clones the repo nightly, analyzes traces,
and commits a consolidation report back. In the morning the user runs
`git pull` and reads the report. Nothing in this pipeline blocks session
close, and report content is for the user to READ — never auto-injected.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in order. Never skip a phase. Never re-order phases.

---

## Pipeline overview (8 phases, 11+ AskUserQuestion since v0.14.0)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 0 | [phase-0-mode.md](phase-0-mode.md) | Pick sleep mode: local-only / remote-only / hybrid | 1 (click-only) |
| 0b | [phase-0b-time.md](phase-0b-time.md) | Pick local trigger time (6 options; Custom adds 1 free-text) | 1-2 (click; +freeText if Custom) |
| 1 | [phase-1-repo-pick.md](phase-1-repo-pick.md) | Pick repo provider + visibility (skipped if local-only) | 2 (click-only) |
| 2 | [phase-2-repo-url.md](phase-2-repo-url.md) | Collect SSH URL (skipped if local-only) | 1 (AskUserQuestion `freeText`) |
| 3 | [phase-3-deploy-key.md](phase-3-deploy-key.md) | Run `kei-sleep-setup.sh`, confirm deploy-key added (skipped if local-only) | 1 (click) |
| 3b | [phase-3b-deep-sleep.md](phase-3b-deep-sleep.md) | v0.13.0 — deep-sleep cadence + fork mode + store backend | 3 (click; +1 free-text if Custom cadence) |
| 4 | [phase-4-test-push.md](phase-4-test-push.md) | Dry-run a test commit (skipped if local-only) | 1 (click) |
| 5 | [phase-5-trigger.md](phase-5-trigger.md) | Render CronCreate and/or `/schedule create` per mode | 1-2 (click; hybrid asks twice) |

**Minimum AskUserQuestion count: 11** (remote-only / hybrid full pipeline).
**Local-only mode: 6 minimum** (Phases 0, 0b, 3b, 5; Phases 1-4 skipped).
All clicks except the repo-URL free-text in Phase 2 (skipped in local-only),
the Custom-time free-text in Phase 0b (optional), and the Custom-cadence /
S3 free-text fields in Phase 3b (optional).

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `SLEEP_MODE` | Phase 0 | `local-only` / `remote-only` / `hybrid` |
| `SLEEP_TIME_LOCAL` | Phase 0b | `HH:MM` 24h format (e.g. `03:00`); user's local time |
| `PROVIDER` | Phase 1 (remote/hybrid only) | `github` / `gitlab` / `bitbucket` / `self-hosted` |
| `VISIBILITY` | Phase 1 (remote/hybrid only) | `private` (recommended) / `public` (explicit user choice) |
| `REPO_URL` | Phase 2 (remote/hybrid only) | Validated SSH URL (`git@host:org/repo.git`) |
| `KEY_ADDED` | Phase 3 (remote/hybrid only) | boolean; was deploy key confirmed added? |
| `DEEP_SLEEP_CRON_DAYS` | Phase 3b | integer ≥0; 0 disables Phase C; default 7 |
| `DEEP_SLEEP_WITH_FORK` | Phase 3b | 0 (plan only) / 1 (plan + fork branch) / 2 (plan + local-patch; local-only mode) |
| `STORE_BACKEND` | Phase 3b | `github` / `forgejo` / `gitea` / `filesystem` / `s3` |
| `TEST_VERIFIED` | Phase 4 (remote/hybrid only) | boolean; did the user see the test commit in the remote? |
| `SLEEP_CRON_UTC` | Phase 5 (remote/hybrid only) | `m h * * *` cron expression derived from `SLEEP_TIME_LOCAL` + local TZ |
| `SCHEDULE_ACTION` | Phase 5 | mode-dependent; see phase-5-trigger.md §5d |

---

## Final report (emit after Phase 5)

Remote-only / hybrid (full pipeline):
```
=== SLEEP-SETUP REPORT ===
Mode:           <SLEEP_MODE>
Time (local):   <SLEEP_TIME_LOCAL>
Provider:       <PROVIDER> (visibility: <VISIBILITY>)
Repo URL:       <REPO_URL>
Deploy key:     ~/.ssh/keisei-memory-sync(.pub)
Sync repo path: ~/.claude/memory/sync-repo/
Env refs:       ~/.claude/secrets/.env  (KEI_MEMORY_REPO_URL, _PATH, _SSH_KEY)
Test push:      <PASS/FAIL> (Phase 4)
Schedule:       <SCHEDULE_ACTION>
```

Local-only (Phases 1-4 skipped):
```
=== SLEEP-SETUP REPORT ===
Mode:           local-only
Time (local):   <SLEEP_TIME_LOCAL>
Provider:       (skipped)
Repo URL:       (skipped)
Deploy key:     (not generated — local-only needs no git)
Sync repo path: (skipped)
Env refs:       ~/.claude/secrets/.env  (no sleep-sync keys written)
Test push:      (skipped)
Schedule:       <SCHEDULE_ACTION>     # local-cron-created / -copy-later / -skipped
```

If `SCHEDULE_ACTION` contains `skipped` (remote) or `local-cron-skipped`,
add:
```
No nightly consolidation. Traces still land locally in
~/.claude/memory/traces/ (and push to the repo on every session end if
mode != local-only). Re-run /sleep-setup any time to register a trigger.
```

---

## Rules (apply throughout — enforced at every phase)

- **Pure-click contract.** Only Phase 2 (repo URL) and Phase 0b (Custom
  time) ask for free text; every other decision is an `AskUserQuestion`.
  No `freeText` outside those two points (plus optional Custom cadence /
  S3 fields in Phase 3b).
- **Local-only skips Phases 1-4 entirely** — no git operations, no SSH
  key, no repo URL collection, no deploy-key walkthrough, no test push.
  The skill jumps phase-0 → phase-0b → phase-3b → phase-5 and never
  touches git.
- **Idempotent.** Re-running the wizard must NOT clobber existing
  `~/.ssh/keisei-memory-sync` or `~/.claude/memory/sync-repo/`; the
  helper script handles re-use.
- **NO DOWNGRADE (RULE -1).** If SSH auth fails, return 2-3 constructive
  paths (re-check deploy key, check `sshd` host, fall back to HTTPS with
  PAT — with warning) — never "cannot set up".
- **NO HALLUCINATION (RULE 0.4).** Never fabricate repo URLs, key
  fingerprints, or commit hashes. Show the real output of the script.
- **RULE 0.8 secrets.** The wizard writes env-var REFERENCES to
  `~/.claude/secrets/.env`, never inline tokens in any generated file.
- ** private remotes.** Recommend private visibility in Phase 1.
  If the user explicitly picks `public`, warn once: "a public memory
  repo leaks your session prompts and tool usage — confirm?".
- **Silent failure (5).** Nothing in the session-end path may
  block the session from closing. The wizard itself may fail loudly.
- **Constructor Pattern (RULE ZERO).** Every phase file < 200 LOC
  (RULE ZERO hard limit). New lightweight phases (0, 0b) target < 100
  LOC; phase-3b and phase-5 are heavier branch-heavy router files and
  sit between 100-200.

---

## References

- `~/.claude/rules/sleep-layer.md` — 5 full text
- `~/.claude/rules/secrets-single-source.md` — RULE 0.8 enforcement
- `_primitives/kei-sleep-setup.sh` — the imperative setup helper
- `_primitives/kei-sleep-sync.sh` — the session-end-dump callback
- `_primitives/templates/sleep-trigger-prompt.md` — cloud agent prompt
- `hooks/session-end-dump.sh` — where `kei-sleep-sync.sh` is invoked
