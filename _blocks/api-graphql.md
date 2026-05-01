# API — GraphQL (schema-first, DataLoader, subscriptions, persisted queries)

Single-endpoint, client-driven query language. Pairs with `auth-sessions.md` / `auth-authorization.md` (identity + field-level authz) and `api-versioning-pagination-ratelimit.md` (Relay cursors + cost-based rate limits).

## When to include

- Client needs shape each response themselves (mobile bandwidth, SPA over-fetch, UI-driven demand).
- Graph-shaped domain (social, sharing, org charts, document tree) where REST nesting explodes.
- Multiple teams own different resolvers behind one gateway (federation / subgraphs).

## What it declares

- **Schema-first, not code-first:** `schema.graphql` is the SSoT, committed to the repo. Resolvers are generated types (TS `graphql-codegen`, Rust `async-graphql` derive, Go `gqlgen`) that must implement the schema. Schema-first beats code-first for reviewability, federation, and client codegen.
- **SDL only, no custom DSL:** use standard GraphQL SDL — `type`, `input`, `interface`, `union`, `enum`, `scalar`, directives. Custom scalars (`DateTime`, `UUID`, `JSON`) declared once; keep the list short.
- **Resolver structure (Apollo / urql / Relay agnostic):** one resolver per field; resolvers return values OR a loader handle, never hit the DB directly in a loop — that's the N+1 trap.
- **DataLoader for every 1-to-many or many-to-many field:** Facebook's `dataloader` pattern (batch + per-request cache). Without it, a query `users { posts { comments { author { name } } } }` issues O(N³) queries; with it, exactly 4. Implementations: `dataloader` (JS, reference), `async-graphql` built-in (Rust), `graphql-dataloader` (Go), `aiodataloader` (Python).
- **Pagination: Relay cursor spec** — `type FooConnection { edges: [FooEdge!]! pageInfo: PageInfo! totalCount: Int } type FooEdge { node: Foo! cursor: String! } type PageInfo { hasNextPage: Boolean! hasPreviousPage: Boolean! startCursor: String endCursor: String }`. See `api-versioning-pagination-ratelimit.md`.
- **Errors:** don't throw — return the GraphQL error envelope. Expected errors (not-found, unauthorized, validation) go in `errors[]` with `extensions.code` taxonomy (`NOT_FOUND`, `FORBIDDEN`, `BAD_USER_INPUT`, `RATE_LIMITED`). Unexpected errors → generic `INTERNAL_SERVER_ERROR`, server-side logged with correlation id.
- **Subscriptions — pick transport explicitly:** **graphql-ws** (RFC-like WebSocket sub-protocol, Apollo-server + urql default; replaces the deprecated `subscriptions-transport-ws`) OR **graphql-sse** (HTTP Server-Sent Events, no WS infra). WebSocket needs auth on `connectionInit` (token in payload), reconnect strategy, and a resumable cursor — SSE is simpler where you don't need client→server push.
- **Persisted queries (APQ / PQ):** hash the query at build time, send only the hash at runtime. Stops query-bombing attacks, cuts bandwidth, and enables CDN caching of `GET /graphql?hash=...`. Apollo Automatic Persisted Queries, Relay persisted queries, Hasura allow-list all implement this. PRODUCTION-ONLY allow-list the hashes — reject unknown queries.
- **Depth + cost limiting:** every query runs through a cost analyser (e.g. `graphql-cost-analysis`, `graphql-armor`) and rejects when depth > N (typically 10) or cost > budget. Without this, a 20-line query can DoS the DB.
- **Introspection:** ON in dev and staging (the whole tooling assumes it). OFF on the public-facing prod endpoint unless you operate a public API — combine with persisted-query allow-list.
- **Field-level authz:** directive-based (`@auth(role: ADMIN)`) OR middleware in the resolver. Either way — check permission INSIDE the resolver, NOT only at the HTTP layer; a single GraphQL POST hits dozens of resolvers.
- **Libraries:** **TS server**: GraphQL Yoga, Apollo Server 4, Mercurius (Fastify). **TS client**: Apollo Client, urql, Relay. **Rust**: async-graphql (schema-first via derive). **Go**: gqlgen. **Python**: Strawberry, Ariadne. **Federation**: Apollo Federation 2 (`@key`, `@extends`, `@external`), Cosmo, Hive — only if you truly have multiple subgraphs.

## References

- GraphQL spec (https://spec.graphql.org/October2021/) [E1 — normative, October 2021 revision current].
- GraphQL over HTTP + GraphQL over WebSocket (graphql-ws) + graphql-sse [E1 — working group specs].
- Relay Cursor Connections (https://relay.dev/graphql/connections.htm) [E1].
- DataLoader — Facebook OSS (https://github.com/graphql/dataloader) [E2].
- Apollo Federation v2 docs, Hasura docs, gqlgen docs, async-graphql docs [E2 — production-deployed].
- Evidence grade [E2] — GitHub v4 API, Shopify Admin, Facebook, Netflix all production GraphQL.
