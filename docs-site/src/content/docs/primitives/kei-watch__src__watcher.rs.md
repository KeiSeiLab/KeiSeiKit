---
title: watcher.rs
path: kei-watch/src/watcher.rs
dna_hash: sha256:528cc5017ecdaf9a
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-watch/src/watcher.rs

Public [`Watcher`] type — owns the notify backend and the pump thread.

Layout invariants:
- exactly one pump thread per watcher
- pump thread exits when `notify::Watcher` is dropped (closes
notify's sender, which closes the pump's `recv`)
- `Drop` explicitly drops the notify watcher first, then joins the
pump — preventing zombie threads

## Public API

- `pub struct Watcher` — Filesystem watcher. Synchronous API only; see crate docs.
- `pub fn new` — Construct a new watcher. Spawns one background thread for the
- `pub fn watch` — Begin watching `path`. When `recursive=true`, all descendants
- `pub fn unwatch` — Stop watching `path`.
- `pub fn next_event` — Block until either an event arrives or `timeout` elapses.
- `pub fn drain` — Non-blocking: drain all currently queued events.

## Related

- parent: `kei-watch/Cargo.toml`
- imports: crate, notify, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
