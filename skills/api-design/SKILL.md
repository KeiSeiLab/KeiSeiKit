---
name: api-design
description: Hub-and-spoke pipeline that produces a production-grade API design plan for a new or evolving service — style (REST / GraphQL / tRPC / gRPC), resource model, machine-readable contract (OpenAPI 3.1 or GraphQL SDL), versioning strategy, rate-limit + auth handoff, and codegen toolchain — via pure-click decisions across six phases. Emits a spec skeleton, SDK pick list, and a per-surface checklist; never writes secrets.
argument-hint: <one-line API description, e.g. "B2C SaaS, public REST API, 3 clients, cursor pagination, needs Google SSO">
---

# API-Design — Style, Contract & Lifecycle Pipeline (index)

## When to use

- Designing a new public or internal API from scratch (REST, GraphQL, tRPC, gRPC).
- Producing an OpenAPI 3.1 or GraphQL SDL contract skeleton before implementation begins.
- Deciding versioning strategy, rate-limit policy, and codegen toolchain for an evolving service.

You are converting "I need an API for X" into a concrete, reviewable plan:
which style to ship, what resources exist, what the machine-readable
contract looks like, how versions evolve, how rate limits + auth integrate,
and which codegen toolchain produces the server stubs + SDKs + docs. Every
decision is a click; the only typed input is the one-line description in
Phase 1 and the resource list in Phase 2.

This skill does NOT write production server code. It emits a PLAN plus a
contract SKELETON (OpenAPI 3.1 YAML scaffold OR GraphQL SDL scaffold), a
versioning decision row, a rate-limit policy row, and a codegen pick list.
Server scaffolding is a separate task owned by `new-agent` or the project's
code-implementer; auth wiring is delegated to `/auth-setup`.

The skill reads the four companion blocks heavily — every phase references
at least one of them:

- `_blocks/api-rest-conventions.md` — verbs, status codes, resources, ETag, idempotency.
- `_blocks/api-openapi-first.md` — OpenAPI 3.1 SSoT + codegen tooling.
- `_blocks/api-graphql.md` — SDL, resolvers, DataLoader, subscriptions, persisted queries.
- `_blocks/api-versioning-pagination-ratelimit.md` — strategies matrix.

---

## Pipeline overview (6 phases, ≥6 AskUserQuestion calls)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Style, audience, scale, clients | 3× |
| 2 | [phase-2-resource-model.md](phase-2-resource-model.md) | Entities → REST resources / GraphQL types | 1× |
| 3 | [phase-3-contract.md](phase-3-contract.md) | Generate OpenAPI spec OR GraphQL SDL skeleton | 1× |
| 4 | [phase-4-versioning.md](phase-4-versioning.md) | URL / header / date-based decision | 1× |
| 5 | [phase-5-limits-auth.md](phase-5-limits-auth.md) | Pagination + rate limit + auth-setup handoff | 1× |
| 6 | [phase-6-codegen.md](phase-6-codegen.md) | openapi-generator / orval / graphql-codegen | 1× |

Minimum AskUserQuestion count across a full session: **8** (3 in Phase 1 +
1 each in Phases 2–6). Exceeds the ≥6 hub-and-spoke contract.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `STYLE` | Phase 1 | REST / GraphQL / tRPC / gRPC / hybrid |
| `AUDIENCE` | Phase 1 | public / partner / internal |
| `SCALE` | Phase 1 | small (<100 rps) / mid (100–10k) / large (>10k rps) |
| `CLIENTS` | Phase 1 | subset of {web-spa, mobile-native, server-to-server, cli, browser-form} |
| `RESOURCES` | Phase 2 | ordered list of entities + relationships (one-to-many, many-to-many) |
| `CONTRACT` | Phase 3 | path to generated `openapi.yaml` OR `schema.graphql` skeleton |
| `VERSIONING` | Phase 4 | url-path / header-media / date / additive-only / graphql-deprecate |
| `PAGINATION` | Phase 5 | cursor / offset / relay-connection |
| `RATELIMIT` | Phase 5 | per-principal bucket + per-endpoint policy row |
| `AUTH_HANDOFF` | Phase 5 | recorded decision to run `/auth-setup` next (or skipped + why) |
| `CODEGEN` | Phase 6 | generator(s) + target languages |

---

## Final report (emit after Phase 6)

```
=== API-DESIGN REPORT ===
Description:  <first 80 chars of intake>...
Style:        <STYLE>
Audience:     <AUDIENCE>   Scale: <SCALE>   Clients: <CLIENTS list>
Resources:    <N entities, M relationships>
Contract:     <CONTRACT path> — <lines LOC> skeleton
Versioning:   <VERSIONING> + deprecation runway <N months>
Pagination:   <PAGINATION>  RateLimit: <policy summary>
Auth handoff: <AUTH_HANDOFF>  (run /auth-setup next? yes/no)
Codegen:      <CODEGEN list — one line per target>
Env vars:     <count> new entries (none if managed-only)
Next:         run `compose-solution` or hand off to project code-implementer
```

---

## Rules (apply throughout)

- **Pure-click contract.** Only the Phase 1 intake paragraph and the Phase 2
  resource list are typed. Every other decision is `AskUserQuestion`.
- **RULE 0.4 NO HALLUCINATION.** Never claim an OpenAPI feature, RFC number,
  or library capability without citing the spec link. If unsure, mark
  `[UNVERIFIED]` in the report and flag a follow-up. No fabricated version
  numbers, no invented library features, no made-up SDK names.
- **RULE 0.8 Secrets SSoT.** The skill emits env VARIABLE NAMES only
  (`STRIPE_API_KEY_NAME`, `RATE_LIMIT_REDIS_URL`, ...). It NEVER echoes a
  token value, never writes to `secrets/*.env`, never suggests hard-coding.
- **NO DOWNGRADE.** If the chosen combination is unsafe or contradictory
  (e.g. "public API + additive-only versioning + no deprecation runway")
  the skill returns 2–3 constructive alternatives, never "not supported".
- **Fail-closed default.** Rate limiter, auth check, and contract-drift
  gate all default to the safer option when the user is unsure.
- **Surgical scope.** Reads the four API blocks; writes a contract
  skeleton file (Phase 3) and nothing else. Production scaffolding is
  delegated. Auth wiring is delegated to `/auth-setup`.

---

## References

- `_blocks/api-rest-conventions.md`, `_blocks/api-openapi-first.md`,
  `_blocks/api-graphql.md`, `_blocks/api-versioning-pagination-ratelimit.md`.
- `_blocks/rule-pre-dev-gate.md` — analogue check before inventing resources.
- `skills/auth-setup/SKILL.md` — Phase 5 handoff target.
- Evidence grade [E2] — pipeline mirrors Stripe, GitHub, Shopify, Twilio
  production API lifecycles.
