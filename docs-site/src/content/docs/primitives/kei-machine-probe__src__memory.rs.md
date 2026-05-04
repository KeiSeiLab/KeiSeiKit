---
title: memory.rs
path: kei-machine-probe/src/memory.rs
dna_hash: sha256:2db03fbc759fd2c5
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-machine-probe/src/memory.rs

RAM detection via `sysctl hw.memsize` + `vm_stat`.

`sysctl -n hw.memsize` → total RAM in bytes (always reliable).
`vm_stat` → per-page activity. Apple's standard page size is 16 KiB on
Apple Silicon, 4 KiB on Intel; we read the first line ("page size of
N bytes") and use it. `available_bytes` ≈ (free + inactive +
speculative + purgeable) × page_size. `pressure_pct` is the share of
total occupied by wired+active+compressed.

## Public API

- Parse a vm_stat line like `"Pages free:    123456."` → 123456.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate, regex

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
