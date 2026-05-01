# Phase 3 — Session strategy + cookie configuration

Decides how the authenticated principal is carried across requests. Reads
`_blocks/auth-sessions.md` heavily. Default bias: server-side opaque
sessions (revocable, simple) unless the user needs horizontal stateless
scale.

## 3a — Strategy click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Session carrier?",
      "header": "Session",
      "multiSelect": false,
      "options": [
        {"label": "Server-side session + cookie (DEFAULT)",
         "description": "Opaque 256-bit id in HttpOnly Secure cookie; row in DB/Redis. Instant revoke. Recommended for >95% of apps."},
        {"label": "JWT access + opaque refresh",
         "description": "ES256 access ≤15 min in HttpOnly cookie; refresh rotated server-side. Use ONLY when you have stateless edge workers that can't reach the session DB."},
        {"label": "JWT access + refresh in native secure storage",
         "description": "Mobile app; refresh in Keychain / Keystore. Same rotation rules; cookie flags N/A."},
        {"label": "Managed (Clerk / Supabase / Auth0)",
         "description": "Provider owns the session primitive; skill records the SDK integration points only."}
      ]
    }
  ]
}
```

Store as `SESSION`.

## 3b — Emit cookie-flag checklist (inline, no AskUserQuestion)

Apply ONLY when `SESSION` involves a browser cookie. For every cookie the
app sets (session, CSRF, anti-re-use nonce):

```
[ ] HttpOnly                — blocks JS read; XSS-resistant
[ ] Secure                  — HTTPS only; reject on cleartext
[ ] SameSite=Lax            — default; use Strict for cross-site-hostile apps
[ ] __Host- prefix          — no Domain, Path=/, Secure — session cookie only
[ ] Max-Age tuned           — 7–30 d sliding (consumer) / 24 h hard (regulated)
[ ] Rotation on login,      — new session_id issued, old row deleted or revoked_at set
    logout-all, passkey/password change, privilege elevation
[ ] Logout deletes BOTH     — server row AND cookie (Max-Age=0, same flags)
```

## 3c — Emit JWT-specific checklist (inline) — only if JWT chosen

```
[ ] Algorithm = ES256       — asymmetric; NOT HS256 for cross-service
[ ] access_token ≤15 min    — minimises revocation-gap window
[ ] refresh_token OPAQUE    — stored server-side, rotated on every use
[ ] refresh-reuse detection — family revocation on stolen refresh
[ ] JWKS rotation + kid     — key rollover without service restart
[ ] Claims validated        — iss, aud, exp, nbf, iat, sub, nonce (if OIDC)
[ ] NEVER in localStorage   — HttpOnly cookie (web) / secure storage (native)
[ ] Logout policy stated    — "revoke refresh only; access valid until exp"
    and accepted by the product (or escalate to server-session strategy)
```

## 3d — CSRF strategy (inline, driven by SESSION)

- Cookie session + same-origin forms → `SameSite=Lax` is enough; plus a
  CSRF token (cookie+header double-submit) for cross-origin POSTs.
- Cookie session + third-party embed (iframes, extensions) → `SameSite=None;
  Secure` + mandatory CSRF token, reject missing/mismatched.
- Bearer-token API (no cookie) → no CSRF (no ambient credential); enforce
  `Origin` header check as defence-in-depth.

## Verify-criterion

- `SESSION` set to exactly one strategy.
- At least one of the three checklists (3b / 3c / 3d) applies and was
  emitted.
- If JWT chosen, 3c is printed in full AND the logout gap was explicitly
  acknowledged in the report.
