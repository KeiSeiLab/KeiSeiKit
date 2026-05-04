---
title: kei_import.rs
path: kei-import-project/src/bin/kei_import.rs
dna_hash: sha256:ae6b09cfe4d010d8
language: rust
size_loc: 176
generated: by-keidocs
---

# kei-import-project/src/bin/kei_import.rs

kei-import — umbrella CLI composing pipeline phases 1-4.
Phase 5 (executor) deferred: run `kei-import-project execute <plan>` separately.

## Public API

- Repository URL (https://, git@) or local path.
- Pause for user confirmation between phases.
- Run all phases without pauses.
- Walk phases without writing any files.
- Output directory for plan.md, gap_report.md, skills/.
- Confidence threshold for trait matching (0.0–1.0).
- Comma-separated phases to skip: walk,map,extract-skills,plan.
- Keep the cloned tempdir for URL inputs.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: clap, kei_import_project, std

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
