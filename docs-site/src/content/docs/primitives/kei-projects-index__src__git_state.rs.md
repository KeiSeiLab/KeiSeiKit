---
title: git_state.rs
path: kei-projects-index/src/git_state.rs
dna_hash: sha256:78edbf5775b7e749
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-projects-index/src/git_state.rs

Git state extraction for one project.

Constructor Pattern: one cube = git2 wrapper that returns a snapshot
of branch / dirty / ahead / behind / last-commit for a single repo.
Non-repo paths short-circuit with `None`.

## Public API

- Snapshot of a repo's state at index time. All fields are optional
- Resolve the current branch name. Detached HEAD or empty repo → None.
- Detect uncommitted changes (working-tree OR staged index changes).
- Compute commits ahead / behind upstream tracking branch. Returns
- Extract last-commit metadata from HEAD. Returns three `None`s on
- `pub fn detect_git_state` — Open `project_root` as a git repo and snapshot its state.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: git2, serde, std

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
