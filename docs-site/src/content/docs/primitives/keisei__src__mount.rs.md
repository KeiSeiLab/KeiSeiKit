---
title: mount.rs
path: keisei/src/mount.rs
dna_hash: sha256:f014a1ec9096b09d
language: rust
size_loc: 118
generated: by-keidocs
---

# keisei/src/mount.rs

`keisei mount <brain-path>` — attach to every detected client.

Constructor Pattern: single responsibility — orchestrate the fan-out
(load brain → enumerate adapters → for each detecting adapter, pick
per-adapter scope via `auto_scope()` → attach → collect successes /
failures → write v4 marker with one attachment per success → print
summary).

v0.22: mount resolves scope per-adapter via `auto_scope()` rather than
forcing `Scope::User` — a user running `keisei mount brain` inside
`team-repo/` with `.cursor/` present will get project-scope Cursor +
user-scope Claude Code in a single command. The v4 marker stores each
attachment's resolved scope so `detach` can reverse the fan-out exactly.

## Public API

- Returns `(succeeded, failed)` where:

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
