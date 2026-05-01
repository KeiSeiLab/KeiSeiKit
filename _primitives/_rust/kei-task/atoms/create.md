---
atom: kei-task::create
kind: command
version: "0.1.0"

input:
  schema: schemas/create-input.json
  required: [title]
  example: { title: "Fix auth bug", priority: "high" }

output:
  schema: schemas/create-output.json
  example: { id: 42, created_at: 1714000000 }

errors:
  - code: InvalidPriority
    http_analog: 400
    description: "Priority must be one of: critical, high, medium, low"
  - code: InvalidTitle
    http_analog: 400
    description: "Title must be non-empty"
  - code: StoreError
    http_analog: 500
    description: "Underlying SQLite store failed to insert the task"

side_effects:
  - { op: write, domain: kei-task-db }
idempotent: false
timeout_ms: 5000

deprecated: null
stability: stable

keywords: [task, todo, create, dag, planning]
related:
  - "[[kei-task::add-dependency]]"
  - "[[kei-task::search]]"
---

# kei-task::create

Creates a new task row in the kei-task SQLite DAG. Returns the inserted
row id and the `created_at` unix timestamp. Also indexes title +
description into the FTS table used by `kei-task::search`.

## Example

    kei-task create "Fix auth bug" --priority high --description "Token rotation fails on leap second"

Returns the new task id on stdout:

    42

Programmatic callers (runtime invocation) receive:

    { "id": 42, "created_at": 1714000000 }

## Gotchas

- `priority` defaults to `"medium"` if omitted. Case sensitive —
  `High` returns `InvalidPriority`.
- `description` defaults to empty string; blank descriptions are still
  indexed by FTS but yield no search hits until populated.
- `milestone_id` in the input schema is reserved for future use; the
  current CLI does NOT accept it — link via `kei-task link-milestone`
  after creation.
- Title uniqueness is NOT enforced at DB level; duplicate titles are
  allowed and will all be returned by `kei-task::search`.

## Related

- [[kei-task::add-dependency]] — wire the new task into the DAG
- [[kei-task::search]] — look up by title / description
