---
title: importer.rs
path: kei-hibernate/src/importer.rs
dna_hash: sha256:d6ccd0b74ae36a51
language: rust
size_loc: 168
generated: by-keidocs
---

# kei-hibernate/src/importer.rs

Import side — verify manifest + extract (or dry-run preview).

Two-pass design:
Pass 1 — read manifest entry only, decide version + list conflicts.
Pass 2 — if `dry_run=false`, re-stream archive and extract each
file, then re-hash to verify sha256 against the manifest.

Safe extraction: each entry path is checked for `..` traversal.

## Public API

- `pub fn import` — Entry point for the `import` CLI + library users.
- Open bundle, locate manifest entry, decode TOML.
- Open a fresh zstd-wrapped tar archive stream over `bundle`.
- Files that would be overwritten on a real import (reporting only).
- Re-open archive, extract every non-manifest entry, verify sha256.
- Extract the forward-slash relative path from a tar entry header.
- Reject any entry whose path contains `..` or is absolute.
- Hash extracted file, compare to manifest entry sha256.
- Reset-to-start helper used for debugging / future streaming passes.

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: crate, std, tar

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
