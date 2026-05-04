---
title: deps.rs
path: kei-task/src/deps.rs
dna_hash: sha256:e0379f189d7eb36d
language: rust
size_loc: 66
generated: by-keidocs
---

# kei-task/src/deps.rs

Dependency edges + cycle detection + dependency-chain traversal.

## Public API

- `pub fn add_dependency` — Add a dependency. Refuses a cycle (taskId -> dependsOn -> ... -> taskId).
- True if adding task_id -> depends_on would create a cycle.
- `pub fn dependency_chain` — Full dependency chain (BFS transitive closure).

## Related

- parent: `kei-task/Cargo.toml`
- imports: anyhow, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
