---
title: policy_no_git_ops.rs
path: kei-agent-runtime/src/gates/policy_no_git_ops.rs
dna_hash: sha256:6cdbc15dd2264296
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-agent-runtime/src/gates/policy_no_git_ops.rs

`policy::no-git-ops` — RULE 0.13 orchestrator-owns-git enforcement.

Denies any Bash command matching `git`, `gh repo`, `gh api /repos`.
Bypass via env `ORCHESTRATOR_META=1` for orchestrator-meta agents.

As of v0.18 convergence wave: thin const wrapper over `PatternGate`.

## Related

- parent: `kei-agent-runtime/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
