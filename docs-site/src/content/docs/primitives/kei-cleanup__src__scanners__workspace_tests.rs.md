---
title: workspace_tests.rs
path: kei-cleanup/src/scanners/workspace_tests.rs
dna_hash: sha256:ea53f9361f3e5359
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-cleanup/src/scanners/workspace_tests.rs

Workspace-tests scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.

For each workspace member crate, parses each `tests/*.rs` file via
`syn` and looks for `extern crate <kei_*>` and `use <kei_*>` items
referencing 2+ DIFFERENT sibling crates. Such tests create dev-dep
cycles and should live in a dedicated `tests` crate (the K2K
workspace-tests pattern).

Severity: MEDIUM, Confidence::High (syn AST is deterministic).

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "workspace_tests".
- Fallback when syn fails (e.g. macro-generated tests):

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std, syn, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
