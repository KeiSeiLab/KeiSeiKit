---
title: spawn_smoke.rs
path: kei-spawn/tests/spawn_smoke.rs
dna_hash: sha256:4373e0b3d42e4315
language: rust
size_loc: 216
generated: by-keidocs
---

# kei-spawn/tests/spawn_smoke.rs

spawn_smoke — integration tests for kei-spawn library API.

These tests set `KEI_SPAWN_LEDGER_NOOP=1` so the ledger subprocess is a
no-op — we exercise the compose + prepare_agent + output shape path
without depending on a real `kei-ledger` binary being on PATH.

Fixtures follow the same pattern as kei-agent-runtime's tests: write a
minimal `_roles/` + `_capabilities/` tree into a tempdir, a task.toml
referencing the role, then call `spawn_from_task` and assert the JSON
shape + on-disk artefacts.

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
