---
title: matcher.rs
path: kei-import-project/src/matcher.rs
dna_hash: sha256:1db8640eb8037f79
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-import-project/src/matcher.rs

matcher — heuristic trait-pattern matcher over a ModuleSource.

Uses regex-based extraction of impl blocks and method names for
improved precision over raw substring search. Avoids false positives
from comments and string literals. No syn/AST dependency.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- Confidence threshold below which a match is omitted.
- A single trait-match result for one pattern.
- Normalised confidence in [0.0, 1.0].
- Required methods that were found in the source.
- Indicator keywords that were found in the source.
- `pub fn match_module` — Analyse all source files in `source` and return confident trait matches.
- Replace string literal contents + line comments with spaces.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
