---
title: test_todo_age.rs
path: kei-cleanup/tests/test_todo_age.rs
dna_hash: sha256:9be3ad4d8d89cacd
language: rust
size_loc: 63
generated: by-keidocs
---

# kei-cleanup/tests/test_todo_age.rs

Integration test for the todo_age scanner.

Builds a tmp git repo with a TODO line, commits it, runs scanner.
If `git` is missing on the test host, the scanner returns
`ToolNotFound` and the test marks itself as a graceful skip.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std, tempfile

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
