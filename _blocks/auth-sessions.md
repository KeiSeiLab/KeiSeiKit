# AUTH — Sessions & Cookies (+JWT tradeoff)

What happens AFTER identity is proven (password / OAuth / passkey / magic-link). Issues a session, enforces it on every request, and kills it on logout. Upstream of `auth-authorization.md`.

## When to include

- Any web or mobile app that needs an authenticated request state beyond a single round-trip.
- Any app that exposes logout, session revocation, or step-up auth.
- API-only backend (mobile/SPA): choose cookie-based session OR short-lived JWT — decision recorded per project.

## What it declares

- **Default: server-side opaque sessions** stored in Postgres / Redis / SQLite, keyed by a 256-bit random `session_id`. Row columns: `id`, `user_id`, `created_at`, `last_seen_at`, `expires_at`, `ip`, `user_agent`, `revoked_at`. Session data NEVER encoded in the cookie itself.
- **Cookie flags — all mandatory:** `HttpOnly` (blocks JS read → XSS-resistant), `Secure` (HTTPS only), `SameSite=Lax` for top-level nav auth / `Strict` for cross-site-hostile apps, `Path=/`, `__Host-` prefix for session cookie (forbids `Domain`, requires `Secure` + `Path=/`). Max-Age tuned to app: 7–30 days sliding, 24 h hard for regulated.
- **Session rotation:** issue a NEW `session_id` on login, logout-everywhere, password/passkey change, privilege elevation. Old row deleted or `revoked_at` set. Rotation defeats session fixation.
- **Logout:** delete the server row AND clear the cookie (`Max-Age=0`, same flags). Logout-everywhere = delete all rows for `user_id`. Client-only logout (cookie clear, server row kept) is a bug, not a feature.
- **CSRF:** `SameSite=Lax` covers most flows. For cross-origin POSTs keep a double-submit CSRF token (cookie + header/form field, server compares). API-only backend with Bearer token → no CSRF (no ambient credential).
- **JWT alternative — use ONLY when stateless horizontal scale matters more than revocation:**
  - `access_token` ≤15 min, signed ES256 (NOT HS256 with shared secret across services), `iat`/`exp`/`aud`/`iss`/`sub` all validated, `kid` header + JWKS rotation.
  - `refresh_token` opaque (NOT a JWT), stored server-side, rotated on every use (detect reuse → revoke family).
  - Logout revokes refresh token ONLY; access token is trusted until `exp`. If you need instant revoke → use server sessions instead.
  - Never store JWT in `localStorage` — use `HttpOnly` cookie or native secure storage. `localStorage` + XSS = total account takeover.
- **Libraries:** axum-login + tower-sessions (Rust), express-session / Better-Auth (Node), iron-session (edge), starlette SessionMiddleware + authlib (Python), SvelteKit `event.cookies`. JWT: jose (TS), jsonwebtoken (Rust), PyJWT.

## References

- OWASP Session Management Cheat Sheet, RFC 6265bis (cookies), RFC 7519 (JWT), RFC 8725 (JWT BCP) [E1].
- `auth-oauth2-oidc.md` / `auth-passkeys.md` — upstream identity proof; `auth-authorization.md` — downstream permission check.
- Evidence grade [E2] — session-cookie pattern stable since 2000s; JWT revocation gap is a well-known tradeoff.
