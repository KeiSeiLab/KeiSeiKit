---
title: pipeline_smoke.rs
path: kei-spawn/tests/pipeline_smoke.rs
dna_hash: sha256:311a4b91df18bfe0
language: rust
size_loc: 179
generated: by-keidocs
---

# kei-spawn/tests/pipeline_smoke.rs

pipeline_smoke — integration tests for `spawn --pipeline` end-to-end.

Same pattern as spawn_smoke.rs: minimal tempdir kit, role + capability
fixtures, then call the library surface and assert on-disk artefacts.
`KEI_SPAWN_LEDGER_NOOP=1` keeps the ledger subprocess a no-op so tests
do not depend on a real kei-ledger binary.

## Related

- parent: `kei-spawn/tests`
- imports: kei_spawn, std, tempfile

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
