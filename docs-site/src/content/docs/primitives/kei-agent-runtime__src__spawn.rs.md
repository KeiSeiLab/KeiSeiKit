---
title: spawn.rs
path: kei-agent-runtime/src/spawn.rs
dna_hash: sha256:46f74151fe454ace
language: rust
size_loc: 69
generated: by-keidocs
---

# kei-agent-runtime/src/spawn.rs

Prepare an agent invocation: write `tasks/<agent-id>/prompt.md`,
record the task.toml alongside it. Actual Claude `Agent` tool call is
the orchestrator's job per RULE 0.13.

## Public API

- `pub fn load_task` — Parse a task.toml file into `TaskSpec`.
- `pub fn prepare_agent` — Prepare a spawnable agent directory.
- Outcome of `prepare_agent`.
- `pub fn resolve_agent_id` — Resolve the effective `agent_id` — validator-checked, never creates

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
