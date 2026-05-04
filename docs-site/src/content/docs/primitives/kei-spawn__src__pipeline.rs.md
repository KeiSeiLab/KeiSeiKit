---
title: pipeline.rs
path: kei-spawn/src/pipeline.rs
dna_hash: sha256:51c150ffc4c918f0
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-spawn/src/pipeline.rs

pipeline — derive downstream handoff chain from a writer's role.

When a spawn is invoked with `--pipeline`, the orchestrator wants to know:
1. Which downstream roles does the writer's role declare in
`[pipeline].handoff`? (e.g. `edit-local` → `["auditor"]`)
2. What agent-ids should those downstream steps use?
3. Where should the pipeline.json chain artefact be written?
4. What skeleton task.toml should be scaffolded for each step?

This module answers all four. It reads the writer's role file, parses
the optional `[pipeline]` section, and emits a `PipelineChain` the
caller can serialise + use to pre-create per-step task directories.

Constructor Pattern: one module = one responsibility (pipeline derivation
only). No git, no shell, no ledger. Pure filesystem + TOML parsing.
No I/O beyond `std::fs::read_to_string` for role lookup and
`std::fs::write` / `create_dir_all` for scaffolding.

## Public API

- One step in a downstream handoff chain.
- Ordered chain of handoff steps derived from a writer's role.
- Raw on-disk shape of `_roles/<name>.toml`'s `[pipeline]` section.
- `pub fn pipeline_from_role` — Read `_roles/<role>.toml` and return its `[pipeline].handoff` list.
- `pub fn derive_steps` — Derive concrete `PipelineStep`s from a writer agent-id + handoff role
- `pub fn emit_pipeline_json` — Serialise `chain` as pretty JSON into `out_path`. Creates parent dirs
- `pub fn scaffold_downstream_tasks` — For each step in `chain`, scaffold a stub `tasks/<step.agent_id>/task.toml`
- `pub fn derive_chain_from_role` — Convenience wrapper: read role, derive steps, return chain. Used by
- `pub fn pipeline_json_path` — Public helper to compute the path where pipeline.json will be written.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: anyhow, serde, std

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
