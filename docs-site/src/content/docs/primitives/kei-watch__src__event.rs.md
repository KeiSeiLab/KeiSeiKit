---
title: event.rs
path: kei-watch/src/event.rs
dna_hash: sha256:40430ca36593df3b
language: rust
size_loc: 85
generated: by-keidocs
---

# kei-watch/src/event.rs

Canonical event type emitted by [`crate::Watcher`].

Decoupled from `notify::Event` so downstream consumers don't bind to
notify's evolving hierarchy. Only four kinds are emitted:
Created / Modified / Deleted / Renamed.

## Public API

- Coarse event classification. All notify sub-kinds fold into these four.
- `pub fn as_str` — Short lowercase tag (matches CLI JSON `kind` field).
- Filesystem event emitted by the watcher.
- Unix seconds since epoch.
- `pub fn new` — Construct a new event; timestamp is captured here.

## Related

- parent: `kei-watch/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
