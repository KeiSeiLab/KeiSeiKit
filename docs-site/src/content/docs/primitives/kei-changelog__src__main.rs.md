---
title: main.rs
path: kei-changelog/src/main.rs
dna_hash: sha256:3ff5cc4b2edce9c0
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-changelog/src/main.rs

kei-changelog — CLI entry point.

Thin wrapper over the library modules. Keeps flag parsing + IO here; all
commit / render logic lives in `lib.rs`.

## Public API

- Starting ref (exclusive). Defaults to the full history root.
- Ending ref (inclusive). Defaults to `HEAD`.
- Treat the range as an Unreleased section (overrides --version heading).
- Version label for the rendered block (e.g. "v0.7.0"). Ignored with --unreleased.
- Repository path. Defaults to current directory.
- Prepend output to this file (creates if missing). Without it, prints to stdout.
- Insert `section` after the top-level `# CHANGELOG` heading if present,

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: anyhow, clap, kei_changelog, std

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
