---
title: main.rs
path: kei-fork/src/main.rs
dna_hash: sha256:88bb1fbbb2b3543d
language: rust
size_loc: 137
generated: by-keidocs
---

# kei-fork/src/main.rs

kei-fork — CLI dispatcher.

Single responsibility: parse args, dispatch to lib ops, print JSON.
Default `kit_root = std::env::current_dir()`.

## Public API

- Override kit_root (default: current dir).
- Spawn a new managed fork.
- Collect a done fork: commit, merge --no-ff, archive.
- List forks, optionally filtered by status.
- active | done | stale | merged | all
- Prune stale forks (no .DONE and age ≥ --older-than hours).
- Copy a fork's files out of band.

## Related

- parent: `kei-fork/Cargo.toml`
- imports: clap, kei_fork, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
