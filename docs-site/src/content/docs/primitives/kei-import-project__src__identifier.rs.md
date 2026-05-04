---
title: identifier.rs
path: kei-import-project/src/identifier.rs
dna_hash: sha256:ee52f17b9a9a39d7
language: rust
size_loc: 189
generated: by-keidocs
---

# kei-import-project/src/identifier.rs

identifier — find manifest files, parse module names, collect source files.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- Category of a detected project module.
- Cargo.toml (Rust crate or workspace member)
- package.json (Node/NPM)
- pyproject.toml or setup.py (Python)
- go.mod (Go module)
- `pub struct ProjectModule` — A language module identified within the walked tree.
- Root-relative path to the manifest file.
- Parent directory of the manifest (root-relative).
- Module name extracted from the manifest.
- Source files (relative to repo root) belonging to this module.
- `pub fn identify_modules` — Identify all modules in a `RepoWalk`.
- Find all manifest files in the walk (relative paths).
- Try to extract the module name.
- Collect source files under `module_root` matching the module's language(s).

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, std

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
