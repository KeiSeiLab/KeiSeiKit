# KeiSeiKit Substrate Schema v1

**STATUS:** Revised after user review (2026-04-22). Open questions resolved inline in §"Decision log" at bottom. Once `SCHEMA-LOCKED.md` marker is committed, this document is **LOCKED** for 6 weeks of parallel stream work (RULE: breaking changes require explicit user revocation + all-streams sync).

**PURPOSE:** Single Source of Truth for the atom / capability / graph schema that enables the substrate composition layer. Four parallel work streams (UI / Atoms refactor / Graph / Runtime) all depend on this contract.

---

## Core concept: atom = one verb

An **atom** is **one verb** (one operation) on a primitive, not one crate. Example: `kei-task` crate decomposes into `kei-task::create`, `kei-task::add-dependency`, `kei-task::search`, … Each atom is independently:

- Documented (one `.md` file)
- Schema-specified (JSON Schema for input + output)
- Callable (one Rust function)
- Discoverable (aggregated into `capabilities.toml`)
- Composable (runtime pipes atoms by schema compatibility)

**Granularity target:** ~150 atoms across the current 47 crates (was 25 at v0.22 lock; 22 crates added v0.23–v0.33). Crate = physical container; atom = unit of composition.

---

## File layout per crate

```
_primitives/_rust/<crate>/
├── Cargo.toml                   ← includes [package.metadata.keisei] for
│                                  crate-level substrate data (see §Cargo
│                                  metadata below)
├── src/
│   ├── main.rs                 ← CLI dispatcher — parses argv, calls atom fn
│   ├── atoms/
│   │   ├── mod.rs
│   │   ├── create.rs           ← one file per atom impl, pub fn run(input: ...) -> ...
│   │   ├── add_dependency.rs
│   │   └── search.rs
│   └── schema.rs               ← Rust types that match JSON Schemas
├── atoms/                       ← SSoT for atoms — docs + machine-parseable frontmatter
│   ├── create.md
│   ├── add-dependency.md
│   ├── search.md
│   └── schemas/
│       ├── create-input.json        ← JSON Schema draft-07
│       ├── create-output.json
│       ├── add-dependency-input.json
│       └── …
└── migrations/                  ← per-crate SQLite migrations (kei-migrate)
    └── 0001_initial.sql
```

**Why split `src/atoms/` and `atoms/`:** code lives with code (Rust convention), docs live in a flat directory easy for kei-sage to walk and for humans to scan.

**No `capabilities.toml` aggregator.** Per user review (2026-04-22): aggregated files cause drift vs source truth. `atoms/*.md` is the ONLY atom source. `kei-sage` walks `.md` files directly; `kei-runtime list-atoms` walks filesystem on demand. Crate-level metadata (db backend, env vars, migrations dir) lives in `Cargo.toml [package.metadata.keisei]` — already a first-class Cargo mechanism.

---

## Atom `.md` frontmatter schema

Every `atoms/<verb>.md` file MUST begin with YAML frontmatter matching this shape:

```yaml
---
# REQUIRED
atom: kei-task::create              # <crate>::<verb> — globally unique ID
kind: command                       # command | query | stream | transform
version: "0.22.3"                   # inherits crate Cargo.toml version

# INPUT / OUTPUT — schemas live in atoms/schemas/ (relative paths)
input:
  schema: schemas/create-input.json
  required: [title]                 # convenience duplication from JSON Schema for CLI help
  example: { title: "Fix auth bug", priority: "high" }

output:
  schema: schemas/create-output.json
  example: { id: 42, created_at: "2026-04-22T15:30:00Z" }

# ERRORS — typed, documented upfront
errors:
  - code: DuplicateTitle
    http_analog: 409
    description: "A task with this title already exists under the same milestone"
  - code: InvalidPriority
    http_analog: 400
    description: "Priority must be one of: low, medium, high"

# SUBSTRATE HINTS — runtime uses these for DAG composition safety
side_effects:                        # [] means pure/readonly
  - { op: write, domain: kei-task-db }   # structured — type-safe, extensible
  - { op: read,  domain: fs }
  # op: read | write | network | subprocess | other
  # domain: free-form, conventionally <crate-name>-db for DB / fs / <api-name>
idempotent: false                    # safe to retry? affects runtime retry logic
timeout_ms: 5000                     # default timeout; runtime enforces

# LIFECYCLE
deprecated: null                     # or: "use kei-task::create-v2 — stricter validation"
stability: stable                    # experimental | beta | stable | deprecated

# DISCOVERY
keywords: [task, todo, gtd, planning]
related:                             # wikilinks rendered by kei-sage
  - "[[kei-task::add-dependency]]"
  - "[[kei-milestone::link]]"
---
```

