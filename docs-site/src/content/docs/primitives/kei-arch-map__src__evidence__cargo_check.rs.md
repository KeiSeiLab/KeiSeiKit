---
title: cargo_check.rs
path: kei-arch-map/src/evidence/cargo_check.rs
dna_hash: sha256:9bccbc6c879e0c7d
language: rust
size_loc: 135
generated: by-keidocs
---

# kei-arch-map/src/evidence/cargo_check.rs

## Public API

- Captured cargo output. Mirrors `std::process::Output` but built from
- Spawn cargo check with fixed argv (NOT through sh).
- Spawn a thread that reads a pipe to EOF into a Vec<u8>.
- Wait with timeout while concurrently draining stdout+stderr in
- Count compiler-error diagnostics in cargo JSON stream.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: serde, std, wait_timeout

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
