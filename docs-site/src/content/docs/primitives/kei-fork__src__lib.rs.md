---
title: lib.rs
path: kei-fork/src/lib.rs
dna_hash: sha256:1d4227576b61a517
language: rust
size_loc: 30
generated: by-keidocs
---

# kei-fork/src/lib.rs

kei-fork — managed git-worktree + ledger lifecycle for agent spawns.

Public API: `create`, `collect`, `list`, `gc`, `rescue`. Each op is
backed by one module under `src/`, keeping every file ≤200 LOC and
every function ≤30 LOC (Constructor Pattern). Shell-out helpers for
`git` live in `git.rs`; TOML round-trip for `.KEI_FORK_META.toml`
lives in `meta.rs`; the `ForkHandle` value type and the
`ForkStatus` enum live in `handle.rs`.

Ledger integration is optional at runtime: if env
`KEI_FORK_SKIP_LEDGER=1` is set, create/collect/gc skip the
`kei-ledger` subprocess call. Tests rely on this for hermeticity.

## Related

- parent: `kei-fork/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
