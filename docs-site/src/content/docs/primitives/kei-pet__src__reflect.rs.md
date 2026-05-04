---
title: reflect.rs
path: kei-pet/src/reflect.rs
dna_hash: sha256:9c93702634b15760
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-pet/src/reflect.rs

Pet self-reflection — analyse user correction signals, propose persona
tune changes.

Pipeline: caller accumulates `CorrectionSignal`s over some window (a
session, a day, since last tune). `propose_tune` groups them by topic
and emits a minimal, idempotent set of `ProposedChange`s against the
current `PetManifest`. Persistence and user-approval UX are the
caller's concern — this module is pure data + pure logic.

## Public API

- Topic label. Two shapes:
- `pub fn propose_tune` — Produce an ordered, idempotent set of proposed changes.
- Collect `forbidden_topic:<payload>` signals preserving first-seen

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
