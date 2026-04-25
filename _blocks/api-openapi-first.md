# API — OpenAPI-First (3.1 as single source of truth)

Machine-readable contract that drives server stubs, client SDKs, docs, mocks, and contract tests from ONE file. Pairs with `api-rest-conventions.md` (the HTTP rules the spec encodes) and `api-versioning-pagination-ratelimit.md` (versioning + pagination schemas).

## When to include

- Any REST API with ≥2 consumers (web + mobile, public + partner, multiple internal services).
- API that must publish SDKs in >1 language — spec-driven codegen beats hand-written clients per language.
- Regulated API (finance / health) where the contract must be reviewable and diff-able as a single artefact.

## What it declares

- **OpenAPI 3.1.0** — the 2021+ version that is a strict superset of JSON Schema 2020-12. Use 3.1 unless a specific tool pins you to 3.0.x; 2.0 (Swagger) is legacy and missing `oneOf/anyOf/nullable` nuances.
- **Single file, single source of truth:** `openapi.yaml` (or `.json`) committed at repo root or under `api/`. ALL of the following are GENERATED, never hand-written:
  - Server routing stubs / request validators (codegen for your stack).
  - Typed client SDKs (TS, Swift, Kotlin, Python, Rust, Go).
  - Human docs site (Swagger UI / Redoc / Scalar / Stoplight Elements).
  - Mock server (Prism, mswjs, Stoplight) for consumer tests before the backend exists.
  - Contract tests (Schemathesis, Dredd, Pact broker feed).
- **Structure:** `info`, `servers` (per environment — prod, staging, sandbox), `paths` (one entry per resource/action pair), `components.schemas` (reusable types), `components.securitySchemes` (bearer / OAuth2 / API-key), `components.parameters` (shared query params like `page`, `cursor`, `limit`), `components.responses` (problem+json 400 / 401 / 403 / 404 / 409 / 422 / 429 / 500 reused by `$ref`), `tags` (grouping for docs).
- **Schemas ARE types:** every `$ref` resolves to `components/schemas/*`; no anonymous objects inline inside responses. This makes the codegen output readable and re-usable.
- **Error model is shared:** define `Problem` schema once (RFC 9457 shape) and `$ref` it from every 4xx/5xx response. Keeps the error contract identical across 120 endpoints.
- **Examples are typed:** every operation has ≥1 request example + ≥1 response example. Examples flow into Redoc docs, mock server responses, and SDK fixtures. Invalid examples break CI — treat them as test data.
- **Tooling pick — ONE per job:**
  - Lint: **Spectral** (`.spectral.yaml` with a ruleset — Google/Microsoft API guidelines ship starter rulesets).
  - Diff / breaking-change gate: **oasdiff** or **openapi-diff** in CI — PR fails on a breaking change unless `breaking: approved` label.
  - Codegen: **openapi-generator** (multi-language, mature; prefer `*-axios`, `*-nullable` templates for TS); **orval** for TS + React Query / SWR first-class; **oapi-codegen** for Go; **progenitor** for Rust.
  - Docs: **Redoc** (read-only, pretty), **Swagger UI** (interactive), **Scalar** (modern, fast), **Stoplight Elements** (embeddable React component). Pick one — documented decision in repo.
- **Governance:** `openapi.yaml` change = PR review like code. No drift between spec and server: CI runs the generated server stubs AND contract tests against the running app.
- **[UNVERIFIED] claims — forbidden:** never quote an OpenAPI feature without checking the 3.1 spec. `discriminator`, `oneOf`, `nullable` (removed — use `type: [string, "null"]`) are easy to get wrong; cite spec link on debate.

## References

- OpenAPI 3.1.0 spec (https://spec.openapis.org/oas/v3.1.0) [E1 — normative].
- JSON Schema 2020-12 (https://json-schema.org/specification.html) [E1].
- RFC 9457 Problem Details + `api-rest-conventions.md` for the HTTP semantics the spec encodes.
- Swagger UI / Redoc / Scalar / Stoplight Elements — all actively maintained as of 2026 [E2].
- openapi-generator (https://openapi-generator.tech/), orval (https://orval.dev/), oapi-codegen (https://github.com/oapi-codegen/oapi-codegen) [E2 — production-deployed].
- Evidence grade [E2] — pattern is the Stripe / GitHub / Twilio / Shopify default.
