---
title: model.rs
path: kei-ping/src/model.rs
dna_hash: sha256:b15e3e4c3ff37164
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-ping/src/model.rs

Heartbeat record + query filter. One file, no dependencies on backends.

## Public API

- One agent's "I'm alive, doing X" record.
- Only return heartbeats newer than this many seconds (TTL filter).
- Only return heartbeats matching this phase prefix.
- Only return heartbeats with branch matching exactly.

## Related

- parent: `kei-ping/Cargo.toml`
- imports: serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
