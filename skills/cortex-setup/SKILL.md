---
name: cortex-setup
description: One-time wizard that provisions the kei-cortex daemon + cortex-ui browser app. Generates ~/.keisei/cortex.token, installs faster-whisper via pip, pre-downloads whisper base.en, builds the Svelte UI bundle, registers a launchd plist (macOS) or systemd user unit (Linux), and prints the click-to-open setup URL. Pure-click except optional custom port / custom whisper model. Idempotent — safe to re-run.
argument-hint: (no arguments)
---

# Cortex Setup — Local Daemon + Browser UI Wizard (index)

You are running the one-time configuration wizard for the KeiSeiKit v0.24
cortex stack. The daemon `kei-cortex` listens on 127.0.0.1:9797 and backs
the Svelte browser app `cortex-ui` (chat / TTS / STT / portrait panels).
This wizard wires the token, Python whisper deps, the pre-downloaded
Whisper model, the built UI bundle, and a supervisor unit so both pieces
come up at login.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in strict order. Never skip a phase. Never re-order phases.

---

## Pipeline overview (7 phases, 5-9 AskUserQuestion)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 0 | [phase-0-preflight.md](phase-0-preflight.md) | Detect OS, check primitive installed, show summary | 0 (auto) |
| 1 | [phase-1-keys.md](phase-1-keys.md) | Verify umbrella env has ANTHROPIC / ELEVENLABS / FAL | 0-3 (click per missing) |
| 2 | [phase-2-config.md](phase-2-config.md) | Pick port, UI host, whisper model | 1 batch (3 questions) |
| 3 | [phase-3-token.md](phase-3-token.md) | Generate token, write to ~/.keisei/cortex.token (chmod 600) | 0 |
| 4 | [phase-4-install.md](phase-4-install.md) | pip install + whisper-pull + ui-build (3 progress stages) | 1 click (confirm proceed) |
| 5 | [phase-5-supervisor.md](phase-5-supervisor.md) | launchd-install OR systemd-install based on OS | 1 click |
| 6 | [phase-6-done.md](phase-6-done.md) | Show setup URL, copy to clipboard, offer "open in browser" | 1 click |

**Minimum AskUserQuestion count: 5** (all keys present, default port / UI / model).
**Maximum AskUserQuestion count: 9** (3 missing keys + 3 custom config fields + 3 confirm clicks).
All clicks except the three optional free-text fields in Phase 1 ("paste now"
for each missing API key), the optional custom port / UI host in Phase 2,
and the optional retry stderr in Phase 4.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `OS`          | Phase 0 | `macos` / `linux` |
| `BIN_PATH`    | Phase 0 | absolute path to `kei-cortex` release binary |
| `SETUP_PATH`  | Phase 0 | absolute path to `kei-cortex-setup.sh` |
| `KEY_STATE`   | Phase 1 | 3-field object: `{anthropic, elevenlabs, fal}` ∈ `<present>` / `<missing>` / `<pasted>` |
| `CORTEX_PORT` | Phase 2 | integer, default `9797`, validated 1024-65535 |
| `UI_HOST`     | Phase 2 | `host:port`, default `127.0.0.1:18080` |
| `MODEL`       | Phase 2 | `base.en` / `small.en` / `medium.en` |
| `TOKEN_PATH`  | Phase 3 | fixed `~/.keisei/cortex.token` (32 hex bytes, chmod 600) |
| `UI_DIST`     | Phase 4 | abs path returned by `ui-build` (`~/.keisei/cortex-ui-dist/`) |
| `WHISPER_CACHE` | Phase 4 | abs path returned by `whisper-pull` |
| `SUPERVISOR`  | Phase 5 | `launchd` / `systemd` / `manual` |
| `SETUP_URL`   | Phase 6 | composed `http://$UI_HOST/?daemon=…&token=…` |

---

## Final report (emit after Phase 6)

```
=== CORTEX-SETUP REPORT ===
OS:              <macos | linux>
Daemon binary:   <BIN_PATH>
Setup script:    <SETUP_PATH>
Port:            <CORTEX_PORT>
UI host:         <UI_HOST>
Whisper model:   <MODEL>  (cache: <WHISPER_CACHE>)
Token:           ~/.keisei/cortex.token  (chmod 600, <hidden>)
UI dist:         <UI_DIST>
Supervisor:      <launchd | systemd | manual>
Keys (env refs): ANTHROPIC=<present/missing>  ELEVENLABS=<present/missing>  FAL=<present/missing>
Setup URL:       [copied to clipboard]
```

Degradation notes (manual supervisor / missing keys / partial install) are
appended by Phase 6 §6f. See `phase-6-done.md` for exact wording.

---

## Rules (apply throughout — enforced at every phase)

- **Pure-click contract.** Every decision is an `AskUserQuestion`. The
  only free-text fields are: optional API key paste in Phase 1, optional
  custom port / UI host in Phase 2. No other `freeText` anywhere.
- **Hard fail fast (Phase 0).** If the `kei-cortex` binary or the
  `kei-cortex-setup.sh` primitive is missing, STOP the wizard with a
  one-line error telling the user to run `./install.sh --profile=cortex`
  first. Do NOT try to limp through.
- **Idempotent.** Re-running the wizard must NOT clobber an existing
  `~/.keisei/cortex.token`, `~/.keisei/cortex-ui-dist/`, or supervisor
  unit. The primitive subcommands are idempotent by contract.
- **NO DOWNGRADE (RULE -1).** If pip / whisper-pull / ui-build fails,
  return 2-3 constructive paths (retry / skip / abort) — never "cannot
  proceed".
- **NO HALLUCINATION (RULE 0.4).** Never fabricate a token value,
  fingerprint, or path. Always show the real output of the primitive.
- **RULE 0.8 secrets.** Never print API key VALUES. Only `<present>` /
  `<missing>` / `<hidden>`. The token itself never lands in chat output
  — only its path and `<hidden>` marker.
- **RULE 0.13 orchestrator-branch.** This skill writes files and invokes
  the primitive only. It does NOT call git, pnpm, or cargo directly. The
  orchestrator (main session) owns all git state changes.
- **Constructor Pattern (RULE ZERO).** Every phase file ≤200 LOC. This
  index stays ≤120 LOC.

---

## References

- `_primitives/_rust/kei-cortex/INSTALL.md` — daemon install notes
- `_primitives/kei-cortex-setup.sh` — imperative setup helper (contract locked)
- `_primitives/_rust/kei-cortex/scripts/requirements.txt` — faster-whisper==1.2.1
- `_ts_packages/packages/cortex-ui/` — Svelte 5 browser frontend
- `~/.claude/secrets/.env` — RULE 0.8 umbrella env file
- `~/.keisei/cortex.token` — daemon bearer token (generated here)
