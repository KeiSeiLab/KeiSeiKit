# Phase 5 — Pagination + rate limits + auth handoff

Lock the three cross-cutting concerns that bite every production API in
its first month. Reference: `_blocks/api-versioning-pagination-ratelimit.md`.
Delegates auth wiring to `skills/auth-setup/SKILL.md`.

## 5a — Combined click (AskUserQuestion, multi-select, pre-checked)

Single AskUserQuestion with three axes fused to stay within the
≥6-AskUserQuestion budget. Pre-select the fail-closed defaults; opting out
requires a click.

```json
{
  "questions": [
    {
      "question": "Confirm the pagination + rate-limit + auth policy (pre-checked fail-closed defaults; deselect only with a compensating control).",
      "header": "Limits+Auth",
      "multiSelect": true,
      "options": [
        {"label": "Pagination: cursor (opaque, keyset)",
         "description": "REQUIRED for any list that can exceed ~1k rows. Response envelope {data, meta:{next_cursor, has_more}}."},
        {"label": "Pagination: offer offset/page too (admin UIs only)",
         "description": "Accept for admin screens where page numbers are expected; clamp limit ≤100 server-side."},
        {"label": "Pagination: Relay Connections (GraphQL only)",
         "description": "Required if STYLE=GraphQL. edges/pageInfo/endCursor per Relay spec."},
        {"label": "Rate limit: per-principal token bucket",
         "description": "Redis-backed. Default tiers: anon < authenticated < partner < internal."},
        {"label": "Rate limit: per-endpoint cost budget",
         "description": "Expensive routes (search, export) get their own budget. GraphQL uses cost-based analyser instead."},
        {"label": "Rate limit: per-IP sliding window (anti-bot)",
         "description": "Defence-in-depth layer. Still applies under auth failures / unauthenticated endpoints."},
        {"label": "Rate-limit headers: IETF RateLimit-* + Retry-After",
         "description": "RateLimit-Limit / RateLimit-Remaining / RateLimit-Reset (IETF draft, 2024 deployed). Plus Retry-After on 429."},
        {"label": "Auth: delegate to /auth-setup (RECOMMENDED)",
         "description": "Runs the hub-and-spoke auth pipeline after this skill finishes — OAuth / passkey / sessions / RBAC."},
        {"label": "Auth: API-key only (server-to-server)",
         "description": "Partner / internal S2S. Long-lived keys stored per client; rotation policy required."},
        {"label": "Auth: mTLS (internal service mesh)",
         "description": "Pick for internal boundaries in a mesh (Istio / Linkerd). Record the CA; no token on the wire."},
        {"label": "Auth: none (open public API)",
         "description": "Rate limits + anti-bot must compensate. Acceptable only for truly-public read-only data."}
      ]
    }
  ]
}
```

Parse the selection into three variables:

- `PAGINATION` ← the pagination option(s) picked (must be ≥1).
- `RATELIMIT` ← the list of rate-limit layers selected.
- `AUTH_HANDOFF` ← one of `run-auth-setup`, `api-key`, `mtls`, `none`.

Validation gates (NO DOWNGRADE: offer alternatives instead of rejecting):

- `STYLE = GraphQL` AND `PAGINATION` does not include "Relay" → STOP,
  re-ask — Relay is the standard for GraphQL list pagination.
- `AUDIENCE = public` AND `AUTH_HANDOFF = none` AND no per-IP rate limit →
  STOP, re-ask with the warning "public + no auth + no per-IP = abuse
  vector".
- `RATELIMIT` empty AND `AUDIENCE != internal` → STOP, re-ask with the
  warning "rate limits mandatory for non-internal APIs".

## 5b — Emit pagination contract (inline)

For `PAGINATION = cursor`:

```yaml
# OpenAPI skeleton — added to components.parameters
Cursor:
  name: cursor
  in: query
  schema: { type: string }
  description: Opaque cursor returned by the previous response.
Limit:
  name: limit
  in: query
  schema: { type: integer, minimum: 1, maximum: 100, default: 50 }
```

For `PAGINATION = Relay Connections`:

```graphql
# GraphQL skeleton — already emitted in Phase 3
type FooConnection { edges: [FooEdge!]! pageInfo: PageInfo! totalCount: Int }
type FooEdge { node: Foo! cursor: String! }
type PageInfo { hasNextPage: Boolean! hasPreviousPage: Boolean! startCursor: String endCursor: String }
```

## 5c — Emit rate-limit policy table (inline)

Print a table the user fills in numbers for. Example tiers:

```markdown
| Tier          | Requests/min | Burst | Notes                              |
|---------------|:------------:|:-----:|------------------------------------|
| anonymous     |      30      |  60   | per-IP sliding window              |
| authenticated |     600      | 1000  | per-principal token bucket         |
| partner       |    6000      |10000  | per-API-key; negotiable in contract|
| internal      |    none      |  -    | inside VPC, trust boundary         |
```

Plus the header contract:

```http
RateLimit-Limit: 600
RateLimit-Remaining: 547
RateLimit-Reset: 42
Retry-After: 30            # only on 429 responses
```

## 5d — Emit auth handoff (inline)

- If `AUTH_HANDOFF = run-auth-setup`:
  print "Next step: run `/auth-setup` with argument `<first 80 chars of INTAKE>`".
  Record this in the final report as the recommended next command.
- If `AUTH_HANDOFF = api-key`:
  emit env-var scaffold into `secrets/api.env` (names only, per RULE 0.8):
  ```bash
  # secrets/api.env — chmod 600, gitignored
  API_KEY_ISSUER_SECRET=
  API_KEY_ROTATION_DAYS=90
  ```
- If `AUTH_HANDOFF = mtls`:
  emit reminder — "Record CA fingerprint in `docs/mtls-trust.md`; client
  cert rotation cadence in runbook."
- If `AUTH_HANDOFF = none`:
  record explicitly in the final report as an accepted risk.

## Verify-criterion

- `PAGINATION`, `RATELIMIT`, `AUTH_HANDOFF` all set.
- At least one rate-limit layer selected unless `AUDIENCE = internal`.
- No literal token value appears in the emitted text (RULE 0.8).
- If `AUTH_HANDOFF = run-auth-setup`, the next-command line is printed.
- No fabricated rate-limit numbers — table cells are placeholders clearly
  marked as defaults ("fill during capacity planning").
