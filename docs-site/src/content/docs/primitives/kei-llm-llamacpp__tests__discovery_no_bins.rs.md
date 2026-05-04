---
title: discovery_no_bins.rs
path: kei-llm-llamacpp/tests/discovery_no_bins.rs
dna_hash: sha256:3cde9b91db424669
language: rust
size_loc: 43
generated: by-keidocs
---

# kei-llm-llamacpp/tests/discovery_no_bins.rs

With both binaries absent, `discover` returns an all-None BinPaths
and `any_found` is false. The Runner is never called because
`locate()` fails first via PATH lookup.

## Related

- parent: `kei-llm-llamacpp/tests`
- imports: kei_llm_llamacpp, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
