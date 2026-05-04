---
title: test_doc_warnings.rs
path: kei-cleanup/tests/test_doc_warnings.rs
dna_hash: sha256:d7acadcfe1f55491
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-cleanup/tests/test_doc_warnings.rs

Integration test for the doc_warnings scanner.

Doc warnings depend on a working `cargo` toolchain plus a
`cargo doc` invocation that finishes within 120 s on a single-file
crate. We exercise the parser directly via a fabricated stderr
string + verify the public API returns ToolNotFound when cargo
is absent — the live cargo path is not unit-tested here because
it would require fetching crate index from network.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
