---
title: debounce.rs
path: kei-projects-watcher/src/debounce.rs
dna_hash: sha256:4d39096d96461e59
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-projects-watcher/src/debounce.rs

Pure helpers used by the watcher: project-root mapping and a 2-second
debounce buffer that collapses many filesystem events for the same
project into one notification.

No async, no I/O, no `notify` types — easy to unit-test.

## Public API

- `pub fn project_root_of` — Map any filesystem path to the immediate child of `root` it sits inside.
- `pub struct Debouncer` — Debounce window: collapse repeated events on the same project into one
- `pub fn new` — Create a debouncer with the given quiet window.
- `pub fn push` — Record an event for `project` at time `now`. Resets the project's
- `pub fn drain_ready` — Return all projects whose last event is older than `window` at
- `pub fn pending_len` — Number of projects currently waiting for their quiet window to

## Related

- parent: `kei-projects-watcher/Cargo.toml`
- imports: std

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
