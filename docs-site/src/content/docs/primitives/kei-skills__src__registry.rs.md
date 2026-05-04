---
title: registry.rs
path: kei-skills/src/registry.rs
dna_hash: sha256:6745997caad685d5
language: rust
size_loc: 96
generated: by-keidocs
---

# kei-skills/src/registry.rs

Name-keyed in-memory skill store with optional hot-reload.

Constructor Pattern: registry owns the `HashMap`, the optional
`notify` watcher, and a thread-safe `RwLock` for read-mostly access.

Hot-reload semantics: when the watcher fires for any path under the
root directory, the registry re-runs `load_all` and atomically swaps
the inner map. Brief readers see either the old set or the new set —
no torn reads, no half-loaded skills.

## Public API

- Public registry handle. Cloneable — the inner state is `Arc`-shared.
- Held to keep the watcher alive. `None` until `enable_hot_reload`.
- `pub fn new` — Build a registry by walking `root` once. No watcher is started;
- `pub fn get` — Look up a skill by name. Returns `None` if absent.
- `pub fn list` — Snapshot the registry. O(N) clone — callers that iterate often
- `pub fn list_by_category` — Filter snapshot by `category` field. Empty result when no skill
- `pub fn reload` — Force a re-scan from disk. Atomic swap — readers never observe
- `pub fn enable_hot_reload` — Start a notify watcher that calls `reload` on any FS event under

## Related

- parent: `kei-skills/Cargo.toml`
- imports: crate, notify, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
