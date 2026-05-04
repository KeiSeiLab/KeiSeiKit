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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
