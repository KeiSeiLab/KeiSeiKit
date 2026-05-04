---
title: memory_events.rs
path: kei-export-trajectories/src/memory_events.rs
dna_hash: sha256:ebc57fe2e985701c
language: rust
size_loc: 78
generated: by-keidocs
---

# kei-export-trajectories/src/memory_events.rs

Tool-event extraction from `kei-memory.sqlite`.

Constructor Pattern: SQL-only cube. The ledger_reader hands us a
connection + an agent_id + a time window; we hand back a
`Vec<ToolEvent>`. Two queries: session-keyed first (cheap, indexed),
ts-windowed fallback for sessions that pre-date the agent_id ↔
session_id linkage.

## Public API

- `pub fn query_tool_events` — Pull tool events from `kei-memory.events` matching this agent. We try

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: anyhow, crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
