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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
