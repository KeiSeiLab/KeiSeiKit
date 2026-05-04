---
title: cache_smoke.rs
path: kei-cache/tests/cache_smoke.rs
dna_hash: sha256:4317a04b604b3b64
language: rust
size_loc: 138
generated: by-keidocs
---

# kei-cache/tests/cache_smoke.rs

cache_smoke — end-to-end integration tests for `wrap_with`.

Uses a `MockExecutor` that returns an incrementing counter so "was the
executor actually re-invoked?" is observable as a different return
value rather than inferred from a side-effect.

## Public API

- Mock executor: each invocation returns `{"n": <call_count>}`.

## Related

- parent: `kei-cache/tests`
- imports: anyhow, kei_atom_discovery, kei_cache, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
