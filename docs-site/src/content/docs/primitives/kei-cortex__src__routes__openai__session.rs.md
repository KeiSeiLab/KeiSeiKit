---
title: session.rs
path: kei-cortex/src/routes/openai/session.rs
dna_hash: sha256:705df3d75b9e743c
language: rust
size_loc: 133
generated: by-keidocs
---

# kei-cortex/src/routes/openai/session.rs

Per-process session continuity store for the OpenAI surface.

Two shapes share this module:
* `X-Kei-Session-Id` (chat-completions) — opt-in continuity for an
otherwise stateless endpoint. Caller passes the same id on each
turn to retain message history.
* `previous_response_id` (responses) — required-by-spec stateful
continuity for the Responses API.

Both keys point at the same `SessionRecord` shape because the agent
loop doesn't care which surface populated the history.

Storage is in-memory only (`DashMap`). Persistence-across-restart
would mean wiring `kei-ledger`; deferred to Phase 1.2 per
`HERMES-MIGRATION-PLAN.md`.

## Public API

- One session = one ordered list of messages + last-response snapshot.
- Process-wide session map. Cheap to clone (Arc inside).
- `pub fn global` — Process-singleton accessor. Each call returns a clone of the same
- `pub fn get` — Look up a session by id; returns `None` if absent.
- `pub fn put` — Replace (or insert) a session.
- `pub fn delete` — Remove a session, returning the prior value if any.
- `pub fn append` — Append messages to an existing session, creating it if needed.
- Number of stored sessions (test helper).
- `pub fn new_response_skeleton` — Build a fresh `ResponseObject` skeleton. Caller fills `output` and

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: dashmap, once_cell, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
