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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
