# AUTH — Authorization (RBAC / ABAC / ReBAC)

Who is allowed to do what, AFTER authentication (`auth-sessions.md`) has identified the principal. Decides on every request; fail-closed.

## When to include

- App has more than one user role OR owner-vs-member resource semantics.
- App exposes admin endpoints, multi-tenant data, or per-resource sharing.
- Regulated domain (health / finance / legal) where permission decisions must be logged and auditable.

## What it declares

- **RBAC (Role-Based)** — static roles (`admin`, `editor`, `viewer`) mapped to permission sets (`posts:write`, `posts:read`, `billing:read`). Simple, O(1) check, enough for most small apps. Roles live in DB; assignment is an admin action, not a code change.
- **ABAC (Attribute-Based)** — decision = f(subject attrs, resource attrs, action, context). Example: "user can edit doc IF `doc.owner_id == user.id` OR `user.role == admin AND doc.tenant_id == user.tenant_id`". Use when RBAC explodes into per-resource special cases.
- **ReBAC (Relationship-Based, Google Zanzibar style)** — graph of `(subject, relation, object)` tuples; check = "does path `user:A` → ... → `doc:X#editor` exist?". Use for hierarchical sharing (folders, orgs, teams). Implementations: SpiceDB, OpenFGA.
- **Permission matrix — always DECLARED, never implicit:** a table `roles × resource_types × actions` in the repo (`docs/permissions.md` or a DB seed). Every new endpoint picks a cell from the matrix. No ad-hoc `if user.is_admin` scattered through handlers.
- **Enforcement point: middleware, not handlers.** Decision computed once per request against a typed `Permission` enum. Handler receives `AuthorizedRequest<Action>` or 403s before it runs. Prevents "forgot the check on the new endpoint" — the dominant authz bug.
- **Fail-closed:** missing role, unknown action, or policy engine error → DENY. Log the denial with subject + action + resource. Never default-allow on error.
- **Policy engines — use when authz logic grows > ~20 rules:** Cerbos (YAML rules, decision-as-a-service, stateless), OPA / Rego (general-purpose, steeper curve), Oso Cloud, SpiceDB (ReBAC). Keep policy files in the repo; treat them as code (tested, reviewed, versioned).
- **Ownership checks scope every query:** `SELECT ... WHERE tenant_id = $1 AND owner_id = $2` — enforced in the data layer, not just the middleware. Double layer defeats IDOR (Insecure Direct Object Reference).
- **Admin + audit:** every permission change, role assignment, and deny-event written to an append-only audit log (`tenant_id`, `actor_id`, `action`, `target`, `timestamp`, `result`). Required for SOC2 / ISO 27001 / HIPAA.

## References

- NIST SP 800-162 (ABAC), Google Zanzibar paper (2019), Cerbos docs, OPA/Rego docs [E1].
- `auth-sessions.md` — source of the authenticated principal; this block decides what that principal can do.
- Evidence grade [E2] — RBAC/ABAC widely deployed; ReBAC via Zanzibar-clones production since ~2022.
