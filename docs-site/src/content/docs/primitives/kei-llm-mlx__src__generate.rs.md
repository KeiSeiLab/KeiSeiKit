---
title: generate.rs
path: kei-llm-mlx/src/generate.rs
dna_hash: sha256:ef996f51f383bf7d
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-llm-mlx/src/generate.rs

Non-streaming generate — `mlx_lm.generate --model X --prompt P`.

Constructor Pattern: this cube builds the argv, calls the Runner,
and parses the canonical mlx_lm stdout footer:

```text
<generated text>
==========
Prompt: 12 tokens, 132.4 tokens-per-sec
Generation: 64 tokens, 78.9 tokens-per-sec
```

The footer regex is permissive — minor mlx_lm version drift in
punctuation (`tokens-per-sec` vs `tokens per second`) is tolerated.

## Public API

- `pub fn generate` — Run a single non-streaming generation.
- `pub fn build_argv` — Build argv for `mlx_lm.generate`. Visible for tests.
- `pub fn parse_response` — Split stdout into `(generation_text, footer_lines)` and decode the

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
