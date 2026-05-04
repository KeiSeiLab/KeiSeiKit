---
title: grep.rs
path: kei-cortex/src/tool/grep.rs
dna_hash: sha256:8c0cb4e88f70f474
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-cortex/src/tool/grep.rs

`grep` tool — regex search over files.

Composition: walkdir over root → optional glob filter → regex match →
emit either `files_with_matches` (one path per line) or `content`
(`path:line_no:line_text`). Limits: 1000 lines, 100 files.

Sandbox: when an absolute `path` is supplied, it must resolve INSIDE
`project_root` (canonicalised). When omitted, the search root is
`project_root` itself.

Uses the workspace `regex` dep (no PCRE, ASCII-flavoured Rust regex).

## Public API

- Walk and dispatch to the requested output mode.
- Scan one file's text, mutating `out`/`total_lines`. Returns false when

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: regex, serde, serde_json, std, tempfile, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
