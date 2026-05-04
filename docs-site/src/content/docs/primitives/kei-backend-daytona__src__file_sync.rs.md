---
title: file_sync.rs
path: kei-backend-daytona/src/file_sync.rs
dna_hash: sha256:d9c69217a6368d69
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-backend-daytona/src/file_sync.rs

Push/pull `~/.keiseikit` to/from a sandbox.

Hermes uses bulk multipart uploads and a tar-stream for downloads; we
ship a simpler per-file path. Bulk transports are tracked as P1.2.x
follow-ups. Deltas are computed via mtime comparison: a file syncs only
if local-mtime differs from the recorded last-sync mtime.

## Public API

- Tracks last-known mtime per file so we don't push unchanged files.
- Map of relative path → last-synced mtime as nanos-since-epoch.
- `pub fn is_dirty` — True if `path` mtime differs from previously-seen value.
- `pub fn mark` — Mark a path as synced at the given mtime.
- `pub struct FileSync` — Bidirectional sync for a single sandbox handle.
- Local root (e.g. `~/.keiseikit`).
- Remote root path (e.g. `/root/.keiseikit`).
- Push every file under `local_root` whose mtime has changed since the
- Pull a single remote file back to the local tree.
- `pub fn state` — Inspect sync state (for tests / observability).
- Walk `root` recursively and return `(abs_path, relative_path)` pairs.
- mtime expressed as nanos since UNIX epoch; 0 if filesystem doesn't expose it.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: crate, std

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
