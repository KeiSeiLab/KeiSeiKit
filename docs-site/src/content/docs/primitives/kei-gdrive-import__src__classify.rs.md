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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
