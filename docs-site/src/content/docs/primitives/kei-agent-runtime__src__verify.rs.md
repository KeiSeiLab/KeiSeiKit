---
title: verify.rs
path: kei-agent-runtime/src/verify.rs
dna_hash: sha256:9faf2ade5165fbdd
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-agent-runtime/src/verify.rs

Run every verify-capability declared by the task's role and collect
results into a `VerifyReport`.

`run-mode` of each capability is not declared in this phase's registry
(declarative side is phase 1's `capability.toml`). Runtime defaults to
`Worktree`; caller passes `RunMode::Both` to get the simulated-merge
pass as well.

## Public API

- `pub fn verify_task` — Run every verify capability listed in the role's required list, in order.
- `pub fn load_role_capabilities` — Extract the ordered capability list from a role.toml file,

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: anyhow, crate, serde, std

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
