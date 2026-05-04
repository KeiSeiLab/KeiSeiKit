---
title: health_all_unavailable.rs
path: kei-llm-router/tests/health_all_unavailable.rs
dna_hash: sha256:e512d9937089afc0
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-llm-router/tests/health_all_unavailable.rs

Test 6 — empty candidate list (every backend down) →
* `decide` returns `Error::NoBackendAvailable`,
* the tried-list mirrors the discovery output (here: `<none>`).

## Related

- parent: `kei-llm-router/tests`
- imports: kei_llm_router, kei_machine_probe

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
