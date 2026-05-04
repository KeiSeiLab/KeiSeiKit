---
title: sse_parsing.rs
path: kei-tty/tests/sse_parsing.rs
dna_hash: sha256:5de648f222e200d0
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-tty/tests/sse_parsing.rs

SSE-frame parsing — feeds canned chunked bytes through
`flush_complete_frames` and asserts the resulting `ChatEvent` stream.

We do NOT spin up a real reqwest server here; the parser is exercised
directly because it is the only part of the client that has logic worth
testing in isolation. The HTTP path is covered manually against a live
daemon (see README).

## Public API

- Drain a fully-formed SSE response in one go.
- Simulate chunked TCP delivery: bytes arrive split across `\n\n`
- Comments (`:` lines) and `event:` / `id:` headers must be ignored.
- Multi-line `data:` fields are concatenated with `\n` per W3C spec.
- Future event tags surface as `Other` rather than dropping the frame.

## Related

- parent: `kei-tty/tests`
- imports: kei_tty

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
