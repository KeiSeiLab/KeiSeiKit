---
title: recommendations.rs
path: kei-machine-probe/src/recommendations.rs
dna_hash: sha256:7891a5791a1229ce
language: rust
size_loc: 144
generated: by-keidocs
---

# kei-machine-probe/src/recommendations.rs

`Machine` → `Recommendations` heuristics.

Decision tree, all rationale strings preserved in the output:
1. OS != macOS                        → NotViable
2. AppleSilicon + ≥16 GB              → RunsLargeModels (Mlx, LlamaCpp, Ollama)
3. AppleSilicon + ≥8 GB <16 GB        → RunsMidModels   (Mlx, LlamaCpp, Ollama)
4. AppleSilicon + <8 GB                → RunsSmallModelsOnly (LlamaCpp, Ollama)
5. Intel + ≥16 GB                      → RunsMidModels   (LlamaCpp, Ollama)
6. Intel + <16 GB                      → RunsSmallModelsOnly (Ollama)
7. Other                               → NotViable

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate, serde

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
