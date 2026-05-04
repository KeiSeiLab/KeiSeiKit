---
title: watcher.rs
path: kei-projects-watcher/src/watcher.rs
dna_hash: sha256:857665a35da84c6c
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-projects-watcher/src/watcher.rs

Async fsevents watcher.

Wraps [`notify::RecommendedWatcher`] (FSEvents on macOS) and exposes a
tokio `mpsc::Receiver<PathBuf>` of debounced project roots. Each
emission means: "something inside this project was touched at least
`debounce` ago and has been quiet since — re-index it now".

Filters: only Modify/Create/Remove kinds; only paths strictly under
the watched root; events received in the first 1 s after watcher
start are dropped (FSEvents replays startup events on macOS).

## Public API

- Tokio mpsc channel capacity for raw `notify` events. 1024 is generous
- `pub struct Watcher` — Async filesystem watcher anchored at a single root directory.
- `pub fn new` — Create a recursive fsevents watcher on `root`. `debounce` is the
- `pub fn events` — Take the receiver of debounced project paths. Single-use; later
- Does this `notify` event kind trigger a re-index?
- Push one raw event's project roots into the debouncer at `now`.
- Pump raw events through the debouncer; emit ready projects every 250 ms.

## Related

- parent: `kei-projects-watcher/Cargo.toml`
- imports: anyhow, crate, notify, std, tokio

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
