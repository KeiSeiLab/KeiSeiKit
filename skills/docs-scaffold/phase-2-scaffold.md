# Phase 2 — Scaffold (run `kei-docs-scaffold.sh`)

Goal: produce the files selected in `GAPS`, non-destructively by default.

## 2a — Confirm scaffold mode (AskUserQuestion #2)

```json
{
  "questions": [
    {
      "question": "Scaffold mode?",
      "header": "Mode",
      "multiSelect": false,
      "options": [
        {"label": "Safe — skip existing files",            "description": "Recommended. Writes only to files not already on disk."},
        {"label": "Force — overwrite existing",            "description": "Pass --force. Existing files are replaced (one git checkpoint emitted first)."},
        {"label": "Dry-run — print planned actions only",  "description": "No writes. Use for an audit report before committing."},
        {"label": "Abort",                                  "description": "Stop — nothing gets written."}
      ]
    }
  ]
}
```

## 2b — Invoke the primitive

Locate the scaffolder at `_primitives/kei-docs-scaffold.sh` (repo-local)
or `~/.claude/agents/_primitives/kei-docs-scaffold.sh` (installed). If
neither exists, abort with a NO DOWNGRADE error listing two paths:

1. Run `./install.sh` from the KeiSeiKit repo to install primitives.
2. Clone KeiSeiKit and invoke the scaffolder directly from the repo.

For each `GAPS` entry (except `CHANGELOG.md` — that is Phase 5, and
`docs/diagrams/` — Phase 4), map to the scaffolder's `--type` value:

| GAPS entry | `--type` |
|---|---|
| CLAUDE.md | `claude` |
| DECISIONS.md | `decisions` |
| docs/runbook.md | `runbook` |
| README.md | `readme` |

Invoke per selected type (or `--type=all` if all four non-Phase 4/5
entries are selected):

```bash
kei-docs-scaffold.sh [--force] [--dry-run] --type=<type> "$DIR"
```

Capture each write line (`[scaffold]   wrote: <path>`) into `SCAFFOLDED`.

## 2c — README banned-public gate

If `README.md` is in `GAPS`, grep the project for banned-public markers
BEFORE invoking the scaffolder:

```
grep -rEi "weight|checkpoint|training-loop|offensive|kernel|guidance-law" "$DIR" --include="*.md" --include="*.toml"
```

If matches found → stop and require the user to type `"yes, deploy"` +
`"confirm publication"` literal phrases before proceeding. See
`~/.claude/rules/security.md`.

## Verify-criterion

- The scaffolder exited with status 0.
- `SCAFFOLDED` is a non-empty list (unless dry-run was chosen).
- No file outside `$DIR` was written.
- If README was scaffolded, the banned-public grep was clean OR the
  double-confirmation was captured.
