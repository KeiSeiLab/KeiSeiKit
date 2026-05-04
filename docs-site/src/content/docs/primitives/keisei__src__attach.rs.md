---
title: attach.rs
path: keisei/src/attach.rs
dna_hash: sha256:c781e88d1ec7d3d0
language: rust
size_loc: 115
generated: by-keidocs
---

# keisei/src/attach.rs

`keisei attach <brain-path> [--scope=<user|project|auto>]` implementation.

Constructor Pattern: single responsibility — orchestrate the 7-step
attach ritual (canonicalize → load manifest → validate schema →
detect client → resolve Auto scope → adapter.attach → merge into SSoT
marker → print summary). No I/O here beyond what the `brain`,
`adapter`, and `config` modules already own.

v0.22:
* `Scope::Auto` (CLI default) is resolved into a concrete `User` /
`Project` by the adapter's `auto_scope()` before the attach runs —
the marker never stores `Auto`.
* The marker merges v4-style: if a v4 marker already exists, the new
attachment is appended (or replaced if `(client_type, scope)`
already matches); otherwise a fresh marker is written.

## Public API

- If the user passed `Scope::Auto`, ask the adapter to pick based on
- Merge a new attachment into the existing marker, or start fresh.

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
