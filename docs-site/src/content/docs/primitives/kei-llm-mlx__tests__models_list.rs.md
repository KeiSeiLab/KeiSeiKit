---
title: models_list.rs
path: kei-llm-mlx/tests/models_list.rs
dna_hash: sha256:7ddf2bcba7c67844
language: rust
size_loc: 33
generated: by-keidocs
---

# kei-llm-mlx/tests/models_list.rs

Model discovery — fixture cache directory with three children.

Layout:
models--mlx-community--Llama-3.2-3B-Instruct-4bit  ← MLX, 4 bits
models--mlx-community--phi-3-mini-mlx-q8           ← MLX, 8 bits
models--meta-llama--Llama-3.2-3B-Instruct          ← NOT mlx

Expected: 2 entries returned (sorted by hf_id).

## Related

- parent: `kei-llm-mlx/tests`
- imports: kei_llm_mlx, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
