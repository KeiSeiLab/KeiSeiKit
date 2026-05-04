---
title: verify.rs
path: kei-spawn/src/verify.rs
dna_hash: sha256:632994d44ff0dd21
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-spawn/src/verify.rs

verify — orchestrator-side post-return verification + ledger bookkeeping.

Given an agent-id and the worktree path the harness returned, this module:
1. Reads `<kit_root>/tasks/<agent-id>/task.toml`
2. Resolves role → ordered capability list
3. Runs `kei_agent_runtime::verify::verify_task` (worktree pass)
4. On pass, marks ledger row `done`; on fail, marks `failed`
5. Emits a `VerifyOutput` JSON (pass/fail + failed-capability list)

Simulated-merge pass is orchestrator-scope (needs git) so we stay in
`RunMode::Worktree`. A future `kei-spawn verify-merge` flavour can be
added once orchestrator-owned git helpers exist.

## Public API

- Outcome of a single verify pass, including failed-capability detail.
- `pub fn verify_agent` — Main verify entry. On pass → ledger done; on fail → ledger failed.
- Resolve and validate `<kit>/tasks/<agent-id>/task.toml`.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: anyhow, crate, kei_agent_runtime, serde, std

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
