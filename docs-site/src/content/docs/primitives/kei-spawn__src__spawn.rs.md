---
title: spawn.rs
path: kei-spawn/src/spawn.rs
dna_hash: sha256:ac2fcd589ebb856c
language: rust
size_loc: 155
generated: by-keidocs
---

# kei-spawn/src/spawn.rs

spawn — orchestrator-driven task → prepared agent + ledger row.

One public entry point: `spawn_from_task`. Given a task.toml and a
kit_root, it:
1. Parses task.toml via `kei_agent_runtime::spawn::load_task`
2. Composes `AgentInvocation` via `kei_agent_runtime::prepare::prepare`
(auto-generates agent-id if absent)
3. Copies the resolved agent-id back into the task and writes
`tasks/<agent-id>/{prompt.md, task.toml}` via
`kei_agent_runtime::spawn::prepare_agent`
4. Computes spec_sha (SHA-256 of the task TOML content)
5. Registers a running row in the ledger via `kei-ledger fork`
6. Returns `SpawnOutput` — everything orchestrator needs to call
Claude Code's `Agent` tool (serialised as JSON).

Never invokes git. Never invokes the Agent tool. Per RULE 0.13.

## Public API

- The bundle orchestrator hands to Claude Code's Agent tool.
- `pub fn spawn_from_task` — Main spawn entry. See module doc for the 6-step pipeline.
- Call `kei-ledger fork`; on failure, remove the prepared task dir so
- `pub fn spawn_with_pipeline` — Variant that additionally derives the downstream handoff chain from the

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: anyhow, crate, kei_agent_runtime, serde, sha2, std

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
