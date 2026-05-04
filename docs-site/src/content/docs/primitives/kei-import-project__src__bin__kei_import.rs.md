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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
