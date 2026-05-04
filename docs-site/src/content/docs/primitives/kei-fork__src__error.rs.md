---
title: error.rs
path: kei-fork/src/error.rs
dna_hash: sha256:3568aa62b6a7bfb0
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-fork/src/error.rs

Typed error — every kei-fork public op returns `Result<_, Error>`.

Categories:
- `Validate` — agent-id failed `kei_agent_runtime::validate`
- `Duplicate` — worktree/branch for this agent-id already exists
- `NotDone` — collect() called before the agent wrote `.DONE`
- `Gone` — rescue() could not find the worktree (live or archived)
- `InvalidRef` — branch / base-branch string rejected by refname guard
- `Io` / `Git` / `Ledger` / `Meta` — subsystem failures

## Related

- parent: `kei-fork/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
