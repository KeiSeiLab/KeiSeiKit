# Phase 1 — Intake (flows, stack, storage, MFA)

One free-text paragraph, then four click batches. This is the only phase
that accepts typed input.

## 1a — Ask for the app description

Emit a regular message (NOT AskUserQuestion):

> Describe the app in one paragraph: what is it, who signs in, and any
> constraint I should know (regulated industry, existing user table,
> multi-tenant, mobile-first, etc.). Reply in one message.

Store the reply verbatim as `INTAKE`.

## 1b — User-flow click (AskUserQuestion, multi-select)

```json
{
  "questions": [
    {
      "question": "Which login flows should the app support?",
      "header": "Flows",
      "multiSelect": true,
      "options": [
        {"label": "Email + password",   "description": "Classic — requires password hash (argon2id), breach-check (HIBP), reset email flow"},
        {"label": "Magic link",         "description": "Email-only, one-time link; removes password surface, depends on email deliverability"},
        {"label": "OAuth / social",     "description": "Google / GitHub / Apple / Microsoft — see _blocks/auth-oauth2-oidc.md"},
        {"label": "Passkey (WebAuthn)", "description": "Phishing-resistant, passwordless — see _blocks/auth-passkeys.md"},
        {"label": "Enterprise SSO",     "description": "SAML / OIDC to Okta / Entra ID / Keycloak — B2B multi-tenant"}
      ]
    }
  ]
}
```

Store the multi-selection as `FLOWS`. Empty selection → re-ask.

## 1c — Stack click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Primary framework / runtime?",
      "header": "Stack",
      "multiSelect": false,
      "options": [
        {"label": "Next.js (App Router)", "description": "Server Components + Server Actions; Better-Auth or Auth.js"},
        {"label": "Remix / React Router", "description": "Loader/action model; remix-auth + Better-Auth"},
        {"label": "SvelteKit",            "description": "Hooks + form actions; Lucia replacement = Better-Auth or custom"},
        {"label": "Astro",                "description": "Islands + middleware; Better-Auth or external auth"},
        {"label": "Rust (axum / actix)",  "description": "axum-login + tower-sessions + webauthn-rs + openidconnect-rs"},
        {"label": "Python (FastAPI)",     "description": "authlib + starlette SessionMiddleware + py_webauthn"},
        {"label": "Other / specify",      "description": "Add a free-text note; skill will pick libraries in Phase 2"}
      ]
    }
  ]
}
```

Store as `STACK`.

## 1d — Storage click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Where will user + session rows live?",
      "header": "Storage",
      "multiSelect": false,
      "options": [
        {"label": "Postgres",        "description": "Recommended default — ACID, row-level security, good for multi-tenant"},
        {"label": "SQLite",          "description": "Single-node, edge-friendly; fine up to ~100k users"},
        {"label": "MySQL / MariaDB", "description": "Existing stack compatibility"},
        {"label": "Managed (Supabase / Clerk / Auth0 / WorkOS)", "description": "Off-load auth entirely — skill emits integration plan, not self-hosted tables"},
        {"label": "Redis (sessions only)", "description": "Paired with a primary DB for users"}
      ]
    }
  ]
}
```

Store as `STORAGE`.

## 1e — MFA click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Multi-factor requirement?",
      "header": "MFA",
      "multiSelect": false,
      "options": [
        {"label": "None",                 "description": "Consumer app, low risk surface"},
        {"label": "TOTP (authenticator app)", "description": "RFC 6238; pair with 10 one-time recovery codes"},
        {"label": "Passkey as 2FA",       "description": "WebAuthn with user-verification=required, after password or magic link"},
        {"label": "Required for admins only", "description": "RBAC rule: role=admin → MFA gate before privileged actions"}
      ]
    }
  ]
}
```

Store as `MFA`.

## Verify-criterion

- `INTAKE` non-empty.
- `FLOWS` has ≥1 entry.
- `STACK`, `STORAGE`, `MFA` each exactly one label.
- If `FLOWS = {Passkey}` ONLY and `MFA = None` → warn "passkey-only requires a
  recovery path" and return to 1b (NO DOWNGRADE: present recovery options).
