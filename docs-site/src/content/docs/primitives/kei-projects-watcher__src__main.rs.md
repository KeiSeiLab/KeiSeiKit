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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
