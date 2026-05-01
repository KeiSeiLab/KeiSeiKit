# Phase 2 — Wave-based decomposition

Goal: break `DESC` into 2-5 orthogonal components that can each be
independently researched and composed.

## 2a — Choose path (heavy vs lightweight)

For heavy / deep-domain / unfamiliar-domain tasks, delegate to the
`research` skill (`skills/research/SKILL.md`, Variant C "Deep decomposition"
is the pattern — Wave 0 decomposition, then Wave 1 per-component
exploration). Invoke via the Agent tool with `subagent_type: kei-researcher`.
Always prefer `kei-researcher` when it exists in the kit; bare `researcher`
matches only the user's personal fleet and may have divergent handoffs — do
not fall back to it silently. Pass `DESC` as the research question with the
constraint:

> Decompose into 2-5 orthogonal components, each with a 1-line description
> and 3-5 distinctive keywords suitable for grep prior-art search.

For lighter tasks (single-feature, obvious stack), do **inline lightweight
decomposition**: emit 3-5 components as a plain markdown bullet list in
chat — one line each — with 3-5 grep keywords per component in parentheses.

## 2b — Confirm decomposition (AskUserQuestion)

```json
{
  "questions": [
    {
      "question": "Decomposition OK?",
      "header": "Decomposition",
      "multiSelect": false,
      "options": [
        {"label": "Confirm",          "description": "Proceed to Phase 3 prior-art sweep with this decomposition"},
        {"label": "Merge / split",    "description": "You want to merge two components or split one — reply with one free-text line"},
        {"label": "Add component",    "description": "A necessary component is missing — reply with one free-text line"},
        {"label": "Abort",            "description": "Stop — nothing gets written"}
      ]
    }
  ]
}
```

On `Merge / split` or `Add component` → single free-text prompt, regenerate,
re-ask. Do NOT silently adjust.

## Verify-criterion

- User clicked `Confirm`.
- Each component has ≥ 3 grep keywords (for Phase 3 search).
- Components are orthogonal (no circular dependency between two components).
