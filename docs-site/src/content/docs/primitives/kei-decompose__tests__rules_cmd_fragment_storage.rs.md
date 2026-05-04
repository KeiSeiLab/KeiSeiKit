---
title: rules_cmd_fragment_storage.rs
path: kei-decompose/tests/rules_cmd_fragment_storage.rs
dna_hash: sha256:20e60665551a635d
language: rust
size_loc: 211
generated: by-keidocs
---

# kei-decompose/tests/rules_cmd_fragment_storage.rs

Integration test: decompose-rules writes real fragment files and registers
real paths in the SQLite registry. Validates Path 1 fix for Wave 14d.

Verify criterion: after `rules_cmd::run(...)`, every active `rule` row in
the registry has a `path` that exists on disk and whose content matches
the registered fragment body.

## Related

- parent: `kei-decompose/tests`
- imports: kei_decompose, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
