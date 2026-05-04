---
title: platform_gate_macos_arm.rs
path: kei-llm-mlx/tests/platform_gate_macos_arm.rs
dna_hash: sha256:067e2403c6ee6913
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-llm-mlx/tests/platform_gate_macos_arm.rs

Platform gate — macOS Apple Silicon side of the truth-table.

On a macos+aarch64 build, `is_supported()` MUST return `supported: true`
and `reason: None`. The reverse (any other target) is asserted in
`platform_gate_other.rs`. Together the two tests pin the two-way
cfg! gate so refactors cannot silently flip it.

## Related

- parent: `kei-llm-mlx/tests`
- imports: kei_llm_mlx

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
