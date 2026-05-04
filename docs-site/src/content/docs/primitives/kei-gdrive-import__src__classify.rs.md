---
title: classify.rs
path: kei-gdrive-import/src/classify.rs
dna_hash: sha256:3b689f01ca0fc37c
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-gdrive-import/src/classify.rs

Single-folder verdict.

Verdict thresholds (from PLAN.md Wave 1 verdict):
* `.git/` present → ALREADY-REPO regardless of score
* score ≥ 8       → PROJECT
* score 5..=7     → AMBIGUOUS
* score < 5       → NOT-A-PROJECT

## Public API

- Apply marker scoring to a flat list of filenames + verdict.
- `pub fn classify_remote` — Remote classify via `rclone lsf <remote-path> --max-depth 1`. Lists ALL

## Related

- parent: `kei-gdrive-import/Cargo.toml`
- imports: crate, serde, std

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
