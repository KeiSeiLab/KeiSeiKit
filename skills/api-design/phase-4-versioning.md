# Phase 4 — Versioning strategy

Decide how the API evolves when backwards-incompatible changes happen.
Reference: `_blocks/api-versioning-pagination-ratelimit.md`. Fail-closed
bias — if the user is unsure, the skill defaults to URL-path versioning
(most visible, hardest to break silently).

## 4a — Strategy click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Versioning strategy?",
      "header": "Versioning",
      "multiSelect": false,
      "options": [
        {"label": "URL path (/v1, /v2)",
         "description": "Most visible, curl-friendly, easy CDN routing. Coarse version bumps. GitHub v3/v4, public REST default."},
        {"label": "Header (media type, Accept: application/vnd.example.v2+json)",
         "description": "Clean URLs. Internal or typed-SDK-only APIs. Requires disciplined clients."},
        {"label": "Date-based (Stripe-Version: 2025-11-01)",
         "description": "Fine-grained; every breaking change pinnable. Keep N-1 versions live. Use for pay-for-stability APIs."},
        {"label": "Additive-only (no versioning, promise to never break)",
         "description": "Simplest. ONLY with small disciplined teams + strong typing + <3 consumers. Risk: one accidental break kills trust."},
        {"label": "GraphQL evolution (no version, @deprecated + telemetry)",
         "description": "Schema grows forever; remove fields after telemetry shows 0 usage. Required for GraphQL-only APIs."}
      ]
    }
  ]
}
```

Store as `VERSIONING`. Gate on `STYLE` from Phase 1:

- `STYLE = GraphQL` → only `GraphQL evolution` is correct. If user picks
  anything else, STOP and re-ask with a one-line explanation.
- `STYLE = REST` → any of URL / Header / Date / Additive.
- `STYLE = gRPC` → versioning usually package-based (`example.v1`,
  `example.v2`); record as `url-path`-equivalent and note in final report.
- `STYLE = tRPC` → additive-only is typical; record as `additive-only`.

## 4b — Deprecation runway click (AskUserQuestion, single-select)

Only if `VERSIONING != additive-only` AND `VERSIONING != GraphQL evolution`.

```json
{
  "questions": [
    {
      "question": "Minimum deprecation runway for breaking changes?",
      "header": "Deprecation runway",
      "multiSelect": false,
      "options": [
        {"label": "6 months (RECOMMENDED for public APIs)",
         "description": "Industry standard (Stripe, GitHub). Deprecation + Sunset headers + changelog + migration guide."},
        {"label": "12 months",
         "description": "Regulated / enterprise partners. SLA-backed."},
        {"label": "3 months",
         "description": "Acceptable for partner or internal APIs where consumers are known and reachable."},
        {"label": "Same-day (internal only)",
         "description": "Inside the mesh; all callers are your own services. Still emit Sunset header for audit."}
      ]
    }
  ]
}
```

Store as `DEPRECATION_MONTHS`. Block "Same-day" if `AUDIENCE = public`
(fail-closed NO DOWNGRADE — re-offer 3 / 6 / 12 with a one-line warning).

## 4c — Emit deprecation headers snippet (inline, no AskUserQuestion)

Print the standards-track header contract — RFC 8594 (Sunset) + RFC 9745
(Deprecation, 2024):

```http
Deprecation: @1735689600            # Unix timestamp of deprecation
Sunset: Wed, 11 Nov 2026 00:00:00 GMT
Link: <https://api.example.com/migration-v2-to-v3>; rel="deprecation"
```

- `Deprecation` — when the endpoint became deprecated (past or future).
- `Sunset` — when it will be removed. `Sunset - Deprecation ≥ DEPRECATION_MONTHS`.
- `Link rel="deprecation"` — URL of the migration guide.

For `VERSIONING = GraphQL evolution`: replace with SDL directive
`@deprecated(reason: "Use field X — removed after 2026-11-01")` and the
removal rule ("remove only after telemetry shows 0 usage for ≥30 days").

## 4d — Emit changelog + telemetry obligations (inline)

For any non-trivial versioning choice, print:

- **Changelog location:** `docs/api/CHANGELOG.md` or `/changelog` endpoint
  on the API itself. Entries: date, version, breaking/non-breaking,
  migration link.
- **Telemetry obligations:** log the version used on every request
  (`api_version` field); alert when a deprecated version's usage does not
  trend down. Without telemetry, "deprecation" is a lie.
- **Versioning + pagination cross-cut:** cursor tokens MUST be opaque and
  treated as versioned data (base64 of signed JSON); don't let a v1 cursor
  accidentally work in v2 with different fields.

## Verify-criterion

- `VERSIONING` exactly one choice, compatible with `STYLE`.
- `DEPRECATION_MONTHS` set (or N/A for additive-only / GraphQL evolution).
- Deprecation header snippet (4c) printed.
- Changelog + telemetry obligations (4d) printed as a checklist.
- If `AUDIENCE = public` and `DEPRECATION_MONTHS < 3` → STOP and re-ask.
