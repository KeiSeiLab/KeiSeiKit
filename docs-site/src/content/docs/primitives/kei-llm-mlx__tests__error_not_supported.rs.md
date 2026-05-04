---
title: error_not_supported.rs
path: kei-llm-mlx/tests/error_not_supported.rs
dna_hash: sha256:18804bdaa9c86c20
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-llm-mlx/tests/error_not_supported.rs

On non-target builds, `generate()` MUST return `Error::NotSupported`
before any subprocess attempt — observable via the cfg! gate.

On target builds (macos+aarch64) the test exercises the parallel
shape: it asserts that `is_supported()` agrees with the cfg! gate
AND that `build_spec` over a remote host still refuses with
`Error::SecurityRefused`. Together this keeps the test useful on
every host while pinning the not-supported semantics on every
non-target build.

## Related

- parent: `kei-llm-mlx/tests`
- imports: kei_llm_mlx

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
