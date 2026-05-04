---
title: sleep.rs
path: kei-decompose/src/parsers/sleep.rs
dna_hash: sha256:d8bfbd90cfd8d479
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-decompose/src/parsers/sleep.rs

Sleep-layer report adapter (RULE 0.15 Phase B/C output).

Detects:
- Frontmatter / commit refs containing `REM:` or `NREM:`
- `## Patterns` section (cross-session pattern listing)
- `## Backlog` section (open follow-ups)

Extracts:
- `- [ ] action` checklist items as Actions
- Pattern rows under `## Patterns` (one Action per pattern)

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
