---
title: discovery.rs
path: kei-llm-mlx/src/discovery.rs
dna_hash: sha256:c01f5f67292e27c0
language: rust
size_loc: 96
generated: by-keidocs
---

# kei-llm-mlx/src/discovery.rs

Binary discovery — `which mlx_lm.generate` / `which mlx_lm.server`.

Constructor Pattern: ONE cube finds the two mlx_lm CLI entry points and
captures their version. Goes through `Runner` so tests can simulate
present-OR-absent without `pip install mlx_lm`.

ENV override: `KEI_MLX_BIN=/path/to/dir` — when set, `which` is
bypassed and we look for `mlx_lm.generate` / `mlx_lm.server` directly
under that directory. Useful for sandbox/CI hosts.

## Public API

- Absolute path to `mlx_lm.generate`, if found.
- Absolute path to `mlx_lm.server`, if found.
- Best-effort version string parsed from `--help` first line.
- `pub fn discover` — Public API — discover binaries via `Runner`.
- Single `which X` lookup. Returns `None` when stdout empty / non-zero
- Parse a version stamp from `<bin> --help` first line. mlx_lm prints
- `pub fn extract_version` — Pull `X.Y.Z` from typical mlx_lm help output. Public so tests can

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
