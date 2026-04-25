# Phase 2 — Identity provider selection + env-var scaffold

Only runs if `FLOWS` contains `OAuth / social` or `Enterprise SSO`. If
neither is selected, skip to Phase 3 (passkey-only or magic-link apps
don't need external IdPs).

## 2a — Provider click (AskUserQuestion, multi-select)

Reference: `_blocks/auth-oauth2-oidc.md`.

```json
{
  "questions": [
    {
      "question": "Which identity providers to register?",
      "header": "Providers",
      "multiSelect": true,
      "options": [
        {"label": "Google",    "description": "OIDC; discovery at accounts.google.com/.well-known/openid-configuration"},
        {"label": "GitHub",    "description": "OAuth2 only (no OIDC discovery); hard-code endpoints"},
        {"label": "Apple",     "description": "OIDC; name/email returned ONLY on first consent — persist immediately"},
        {"label": "Microsoft", "description": "OIDC multi-tenant via login.microsoftonline.com/common/v2.0"},
        {"label": "Enterprise OIDC (Okta / Auth0 / Keycloak / Entra)", "description": "B2B SSO; per-tenant discovery URL"},
        {"label": "SAML 2.0 (legacy enterprise)", "description": "Use a library like samlify (TS) or python3-saml; NOT OAuth"}
      ]
    }
  ]
}
```

Store as `PROVIDERS`. Empty → skip Phase 2.

## 2b — Emit env-var scaffold (no AskUserQuestion)

For EACH provider in `PROVIDERS`, emit the env-var rows the user must add
to `secrets/auth.env`. NEVER emit values — names only. Example for Google:

```bash
# secrets/auth.env — add these, then `chmod 600` the file
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GOOGLE_REDIRECT_URI=https://<app>/auth/google/callback
GOOGLE_OIDC_DISCOVERY=https://accounts.google.com/.well-known/openid-configuration
```

Per-provider scaffold rules:

- **Google / Microsoft / Apple / Enterprise OIDC:** `*_CLIENT_ID`,
  `*_CLIENT_SECRET` (confidential) OR `*_CLIENT_ID` + PKCE only (public
  SPA/mobile), `*_REDIRECT_URI`, `*_OIDC_DISCOVERY`.
- **Apple** adds `APPLE_TEAM_ID`, `APPLE_KEY_ID`, `APPLE_PRIVATE_KEY_PATH`
  (path to the `.p8` file — stored inside `secrets/`, never inline).
- **GitHub:** `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`,
  `GITHUB_REDIRECT_URI`. No discovery URL.
- **SAML:** `SAML_IDP_METADATA_URL`, `SAML_SP_ENTITY_ID`,
  `SAML_SP_ACS_URL`, `SAML_SP_PRIVATE_KEY_PATH`,
  `SAML_SP_CERT_PATH`.

Emit the snippet as a fenced code block in chat. Remind the user once:
"File `secrets/auth.env` must be `chmod 600` and listed in `.gitignore`
BEFORE the first write. See `_blocks/domain-has-secrets.md`."

## 2c — Library pick (emitted inline, no AskUserQuestion)

Driven by `STACK` from Phase 1:

- **Next.js / Remix / SvelteKit / Astro:** Better-Auth (preferred 2025–2026)
  OR NextAuth/Auth.js (Next-only, mature). Both support PKCE by default.
- **Rust (axum):** `openidconnect-rs` (OIDC) or `oauth2-rs` (OAuth2 bare).
- **Python (FastAPI):** `authlib` (covers both OAuth2 and OIDC).
- **Managed (Clerk / Supabase / WorkOS):** provider SDK only; this phase
  just records the SDK name.

## Verify-criterion

- Every provider in `PROVIDERS` has its env-var scaffold printed.
- No literal token value appears anywhere in the emitted text
  (RULE 0.8 / `auth-oauth2-oidc.md` enforcement).
- Library pick is one line, matches `STACK`.
