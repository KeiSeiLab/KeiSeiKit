# Phase 2 — Entities + relations matrix

Collect the entity list (typed) once, then click the relations matrix. This
is the second (and last) phase that accepts typed input.

## 2a — Ask for entities (plain message, NOT AskUserQuestion)

> List the entities (tables) and for each a short comma-separated field
> list. One entity per line, format: `<Entity>: field1, field2, ...`.
> Example:
>
> ```
> User: email, name, created_at
> Organization: name, plan
> Membership: user_id, org_id, role
> ```
>
> 3–15 entities is typical. Keep it short — we'll refine fields in Phase 3.

Parse the reply into `ENTITIES = [{name, fields: [...]}, ...]`. Validate:
- Entity names are `PascalCase` (normalize if user types `user_profile` →
  `UserProfile`, record normalization in state).
- Each entity has ≥1 field.
- No duplicate entity names.
- If parse fails → re-ask once with a corrected example.

## 2b — Relations matrix click (AskUserQuestion, multi-select)

For each UNORDERED PAIR of entities `(A, B)`, ask one multi-select row.
Skip pairs the user hasn't mentioned any cross-reference for (heuristic:
if `A`'s fields include `b_id` or `B`'s fields include `a_id`, or the
user's intake paragraph mentions both).

Build ONE `AskUserQuestion` call with up to 5 questions. If the entity
count yields > 5 candidate pairs, batch into multiple calls (still counts
toward the ≥1 AskUserQuestion minimum).

Per-pair question template:

```json
{
  "question": "Relation between <A> and <B>?",
  "header": "<A>↔<B>",
  "multiSelect": false,
  "options": [
    {"label": "None",              "description": "No direct FK; entities are independent"},
    {"label": "One-to-one",        "description": "A.b_id UNIQUE FK to B.id (or vice versa)"},
    {"label": "One-to-many (A→B)", "description": "B.a_id FK to A.id; one A has many B"},
    {"label": "One-to-many (B→A)", "description": "A.b_id FK to B.id; one B has many A"},
    {"label": "Many-to-many",      "description": "Requires a junction table; skill will auto-name it <A><B>"}
  ]
}
```

Store the result in `ENTITIES` as `.relations = [{from, to, kind}, ...]`.

## 2c — Auto-generate junction tables

For each pair marked `Many-to-many`, append a synthetic entity to
`ENTITIES`:

```
<A><B>:
  <a>_id FK → <A>.id  (ON DELETE CASCADE)
  <b>_id FK → <B>.id  (ON DELETE CASCADE)
  PRIMARY KEY (<a>_id, <b>_id)
  created_at TIMESTAMPTZ DEFAULT now()
```

Names must be deterministic (alphabetical order: `OrganizationUser`, not
`UserOrganization`, for pair `(User, Organization)`). Record the rule in
state so Phase 3 renders it consistently.

## Verify-criterion

- `ENTITIES` has ≥1 entry after parse.
- Every relation in `ENTITIES[*].relations` references two distinct
  existing entities.
- Every `Many-to-many` has produced a junction entity.
- No entity is orphaned (zero relations AND not mentioned in INTAKE) —
  warn the user with "Entity X has no relations; keep it?" (NO DOWNGRADE:
  offer `keep / drop / add relation` as follow-up click).
