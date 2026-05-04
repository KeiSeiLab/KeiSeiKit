---
title: walker.rs
path: kei-import-project/src/walker.rs
dna_hash: sha256:135d0ee37f06bf17
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-import-project/src/walker.rs

walker — traverse a repo root, classify files by language, skip noise dirs.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- Language detected from file extension.
- `pub struct FileEntry` — A single file entry from `walk_repo`.
- Path relative to the repo root.
- Language detected by extension; `None` means binary/unknown/oversized.
- File size in bytes.
- `pub struct RepoWalk` — Result of walking a repository root.
- `pub fn walk_repo` — Walk `root`, ignoring noise directories and files >10 MB.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, std, tempfile, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
