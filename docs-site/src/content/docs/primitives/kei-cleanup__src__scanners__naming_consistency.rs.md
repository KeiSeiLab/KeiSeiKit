---
title: naming_consistency.rs
path: kei-cleanup/src/scanners/naming_consistency.rs
dna_hash: sha256:977e902c0929cd0c
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-cleanup/src/scanners/naming_consistency.rs

Naming-consistency scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
Appendix A.

Reads `[naming_pairs] pairs = [["A", "B"], ...]` from cleanup.toml.
For each synonym group, counts whole-word occurrences across
workspace src/. If 2+ variants from the same group are present in
the workspace, emits a Severity::Low NamingDrift finding.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "naming_consistency".
- Map of variant → (file_count, first-seen path) across workspace.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, regex, std, walkdir

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
