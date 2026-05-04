---
title: todo_age.rs
path: kei-cleanup/src/scanners/todo_age.rs
dna_hash: sha256:9bf93a11b0563208
language: rust
size_loc: 150
generated: by-keidocs
---

# kei-cleanup/src/scanners/todo_age.rs

TODO/FIXME age scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.

Walks workspace src/, finds `(TODO|FIXME|XXX|HACK)` markers, then
shells out to `git blame -L N,N <file>` to get the commit timestamp
of each match. Severity grades from age:
* age > cfg.todo_age.fail_days → HIGH
* age > cfg.todo_age.warn_days → MEDIUM
* else → LOW

If `git` is not on PATH, scanner returns `ToolNotFound` and the
runtime records it in `scanners_skipped` per CLEANUP §3.3 contract.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "todo_age".
- `git blame -L N,N --porcelain <file>` → first `author-time <unix>` line.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: chrono, crate, regex, std, walkdir

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
