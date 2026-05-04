---
title: fs_list.rs
path: kei-cortex/src/handlers/fs_list.rs
dna_hash: sha256:e29c79617358a9a0
language: rust
size_loc: 182
generated: by-keidocs
---

# kei-cortex/src/handlers/fs_list.rs

`GET /api/v1/cortex/fs/list?path=<rel-or-abs>` — directory listing.

Path is treated as relative-to-`project_root` when not absolute. Absolute
paths must already be inside `project_root` (canonicalised). Symlinks are
NOT followed; hidden noise (`node_modules`, `.git`, `target`, etc.) is
filtered out. Sorted dirs-first, alpha within each group.

## Public API

- Cap on entries to keep the response bounded.
- Directories whose contents we never enumerate (noise filter).
- Optional query — `?path=<rel>` defaults to `""` (project root).
- One entry in the listing response.
- Response body the UI parses.
- Handler entry point.
- Resolve `rel` to an absolute path inside `project_root`. Rejects parent
- Read a directory and produce `FsEntry`s, filtering noise + hiding
- Convert one `DirEntry` to an `FsEntry`. Skips symlinks (not followed).
- True when this name should be omitted from the listing (noise filter).
- Match `name` against a HIDE_DIRS token. Exact match wins; otherwise the
- Sort dirs-first / alpha-within-group; cap-note when truncated.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde, std

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
