# Phase 4 — Gap analysis (AskUserQuestion multi-select)

Present the classification matrix from Phase 3 as a code block (markdown
list) in chat, then emit:

```json
{
  "questions": [
    {
      "question": "Which gaps to close this session?",
      "header": "Gaps",
      "multiSelect": true,
      "options": [
        {"label": "Component N — CREATE new block",      "description": "No prior art found — draft a new _blocks/ entry in Phase 6"},
        {"label": "Component M — ADAPT existing block",  "description": "Prior art found but needs edits — copy + modify in Phase 6"},
        {"label": "Component K — wire external API",     "description": "External dep — reference api-*.md block or add a new one"},
        {"label": "Skip — components K, L reuse as-is",  "description": "No action needed, they're already covered"}
      ]
    }
  ]
}
```

Options are GENERATED dynamically — one per component from Phase 3 whose
class ∈ {ADAPT, CREATE, EXTERNAL}. User clicks zero or more. Empty
multi-select is valid: means "reuse only, skip Phase 6".

Substitute the literal component descriptions in the option labels (not the
placeholders shown above — those are the shape). For example, if
Component 2 is "cost guard for fal.ai calls" and its class is CREATE, the
option label becomes `"Component 2: cost guard for fal.ai calls — CREATE new block"`.

## Verify-criterion

- Selected gap list stored as `GAPS` (a list of component-indices with
  their chosen action: CREATE / ADAPT / EXTERNAL).
- Empty list is allowed and means Phase 6 is skipped entirely.
- No component has two contradicting actions (e.g. REUSE + CREATE).