### Body (Markdown, free-form)

After frontmatter, the body is **human-facing** with fixed section conventions:

```markdown
# kei-task::create

Creates a new task in the DAG. Title must be unique within its milestone scope.

## Example

    kei-task create \
      --title "Fix auth bug" \
      --priority high \
      --description "Token rotation fails on leap second"

Returns JSON: `{"id": 42, "created_at": "2026-04-22T..."}`

## Gotchas

- Title uniqueness is per-milestone, NOT global. Two tasks `"Fix bug"` in
  different milestones is valid.
- `priority` is case-sensitive — `High` returns `InvalidPriority`.

## Related
- [[kei-task::add-dependency]] — link this task into DAG as parent/child
- [[kei-milestone::link]] — group this task under a milestone
- [[rules/RULE 0.12]] — task DAG per Agent Git Model
```

Sections `# <atom-id>`, `## Example`, `## Gotchas`, `## Related` are **convention, not requirement** — but recommended for uniformity so kei-sage can extract sections predictably.

---

## Crate-level metadata — `Cargo.toml [package.metadata.keisei]`

Crate-level data (db backend, env vars, migrations) lives in a Cargo-native `[package.metadata.*]` section. Cargo reserves `[package.metadata.*]` explicitly for tool-specific extensions — no spec violation, no third-party file.

```toml
# _primitives/_rust/kei-task/Cargo.toml

[package]
name = "kei-task"
version = "0.22.3"
description = "SQLite-backed task DAG with dependencies, milestones, FTS search"
# … rest of Cargo.toml unchanged

[package.metadata.keisei]
# Substrate declares crate-level state — atoms themselves are in atoms/*.md
backend = "sqlite"                           # sqlite | filesystem | memory | remote
db_env = "KEI_TASK_DB"
db_default = "~/.claude/task/task.sqlite"
migrations_dir = "migrations/"
schema_version = 3
```

Atoms are discovered by walking `atoms/*.md` and parsing frontmatter. No aggregator file, no build.rs regeneration, no drift.

**Discovery:**

```bash
# Runtime lists atoms — walks filesystem on demand (~ms for 150 atoms)
kei-runtime list-atoms [--crate kei-task] [--kind command]
# → reads atoms/*.md frontmatter across ~/.claude/agents/_primitives/_rust/*/

# Sage indexes atoms — walks on install + inotify rebuild on change
kei-sage rank-atoms
# → same corpus, persisted to ~/.claude/sage/vault.sqlite for FTS + PageRank
```

**Validation**: `kei-schema-lint` (new tool in Runtime stream) validates:
1. Every `atoms/*.md` has valid frontmatter matching the schema above
2. Every `schema` path in frontmatter points to an existing JSON Schema file
3. Every `[[related]]` wikilink target exists (atom or rule)
4. `Cargo.toml [package.metadata.keisei]` has required fields

Runs in CI per-crate + globally across all installed primitives.

---

## JSON Schema conventions (input / output)

- **Draft:** JSON Schema **draft-07** (widely supported, `jsonschema` + `schemars` Rust crates).
- **File naming:** `<verb>-input.json`, `<verb>-output.json`.
- **Shared types:** put under `atoms/schemas/_shared/<Type>.json`, reference via `$ref`.
- **Examples:** every schema MUST have `examples: [...]` (used by kei-forge live preview + runtime smoke tests).

