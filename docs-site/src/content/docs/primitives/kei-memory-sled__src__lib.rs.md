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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
