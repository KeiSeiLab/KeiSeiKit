---
title: phase-2-resource-model
path: api-design/phase-2-resource-model.md
dna_hash: sha256:0c2b22141f0c484f
language: markdown
size_loc: 93
generated: by-keidocs
---

# api-design/phase-2-resource-model.md

## Public API

- `Phase 2 — Resource model (entities → resources / types)` — Turn the app description into a list of entities, their relationships, and
- `2a — Ask for entities + relationships (typed)` — Emit a regular message (NOT AskUserQuestion):
- `2b — Shape click (AskUserQuestion, single-select)` — Reference: `_blocks/api-rest-conventions.md` (REST resources),
- `2c — Emit resource-to-action matrix (inline, no AskUserQuestion)` — Print a table the user can tweak before Phase 3 generates the contract.
- `Verify-criterion` — - `RESOURCES` has ≥1 entry; parsed shape `{name, owns, many_to_many}` valid.

## Related

- parent: `api-design`

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
