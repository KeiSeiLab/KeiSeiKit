---
atom: kei-task::add-dependency
kind: command
version: "0.1.0"

input:
  schema: schemas/add-dependency-input.json
  required: [from, to]
  example: { from: 42, to: 17, dep_type: "blocks" }

output:
  schema: schemas/add-dependency-output.json
  example: { ok: true }

errors:
  - code: SelfDependency
    http_analog: 400
    description: "A task cannot depend on itself"
  - code: InvalidDepType
    http_analog: 400
    description: "dep_type must be one of: blocks, feeds_into, subtask_of, milestone_of, assigned_to, depends_on"
  - code: CycleDetected
    http_analog: 409
    description: "The new edge would close a cycle in the task DAG"
  - code: StoreError
    http_analog: 500
    description: "Underlying SQLite store failed to insert the dependency row"

side_effects:
  - { op: write, domain: kei-task-db }
idempotent: false
timeout_ms: 5000

deprecated: null
stability: stable

keywords: [task, dependency, dag, blocks, graph]
related:
  - "[[kei-task::create]]"
---

# kei-task::add-dependency

Inserts a typed edge `from -> to` in the task DAG, rejecting cycles
and self-loops at write time. Edge is stored idempotently via
`INSERT OR IGNORE` — re-adding the same triple is a no-op.

## Example

    kei-task add-dependency 42 17 --dep-type blocks

Stdout:

    dep: 42 -> 17 (blocks)

Programmatic callers receive:

    { "ok": true }

## Gotchas

- `dep_type` defaults to `"blocks"`. Empty string is also treated as
  `"blocks"` for CLI convenience.
- Cycle check is transitive — the implementation walks the existing
  DAG from `to` and refuses if it can reach `from`.
- Both task ids must already exist; missing ids do NOT surface a
  dedicated error code in the current impl and bubble up as
  `StoreError` via foreign-key violation.
- Re-adding an existing edge is silently idempotent (`INSERT OR
  IGNORE`) even though the atom declares `idempotent: false` — that
  flag reflects the CONTRACT (callers should not rely on retry
  semantics), not the specific SQL behaviour.

## Related

- [[kei-task::create]] — create endpoints of the edge first
