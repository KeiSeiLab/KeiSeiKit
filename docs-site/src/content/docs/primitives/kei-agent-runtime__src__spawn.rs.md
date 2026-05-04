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
