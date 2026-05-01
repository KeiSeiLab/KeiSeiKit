# API — Versioning, Pagination, Rate Limiting

Three cross-cutting concerns that every production API hits within the first month. Pairs with `api-rest-conventions.md` (HTTP semantics), `api-openapi-first.md` (where the policy is encoded), and `api-graphql.md` (Relay cursors + cost-based limits).

## When to include

- Any API expected to outlive one client release — versioning decided BEFORE launch, not during the first breaking change.
- Any endpoint returning a collection — pagination decided BEFORE the dataset grows past 10k rows.
- Any API on the public internet or behind a partner quota — rate limits decided BEFORE the first abusive client.

## What it declares

### Versioning — pick one strategy, document it

| Strategy | Example | Pros | Cons | Use when |
|---|---|---|---|---|
| **URL path** | `/v1/invoices` → `/v2/invoices` | Most visible, curl-friendly, easy CDN routing | Pollutes every path; "v2" is vague | Public API, coarse versions, infrequent bumps. GitHub v3/v4, Stripe-compatible mirrors. |
| **Header (media type)** | `Accept: application/vnd.example.v2+json` | Clean URLs, content negotiation native | Invisible in logs/curl; needs client support | Internal APIs with typed SDKs, GitHub v4 hybrid. |
| **Date-based** | `Stripe-Version: 2025-11-01` | Fine-grained, every breaking change pinnable | Complex rollout matrix; server must keep N-1 versions live | Pay-for-stability APIs (Stripe, Shopify); regulated domains. |
| **GraphQL evolution** | Never break the schema; mark fields `@deprecated(reason: "use X")` and remove after telemetry shows 0 usage | No versions to maintain | Schema grows forever; deprecation discipline required | Any GraphQL API — see `api-graphql.md`. |
| **No versioning (additive-only)** | Promise: additions never break clients; removals need a new endpoint | Simplest | Only works with disciplined teams + strong typing | Small internal APIs with ≤3 consumers. |

Rules that apply to ALL strategies: (a) deprecate with `Deprecation` + `Sunset` headers (RFC 8594, RFC 9745) + 6-month minimum runway, (b) publish a changelog, (c) run the old + new in parallel until telemetry shows the old is unused.

### Pagination — three patterns, one rule

- **Offset / page (LIMIT N OFFSET M):** `?page=3&limit=50`. OK for admin UIs over small tables. BROKEN for real data — rows drift during paging, `OFFSET 10000` scans 10k rows on every call. Returns `X-Total-Count` or a `meta.total` field; clients assume random access.
- **Cursor (opaque token, keyset/seek):** `?cursor=eyJpZCI6MTIzfQ&limit=50`. Cursor = base64 of `(id, created_at, …)` — opaque to client, ordered by the server's index. Handles drift, O(log n) lookups. Response envelope: `{ data: [...], meta: { next_cursor, prev_cursor, has_more } }`. REQUIRED for any list that can exceed 1k rows or where concurrent writes happen.
- **Relay (GraphQL spec):** `first: 50, after: "cursor"` + `Connection { edges, pageInfo { endCursor, hasNextPage } }`. Standardised cursor pattern for GraphQL — see `api-graphql.md`.

Rule: **default cursor, offer offset only when the UI genuinely needs page numbers**. Never return >1000 items per page; clamp `limit` server-side.

### Rate limiting — headers + strategy

- **Token bucket or sliding-window**, per authenticated principal (user / API key / IP). Redis-backed, atomic via Lua. Policy tiers: anon < authenticated < partner < internal.
- **Response headers — IETF `RateLimit` (draft-ietf-httpapi-ratelimit-headers, shipped in Cloudflare / GitHub as of 2024):**
  - `RateLimit-Limit: 1000` — quota in the current window.
  - `RateLimit-Remaining: 947` — left in the current window.
  - `RateLimit-Reset: 47` — seconds until reset.
  - Also accept legacy `X-RateLimit-*` for GitHub/Stripe parity during migration.
- **On block: `429 Too Many Requests` + `Retry-After: <seconds>`** (RFC 9110 §10.2.3) + Problem+json body describing the limit that was hit. Always include `Retry-After`; idempotent clients retry cleanly.
- **Cost-based for GraphQL:** each field has a cost (e.g. `user: 1, user.posts: 5 per item, search: 50`); query total checked against per-principal budget. See `api-graphql.md`.
- **Fail-open on metering outage** is a bug, not a feature — fail-closed with a clear error code (`RATE_LIMITER_UNAVAILABLE`) so clients can alert. Silent "no limit" costs more than a short outage.
- **Defence-in-depth:** per-IP (anti-bot), per-principal (anti-abuse), per-endpoint (protect expensive routes), global (protect the cluster). Document all four layers in the repo — hidden layers surprise on-call.

## References

- RFC 8594 (Sunset header), RFC 9745 (Deprecation header, 2024), RFC 9110 §10.2.3 (`Retry-After`) [E1 — IETF].
- draft-ietf-httpapi-ratelimit-headers (https://datatracker.ietf.org/doc/draft-ietf-httpapi-ratelimit-headers/) [E1 — active working group draft, deployed by Cloudflare + GitHub].
- Relay Cursor Connections (https://relay.dev/graphql/connections.htm) [E1].
- Stripe API versioning post (https://stripe.com/blog/api-versioning) [E2 — production-documented 2017 onward].
- GitHub v3 → v4 migration notes, Shopify API versioning [E2].
- Evidence grade [E2] — all three policies are production-deployed at Stripe, GitHub, Shopify, Cloudflare.
