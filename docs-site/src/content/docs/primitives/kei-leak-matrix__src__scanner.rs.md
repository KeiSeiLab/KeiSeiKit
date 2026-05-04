---
title: scanner.rs
path: kei-leak-matrix/src/scanner.rs
dna_hash: sha256:6d1b743339880783
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-leak-matrix/src/scanner.rs

Scanner — file / string / tree scan helpers + JSON output + exit codes.

Output never contains the rule's pattern string. Match excerpts are
redacted to first 12 chars + "…" so the SSoT regex stays in the matrix.

## Public API

- `pub fn scan_string` — Scan one in-memory string. Used by scan_file and scan-cmd.
- Allowed extensions for scan-tree (text-ish files only).
- `pub fn exit_code` — Exit code from a violation set.
- `pub fn cmd_scan_file` — Handler: scan one file → JSON + exit code.
- `pub fn cmd_scan_tree` — Handler: recurse dir → JSON + exit code.
- `pub fn cmd_scan_cmd` — Handler: scan literal command string → JSON + exit code.

## Related

- parent: `kei-leak-matrix/Cargo.toml`
- imports: anyhow, crate, serde, std, walkdir

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
