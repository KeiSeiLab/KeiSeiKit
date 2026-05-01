# Phase 2 — Resource model (entities → resources / types)

Turn the app description into a list of entities, their relationships, and
the actions on them. This is the second and last typed phase — the user
types a short entity list in 2a, then one click in 2b locks the shape.

## 2a — Ask for entities + relationships (typed)

Emit a regular message (NOT AskUserQuestion):

> List the core entities (one per line) with an optional `owns→` arrow for
> relationships. Example:
> ```
> User
> Invoice owns→ InvoiceItem
> Customer owns→ Invoice
> Tag
> Invoice many-to-many→ Tag
> ```
> Keep it to ≤10 entities. Anything beyond core can be added after launch.

Store the parsed list as `RESOURCES`. Each entry is
`{name, owns: [child...], many_to_many: [peer...]}`. If parsing fails,
re-ask with the exact syntax rules shown above.

## 2b — Shape click (AskUserQuestion, single-select)

Reference: `_blocks/api-rest-conventions.md` (REST resources),
`_blocks/api-graphql.md` (types + connections).

```json
{
  "questions": [
    {
      "question": "How should the resources surface?",
      "header": "Shape",
      "multiSelect": false,
      "options": [
        {"label": "Flat REST (one resource per entity, ≤2 levels nesting)",
         "description": "`/invoices`, `/invoices/{id}/items`. Deeper nesting flattened via query filters. Default for REST."},
        {"label": "REST + sub-resources for every owns→ relation",
         "description": "`/customers/{id}/invoices`, `/invoices/{id}/items`. Readable, curl-friendly; nesting budget 2 levels."},
        {"label": "GraphQL types + Relay Connections",
         "description": "Each entity → `type Foo { ... }`, each list → `FooConnection` with cursor pagination. See api-graphql.md"},
        {"label": "GraphQL federation (subgraph per entity cluster)",
         "description": "Apollo Federation 2 @key directives. ONLY pick when you truly have multiple teams / subgraphs."},
        {"label": "Mixed: REST for public CRUD, GraphQL for dashboards",
         "description": "Record both — this skill will generate primary surface; rerun for secondary."}
      ]
    }
  ]
}
```

Store as `SHAPE`. Gate on `STYLE` from Phase 1:

- `STYLE = REST` → accept Flat REST or sub-resource REST; reject GraphQL
  options (re-ask).
- `STYLE = GraphQL` → accept GraphQL types or federation.
- `STYLE = tRPC / gRPC` → SHAPE defaults to "flat" (procedures per entity);
  skip the click, record `SHAPE = flat-procedures`.
- `STYLE = Hybrid` → emit a warning that both shapes will be skeleton'd in
  Phase 3.

## 2c — Emit resource-to-action matrix (inline, no AskUserQuestion)

Print a table the user can tweak before Phase 3 generates the contract.
Example for a notes + tags API:

```markdown
| Entity        | Create | Read | Update | Delete | List | Search |
|---------------|:------:|:----:|:------:|:------:|:----:|:------:|
| User          |   -    |  ✓   |   ✓    |   -    |  ✓   |   ✓    |
| Note          |   ✓    |  ✓   |   ✓    |   ✓    |  ✓   |   ✓    |
| Tag           |   ✓    |  ✓   |   -    |   ✓    |  ✓   |   -    |
| NoteTag (m2m) |   ✓    |  -   |   -    |   ✓    |  -   |   -    |
```

- Rows = entries from `RESOURCES`.
- Columns = CRUD + List + Search (drop columns that don't apply per
  entity).
- Cells = `✓` (endpoint exists) / `-` (intentionally absent).
- Admin-only actions marked `admin✓`.

User ackowledges the table (no click — implicit); Phase 3 uses it as input.

## Verify-criterion

- `RESOURCES` has ≥1 entry; parsed shape `{name, owns, many_to_many}` valid.
- `SHAPE` is compatible with `STYLE` (gate above).
- Resource-to-action matrix printed with every entity as a row.
- Non-trivial m2m relations surfaced as explicit join entities OR
  GraphQL edge types — no implicit joins hidden in handlers.