Minimal example — `atoms/schemas/create-input.json`:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "kei-task/atoms/schemas/create-input.json",
  "title": "kei-task::create input",
  "type": "object",
  "required": ["title"],
  "properties": {
    "title": { "type": "string", "minLength": 1, "maxLength": 200 },
    "priority": { "type": "string", "enum": ["low", "medium", "high"] },
    "description": { "type": "string" },
    "milestone_id": { "type": "integer", "minimum": 1 }
  },
  "additionalProperties": false,
  "examples": [
    { "title": "Fix auth bug", "priority": "high" }
  ]
}
```

---

## Atom kinds (the 4 allowed values)

| kind | Meaning | Pipe safety |
|---|---|---|
| `command` | Mutates state (write DB, send request) | Sequential only; runtime rejects parallel if overlapping `side_effects` |
| `query` | Read-only (FTS, lookup) | Parallel-safe |
| `stream` | Emits a sequence over time (SSE, file tail) | Single consumer per invocation |
| `transform` | Pure function (input → output, no state) | Parallel-safe, cacheable |

**Runtime uses `kind` + `side_effects` + `idempotent`** to decide:
- Can this atom be retried on failure? (needs `idempotent: true` OR `kind=query|transform`)
- Can this atom be parallelized with another? (non-overlapping `side_effects` + both commands OR at least one `query|transform`)
- Should output be cached? (`transform` with same input = deterministic)

---

## Naming conventions

| Thing | Convention | Example |
|---|---|---|
| Crate name | `kei-<noun>` kebab-case | `kei-task` |
| Atom verb | lowercase, kebab-case, single word if possible | `create`, `add-dependency`, `search` |
| Full atom ID | `<crate>::<verb>` | `kei-task::add-dependency` |
| Side-effect domain | `<op>:<domain>` | `write:kei-task-db`, `read:fs`, `network:anthropic-api` |
| Error code | PascalCase | `DuplicateTitle`, `InvalidPriority` |
| JSON Schema file | `<verb>-{input,output}.json` | `create-input.json` |

---

## Versioning & deprecation

- **Atoms inherit crate SemVer.** `kei-task::create` version = `kei-task` Cargo.toml version.
- **Breaking change to an atom** (signature change, required field added, error semantics shifted) = **new atom** with suffix: `create-v2`. Old atom gets `deprecated: "use kei-task::create-v2"` frontmatter.
- **Deprecated atoms** stay functional for ≥ 2 minor versions, then removed.
- **Non-breaking changes** (new optional input field, new output field, new error code): bump patch version, no rename.

---

## Runtime invocation contract

The Runtime stream implements `kei-runtime` that exposes:

```bash
# Invoke one atom
kei-runtime invoke kei-task::create --input '{"title":"Fix bug"}'
# → stdout: {"result": {...}, "metadata": {"duration_ms": 12, "atom": "kei-task::create"}}
# → exit 0 on success, 2 on atom error (see frontmatter errors[]), 1 on usage/IO

# Invoke a DAG
kei-runtime pipe dag.toml
# dag.toml declares:
#   [[steps]]
#   atom = "kei-task::create"
#   input = { title = "X" }
#   capture_as = "task"
#
#   [[steps]]
#   atom = "kei-task::add-dependency"
#   input = { parent = "$task.id", child = 17 }

