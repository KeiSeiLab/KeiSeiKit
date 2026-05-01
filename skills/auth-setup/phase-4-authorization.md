# Phase 4 — Authorization model + permission matrix

Decides who-can-do-what after authentication. Reads
`_blocks/auth-authorization.md`. Fail-closed by default.

## 4a — Model click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Authorization model?",
      "header": "Authz",
      "multiSelect": false,
      "options": [
        {"label": "RBAC (roles → permissions)",
         "description": "Static roles (admin / editor / viewer). Simplest, enough for most apps with <5 roles."},
        {"label": "RBAC + ownership",
         "description": "Roles + per-row owner_id check. The sweet spot for multi-tenant SaaS."},
        {"label": "ABAC (policy engine)",
         "description": "Attributes + context (time, IP, resource tier). Use Cerbos or OPA. Adopt when rule count >~20."},
        {"label": "ReBAC (Google Zanzibar)",
         "description": "Graph-shaped sharing (folders/teams/orgs). SpiceDB or OpenFGA. Pick only if your domain is inherently graph-shaped."},
        {"label": "None — single-user app",
         "description": "No authz layer beyond authentication. Record explicitly."}
      ]
    }
  ]
}
```

Store as `AUTHZ`.

## 4b — Emit permission matrix skeleton (inline)

For `RBAC` / `RBAC + ownership`, emit a table stub the user must fill in
before coding. Example for a notes app:

```markdown
| Role   | notes:read | notes:write | notes:delete | users:manage |
|--------|:---------:|:-----------:|:------------:|:------------:|
| admin  |    all    |     all     |     all      |     yes      |
| editor |   owned   |   owned     |   owned      |      no      |
| viewer |   owned   |     no      |     no       |      no      |
```

- Columns = `resource:action` tokens — these become the `Permission` enum
  variants in code.
- Cells = `all` / `owned` / `no` / `shared:<relation>` — NEVER free-text.
- Save as `docs/permissions.md` in the target repo; treat it as code
  (tested, reviewed, versioned).

## 4c — Enforcement-point rule (inline)

- Middleware, not handlers. Every authenticated request runs an authz
  decision BEFORE the handler sees it. Handler receives a typed
  `AuthorizedRequest<Action, Resource>` or the request 403s earlier in the
  stack.
- Ownership checks enforced in BOTH the middleware AND the data layer
  (`WHERE tenant_id=$1 AND owner_id=$2`). Double layer defeats IDOR.
- Fail-closed: unknown action, missing role, policy-engine error → 403.
  Log every denial with subject + action + resource + reason.
- Audit log append-only row on every privilege change, role assignment,
  and denied action. Required for SOC2 / HIPAA / ISO 27001.

## 4d — Policy-engine pick (inline, driven by AUTHZ)

- `RBAC` / `RBAC + ownership` → in-code match on `Permission` enum; no
  engine.
- `ABAC` → Cerbos (YAML rules, stateless decision service) OR OPA/Rego
  (general-purpose, steeper curve). Keep `.cerbos.yaml` / `.rego` files
  in the repo, unit-tested like code.
- `ReBAC` → SpiceDB (Zanzibar reference) OR OpenFGA (Auth0-backed).
  Define the schema, seed relationships, use the client SDK.
- `None` → emit a single line "authz skipped — no multi-user model".

## Verify-criterion

- `AUTHZ` is exactly one choice.
- If RBAC chosen, permission-matrix skeleton with ≥1 row + ≥1 column is
  printed.
- Enforcement-point rule (4c) is emitted verbatim — non-negotiable.
- If a policy engine is implied by `AUTHZ`, the pick is named (Cerbos /
  OPA / SpiceDB / OpenFGA).
