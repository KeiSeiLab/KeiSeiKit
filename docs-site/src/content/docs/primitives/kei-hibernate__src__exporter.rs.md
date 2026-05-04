---
title: exporter.rs
path: kei-hibernate/src/exporter.rs
dna_hash: sha256:fea48c513ce98ca8
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-hibernate/src/exporter.rs

Export side — build a `tar.zst` bundle + manifest.

Pipeline:
collector::collect → sha::hash_file per entry → tar append →
manifest append last → zstd compress to `out`.

Manifest is written LAST so all file hashes are already known.
Single-pass compression: bundle stays small even on large stores.

## Public API

- `pub fn export` — Export the KeiSei installation rooted at `kit_root` into `out` (tar.zst).
- Hash every collected file and assemble the manifest. One loop,
- Open `out`, wrap in zstd encoder, tar-append each file, then the
- Serialise manifest to TOML and append as a tar entry (no fs temp file).
- Best-effort machine identifier. `HOSTNAME` env first, hostname

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: crate, std

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
