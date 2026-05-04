---
title: main.rs
path: kei-graph-check/src/main.rs
dna_hash: sha256:7d04899d155141b8
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-graph-check/src/main.rs

kei-graph-check — binary entry.

Exit 0 if all refs resolve; exit 2 if any broken. Useful as a gate
BEFORE the orchestrator commits the deep-sleep fork branch.

## Public API

- Root directory (e.g. memory-repo clone).
- Optional patch file — any `+++ /dev/null` removal or `# removed: <p>`
- JSON output (default is human).

## Related

- parent: `kei-graph-check/Cargo.toml`
- imports: clap, kei_graph_check, std

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
