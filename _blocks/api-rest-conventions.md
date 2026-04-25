# API — REST Conventions (verbs, status codes, resources, idempotency, ETag)

HTTP-level contract for resource-oriented APIs. Pairs with `api-openapi-first.md` (spec as SSoT), `api-versioning-pagination-ratelimit.md` (list + version policy), and `auth-oauth2-oidc.md` / `auth-sessions.md` (principal + scopes).

## When to include

- Public or partner JSON-over-HTTP API where clients are heterogeneous (mobile, SPA, third-party integrations, curl).
- Internal service boundary that you want reviewable by humans without generated tooling.
- Any API that must degrade gracefully through an HTTP cache / proxy / API gateway.

## What it declares

- **Resource naming:** plural nouns, lowercase, kebab-case (`/invoices`, `/invoice-items/{id}`), no verbs in path. Nested resources ≤2 levels deep (`/invoices/{id}/items`); beyond that flatten with query filters. One canonical URL per resource — never two paths for the same entity.
- **Verbs (RFC 9110):** `GET` safe + idempotent, `HEAD` metadata only, `PUT` full replace + idempotent, `PATCH` partial (JSON Merge Patch RFC 7396 OR JSON Patch RFC 6902, pick one per API), `POST` create / non-idempotent action, `DELETE` idempotent. Non-CRUD actions → `POST /resource/{id}:action` (Google AIP-136) or a child resource — never `GET /do-thing`.
- **Status codes — pick from this set, no creativity:** `200 OK`, `201 Created` (+ `Location` header), `202 Accepted` (async), `204 No Content`, `301/308` (moved), `400 Bad Request` (validation), `401 Unauthorized` (no/invalid credential), `403 Forbidden` (authenticated but not allowed), `404 Not Found`, `409 Conflict` (optimistic-lock / duplicate), `410 Gone`, `412 Precondition Failed` (If-Match mismatch), `415 Unsupported Media Type`, `422 Unprocessable Entity` (semantic validation), `429 Too Many Requests`, `500 Internal Server Error`, `502/503/504` (upstream). `418` is a joke, not a status.
- **Error body: RFC 9457 Problem Details** — `{ "type": "https://api.example.com/errors/invoice-not-found", "title": "...", "status": 404, "detail": "...", "instance": "/invoices/42", "errors": [{"field":"amount","code":"negative"}] }`. Content-Type `application/problem+json`. Stable `type` URI = machine key; `title` = human; `detail` = this instance.
- **Idempotency-Key header (Stripe / IETF draft-ietf-httpapi-idempotency-key-header):** required on `POST` that creates/charges. Server stores `(key, route, response)` for ≥24 h and replays on retry. Different body with same key → `422`. Missing key on mutating `POST` → `400` for strict APIs, accept + warn for lenient.
- **Conditional requests (RFC 9110 §13):** `ETag` on every resource representation (strong `"abc123"` unless you truly serve byte-equivalent variants). Clients send `If-Match: "abc123"` on `PUT` / `PATCH` / `DELETE` — server replies `412` on mismatch. `If-None-Match` + `304 Not Modified` on `GET` for cache revalidation. `Last-Modified` as a weaker fallback only.
- **Content negotiation:** `Accept`, `Accept-Language`, `Accept-Encoding` honoured. Default `application/json; charset=utf-8`. Version media types (`application/vnd.example.v2+json`) ONLY if you commit to header-based versioning — see `api-versioning-pagination-ratelimit.md`.
- **HATEOAS / hypermedia:** OPTIONAL. Include a `_links` / `links` object per resource when the API is explicitly browsable (HAL, JSON:API, Siren) — it's not required for typed SDKs. Document the choice in `openapi.yaml` and stay consistent.
- **Safe-by-default surface:** `GET` never mutates. `DELETE` is idempotent — repeated calls return `204` even if the row is already gone. `PUT` requires the FULL representation; partial field on `PUT` = `400`.

## References

- RFC 9110 (HTTP Semantics), RFC 9111 (HTTP Caching), RFC 9457 (Problem Details, 2023), RFC 7396 / 6902 (Merge Patch / JSON Patch), RFC 5988 + 8288 (Web Linking) [E1 — IETF standards-track].
- Google AIP (https://google.aip.dev/) and Microsoft REST API Guidelines (https://github.com/microsoft/api-guidelines) — production-grade conventions [E2].
- `api-openapi-first.md` — encode this block as the machine-readable SSoT; `api-versioning-pagination-ratelimit.md` — list, cursor, and version policy.
- Evidence grade [E2] — every rule here is deployed across Stripe, GitHub, Google, Microsoft production APIs.
