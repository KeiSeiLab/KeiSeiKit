---
title: main.rs
path: kei-frustration-loop/src/main.rs
dna_hash: sha256:92cee33733f73ef2
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-frustration-loop/src/main.rs

kei-frustration-loop — per-user frustration learning loop binary.

Five subcommands: bootstrap / nightly-scan / feedback / auto-train /
personalize. All work happens in cubes; this file dispatches only.

Constructor Pattern: main.rs only routes parsed args to `cli::dispatch`.

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: clap, kei_frustration_loop, std

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
