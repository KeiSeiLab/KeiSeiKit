---
title: replay.rs
path: kei-replay/src/replay.rs
dna_hash: sha256:c78784fda7c7a221
language: rust
size_loc: 96
generated: by-keidocs
---

# kei-replay/src/replay.rs

Replay — reconstruct a spawn's composed prompt from a DNA string.

Pipeline:
1. Parse DNA (validates shape).
2. Resolve ledger hit (agent-id, worktree path, spec_sha).
3. Locate `task.toml` (explicit override OR `<worktree>/tasks/<agent-id>/task.toml`).
4. Load task + kit root, re-run `kei_agent_runtime::compose::compose_prompt`.
5. Recompute body hash from the re-loaded `task.body.text` and compare
to the DNA body segment — mismatch = schema drift.

## Public API

- Outcome of a replay pass.
- `pub fn replay` — Reconstruct the spawn.
- Prefer explicit override; else derive from ledger worktree_path + agent-id.
- 8-hex SHA-256 prefix — mirrors `kei_agent_runtime::dna::short_sha256`.

## Related

- parent: `kei-replay/Cargo.toml`
- imports: anyhow, kei_agent_runtime, sha2, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
