# Public Mirror Workflow

This repository is the **public-clean** mirror of the private KeiSeiKit
research repo. Every commit here corresponds to a one-way sync from the
private side that has been scrubbed of Genesis / KeiLab internal cross-
references per RULE 0.1 (NO GITHUB PUSH for repos with patent-IP).

## Layout

```
~/Projects/KeiSeiKit/         private (research) — Genesis-aware OK
       ↓ tools/sync-public.sh
~/Projects/KeiSeiKit-public/  this repo — public-ready
       ↓ git push github main (manual, after review)
github.com/KeiSei84/KeiSeiKit
```

## Sync workflow

1. Develop in `~/Projects/KeiSeiKit/`. Do **not** edit files directly here.
2. When ready to publish: `tools/sync-public.sh --dry-run`
   - rsyncs tracked files (excludes data/corpus/, training set, firmwares,
     worktrees, _forks/, target/)
   - applies sed-substitutions for known leak patterns (`Genesis`,
     `KeiLab/Genesis/`, `Genesis HISTORY.md` → `internal calibration`)
   - runs a residual-leak grep and shows hits
   - shows `git diff --cached --stat`
3. Review. If clean: `tools/sync-public.sh --confirm`
4. Manual push: `git push github main` — **never automatic** per RULE 0.1.

## What is excluded permanently

- `_primitives/_rust/frustration-matrix/data/corpus/` — raw user-line
  training data containing Genesis-session quotes
- `_primitives/_rust/frustration-matrix/data/labeled-training-set.jsonl`
  — 27 KB of labeled Genesis-session lines
- `_primitives/_rust/frustration-matrix/data/firmwares/*.fw` — pre-trained
  n-gram statistics derived from Genesis corpus
- `.claude/worktrees/` — per-agent temporary worktrees
- `_forks/` — Wave-fork scratch space
- `target/` — Cargo build output

Public users training their own classifier follow
`_primitives/_rust/frustration-matrix/data/README.md`.

## Substitution rules (sync-public.sh)

| Private text | Public text |
|---|---|
| `Port of Genesis compute_firmware_v2.py + deep_firmware.py. Theorem` | `Theorem` |
| `Genesis HISTORY.md §N` | `internal calibration` |
| ` ~/Projects/KeiLab/Genesis/...` (path) | (stripped) |
| `DERIVED from Genesis` | `DERIVED from internal` |
| `Genesis-clean: no normalize...` | (line removed) |
| `Genesis dissolves those` | `as ill-posed` |
| `Genesis Phase 3 compression ratio` | `internal compression-ratio target` |
| `[E2 VERIFIED: Genesis ...]` / `[E1 VERIFIED: source]` | (stripped) |
| `for KeiLab / bio work` | `for any pre-registered experiment work` |

Add new rules by editing `SUBSTITUTIONS` in `tools/sync-public.sh`.

## Author attribution

`README.md`, `docs/PHILOSOPHY.md`, `docs/ARCHITECTURE.md` legitimately
mention "KeiLab" as the authoring organisation and "FILING-SSOT.md /
PATENT-GRAPH.md" as filenames the kit's `patent:` commit type touches.
These are NOT leaks — public users need them to understand the kit's
purpose (it ships tools so users can protect their own patent IP).

The sync script does not strip author attribution.

## RULE 0.1 reminder

The author's `~/.claude/hooks/no-github-push.sh` blocks `git push` to any
github.com URL by default. Override only after reviewing the diff and
running:

```bash
GENESIS_LEAK_BYPASS=1 git push github main
```

The bypass is logged to `~/.claude/memory/genesis-bypass-log.md`.
