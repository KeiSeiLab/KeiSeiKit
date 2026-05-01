# AUTH — OAuth2 + OIDC (Authorization Code + PKCE)

Identity delegation to external providers (Google / GitHub / Apple / Microsoft / any OIDC-compliant IdP). For first-party login see `auth-passkeys.md` / `auth-sessions.md`; for post-login permissions see `auth-authorization.md`.

## When to include

- App supports "Sign in with Google / GitHub / Apple / Microsoft" or federates to an enterprise OIDC IdP (Okta, Auth0, Keycloak, Entra ID).
- App needs a short-lived API access token for the user (Gmail, Calendar, GitHub API).
- Regulated context where the IdP — not the app — is the system of record for identity.

## What it declares

- **Flow: Authorization Code + PKCE for EVERY client** (public SPA, mobile, confidential server). PKCE is mandatory in OAuth 2.1 and removes the implicit flow entirely.
- **PKCE params:** `code_verifier` 43–128 chars random, `code_challenge = BASE64URL(SHA256(verifier))`, `code_challenge_method=S256`. Never `plain`.
- **State + nonce:** `state` (CSRF, 32+ bytes random, bound to session) on every auth request; `nonce` (replay, in ID token claim) for OIDC. Reject response if either mismatches.
- **Redirect URIs:** exact-match, pre-registered at the IdP. No wildcards. `localhost` and custom schemes OK for native; HTTPS required for web.
- **Providers: Google** (`accounts.google.com/.well-known/openid-configuration`), **GitHub** (OAuth2 only, no OIDC discovery — hard-code `https://github.com/login/oauth/authorize`, `token`, `https://api.github.com/user`), **Apple** (OIDC, but only returns user name/email on FIRST consent — persist on first login or lose it), **Microsoft** (`login.microsoftonline.com/{tenant}/v2.0/.well-known/openid-configuration`).
- **Token handling:** `access_token` short-lived (≤1 h), kept server-side only. `refresh_token` rotated on every use (RFC 6749 §6 + OAuth 2.1), stored encrypted at rest, NEVER sent to the browser. `id_token` validated (JWKS signature + `iss` + `aud` + `exp` + `nonce`) and discarded — do NOT re-use as a session token.
- **Secrets:** `CLIENT_ID` + `CLIENT_SECRET` per provider in `secrets/*.env`; referenced by env var name only. Public clients (SPA/mobile) use PKCE WITHOUT a secret.
- **Libraries:** prefer Better-Auth (TS), NextAuth/Auth.js (Next.js), authlib (Python), openidconnect-rs or oauth2-rs (Rust). Avoid rolling your own — every major CVE in this space is custom code.

## References

- RFC 6749 (OAuth 2.0), RFC 7636 (PKCE), RFC 9700 (OAuth 2.0 Security BCP, 2024), OAuth 2.1 draft, OpenID Connect Core 1.0 [E1 — standards-track RFCs].
- `auth-sessions.md` for what to do AFTER the IdP handshake returns.
- Evidence grade [E2] — implementation widely deployed, spec stable since 2024.