# Discover what's installed
kei-runtime list-atoms [--kind command|query|…] [--crate kei-task]
```

**Runtime validates at invocation:** input against `input_schema`, output against `output_schema`. Mismatch = exit 2 with schema-violation error.

**Runtime records to `kei-ledger`:** every invocation emits a ledger row (atom-id, spec-sha, input-sha, duration, exit, errors). Same RULE 0.12 lifecycle as agent forks.

---

## Graph / discovery contract

The Graph stream (kei-sage as substrate) exposes:

```bash
kei-sage rank-atoms                         # PageRank over [[atom-id]] wikilinks
kei-sage related kei-task::create           # BFS from atom
kei-sage search "task create"               # FTS over atom bodies + frontmatter
kei-sage graph kei-task::create --depth=2   # GraphML export
```

`kei-sage` auto-imports on install:
1. Walks `~/.claude/agents/_primitives/_rust/*/atoms/*.md`
2. Parses frontmatter + body
3. Resolves `[[atom-id]]` wikilinks to atom nodes
4. Resolves `[[rules/RULE 0.X]]` wikilinks to rule file nodes
5. Re-indexes on file modification (inotify / fsevents)

---

## UI (kei-forge) contract

The UI stream generates new atoms via web wizard (`keisei forge`):

**Inputs from user (form):**
- Crate (existing or new)
- Atom verb name (kebab-case)
- Kind (command / query / stream / transform)
- Input fields (JSON Schema builder UI)
- Output fields
- Error codes
- Side effects

**Outputs (generated on submit):**
- `atoms/<verb>.md` with frontmatter + skeleton body
- `atoms/schemas/<verb>-input.json` + `<verb>-output.json`
- `src/atoms/<verb>.rs` with `pub fn run(input: …) -> Result<Output, Error>` skeleton
- Test file `tests/<verb>_smoke.rs`
- Regenerated `capabilities.toml`

**Postcondition:** `cargo check` passes, `kei-schema-lint` passes, new atom visible to `kei-runtime list-atoms`.

---

## Stream interfaces (the 4 contracts)

Here is exactly what each parallel stream can assume from this schema:

### Stream A — UI (kei-forge)
- **Reads:** this schema doc, JSON Schema draft-07, existing `atoms/*.md` as templates
- **Writes:** generates new `.md` + `.json` + `.rs` per above contract
- **Does NOT depend on:** Atoms-refactor (can work against any single atom template), Graph (independent), Runtime (independent)

### Stream B — Atoms refactor
- **Reads:** current 47 crates (25 at v0.22 lock; 22 added v0.23–v0.33)
- **Writes:** `atoms/<verb>.md` + `atoms/schemas/*.json` + splits `src/main.rs` → `src/atoms/*.rs`, adds `[package.metadata.keisei]` to each `Cargo.toml`
- **Does NOT depend on:** UI (can progress independently), Graph, Runtime. No build.rs, no generated files — atoms/*.md is SSoT.

### Stream C — Graph (kei-sage substrate)
- **Reads:** `~/.claude/agents/_primitives/_rust/*/atoms/*.md` (real or test fixtures)
- **Writes:** extends `kei-sage` to auto-walk the atom corpus, resolves `[[atom-id]]` wikilinks, exposes rank/related/search/graph over atoms
- **Does NOT depend on:** UI; depends on Atoms stream ONLY for real test corpus (can ship against fixture .md files if Atoms not done)

### Stream D — Runtime (kei-runtime, NEW crate)
- **Reads:** `atoms/*.md` frontmatter + JSON Schema files + `Cargo.toml [package.metadata.keisei]`
- **Writes:** new crate `_primitives/_rust/kei-runtime/` with `invoke`, `pipe`, `list-atoms`, `kei-schema-lint`
- **Does NOT depend on:** UI, Graph. Depends on Atoms stream ONLY for real atoms (can ship against hand-crafted test atom for initial dev)

---

## What this schema deliberately leaves open

Things NOT specified here — intentionally left for streams to decide:

1. **Exact YAML library** (serde_yaml vs yaml-rust vs …) — Rust convention choice
2. **Build.rs mechanics** for capabilities.toml generation — implementation detail
3. **Web UI framework** for kei-forge (HTMX / Leptos / Yew) — Stream A's call
4. **Runtime concurrency model** (async tokio / sync threads / subprocess) — Stream D's call
5. **kei-sage GraphML vs Mermaid vs DOT** output format — Stream C's call
6. **Atom test harness** shape — streams B + D coordinate

---

## Schema lock declaration

Once this document is approved by the user and a `SCHEMA-LOCKED.md` marker is committed, the schema is **immutable for 6 weeks** of parallel work. Breaking changes during lock period require:

1. Explicit revocation by user
2. All 4 stream agents paused + sync commit rebasing all streams to new schema
3. `kei-ledger` entry: reason + revocation timestamp

Non-breaking additions (new optional fields, new atom kinds, new side-effect domains) are allowed during lock with standard git flow.

## Decision log — resolved 2026-04-22

| # | Question | Decision | Rationale |
|---|---|---|---|
| 1 | JSON Schema draft-07 vs 2020-12 | **draft-07** | Stable, every Rust crate supports. Migration later = sed + bump validator lib, not catastrophic. |
| 2 | Atom ID separator `::` vs `/` | **`::`** | Rust-native (`std::fs::read`). Cost: quoting in shell (`"kei-task::create"`). Accepted. |
| 3 | `side_effects` string vs structured object | **structured `{ op, domain }`** | Type-safe, adds 3rd field later without migration. "С запасом." |
| 4 | `capabilities.toml` committed vs gitignored | **DROP entirely** | Aggregator = drift risk + double maintenance. SSoT is `atoms/*.md`. Crate-level metadata moves to `Cargo.toml [package.metadata.keisei]` (Cargo-native mechanism). kei-sage + kei-runtime walk filesystem directly. |
| 5 | `kei-atom-template/` in this PR or defer to Stream A | **Include in this PR** | Template + `scripts/new-atom.sh` ships together with schema. Streams B/C/D can test-drive atom creation from day 0 without waiting for UI. UI (Stream A) wraps the same template in web wizard. |
| 6 | Error model per-atom vs shared registry | **Per-atom** | Simpler to start. Registry can be added later non-breakingly. |

**Locked values:** all of the above. Breaking changes to any of these during 6-week parallel window require explicit user revocation + all-streams sync + ledger row.

## Amendments — non-breaking clarifications

| # | Date | Clarification | Reason |
|---|---|---|---|
| A-1 | 2026-04-23 | **`input.schema` and `output.schema` are REQUIRED for all atom kinds** (`command` / `query` / `stream` / `transform`). An atom with no inputs should declare `input.schema` pointing to a JSON Schema with `{"type": "object", "properties": {}, "additionalProperties": false}` — i.e., "empty object". Similarly for no-output. The runtime + graph lint BOTH enforce presence of the schema ref; shared `kei-atom-discovery` parses them as `Option<PathBuf>` only to allow tolerant skip-on-missing (with stderr warning) rather than aborting the whole scan on one bad atom. | Architect P0-a (post-audit 2026-04-23) — Stream C parsed input/output Optional, Stream D required. Asymmetric enforcement → "sage sees atom, runtime skips" drift. Both streams now agree: Optional at parse layer, required at lint layer. |

These amendments document interpretations consistent with the locked schema — no frontmatter-shape change, no wire-format change, no stream refactor required.
