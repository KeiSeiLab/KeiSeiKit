---
title: discovery.rs
path: kei-llm-router/src/discovery.rs
dna_hash: sha256:9f5017226cf7a380
language: rust
size_loc: 147
generated: by-keidocs
---

# kei-llm-router/src/discovery.rs

Cross-backend model discovery.

Constructor Pattern: ONE responsibility — given a `model_id`, ask each
viable backend "do you have this model installed?" and return the
aggregated answers. Each backend's lookup goes through its own crate
(W57 tags / W58 .gguf scan / W59 HF cache).

Match grading:
* **exact**: backend's own identifier is byte-equal to `model_id`
(e.g. Ollama tag `qwen3:4b` vs CLI input `qwen3:4b`).
* **fuzzy**: a substring match in either direction on a normalised
base name (drop suffixes like `-mlx-q4`, `-Q4_K_M`, tag `:Xb`).
* **none**: backend doesn't have it — entry omitted from the result.

## Public API

- Per-backend match outcome.
- Populated on a fuzzy match — the backend-side identifier the
- Walk the machine's viable backends and ask each one whether it has
- `pub fn pick_match` — Public for direct unit-testing — picks an exact then fuzzy match.
- `pub fn normalise_base` — Strip well-known suffixes / quant tags / version separators and

## Related

- parent: `kei-llm-router/Cargo.toml`
- imports: crate, kei_machine_probe, serde

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
