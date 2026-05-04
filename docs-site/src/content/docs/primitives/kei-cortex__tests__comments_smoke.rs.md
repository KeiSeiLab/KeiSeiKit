---
title: comments_smoke.rs
path: kei-cortex/tests/comments_smoke.rs
dna_hash: sha256:84c736136dcea638
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-cortex/tests/comments_smoke.rs

Integration smoke test for `/api/v1/cortex/comments/*`.

Spawns the full router on an ephemeral port, walks
POST → GET → react → DELETE → GET (asserts deleted=true).
`KEI_COMMENTS_DB` is set to a per-test tempfile so the suite
never touches the real `~/.keisei/comments.sqlite`.

## Related

- parent: `kei-cortex/tests`
- imports: common, reqwest, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
