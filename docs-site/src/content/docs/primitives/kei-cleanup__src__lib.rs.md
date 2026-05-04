---
title: lib.rs
path: kei-cleanup/src/lib.rs
dna_hash: sha256:d113ae1d42dddb14
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-cleanup/src/lib.rs

kei-cleanup library facade — see CLEANUP-RUNTIME-SPEC.md §3.3.

v0.1 detection-only API:
* [`run_all`] — invoke every enabled scanner, aggregate findings.
* [`run_scanner`] — invoke one scanner by name.

Both produce a [`CleanupReport`] (or `Vec<Finding>` for the
single-scanner path). The library never edits files, never
commits — see RULE 0.13 ORCHESTRATOR-BRANCH-FIRST.

## Public API

- `pub fn run_all` — Run every enabled scanner. Per CLEANUP-RUNTIME-SPEC.md §3.3:
- `pub fn run_scanner` — Run a single scanner by name.
- Best-effort short SHA — returns "unknown" if not a git workspace

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: chrono, std

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
