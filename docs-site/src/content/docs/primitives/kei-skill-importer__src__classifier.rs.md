---
title: classifier.rs
path: kei-skill-importer/src/classifier.rs
dna_hash: sha256:350a3997cfa9c930
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-skill-importer/src/classifier.rs

Atom-call classifier — detects bash fences, `kei-<crate> <verb>`
invocations, and slash commands inside each phase body. Resolves
`atom_id` against a static registry (Wave 26.5 scope; Wave 27 will
swap this for a live `kei-atom-discovery::discover_atoms` lookup).

## Public API

- Static registry of known KeiSeiKit primitive verbs (Wave 26.5).
- `pub fn classify_atom_calls` — Walk every phase, populate `phase.atom_calls`.
- `pub fn has_unresolved_atom_calls` — Public predicate used by the emit-path decision.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: crate, regex, std

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
