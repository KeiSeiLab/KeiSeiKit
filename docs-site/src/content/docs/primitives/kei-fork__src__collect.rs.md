---
title: collect.rs
path: kei-fork/src/collect.rs
dna_hash: sha256:0789cc5a2e6a856c
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-fork/src/collect.rs

`collect(agent_id, commit_msg, kit_root)` — merge the fork back.

Contract:
1. `.DONE` must exist inside the worktree, else `Error::NotDone`
2. Compute an EXPLICIT path list (untracked + modified), minus the
reserved exclusion set, then `git add <paths>` + `git commit`
3. Capture commit SHA, then `git merge --no-ff fork/<id>` in kit_root
4. Move worktree to `_archive/forks/YYYY-MM-DD/<id>/` (preserving
the agent's artefacts for post-hoc review / rescue)
5. `git worktree prune && git branch -D fork/<id>` to clean up refs
6. `kei-ledger done <id>` unless `KEI_FORK_SKIP_LEDGER=1`

HIGH #1 mitigation: the earlier `git add -A` was replaced by an
explicit path list. Reserved names (`.DONE`, `.KEI_FORK_META.toml`,
`_archive/**`, `_forks/**`) are stripped before staging so they
never land in the merge commit even if an agent wrote them.

## Public API

- Paths that never belong in the merged history.
- Path prefixes (relative to worktree root) that are kit-internal.
- Union of (untracked, exclude-standard) + (modified-tracked),

## Related

- parent: `kei-fork/Cargo.toml`
- imports: chrono, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
