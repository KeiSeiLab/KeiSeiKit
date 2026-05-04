---
title: schema.rs
path: kei-arch-map/src/schema.rs
dna_hash: sha256:bd29b1fc4c7a4d41
language: rust
size_loc: 145
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
- HTTP GET URL returns one of `expected` codes. SSRF-hardened.
- `pub fn evidence_kind` — Short label for evidence kind (for tables in rendered docs).
- `pub fn evidence_repr` — Short representation of evidence for table cells.
- UTF-8-safe truncate by character count, appending "…" if truncated.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: serde, std

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
