---
title: config.rs
path: kei-cleanup/src/config.rs
dna_hash: sha256:590a9c2427f12dea
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-cleanup/src/config.rs

Configuration loader — see CLEANUP-RUNTIME-SPEC.md §3.5.

Loads `cleanup.toml` from the workspace root. Falls back to
Constructor-Pattern defaults (file ≤200 LOC, fn ≤30 LOC) when
absent. v0.3 covers all sections: `[loc]`, `[scanners]`,
`[todo_age]`, `[coverage_map]`, `[naming_pairs]`.

## Public API

- Top-level config — Constructor Pattern: 1 struct = 1 concern.
- LOC limits (Rule ZERO).
- Which scanners to run (empty = all).
- TODO/FIXME age thresholds.
- Coverage-map markers.
- Naming-pair drift detection.
- `[loc]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
- Max LOC per file (default 200, Constructor Pattern).
- Max LOC per function (default 30).
- Path substrings to exclude from LOC scan.
- `[scanners]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
- Enabled scanner names (empty = run all defaults).
- `[todo_age]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
- Days after which a TODO bumps to MEDIUM severity.
- Days after which a TODO bumps to HIGH severity.
- `[coverage_map]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
- Marker prefix for derivations (default `// derive:`).
- Marker prefix for test coverage (default `// covers:`).
- `[naming_pairs]` section — see CLEANUP-RUNTIME-SPEC.md §3.5.
- List of synonym groups; drift = >1 variant present in workspace.
- `pub fn load_or_default` — Load from `<workspace>/cleanup.toml` if present, else defaults.
- `pub fn scanner_enabled` — Whether `name` is enabled (empty whitelist = all enabled).
- `pub fn excluded` — Whether `path` matches any exclude entry.

## Related

- parent: `kei-cleanup/Cargo.toml`
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
