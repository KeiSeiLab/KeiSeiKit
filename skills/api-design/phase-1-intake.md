# Phase 1 — Intake (style, audience, scale, clients)

One free-text paragraph, then three click batches. This is one of only two
phases that accepts typed input (the other is Phase 2's resource list).

## 1a — Ask for the API description

Emit a regular message (NOT AskUserQuestion):

> Describe the API in one paragraph: what problem does it solve, who calls
> it, and any constraint I should know (regulated, existing schema,
> mobile-first, partner network, realtime). Reply in one message.

Store the reply verbatim as `INTAKE`.

## 1b — Style click (AskUserQuestion, single-select)

Reference: `_blocks/api-rest-conventions.md`, `_blocks/api-graphql.md`.

```json
{
  "questions": [
    {
      "question": "Which API style fits the problem?",
      "header": "Style",
      "multiSelect": false,
      "options": [
        {"label": "REST + JSON",
         "description": "Resource-oriented, HTTP-native, CDN-cacheable. Default for public APIs with heterogeneous clients. See _blocks/api-rest-conventions.md"},
        {"label": "GraphQL",
         "description": "Client-shaped queries, graph-shaped domain, federation-ready. Apollo / Yoga / async-graphql. See _blocks/api-graphql.md"},
        {"label": "tRPC (TypeScript end-to-end)",
         "description": "TS-only monorepo; server exports types, client imports. Zero codegen. NOT suitable for multi-language consumers."},
        {"label": "gRPC / Protobuf",
         "description": "Service-to-service, strong typing, streaming. Over HTTP/2. Browsers need grpc-web. Choose for internal backbones."},
        {"label": "Hybrid (REST public + GraphQL internal, or gRPC internal + REST edge)",
         "description": "Common at scale — record BOTH surfaces, run this skill twice if needed."}
      ]
    }
  ]
}
```

Store as `STYLE`. If `Hybrid` → warn user that this skill will design the
PRIMARY surface first; they can re-run for the secondary.

## 1c — Audience + scale click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Audience + traffic class?",
      "header": "Audience+Scale",
      "multiSelect": false,
      "options": [
        {"label": "Public internet, small (<100 rps)",
         "description": "Landing-site API, indie SaaS MVP. Rate limit per IP + per key."},
        {"label": "Public internet, mid (100–10k rps)",
         "description": "Growth-stage product. Needs proper quotas, partner tiers, SDK story."},
        {"label": "Public internet, large (>10k rps)",
         "description": "Stripe / GitHub tier. Date-based versioning, cost-based limits, federation."},
        {"label": "Partner / B2B only",
         "description": "Known callers, NDA, mTLS or signed requests possible. Simpler abuse surface."},
        {"label": "Internal service boundary",
         "description": "Inside the VPC / mesh. Skip public rate limits; keep contract tests."}
      ]
    }
  ]
}
```

Store label components as `AUDIENCE` (public / partner / internal) and
`SCALE` (small / mid / large — internal defaults to `small` unless noted).

## 1d — Clients click (AskUserQuestion, multi-select)

```json
{
  "questions": [
    {
      "question": "Which clients will consume the API?",
      "header": "Clients",
      "multiSelect": true,
      "options": [
        {"label": "Web SPA (React / Svelte / Vue)",
         "description": "CORS, HttpOnly session cookie or Bearer; SDK in TS"},
        {"label": "Mobile native (iOS / Android)",
         "description": "Typed SDK (Swift / Kotlin) — favours OpenAPI codegen or GraphQL client"},
        {"label": "Server-to-server",
         "description": "Partner backends; mTLS, signed requests, long-lived API keys"},
        {"label": "CLI / scripts",
         "description": "curl-friendly, URL-path versioning preferred, stable query params"},
        {"label": "Third-party developers (public docs)",
         "description": "Needs Swagger UI / Redoc / Scalar + SDK published to npm / crates.io / PyPI"},
        {"label": "Browser form / webhook receiver",
         "description": "Content-Type application/x-www-form-urlencoded or webhook body; idempotency-key required"}
      ]
    }
  ]
}
```

Store as `CLIENTS`. Empty selection → re-ask (an API without a known
client is premature; push back per NO DOWNGRADE with suggested defaults).

## Verify-criterion

- `INTAKE` non-empty (≥40 chars).
- `STYLE` exactly one label.
- `AUDIENCE` and `SCALE` parsed from the Phase 1c label.
- `CLIENTS` has ≥1 entry.
- If `STYLE = tRPC` and `CLIENTS` contains any non-TS client → STOP and
  re-ask 1b (tRPC is TS-only; offer REST or GraphQL alternatives per NO
  DOWNGRADE).
