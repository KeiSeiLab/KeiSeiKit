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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
