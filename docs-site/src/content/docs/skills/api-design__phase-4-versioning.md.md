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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
