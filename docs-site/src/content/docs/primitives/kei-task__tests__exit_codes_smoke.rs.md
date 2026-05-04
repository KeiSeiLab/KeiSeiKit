---
title: exit_codes_smoke.rs
path: kei-task/tests/exit_codes_smoke.rs
dna_hash: sha256:f04b8b1406125101
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-task/tests/exit_codes_smoke.rs

kei-task CLI exit-code smoke tests (§Runtime contract).

Atom-layer errors (validation / semantic) → exit 2.
Storage/IO errors → exit 1.

`create --title ""` is the canonical validation-failure case: the
atom's typed Error enum returns `InvalidTitle`, which main.rs maps
to exit 2, NOT the old anyhow collapse at exit 1.

## Related

- parent: `kei-task/tests`
- imports: std

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
