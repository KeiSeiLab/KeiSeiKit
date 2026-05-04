---
title: hot_reload.rs
path: kei-pipe/src/hot_reload.rs
dna_hash: sha256:be386551c47315ea
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-pipe/src/hot_reload.rs

`HotReloader` — kei-watch → kei-pipe bridge.

Thin wrapper that owns a [`kei_watch::Watcher`] plus a user callback
fired on every accepted filesystem event. Typical usage: a driver
loop watches a DAG TOML + its source tree; every
`Created`/`Modified`/`Deleted` event triggers a re-parse and re-run.

The wrapper is synchronous (no async runtime). `poll_once` blocks up
to `timeout` for the first event, then drains anything else already
queued and returns the full batch. Zero events → empty vec.

Trust boundary: callback runs on the caller's thread inside
`poll_once`, NOT on the internal pump thread.

## Public API

- Public error surface: mirrors [`kei_watch::WatchError`] plus a local
- `pub struct HotReloader` — Owns the [`Watcher`] and the per-event callback.
- `pub fn new` — Build a reloader that watches each entry in `paths` recursively.
- `pub fn poll_once` — Block up to `timeout` for the first event, drain anything else

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: kei_watch, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
