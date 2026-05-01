# Phase 4 — Mermaid architecture starter

Goal: seed `<DIR>/docs/diagrams/` with a minimal Mermaid file the user
can evolve. Keep it small; the block `_blocks/docs-architecture-diagrams.md`
carries the full pattern catalogue.

## 4a — Pick starter pattern (AskUserQuestion #4)

```json
{
  "questions": [
    {
      "question": "Seed which Mermaid pattern?",
      "header": "Diagram",
      "multiSelect": false,
      "options": [
        {"label": "System context (flowchart LR)", "description": "One-page overview — User / API / Service / DB / Queue. Good default."},
        {"label": "Sequence (sequenceDiagram)",     "description": "Request flow — Client / API / DB. Pick for API-first projects."},
        {"label": "State machine (stateDiagram-v2)","description": "FSM-driven projects — Pending / Running / Done / Failed."},
        {"label": "ER (erDiagram)",                 "description": "DB schema summary — two related entities."},
        {"label": "Skip this phase",                 "description": "No diagram seeded; move to Phase 5"}
      ]
    }
  ]
}
```

On `Skip` → `DIAGRAMS = 0`, continue to Phase 5.

## 4b — Write the starter file

Create `<DIR>/docs/diagrams/` and write one `.mmd` file matching the
click:

- `context.mmd`  — system context
- `request.mmd`  — sequence
- `lifecycle.mmd` — state machine
- `schema.mmd`   — ER

Use the short templates from `_blocks/docs-architecture-diagrams.md` §1-4
verbatim. Placeholders (User / API / Service / DB / Queue) are fine —
the user evolves them next session.

If the target file exists → skip and warn; do not overwrite without
`--force` (same contract as Phase 2).

Set `DIAGRAMS = 1`.

## 4c — Preview hint (no write)

After writing, print:

```
[docs-scaffold] preview locally:
  npm install -g @mermaid-js/mermaid-cli    # one-time
  mmdc -i <DIR>/docs/diagrams/<file>.mmd -o /tmp/preview.svg
```

No AskUserQuestion; this is just a hint line.

## Verify-criterion

- `<DIR>/docs/diagrams/` directory exists after writing.
- Exactly one `.mmd` file was created (or zero if `Skip` was chosen).
- File is syntactically valid Mermaid (heading matches the picked pattern).
- No other files under `<DIR>/` were touched.
