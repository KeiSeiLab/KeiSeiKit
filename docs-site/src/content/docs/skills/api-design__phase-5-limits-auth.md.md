---
title: phase-5-limits-auth
path: api-design/phase-5-limits-auth.md
dna_hash: sha256:14e3e348869bdacd
language: markdown
size_loc: 138
generated: by-keidocs
---

# api-design/phase-5-limits-auth.md

## Public API

- `Phase 5 — Pagination + rate limits + auth handoff` — Lock the three cross-cutting concerns that bite every production API in
- `5a — Combined click (AskUserQuestion, multi-select, pre-checked)` — Single AskUserQuestion with three axes fused to stay within the
- `5b — Emit pagination contract (inline)` — For `PAGINATION = cursor`:
- `OpenAPI skeleton — added to components.parameters` — Cursor:
- `GraphQL skeleton — already emitted in Phase 3` — type FooConnection { edges: [FooEdge!]! pageInfo: PageInfo! totalCount: Int }
- `5c — Emit rate-limit policy table (inline)` — Print a table the user fills in numbers for. Example tiers:
- `5d — Emit auth handoff (inline)` — - If `AUTH_HANDOFF = run-auth-setup`:
- `Verify-criterion` — - `PAGINATION`, `RATELIMIT`, `AUTH_HANDOFF` all set.

## Related

- parent: `api-design`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
