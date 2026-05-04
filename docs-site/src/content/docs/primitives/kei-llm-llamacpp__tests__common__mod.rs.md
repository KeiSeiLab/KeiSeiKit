---
title: mod.rs
path: kei-llm-llamacpp/tests/common/mod.rs
dna_hash: sha256:02f221bc3e268c33
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-llm-llamacpp/tests/common/mod.rs

Shared MockRunner for integration tests.

Each test pushes canned `Behaviour` entries onto a queue; the next
`Runner::run*` call pops the head entry. Tests assert side-effects
(e.g. `last_args`) plus return values.

`#![allow(dead_code)]` because each integration-test binary compiles
`common/mod.rs` independently and may use only a subset of the
exposed surface.

## Public API

- One canned response from the mock.
- `run` returns this RunOutput.
- `run` returns Err(BinaryNotFound { name }) — simulates "not found".
- `run_stream` returns these lines.
- `spawn_server` succeeds; the test holds the kill_flag handle.
- `pub struct MockRunner` — Canned-response runner.

## Related

- parent: `kei-llm-llamacpp/tests/common`
- imports: kei_llm_llamacpp, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
