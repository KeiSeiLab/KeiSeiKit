---
title: skill_extractor_tests.rs
path: kei-import-project/tests/skill_extractor_tests.rs
dna_hash: sha256:c299b43b99dffe7c
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-import-project/tests/skill_extractor_tests.rs

Integration tests for skill_extractor (≤ 200 LOC).
All tests use tempfile::TempDir — no ~/.claude/ writes.

## Public API

- 1. Happy path: README + docs/setup.md → ≥2 skills extracted, SKILL.md exists.
- 2. Idempotent: re-run on same repo → 0 new registered, 0 superseded, all unchanged.
- 3. Modify body → re-run → 1 superseded, rest unchanged.
- 4. Dry run (registry_db = None): no fragment files written, no registry rows.
- 5. Deduplication: two markdown files with same heading produce distinct skills.
- 6. Empty body section is skipped.

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
