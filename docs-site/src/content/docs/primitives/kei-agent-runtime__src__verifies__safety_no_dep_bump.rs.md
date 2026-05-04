---
title: safety_no_dep_bump.rs
path: kei-agent-runtime/src/verifies/safety_no_dep_bump.rs
dna_hash: sha256:67853aeed59a529f
language: rust
size_loc: 47
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/safety_no_dep_bump.rs

`safety::no-dep-bump` verify — git-diffs Cargo.toml / Cargo.lock between
main and HEAD of the agent worktree; fails if any `version =` line changed.

As of v0.18 convergence wave: `CommandVerify` wrapper with a custom
runner (git-diff + regex, not a plain exit-code check).

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, once_cell, regex

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
