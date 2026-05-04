---
title: umbrella_e2e.rs
path: kei-import-project/tests/umbrella_e2e.rs
dna_hash: sha256:e18c0ef58f390e7a
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-import-project/tests/umbrella_e2e.rs

Integration tests for the umbrella pipeline (phases 1-4, synthetic TempDir repos only).

## Public API

- Runs phases 1-4 in-process (no CLI, no git). Returns nothing on success.
- Test 1: path input + non-interactive — plan.md + gap_report.md + executor-plan.json exist.
- Test 2: --skip plan — no plan.md but skills/ and gap_report.md exist.
- Test 3: --dry-run — no output files written, content is non-empty.
- Test 4: two-crate repo — plan.md summary shows exactly 2 modules.

## Related

- parent: `kei-import-project/tests`
- imports: kei_import_project, std, tempfile

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
