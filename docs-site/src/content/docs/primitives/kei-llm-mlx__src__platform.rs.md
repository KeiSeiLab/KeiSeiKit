---
title: platform.rs
path: kei-llm-mlx/src/platform.rs
dna_hash: sha256:4996ca750ee97f8a
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-llm-mlx/src/platform.rs

Platform gate — macOS Apple Silicon ONLY.

Constructor Pattern: this cube has ONE responsibility — answer
"is this host able to run mlx_lm?". The gate is checked FIRST in every
subcommand handler before any subprocess attempt.

Detection uses `cfg!()` so the compiled binary carries the answer
statically. A linux ARM box and a macOS Intel box BOTH return
`supported = false` with the same stable reason string.

## Public API

- `pub const REASON_NOT_MACOS` — Reason string is exposed to JSON consumers and tests; treat as stable.
- `Some(reason)` iff `supported == false`. JSON consumers can assume
- `pub fn is_supported` — Compile-time gate: `macos + aarch64`. Both must be true.
- `pub fn host_arch_label` — Stable arch label for JSON output.
- `pub fn host_os_label` — Stable OS label for JSON output.

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: serde

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
