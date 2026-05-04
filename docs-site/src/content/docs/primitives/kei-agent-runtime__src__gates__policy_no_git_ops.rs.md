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
