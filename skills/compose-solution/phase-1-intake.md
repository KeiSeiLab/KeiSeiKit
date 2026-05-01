# Phase 1 — Intake

One free-text line + one click batch. This is the only typed input required
by the whole skill (Phase 6 may add optional per-block edit prose).

## 1a — Ask for the task description

Emit a regular message (NOT AskUserQuestion):

> Describe the task in one paragraph: what do you want to solve, for which
> project or concern, and what's the expected surface (something that runs,
> something that blocks a mistake, something that documents a pattern)?
> Reply in one message.

Store the user's reply verbatim as `DESC`.

## 1b — Target-type click (AskUserQuestion, ONE call)

```json
{
  "questions": [
    {
      "question": "Target artefact type?",
      "header": "Type",
      "multiSelect": false,
      "options": [
        {"label": "Auto-detect",          "description": "Orchestrator infers from intake — recommended when unsure"},
        {"label": "Agent (specialist)",   "description": "New kit-agent manifest — specialist for a project or concern"},
        {"label": "Skill (user-invoked)", "description": "A /slash skill in ~/.claude/skills/ or KeiSeiKit/skills/"},
        {"label": "Hook (enforcement)",   "description": "Shell hook registered in settings.json (block / enforce / warn / remind)"},
        {"label": "Rule (documentation)", "description": "Rule file in ~/.claude/rules/ with optional hook partner"},
        {"label": "Block (reusable)",     "description": "Behavioural block in _blocks/ — composable via manifests"}
      ]
    }
  ]
}
```

Store the choice as `T`. If `T == "Auto-detect"`, leave resolution until
Phase 7 (after architecture is known).

## Verify-criterion

- `DESC` is non-empty.
- `T` is exactly one of the six labels above.
- If either fails — re-ask the failing input; do not fall through.
