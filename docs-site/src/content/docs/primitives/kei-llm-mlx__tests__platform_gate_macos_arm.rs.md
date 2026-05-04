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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
