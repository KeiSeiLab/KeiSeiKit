# Phase 3 — First ADR walk-through (optional)

Goal: if `DECISIONS.md` was just scaffolded, offer to append one real ADR
now while context is fresh. Otherwise skip.

## 3a — Gate (AskUserQuestion #3)

```json
{
  "questions": [
    {
      "question": "Append a real ADR now?",
      "header": "ADR",
      "multiSelect": false,
      "options": [
        {"label": "Yes — walk me through ADR-002",   "description": "Interactive: I ask context / drivers / options / outcome; I write the entry"},
        {"label": "No — keep the template only",    "description": "Phase 2 already wrote ADR-001 Constructor Pattern template; leave it"},
        {"label": "Skip this phase",                 "description": "Move to Phase 4 diagrams"}
      ]
    }
  ]
}
```

On `Skip` or `No` → `ADR_N = 0`, continue to Phase 4.

## 3b — Free-text elicitation (only if "Yes")

Ask the user, in a single message (no AskUserQuestion), for four lines:

1. **Title** (≤ 60 chars) — short decision name
2. **Context** (1-2 sentences) — what forced the decision
3. **Options considered** — comma-separated list (2-4 items)
4. **Chosen option + evidence grade [E1-E6]** — one line

## 3c — Compose the ADR entry

Renumber: Read `DECISIONS.md`, find the highest existing `ADR-NNN`,
assign `NNN+1` (three-digit zero-pad). Append the block using the MADR
4.0 shape from `_blocks/docs-decisions-adr.md`. Never rewrite existing
ADR entries. Never drop below the highest existing number.

Set `ADR_N = 1`.

## 3d — Show the user the appended block

Print the new entry inline so they can confirm correctness. No
AskUserQuestion here — they can ask to amend in the next turn. Append-
only invariant stands: amendments become ADR-MMM that supersedes.

## Verify-criterion

- `DECISIONS.md` exists at `<DIR>/DECISIONS.md`.
- The new ADR number is strictly greater than all prior numbers.
- Evidence grade is one of E1-E6; if missing, re-ask before writing.
- No existing ADR entry was modified.
