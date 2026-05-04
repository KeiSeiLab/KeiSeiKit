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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
