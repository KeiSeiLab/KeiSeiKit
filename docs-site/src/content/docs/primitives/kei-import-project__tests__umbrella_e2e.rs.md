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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
