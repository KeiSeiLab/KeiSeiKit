---
title: git.rs
path: kei-fork/src/git.rs
dna_hash: sha256:5fbfc6774d210308
language: rust
size_loc: 198
generated: by-keidocs
---

# kei-fork/src/git.rs

Thin `Command::new(git_bin())` wrappers.

Every helper runs `git` in `kit_root` (or a specified worktree),
captures stdout/stderr, and returns `Error::Git` on non-zero exit.
No parsing beyond `trim()` on stdout — callers interpret the string.

PATH hijack mitigation (HIGH #4): the git binary is resolved via
`git_bin()`, which honours `KEI_FORK_GIT_BIN` if set. Ops can pin to
an absolute path (e.g. `/usr/bin/git`) in trusted environments.

Arg-injection mitigation (HIGH #3): `worktree_add` uses the `--`
sentinel before the base commit-ish and validates the refname shape.

## Public API

- `pub fn git_bin` — Resolve the `git` binary. Honours `KEI_FORK_GIT_BIN` for hardening.
- `pub fn is_safe_refname` — Conservative git refname validator. Accepts the subset we emit and
- `pub fn add_paths` — Stage an explicit list of paths. Replacement for `add -A` which
- `pub fn ls_untracked` — List untracked files (respects `.gitignore`) relative to `cwd`.
- `pub fn ls_modified` — List modified tracked files relative to `cwd`.
- `pub fn branch_exists` — Check whether `branch` exists. `git show-ref` exits 0 if the ref is

## Related

- parent: `kei-fork/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
