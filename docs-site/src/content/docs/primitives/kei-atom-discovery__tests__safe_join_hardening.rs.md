---
title: safe_join_hardening.rs
path: kei-atom-discovery/tests/safe_join_hardening.rs
dna_hash: sha256:d5443fe3b2933411
language: rust
size_loc: 115
generated: by-keidocs
---

# kei-atom-discovery/tests/safe_join_hardening.rs

MEDIUM-severity hardening of `safe_join`.

Covers two regressions that the original lexical-fallback implementation
missed:
1. Accepting a non-existent `base` (no well-defined sandbox).
2. Accepting a symlinked target that escapes `base`.

## Related

- parent: `kei-atom-discovery/tests`
- imports: kei_atom_discovery, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
