# Phase 6 — Codegen toolchain

Pick the generator(s) that turn the Phase 3 contract into server stubs,
typed clients, and docs. This is the last click; after it, the skill emits
the final report. Reference: `_blocks/api-openapi-first.md`,
`_blocks/api-graphql.md`.

## 6a — Codegen click (AskUserQuestion, multi-select)

Options gate on `STYLE` from Phase 1 and `CLIENTS` from Phase 1d.

### If `STYLE = REST` (OpenAPI spec)

```json
{
  "questions": [
    {
      "question": "Pick the REST codegen targets (spec → server + clients + docs)?",
      "header": "REST Codegen",
      "multiSelect": true,
      "options": [
        {"label": "openapi-generator (multi-language server + clients)",
         "description": "Generates TS/Swift/Kotlin/Python/Rust/Go/Java stubs. Prefer axios (TS), okhttp (Kotlin), urlsession (Swift). Version 7.x as of 2026."},
        {"label": "orval (TS clients with React Query / SWR / Zod)",
         "description": "Best-in-class TS client DX. Integrates with msw mock server."},
        {"label": "oapi-codegen (Go, type-safe chi/echo/gin handlers)",
         "description": "Reference Go generator. Server interface + client + type-safe handlers."},
        {"label": "progenitor (Rust, async clients)",
         "description": "Oxide Computer's generator. reqwest-based, serde + typed errors."},
        {"label": "Swagger UI (interactive docs)",
         "description": "Classic, Try-it-out buttons. Good for partner onboarding."},
        {"label": "Redoc (read-only, pretty docs)",
         "description": "Stripe-style three-pane docs. Markdown-friendly."},
        {"label": "Scalar (modern docs, built-in request builder)",
         "description": "2024+ popular, React-based, fast. Good for public APIs."},
        {"label": "Stoplight Elements (embeddable React docs)",
         "description": "Drop into an existing marketing site as a component."},
        {"label": "Prism (mock server from spec)",
         "description": "stoplight/prism — mock server for frontend devs before backend exists."},
        {"label": "Schemathesis (contract tests)",
         "description": "Property-based tests that hit the real server and verify every operation against the spec."}
      ]
    }
  ]
}
```

### If `STYLE = GraphQL`

```json
{
  "questions": [
    {
      "question": "Pick the GraphQL codegen targets (schema → server + clients + docs)?",
      "header": "GraphQL Codegen",
      "multiSelect": true,
      "options": [
        {"label": "graphql-codegen (TS clients + resolver types)",
         "description": "The Guild's codegen. Plugins for typescript-operations, typescript-react-apollo, typed-document-node."},
        {"label": "async-graphql (Rust, schema-first via derive)",
         "description": "Production Rust server. Derive macros implement schema; DataLoader built-in."},
        {"label": "gqlgen (Go, schema-first)",
         "description": "The Go standard. Resolver stubs from schema."},
        {"label": "Strawberry (Python, code-first with type hints)",
         "description": "Python async-friendly; emits SDL. NOT schema-first but acceptable if STYLE = GraphQL and team prefers Python types."},
        {"label": "Apollo Server 4 / GraphQL Yoga (TS server)",
         "description": "Runtime, not codegen, but paired with graphql-codegen for the types."},
        {"label": "Relay Compiler (persisted queries + typegen for React)",
         "description": "Required if FB Relay is the client. Produces persisted query IDs for production allow-list."},
        {"label": "GraphiQL / Apollo Sandbox (interactive docs)",
         "description": "Built into every GraphQL server; dev-only by default."},
        {"label": "graphql-inspector (schema diff + breaking-change gate)",
         "description": "CI gate. Fail PR on breaking changes unless label applied."}
      ]
    }
  ]
}
```

### If `STYLE = tRPC` / `gRPC`

```json
{
  "questions": [
    {
      "question": "Pick the codegen targets?",
      "header": "Codegen (tRPC/gRPC)",
      "multiSelect": true,
      "options": [
        {"label": "tRPC: infer from server routers (no codegen)",
         "description": "TS end-to-end; client imports `AppRouter` type. Only works for TS clients."},
        {"label": "gRPC: buf + protoc-gen-go / protoc-gen-ts / protoc-gen-swift",
         "description": "Buf CLI for lint + breaking-change detection. Per-language protoc plugins for clients."},
        {"label": "gRPC: connectrpc (grpc-web + browser support)",
         "description": "Buf's connect — browser-compatible, simpler than grpc-web."},
        {"label": "Buf Schema Registry (docs + breaking-change gate)",
         "description": "Managed registry or self-host; PR comment on breaking schema change."}
      ]
    }
  ]
}
```

Store the selection as `CODEGEN` (a list of labels). Validate:

- At least one server generator OR "tRPC infer from server" is picked.
- At least one docs target is picked (unless `AUDIENCE = internal`).
- At least one contract-test / drift-gate is picked (`Schemathesis`,
  `graphql-inspector`, or `Buf` — fail-closed NO DOWNGRADE: re-ask if
  none selected).

## 6b — Emit CI snippet (inline)

Print a minimal CI snippet (GitHub Actions shape; user adapts to their
runner). Example for REST + openapi-generator + Spectral + oasdiff:

```yaml
# .github/workflows/api-contract.yml (SKELETON — adjust to your runner)
name: api-contract
on: [pull_request]
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npx -y @stoplight/spectral-cli lint api/openapi.yaml
  diff:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { fetch-depth: 0 }
      - run: npx -y oasdiff breaking origin/main:api/openapi.yaml api/openapi.yaml
  codegen-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npx -y @openapitools/openapi-generator-cli validate -i api/openapi.yaml
```

For GraphQL, emit the equivalent with `graphql-schema-linter` +
`graphql-inspector diff` + `graphql-codegen --check`.

## 6c — Final report assembly

Emit the final report template from `SKILL.md` with all variables filled.
Add at the bottom:

```
Deselected / risk-accepted:
- <item>: <one-line justification>
(or "none" if the pipeline ran fail-closed)

Next recommended command:
- /auth-setup "<first 80 chars of INTAKE>"   (if AUTH_HANDOFF = run-auth-setup)
- /compose-solution                           (to assemble blocks into a project plan)
```

## Verify-criterion

- `CODEGEN` has ≥1 entry and passes the two validation rules.
- CI snippet (6b) printed.
- Final report (6c) emits the full `=== API-DESIGN REPORT ===` block with
  every variable from `SKILL.md` filled.
- No library version number invented — references are to current major
  versions per the upstream blocks (RULE 0.4).
