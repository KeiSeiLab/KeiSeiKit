---
title: models.rs
path: kei-llm-mlx/src/models.rs
dna_hash: sha256:d8d0d25f25bb8b55
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-llm-mlx/src/models.rs

Model discovery — scan HuggingFace cache for MLX-quantised entries.

mlx_lm caches models in `~/.cache/huggingface/hub/`, with each entry
named `models--<org>--<repo>`. We treat an entry as "MLX-quantised" if
its repo segment matches one of:
*-mlx-q4 / *-mlx-q8 / *-4bit / *-8bit / *-mlx (suffix)
and infer `quant_bits` from the suffix.

Constructor Pattern: this cube ONLY scans the directory tree and
classifies via regex. No network, no Runner, no I/O beyond `read_dir`.

## Public API

- HuggingFace id, e.g. `mlx-community/Llama-3.2-3B-Instruct-4bit`.
- Absolute on-disk cache directory.
- Best-effort quantisation width parsed from the repo suffix.
- `pub fn list_models` — Public API — scan a HuggingFace hub cache directory and return all
- `pub fn classify` — Decide whether `name` looks like a HF cache dir for an MLX-quantised
- Convert `models--org--repo` into `org/repo`. Returns `None` for any
- `pub fn is_mlx_quantised` — Match the five canonical suffix patterns. Visible for unit tests.
- `pub fn parse_quant_bits` — Parse 4 or 8 from the suffix. `None` for plain `-mlx`.
- `pub fn default_cache_dir` — Default cache dir, `~/.cache/huggingface/hub`. Returns `None` if HOME

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
