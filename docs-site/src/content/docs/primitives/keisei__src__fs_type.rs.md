---
title: fs_type.rs
path: keisei/src/fs_type.rs
dna_hash: sha256:6c33dee340c06057
language: rust
size_loc: 103
generated: by-keidocs
---

# keisei/src/fs_type.rs

Filesystem type detection for brain root.

Warns when the brain sits on exFAT / FAT32, where SQLite WAL shared-
memory mmap (used by `kei-memory`, `kei-artifact`, `kei-social-store`)
is unreliable and `keisei mount` (multi-client) will corrupt DBs.
Single-client `keisei attach` stays supported, hence the warning is
advisory, never blocking. Platform calls: `statfs(2)` on macOS +
Linux; Windows returns `Unknown` until `GetVolumeInformationW` lands.

## Public API

- `pub fn warn_on_unsafe_fs` — Print a stderr advisory when the brain root lives on exFAT / FAT32.
- `pub fn detect_fs_warning` — Classify the filesystem at `path`. NEVER returns `Result` — errors

## Related

- parent: `keisei/Cargo.toml`
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
