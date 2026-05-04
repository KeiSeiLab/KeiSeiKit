---
title: main.rs
path: kei-projects-watcher/src/main.rs
dna_hash: sha256:4ac3991d3c03645a
language: rust
size_loc: 66
generated: by-keidocs
---

# kei-projects-watcher/src/main.rs

kei-projects-watcher — CLI binary.

Expected install path (referenced by the launchd plist template
`kei-projects-watcher.plist.tmpl` shipped by the orchestrator):
${KIT}/_rust/target/release/kei-projects-watcher run

Subcommands:
run     — daemon: watch ~/Projects and re-index on the fly
status  — print last-indexed-ts of every project as JSON

## Public API

- Run the watcher daemon until SIGINT / SIGTERM.
- Print last-indexed-ts of each project as JSON.

## Related

- parent: `kei-projects-watcher/Cargo.toml`
- imports: anyhow, clap, kei_projects_watcher, std

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
