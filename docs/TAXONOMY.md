# TAXONOMY — Canonical Facet Vocabulary

> Graph, not tree. Every primitive is a node; facets are orthogonal labels.
> Multi-faceted nodes are allowed (and expected). No facet is mandatory —
> the entire `[taxonomy]` and `[lineage]` sections are OPTIONAL on every
> manifest shape (`capability.toml`, `_manifests/**/*.toml`, `_roles/*.toml`,
> atom markdown frontmatter).

---

## Why facets, not a tree

A classical rooted tree (e.g. "capability → gate → policy → no-git-ops")
forces an arbitrary primary axis. Real primitives live in several axes at
once: `no-git-ops` is a *capability* (kingdom), a *gate* (mechanism), a
*policy* (domain), targets the *agent-substrate* (layer), is *stable*, and
ships as a *rust* module. A tree makes five of those six second-class.

Facets let a catalog query along any axis independently:

- "all `gate` mechanisms" — security review surface
- "all `verify` mechanisms" — quality/CI surface
- "all `policy`-domain primitives" — rule-coverage surface
- "all `experimental` stability" — risk review
- "all `rust` language" — build-graph

No primitive needs to choose a primary axis. Multiple facets coexist.

---

## Facets

### `kingdom` — What kind of thing is this?

```
kingdom = capability | atom | skill | block | runtime | schema | role | manifest
```

- `capability` — agent-substrate capability (gate / verify / transform)
- `atom` — substrate atom (command / query / stream / transform)
- `skill` — user-invocable skill (`/skill-name`)
- `block` — composable prompt-block
- `runtime` — runtime module consuming atoms/capabilities
- `schema` — JSON schema referenced by atom I/O
- `role` — agent-role manifest (`_roles/*.toml`)
- `manifest` — assembled agent manifest (`_manifests/**/*.toml`)

### `mechanism` — How does it act?

```
mechanism = gate | verify | transform | store | compose | fetch | analyze | router | cache
```

- `gate` — PreToolUse-style deny decision (e.g. `no-git-ops`, `bash-allowlist`)
- `verify` — post-condition check (e.g. `cargo-check-green`)
- `transform` — pure value-in/value-out (no side-effects)
- `store` — persisted state (SQLite, filesystem, ledger)
- `compose` — assembles other primitives (manifests, pipes)
- `fetch` — retrieves external data (provider, api)
- `analyze` — inspects input, emits report
- `router` — dispatches based on classification
- `cache` — memoizes pure invocations

### `domain` — What subject-matter area?

```
domain = policy | quality | scope | safety | output | tools | research | content | social | task | sage
```

- `policy` — RULE 0.x enforcement / compliance gates
- `quality` — cargo-check, tests-green, constructor-pattern
- `scope` — write-whitelist, file-denylist, path-guards
- `safety` — secret scanning, citation verification
- `output` — response shape, formatter, report-gen
- `tools` — tool allowlists, bash patterns, deny-tools
- `research` — research agents, search-core, fetch primitives
- `content` — content-store, content-normalizer
- `social` — social-store, social-normalizer
- `task` — task primitives (kei-task)
- `sage` — higher-level reasoning / kei-sage primitives

### `layer` — Which substrate does it live in?

```
layer = atom-substrate | agent-substrate | cross | tooling
```

- `atom-substrate` — substrate for callable atoms (kei-runtime, kei-pipe)
- `agent-substrate` — substrate for agent manifests (capabilities, roles)
- `cross` — spans both (shared discovery, schemas)
- `tooling` — pure developer tooling (kei-forge, validators)

### `stage` — When is it active?

```
stage = runtime | design-time | ephemeral
```

- `runtime` — executes during agent turns
- `design-time` — consumed at assembly / scaffold time
- `ephemeral` — one-shot (migration, provision, smoke)

### `stability` — Maturity

```
stability = experimental | beta | stable | deprecated
```

Standard semver-style ladder. `deprecated` primitives must name a successor
in `[lineage]` or their `text.md`.

### `language` — Implementation medium

```
language = rust | shell | md | toml | json | jsonschema
```

- `rust` — primary implementation in a Rust crate
- `shell` — bash / posix script
- `md` — markdown (atoms, capability text, documentation)
- `toml` — config-only (capability manifest, role manifest)
- `json` / `jsonschema` — data / schema definitions

Multiple languages can apply (e.g. atom markdown with a JSON schema attached
and a Rust runtime) — but the `language` facet names the PRIMARY medium of
the node being described.

---

## `[lineage]` — Graph edges, not tree edges

```
parents = ["[[ancestor-one]]", "[[ancestor-two]]"]   # wikilinks to predecessors
creator = "ag-orchestrator-human"                    # DNA id or human slug
created = "2026-04-23"                               # ISO-8601 date
fork_from = "dna-abc123..."                          # parent DNA if forked
```

- `parents` — wikilinks (`[[slug]]`) to primitives this one extends or
  composes. Multiple parents allowed (diamond lineage). A primitive with
  no `parents` is a root of its sub-graph.
- `creator` — identity responsible for the primitive's existence. For
  human-authored nodes: `ag-orchestrator-human` or a slug. For agent-
  authored: the agent's DNA id.
- `created` — ISO-8601 date (YYYY-MM-DD). When the manifest was first
  authored, not when it was last edited.
- `fork_from` — if this primitive was forked from another (DNA id), record
  the source here so the graph shows the edge.

---

## Example — fully-faceted capability manifest

```toml
[capability]
name = "policy::no-git-ops"
category = "policy"
version = "1.0"
description = "..."
rationale = "..."

[restricts]
tool-patterns = ['^git( |$)', '^gh repo']

[parameterized]
accepts = []

[text]
path = "text.md"

[gate]
rust-module = "gates::policy_no_git_ops"
event = "PreToolUse:Bash"
severity = "block"

# Optional — all fields optional individually too.
[taxonomy]
kingdom = "capability"
mechanism = "gate"
domain = "policy"
layer = "agent-substrate"
stability = "stable"
language = "rust"

[lineage]
parents = []
creator = "ag-orchestrator-human"
created = "2026-04-23"
```

---

## Non-breaking contract

- Every field in `[taxonomy]` and `[lineage]` is OPTIONAL.
- The entire `[taxonomy]` and `[lineage]` sections are OPTIONAL.
- Manifests without either section parse exactly as before (backward-compat
  guaranteed by `taxonomy_smoke.rs` tests in `kei-atom-discovery`).
- New primitives SHOULD include at least `kingdom` + `mechanism` + `domain`.
- The facet vocabularies are additive — new values can be appended without
  breaking existing consumers. Unknown values pass through as strings.

---

## Rule lock

2026-04-23. Vocabularies live in this file; any new allowed value lands here
first. Runtime consumers (kei-atom-discovery, kei-sage, kei-runtime) MUST
treat unknown values as strings (never crash on new vocabulary).
