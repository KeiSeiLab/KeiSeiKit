# Phase 1 — Intake (auto-detect + audit + pick gaps)

Goal: identify the target repo, detect its stack, enumerate docs that
already exist, and let the user pick which gaps to scaffold.

## 1a — Target directory + stack detect

If the skill was invoked with a directory argument, use it. Otherwise use
`$PWD`. Store as `DIR`. Reject non-directories with a short error + the
user re-enters.

Run `_primitives/kei-docs-scaffold.sh --dry-run --type=all "$DIR"` just
to print the detected stack, or replicate the detection via Read:

- `Cargo.toml`    → **Rust (Cargo)**
- `pubspec.yaml`  → **Flutter / Dart**
- `package.json`  → **Node.js / TypeScript**
- `pyproject.toml` | `requirements.txt` → **Python**
- `go.mod`        → **Go**
- `Package.swift` → **Swift (SPM)**
- `docker-compose.yml` → **Docker (compose)**
- else            → **Unknown**

Store as `STACK`. Print: `[docs-scaffold] DIR=<DIR> STACK=<STACK>`.

## 1b — Audit existing docs

List presence of each target file; store the set as `EXISTING`:

- `<DIR>/CLAUDE.md`
- `<DIR>/DECISIONS.md`
- `<DIR>/docs/runbook.md`
- `<DIR>/README.md`
- `<DIR>/docs/diagrams/` (directory, non-empty)
- `<DIR>/CHANGELOG.md`

## 1c — Pick gaps to scaffold (AskUserQuestion #1, multi-select)

```json
{
  "questions": [
    {
      "question": "Which docs to scaffold? (existing files are skipped unless --force selected in Phase 2)",
      "header": "Gaps",
      "multiSelect": true,
      "options": [
        {"label": "CLAUDE.md",           "description": "Agent-facing project guide (architecture, stack, constraints)"},
        {"label": "DECISIONS.md",        "description": "MADR 4.0 append-only ADR log"},
        {"label": "docs/runbook.md",     "description": "Ops playbook (symptom → check → fix → escalation)"},
        {"label": "README.md",           "description": "Public README — scaffolder checks banned-public list first"},
        {"label": "docs/diagrams/",      "description": "Mermaid architecture starter (Phase 4 seeds one file)"},
        {"label": "CHANGELOG.md",        "description": "Via kei-changelog from conventional commits (Phase 5)"}
      ]
    }
  ]
}
```

Store selection as `GAPS`. If empty → skip to final report with
"Nothing to scaffold".

## Verify-criterion

- `DIR` exists as a directory on disk.
- `STACK` is one of the labels above (or `Unknown`).
- `EXISTING` is computed and reported to the user inline.
- `GAPS` is captured from the click.
