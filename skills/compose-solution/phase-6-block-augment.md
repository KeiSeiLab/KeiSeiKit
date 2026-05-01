# Phase 6 — Block augmentation (conditional)

Only runs if `GAPS` from Phase 4 is non-empty. For EACH `CREATE` or `ADAPT`
entry in `GAPS`, run this sub-phase in sequence (NOT in parallel — user must
approve each before the next).

## 6a — Draft the block

Template (follow the shape of `_blocks/baseline.md` or
`_blocks/rule-math-first.md` — short, single-concern, imperative voice):

```markdown
# <HEADING — pattern name in short caps>

<One-line purpose statement.>

## When to include

<1-3 bullets describing which manifests should list this block in
their `blocks = [...]` array.>

## What it declares

<3-8 imperative bullets. Constructor Pattern: one concern only.>

## References

- <link to upstream rule or external doc, if any>
- <evidence grade [E1-E6]>
```

Target length: 20-40 LOC markdown. Hard ceiling: 60 LOC — above that, SPLIT
into two blocks before continuing.

Slug: kebab-case, 2-4 words. Must not collide with existing `_blocks/*.md`.
Verify via:

```bash
ls _blocks/<slug>.md 2>/dev/null && echo "COLLISION" || echo "free"
```

If collision: append a disambiguator (`<slug>-v2`, or a domain suffix like
`<slug>-embedded`).

## 6b — Preview + per-block click (AskUserQuestion)

Emit the draft inline in chat, then:

```json
{
  "questions": [
    {
      "question": "Write this block?",
      "header": "Block",
      "multiSelect": false,
      "options": [
        {"label": "Write to _blocks/<slug>.md", "description": "Save permanently — enriches the kit for all future sessions"},
        {"label": "Edit (free-text)",           "description": "Reply with one free-text message describing changes; I regenerate"},
        {"label": "Skip this block",            "description": "Don't save this one; proceed to next gap"},
        {"label": "Abort session",              "description": "Stop the whole skill; nothing else gets written"}
      ]
    }
  ]
}
```

Resolution:

- **Write** → use Write tool to create `_blocks/<slug>.md` under the repo
  root (`~/Projects/KeiSeiKit/_blocks/` when running against the kit repo;
  or wherever `$PWD`'s `_blocks/` lives when invoked from another KeiSeiKit
  fork).
- **Edit** → single free-text prompt, regenerate, re-preview.
- **Skip** → move to next gap.
- **Abort** → stop; no writes Phase 6 onward.

## 6c — After all gaps processed

Report the block-count delta:

```bash
ls _blocks/ | wc -l
```

Show `before → after` count so the user sees the kit got N blocks smarter
this session. This is the feedback-loop signal — make it visible in every
session that touched Phase 6.

## Verify-criterion

Every block written passes two sanity checks:
- File exists on disk after Write.
- No `{{placeholder}}` literals remain (the assembler's `validator.rs`
  rejects those; same hygiene applies here).
