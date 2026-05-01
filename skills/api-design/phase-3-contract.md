# Phase 3 — Contract skeleton (OpenAPI 3.1 OR GraphQL SDL)

Generate the machine-readable SSoT from `RESOURCES` + `SHAPE`. This is the
ONLY phase that writes a file outside the skill — the user picks WHERE
(phase 3a) and the skill writes a skeleton there (phase 3c).

Reference: `_blocks/api-openapi-first.md`, `_blocks/api-graphql.md`.

## 3a — Contract path click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Where should the contract skeleton be written?",
      "header": "Contract path",
      "multiSelect": false,
      "options": [
        {"label": "api/openapi.yaml (repo root / api/)",
         "description": "Default for REST. Picks up OpenAPI 3.1 tooling by convention."},
        {"label": "schema.graphql (repo root)",
         "description": "Default for GraphQL. Picks up graphql-codegen / async-graphql by convention."},
        {"label": "docs/api/<name>.yaml (docs folder)",
         "description": "Keep contract alongside human docs; publish site from same folder."},
        {"label": "Skip — I'll commit the skeleton manually from chat",
         "description": "Skill prints the full skeleton inline; user copies into repo."}
      ]
    }
  ]
}
```

Store as `CONTRACT_PATH`. Validate: the path must be consistent with
`STYLE` (REST → `.yaml`/`.json`; GraphQL → `.graphql`/`.graphqls`).
If inconsistent → re-ask per NO DOWNGRADE, listing the two correct options.

## 3b — Contract skeleton rules

The skeleton is NOT the final spec — it is a scaffold the user will flesh
out. Rules:

- **OpenAPI 3.1 skeleton** must include:
  - `openapi: 3.1.0` — never 3.0.x, never 2.0 (per `api-openapi-first.md`).
  - `info` with `title`, `version: 0.1.0`, `description` (first 2 lines of
    `INTAKE`).
  - `servers` list with at least `production` + `staging` placeholder URLs.
  - `components.schemas.Problem` ($ref'd by every 4xx/5xx — RFC 9457 shape).
  - `components.securitySchemes` placeholder (`bearerAuth` by default;
    `oauth2` if Phase 5 chooses OAuth; filled in Phase 5).
  - `components.parameters` for `cursor`, `limit`, `page`
    (depending on `PAGINATION` — filled in Phase 5).
  - One `paths` entry PER entity × action cell from Phase 2c:
    `GET /foos`, `POST /foos`, `GET /foos/{id}`, `PATCH /foos/{id}`,
    `DELETE /foos/{id}`. Each `$ref`s to `components/schemas/Foo` (stub
    with `id`, `created_at`, `updated_at`, plus placeholder fields).
  - ETag + idempotency hints as comments in the skeleton where relevant
    (`api-rest-conventions.md`).

- **GraphQL SDL skeleton** must include:
  - `scalar DateTime` + `scalar UUID` declared once.
  - `interface Node { id: ID! }` (Relay convention).
  - One `type Foo implements Node { id: ID! createdAt: DateTime! ... }` per
    entity.
  - `type FooConnection { edges: [FooEdge!]! pageInfo: PageInfo! totalCount: Int }`
    + `type FooEdge { node: Foo! cursor: String! }` for every listable
    entity (Relay spec).
  - Root `type Query { foo(id: ID!): Foo foos(first: Int, after: String): FooConnection! }`
    per entity per action cell.
  - Root `type Mutation { createFoo(input: CreateFooInput!): Foo! ... }`
    with `input` types per action cell.
  - Root `type Subscription` stub — empty if Phase 5 says no realtime.
  - `enum ErrorCode { NOT_FOUND FORBIDDEN BAD_USER_INPUT RATE_LIMITED INTERNAL_SERVER_ERROR }`
    — referenced by resolver error extensions.

## 3c — Emit / write the skeleton

If `CONTRACT_PATH` != "Skip":
- Compute absolute path in the current repo.
- If file exists → STOP, re-ask: "File exists at <path>. Overwrite, merge,
  or pick a new name?" (three-option AskUserQuestion, fail-closed default
  = "pick a new name").
- Write the skeleton. Record `CONTRACT = <absolute path>` and
  `CONTRACT_LINES = <LOC>` for the final report.

If `CONTRACT_PATH` == "Skip":
- Emit the full skeleton as a fenced code block in chat.
- Record `CONTRACT = <inline>` and `CONTRACT_LINES = <LOC>`.

## 3d — Lint + drift-gate hint (inline)

Remind the user ONCE:

> Add these to CI next:
> 1. `spectral lint <contract>` — OpenAPI/Spectral ruleset OR
>    `graphql-schema-linter schema.graphql` — style + breaking-change catch.
> 2. `oasdiff breaking` (REST) OR `graphql-inspector diff` (GraphQL) on
>    every PR — blocks breaking changes unless `breaking: approved`
>    label is set.
> 3. Contract tests (Schemathesis / Dredd / Pact) run against the deployed
>    server in staging. Drift = test fail.

No AskUserQuestion here — this is guidance.

## Verify-criterion

- `CONTRACT_PATH` picked; file written or skeleton printed inline.
- Every entity from `RESOURCES` surfaces in the skeleton as a schema/type.
- Every action cell from Phase 2c has a matching path/operation OR a
  matching query/mutation field.
- No field values invented — placeholder-only schemas, marked with a
  `# TODO: define fields` comment; RULE 0.4 no fabricated sample data.
