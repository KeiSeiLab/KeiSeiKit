---
title: ledger_reader.rs
path: kei-export-trajectories/src/ledger_reader.rs
dna_hash: sha256:0445e8b65fcb8f6b
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-export-trajectories/src/ledger_reader.rs

Read agent rows from `kei-ledger.sqlite`, optionally enrich with tool
events from `kei-memory.sqlite`, and read chatlog text from
`.claude/agents/<id>/chatlog.md`.

Constructor Pattern: ledger query + chatlog file read here; memory
event queries live in the `memory_events` sibling cube. Each helper
<30 LOC; missing inputs degrade gracefully (missing chatlog = empty
string; missing memory DB = zero tool events).

HERMES P0.2 deviation note: the spec said "extract tool events from
agent_runs.events JSON" — but the ledger schema has no events column.
Tool events live in the SIBLING `kei-memory.sqlite` `events` table
(rusqlite ATTACH avoided to keep the two stores independently
migratable). We open them as separate connections.

## Public API

- One agent's complete trajectory record, ready to convert to ShareGPT.
- `pub fn completed` — `completed` per Hermes spec: terminal-state-with-success.
- Reads a single ledger DB; optionally cross-references a memory DB and
- `pub fn new` — Construct with explicit ledger path. Memory + repo are optional —
- `pub fn read_since` — Materialize every agent in the ledger that has `started_ts >=
- `pub fn count_since` — Just count rows >= from_ts; fast path for the `count` subcommand.
- Internal raw row from the ledger — only the columns we need.
- Read `<repo>/.claude/agents/<id>/<filename>` if present; empty string

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: anyhow, crate, rusqlite, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
