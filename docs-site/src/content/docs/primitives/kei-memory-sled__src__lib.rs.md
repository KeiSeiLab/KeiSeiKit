---
title: lib.rs
path: kei-memory-sled/src/lib.rs
dna_hash: sha256:dbe4b36db64d7920
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-memory-sled/src/lib.rs

kei-memory-sled — `MemoryBackend` impl over sled v0.34.

Embedded, single-process key-value store. Suitable for:
- per-user VM local memory store
- offline-first agents needing structured `MemoryItem` storage
- test fixtures (cheap to spin up via `tempfile::tempdir`)

Out of scope:
- cross-process concurrency beyond what sled itself offers
- remote mirroring (`mirror_to_remote` returns `Provider` error;
git-push is the responsibility of `kei-sleep-sync.sh` per RULE 0.15)

## Related

- parent: `kei-memory-sled/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
