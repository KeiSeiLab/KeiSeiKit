---
title: store_trait.rs
path: kei-store/src/store_trait.rs
dna_hash: sha256:0ec4251d6ccd2df4
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-store/src/store_trait.rs

MemoryStore trait — single point of truth for every backend.

## Public API

- Read a byte blob at a relative path.
- Write a byte blob at a relative path. Creates parents.
- List regular files under a relative directory (non-recursive).
- Create a branch (git) or a logical "snapshot namespace" (S3).
- Commit staged changes; returns the object id / manifest hash.
- Push a branch to the remote (no-op for FilesystemStore).
- Pull a branch from the remote (no-op for FilesystemStore).
- Human-readable backend name for `status` reporting.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
