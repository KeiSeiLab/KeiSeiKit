---
title: phase-4-versioning
path: api-design/phase-4-versioning.md
dna_hash: sha256:79a83c2b1e5ba479
language: markdown
size_loc: 111
generated: by-keidocs
---

# api-design/phase-4-versioning.md

## Public API

- `Phase 4 — Versioning strategy` — Decide how the API evolves when backwards-incompatible changes happen.
- `4a — Strategy click (AskUserQuestion, single-select)` — ```json
- `4b — Deprecation runway click (AskUserQuestion, single-select)` — Only if `VERSIONING != additive-only` AND `VERSIONING != GraphQL evolution`.
- `4c — Emit deprecation headers snippet (inline, no AskUserQuestion)` — Print the standards-track header contract — RFC 8594 (Sunset) + RFC 9745
- `4d — Emit changelog + telemetry obligations (inline)` — For any non-trivial versioning choice, print:
- `Verify-criterion` — - `VERSIONING` exactly one choice, compatible with `STYLE`.

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
