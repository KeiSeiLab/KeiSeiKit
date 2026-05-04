---
title: walk.rs
path: kei-projects-index/src/walk.rs
dna_hash: sha256:494eeccae91b8690
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-projects-index/src/walk.rs

Top-level walker for `~/Projects/`.

Constructor Pattern: one cube = directory enumeration. Returns a flat
list of `ProjectEntry` for each top-level dir under the supplied root.
Hidden dirs (leading `.`) and `_archive` are skipped — they are never
active project workspaces.

## Public API

- One enumerated project candidate. `has_git` is a quick precondition
- Absolute path to the project directory.
- Basename of `path` (final component).
- True if `path/.git` exists (file or dir). Does NOT validate the
- Returns true if a directory entry should be skipped during the walk.
- Detect `.git` (file or dir) at `project_root`. Both shapes are valid:
- Build a `ProjectEntry` from one walkdir entry. Returns `None` if the
- `pub fn walk_projects_root` — Walks `projects_root` exactly one level deep and returns one

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: std, walkdir

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
