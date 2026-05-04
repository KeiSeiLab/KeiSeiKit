---
title: route_no_local_for_anthropic_model.rs
path: kei-llm-router/tests/route_no_local_for_anthropic_model.rs
dna_hash: sha256:ed59cc35933e9720
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-llm-router/tests/route_no_local_for_anthropic_model.rs

Test 3 — `claude-opus-4-7` + `--require-local` → NoBackendAvailable.

Even though the machine could run local models, NO local backend has
a remote-only model id. With `require_local=true`, route MUST refuse
to walk the registry fallback chain (which would yield Anthropic).

## Related

- parent: `kei-llm-router/tests`
- imports: kei_llm_router, kei_machine_probe

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
