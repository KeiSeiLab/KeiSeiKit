# Rules as Cubes (v0.52)

> Hooks ARE rules — same primitive, two formats. Unify them as folder cubes.

## Problem

Rules and hooks currently live in separate trees:

```
~/.claude/rules/<slug>.md            ← human-readable, LLM context
~/.claude/hooks/<slug>-guard.sh      ← machine-enforceable, PreToolUse
```

Each rule and each guard are two halves of the **same primitive** — what the system enforces. Today they drift:

- `rule.md` updates without `guard.sh` update → enforcement is stale
- `guard.sh` updates without `rule.md` update → no human can read what it catches
- Cross-references (`Rule: ~/.claude/rules/<slug>.md`) live in hook header comments — manual hygiene
- `RULES.md` registry has separate columns for rule and hook — same row, two paths
- Tests for guard correctness live nowhere consistent

This is overlay-pattern (Core Rule 1: No Patching / No Overlays). The fix is the same as v0.49 `kei_is_interactive`: one cube = one truth.

## Solution — folder cube per rule

```
~/.claude/rules/<slug>/
  ├─ rule.md      ← human-readable for LLM context
  ├─ guard.sh     ← PreToolUse / UserPromptSubmit enforcer
  ├─ test.sh      ← fixtures asserting guard catches violations
  └─ README.md    ← (optional) extended rationale + incident log
```

**Properties:**
- One folder = one rule = one truth.
- `rule.md` defines intent, `guard.sh` enforces, `test.sh` proves enforcement matches intent.
- Test catches drift: if `guard.sh` changes but stops catching what `rule.md` describes, `test.sh` fails.
- `RULES.md` registry row points to the folder, not 2-3 files.
- `settings.json` PreToolUse entry references `~/.claude/rules/<slug>/guard.sh` directly.

## POC — `tty-interactivity-gate`

The cube already exists at `~/.claude/rules/tty-interactivity-gate/`:

```
~/.claude/rules/tty-interactivity-gate/
  ├─ rule.md        ← was rules/tty-interactivity-gate.md (moved)
  ├─ guard.sh       → symlink ../../hooks/tty-interactivity-gate-guard.sh
  └─ test.sh        ← new: 3-case fixture (3/3 passing)
```

Run the test:
```bash
~/.claude/rules/tty-interactivity-gate/test.sh
```

settings.json path remains unchanged (still `~/.claude/hooks/tty-interactivity-gate-guard.sh`) — symlink lets us migrate the LAYOUT without touching the wiring. Future migrations can move the real file inside the cube and update settings.json.

## Migration plan (full)

**Phase A — POC (this commit, v0.52):**
- 1 rule (`tty-interactivity-gate`) migrated to folder + test.sh + symlink
- Design doc (this file)
- RULES.md table column added for "cube?" status

**Phase B — Critical-path rules (v0.53):**
- `no-github-push` (RULE 0.1) — has rule text in CLAUDE.md
- `secrets-single-source` (RULE 0.8) — already extracted to `hooks/_lib/secret-patterns.sh`
- `rust-first` (RULE 0.2) — has rule + hook
- Each gets its own cube with `rule.md` extracted from CLAUDE.md / inline.

**Phase C — Bulk migration (v0.54):**
- All 28 entries in RULES.md migrated to folder cubes
- Real file moves: `~/.claude/hooks/<slug>-guard.sh` → `~/.claude/rules/<slug>/guard.sh`
- settings.json paths updated atomically
- Test fixtures backfilled where missing

**Phase D — Substrate ships rule cubes (v0.55):**
- KeiSeiKit substrate `hooks/<slug>/` directories carry both `guard.sh` AND `rule.md`
- `install.sh` deploys cube folder → `~/.claude/rules/<slug>/`
- settings.json registration points at deployed cube path
- Users edit `rule.md` and `guard.sh` in lock-step — same git diff

## Backward-compatibility

- During Phase A-C: existing `~/.claude/hooks/<slug>-guard.sh` paths preserved via symlinks
- `RULES.md` registry adds a "cube?" column — `Y` = migrated, `N` = legacy flat
- `settings.json` PreToolUse entries unchanged until Phase C atomic flip
- `/escalate-recurrence` skill writes new rules as cubes directly (no flat-file legacy from this point)

## Why this matters

The pattern repeats the v0.49 `kei-prompt` win:

| Pattern | Before | After | Reduction |
|---|---|---|---|
| v0.49 interactivity | 15 inline `[ -t 0 ]` gates | 1 `kei_is_interactive` cube | 15→1 |
| v0.52 rules | 28 split rule.md + guard.sh | 28 folder cubes | 56→28 (single source per rule) |

Same architectural rule: 1 file → 1 cube → no drift. Constructor Pattern at the rules layer.
