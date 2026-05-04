---
title: gc.rs
path: kei-fork/src/gc.rs
dna_hash: sha256:028a7710b92bc236
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-fork/src/gc.rs

`gc(kit_root, older_than_hours)` — prune stale forks.

A fork is STALE when `.DONE` is absent AND `age > older_than_hours`.
For each stale fork we:
1. `git worktree remove --force <worktree>`
2. `git branch -D fork/<id>`
3. `kei-ledger fail <id>` unless `KEI_FORK_SKIP_LEDGER=1`

Returns the list of agent_ids pruned. Errors on individual forks are
swallowed into the report so a single bad fork cannot block cleanup
of the rest.

## Related

- parent: `kei-fork/Cargo.toml`
- imports: crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
