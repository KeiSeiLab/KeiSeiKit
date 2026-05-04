---
title: main.rs
path: kei-agent-runtime/src/main.rs
dna_hash: sha256:331015e67e89e1c3
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-agent-runtime/src/main.rs

kei-agent-runtime — CLI dispatcher for compose | spawn | verify | run.

## Public API

- Compose prompt from a task.toml and write tasks/<agent-id>/prompt.md.
- Prepare spawn dir (tasks/<agent-id>/) — orchestrator invokes Agent tool.
- Run every verify capability declared by the task's role.
- One-shot helper: compose + spawn + verify (tests only).
- Assemble everything orchestrator needs to invoke Agent tool.
- Output format: human (default) | json | toml

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: clap, kei_agent_runtime, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
