---
title: rules.rs
path: kei-conflict-scan/src/scanners/rules.rs
dna_hash: sha256:eecb0ac7c2a3cc59
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/rules.rs

Rule-file conflict detector.

Heuristic: look for contradictory directive pairs like
"never X" vs "prefer X" or "forbidden: X" vs "required: X" across
`rules/*.md`. Tokens compared after stripping filler words.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
