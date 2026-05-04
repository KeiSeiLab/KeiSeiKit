---
title: research.rs
path: kei-decompose/src/parsers/research.rs
dna_hash: sha256:b0a2bb816d3f8238
language: rust
size_loc: 193
generated: by-keidocs
---

# kei-decompose/src/parsers/research.rs

/research MASTER-REPORT.md adapter.

Mirrors the kei-decision parser shape (Wave 51): scans for an
`## Actionable plan` / `## Backlog` / `## Action items` section, then
parses the markdown table that follows. The trait wrapper lets the
registry treat it as one adapter among many.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
