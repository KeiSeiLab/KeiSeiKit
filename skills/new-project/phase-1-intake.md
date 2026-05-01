# Phase 1 — Intake

One free-text line + one click batch of 5 questions. Only Phase 1a is typed.

## 1a — Ask for the project goal

Emit a regular message (NOT AskUserQuestion):

> Describe the project in one line: what do you want to produce / prove /
> ship, at what scope (feature / service / paper / proof)? Reply in one
> message. The main agent and sub-agents will read this verbatim.

Store the reply as `GOAL`. Derive `PROJECT_SLUG = kebab-case(first 4-6
meaningful words of GOAL)`, ≤ 40 chars, ASCII only.

## 1b — Shape click (AskUserQuestion, ONE batch of 5)

```json
{
  "questions": [
    {
      "question": "Project type?",
      "header": "Type",
      "multiSelect": false,
      "options": [
        {"label": "new code",       "description": "Greenfield implementation — ship a running artefact"},
        {"label": "research",       "description": "Empirical investigation — experiments + results doc"},
        {"label": "theoretical",    "description": "Math / physics / algorithmic derivation — proofs + chatlog"},
        {"label": "hybrid",         "description": "Code + theory in parallel — both tracks merged at end"},
        {"label": "documentation",  "description": "Docs-only project — no new runtime code"}
      ]
    },
    {
      "question": "Theoretical component shape?",
      "header": "Theory",
      "multiSelect": false,
      "options": [
        {"label": "none",                 "description": "Pure-implementation project"},
        {"label": "math derivation",      "description": "Lemma → theorem chain, reviewed by physics-deriver"},
        {"label": "prior-art research",   "description": "Literature + existing-project sweep before any write"},
        {"label": "architecture spec",    "description": "Design document + interface contracts before code"},
        {"label": "paradigm analysis",    "description": "Observable classification + falsifier design"}
      ]
    },
    {
      "question": "Parallel sub-agent budget?",
      "header": "Fanout",
      "multiSelect": false,
      "options": [
        {"label": "single",       "description": "Main agent only — sequential work, no sub-forks"},
        {"label": "up to 3",      "description": "Small fleet — typical feature work"},
        {"label": "up to 5",      "description": "Medium fleet — multi-track research or cross-cutting refactor"},
        {"label": "up to 10",     "description": "Wide fanout — parallel experiments / audits / prior-art sweeps"}
      ]
    },
    {
      "question": "Main-agent role?",
      "header": "Main",
      "multiSelect": false,
      "options": [
        {"label": "meta-orchestrator",        "description": "Generic orchestrator that only fans out and merges"},
        {"label": "spawn specialist",         "description": "Create a new dedicated agent via /new-agent before Phase 2"},
        {"label": "compose-solution decides", "description": "Hand off project shape to /compose-solution for auto-routing"}
      ]
    },
    {
      "question": "DB / ledger mirror?",
      "header": "DB",
      "multiSelect": false,
      "options": [
        {"label": "file-only",       "description": "No SQLite — write bundle files only; skip kei-ledger"},
        {"label": "SQLite ledger",   "description": "Use kei-ledger (default — RULE 0.12 compliant)"},
        {"label": "external tool",   "description": "Mirror to another tracker (Jira / Linear / Forgejo issues)"}
      ]
    }
  ]
}
```

Store answers as `PROJECT_TYPE`, `THEORY_PART`, `FANOUT`, `MAIN_AGENT`,
`DB_MODE`.

## Verify-criterion

- `GOAL` non-empty.
- `PROJECT_SLUG` matches `^[a-z0-9][a-z0-9-]{2,39}$`.
- All 5 click answers are exactly one of the labels above.
- If `MAIN_AGENT == "spawn specialist"` — Phase 2 begins with a handoff to
  `/new-agent` before any ledger fork.
- If `DB_MODE == "file-only"` — Phase 2 skips the `kei-ledger fork` call
  but STILL writes the 6-file bundle. Report this deviation in the final
  report so the user sees the ledger SSoT was bypassed by explicit choice.
