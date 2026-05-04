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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
