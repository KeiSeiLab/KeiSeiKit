---
title: error.rs
path: kei-watch/src/error.rs
dna_hash: sha256:9a8078bf3f3a513b
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-watch/src/error.rs

Error type for the kei-watch primitive.

Wraps `notify` + `io` errors behind a stable surface so downstream
consumers don't bind to notify's error hierarchy.

## Public API

- Failure modes for [`crate::Watcher`] operations.
- Underlying OS I/O failure.
- notify backend failed to start or watch.
- Path given to `watch` does not exist on disk.
- `unwatch` called on a path that was never registered.
- `pub fn from_notify` — Convert a `notify::Error` into `WatchError`, preserving the path hint

## Related

- parent: `kei-watch/Cargo.toml`
- imports: notify, std

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
