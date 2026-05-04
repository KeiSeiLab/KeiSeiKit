---
title: schema.rs
path: kei-arch-map/src/schema.rs
dna_hash: sha256:3bdd17aa3adcd82a
language: rust
size_loc: 166
generated: by-keidocs
---

# kei-arch-map/src/schema.rs

## Public API

- Allowlisted evidence kinds. NO raw shell.
- File exists at `path` (relative to repo root).
- Regex matches in file. Pattern compiled with size_limit cap.
- Count lines matching regex in file equals `expected`.
- File size in bytes is in `range` (inclusive). Use `[N, N]` for exact.
- Dotted JSON path equals `expected` string. No wildcards.
- `cargo check --workspace --offline --message-format=json` produces
- `cargo check --workspace --offline --message-format=json` produces zero
- Allowlist of paths (relative to repo root) authorised to run
- HTTP GET URL returns one of `expected` codes. SSRF-hardened.
- `pub fn evidence_kind` — Short label for evidence kind (for tables in rendered docs).
- `pub fn evidence_repr` — Short representation of evidence for table cells.
- UTF-8-safe truncate by character count, appending "…" if truncated.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
