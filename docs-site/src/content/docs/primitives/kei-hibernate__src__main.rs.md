---
title: main.rs
path: kei-hibernate/src/main.rs
dna_hash: sha256:9d97c2374ab6e972
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-hibernate/src/main.rs

kei-hibernate CLI.

Subcommands: `export`, `import`, `inspect`. Thin dispatcher over
the library surface; each arm is <30 LOC.

## Public API

- Bundle kit_root into a tar.zst archive.
- Output bundle path (e.g. bundle.tar.zst).
- Kit root (defaults to current directory).
- Extract a bundle into kit_root (pass --dry-run to preview).
- Print manifest contents without extracting.

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: clap, kei_hibernate, std

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
