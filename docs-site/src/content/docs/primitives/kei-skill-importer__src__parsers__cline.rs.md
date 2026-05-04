---
title: cline.rs
path: kei-skill-importer/src/parsers/cline.rs
dna_hash: sha256:64d395406e81ecad
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-skill-importer/src/parsers/cline.rs

Cline rules parser.

Format research (verified 2026-04-25 via Cline docs page metadata):

- Files live under `.clinerules/*.md` in the project root.
- Frontmatter: optional YAML — common keys are `description`,
`paths` (glob array — file scope filter; NOT a tools-required list),
`tags`. The `paths:` key is a SCOPE FILTER (which files this rule
applies to), not an invocation list.
- Body: free-form markdown. No standard phase convention; many rules
are flat single-message instructions. We map the entire body to
one synthetic phase.
- Bash code-fences are uncommon but valid; classifier picks them up.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, crate, serde, serde_yaml_ng, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
