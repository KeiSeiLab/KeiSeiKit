---
atom: kei-task::search
kind: query
version: "0.1.0"

input:
  schema: schemas/search-input.json
  required: [query]
  example: { query: "refactor", limit: 20 }

output:
  schema: schemas/search-output.json
  example:
    results:
      - { id: 42, title: "refactor router", status: "pending", priority: "high" }

errors:
  - code: InvalidQuery
    http_analog: 400
    description: "FTS5 rejected the query syntax"
  - code: StoreError
    http_analog: 500
    description: "Underlying SQLite store failed during read"

side_effects: []
idempotent: true
timeout_ms: 5000

deprecated: null
stability: stable

keywords: [task, search, fts, lookup, query]
related:
  - "[[kei-task::create]]"
  - "[[kei-task::add-dependency]]"
---

# kei-task::search

Runs a FTS5 full-text search over task titles + descriptions and
returns matches ordered by `rank` (FTS5 BM25 relevance).

## Example

    kei-task search "refactor" --limit 10

Tab-separated on stdout, one row per hit:

    42	pending	refactor router
    57	in_progress	refactor auth layer

Programmatic callers receive a typed array:

    {
      "results": [
        { "id": 42, "title": "refactor router", "status": "pending", ... }
      ]
    }

## Gotchas

- `limit` defaults to 20 and is clamped to a positive integer — pass
  `0` or negative and the implementation silently uses 20.
- Query uses FTS5 syntax — phrase search needs double quotes inside
  the query string (shell escape required).
- Returned rows always include the full `Task` shape; callers that
  only need `id` should project client-side.
- Results are ordered by FTS rank, NOT by `created_at` — recent tasks
  may be returned in the middle of the result set.

## Related

- [[kei-task::create]] — tasks only appear here once created
- [[kei-task::add-dependency]] — traverse from a search hit into the DAG
