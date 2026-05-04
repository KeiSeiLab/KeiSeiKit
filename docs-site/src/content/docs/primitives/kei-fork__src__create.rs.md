---
title: create.rs
path: kei-fork/src/create.rs
dna_hash: sha256:00284ee432dcb24f
language: rust
size_loc: 131
generated: by-keidocs
---

# kei-fork/src/create.rs

`create(agent_id, base_branch, kit_root)` — spawn a managed fork.

Steps:
1. `validate_agent_id` (path-traversal defence)
2. Reject if `_forks/<agent_id>/` OR branch `fork/<agent_id>` already exist
3. `git worktree add _forks/<agent_id> -b fork/<agent_id> <base>`
4. Write `.KEI_FORK_META.toml` with agent_id + started_ts + base_branch + ledger_id
5. `kei-ledger fork` unless env `KEI_FORK_SKIP_LEDGER=1`

HIGH #2 mitigation: after the worktree exists, any failure in
steps 4 or 5 triggers a rollback — the worktree is force-removed
and the branch is deleted — so `create()` is either fully-committed
or leaves no trace. Callers can retry safely.

Test hook: if env `KEI_FORK_FORCE_LEDGER_FAIL=1` is set, the ledger
call returns `Error::Ledger` unconditionally (regardless of
`KEI_FORK_SKIP_LEDGER`). Used by rollback regression tests.

Worktree path is indexed by `agent_id`, not UUID, so `rescue()` /
`collect()` can be resolved from a human-readable CLI arg.

## Public API

- Best-effort cleanup after a partial failure. Errors from the

## Related

- parent: `kei-fork/Cargo.toml`
- imports: crate, kei_agent_runtime, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
