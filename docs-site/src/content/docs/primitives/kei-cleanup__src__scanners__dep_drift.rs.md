---
title: dep_drift.rs
path: kei-cleanup/src/scanners/dep_drift.rs
dna_hash: sha256:a2f19b2946b6789f
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-cleanup/src/scanners/dep_drift.rs

Workspace-deps drift scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
Appendix A. Detects member-crate dependencies that pin a different
version than the workspace's `[workspace.dependencies]` table.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "dep_drift".
- Read `[workspace.dependencies]` from the workspace Cargo.toml.
- Pull `version = "x"` out of either string-form dep or table-form.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std, walkdir

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
