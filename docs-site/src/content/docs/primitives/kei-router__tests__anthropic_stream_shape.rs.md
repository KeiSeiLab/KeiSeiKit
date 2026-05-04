---
title: anthropic_stream_shape.rs
path: kei-router/tests/anthropic_stream_shape.rs
dna_hash: sha256:5ad9bf1a1f8a25af
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-router/tests/anthropic_stream_shape.rs

Anthropic provider streaming wire-shape test.

Stands up a wiremock fake of /v1/messages, replies with a hand-crafted SSE
body, asserts the StreamEvent sequence the parser produces.

## Related

- parent: `kei-router/tests`
- imports: futures, kei_router, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
