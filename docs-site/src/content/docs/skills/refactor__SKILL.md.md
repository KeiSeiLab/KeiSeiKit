---
title: SKILL
path: refactor/SKILL.md
dna_hash: sha256:6f663e716a32aba5
language: markdown
size_loc: 57
generated: by-keidocs
---

# refactor/SKILL.md

## Public API

- `Refactor Workflow` — ---
- `When to use` — - Restructuring code that violates the Constructor Pattern (file >200 LOC, function >30 LOC, multiple responsibilities, duplicated code).
- `Step 1: Understand Current State` — - Read target file(s) completely
- `Step 2: Plan Refactoring` — - Identify what violates Constructor Pattern:
- `Step 3: Checkpoint` — - `checkpoint: before refactor $target`
- `Step 4: Refactor (Incremental)` — - ONE structural change at a time
- `Step 5: Audit` — - Check: does refactored code follow Constructor Pattern?
- `Step 6: Final Verification` — - Run full test suite — all pass
- `Step 7: Commit` — - `refactor: <what was restructured and why>`

## Related

- parent: `refactor`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
