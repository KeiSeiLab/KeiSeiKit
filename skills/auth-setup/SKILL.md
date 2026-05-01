---
name: auth-setup
description: Hub-and-spoke pipeline that produces a production-grade auth/IAM plan for a new or existing app — user flows, identity providers, session strategy, authorization model, and threat mitigations — via pure-click decisions across five phases. Emits a scaffolded env-var list, library picks, and a per-threat checklist; never writes secrets.
argument-hint: <one-line app description, e.g. "B2C SaaS, Next.js + Postgres, needs Google + passkeys">
---

# Auth-Setup — Identity, Session & Authorization Pipeline (index)

> See `_blocks/pipeline-5phase-template.md` for the 5-phase wizard contract
> and `_blocks/rule-pure-click-contract.md` for the AskUserQuestion rule.
> Skill-specific phase tables are inline below.

You are converting "I need auth for app X" into a concrete, reviewable plan:
which identity methods to ship, which providers to register, which session
strategy to pick, which authorization model to enforce, and which threats to
mitigate up front. Every decision is a click; the only typed input is the
one-line app description in Phase 1.

This skill does NOT write production code. It emits a plan, the env-var
scaffold, the library picks, and a per-threat checklist. Code scaffolding
is a separate task owned by `new-agent` or the project's own
code-implementer.

The skill reads the four companion blocks heavily — every phase references
at least one of them:

- `_blocks/auth-oauth2-oidc.md` — OAuth2 / OIDC flows, PKCE, providers.
- `_blocks/auth-passkeys.md` — WebAuthn registration + assertion, RP ID.
- `_blocks/auth-sessions.md` — server sessions vs JWT tradeoff, cookies.
- `_blocks/auth-authorization.md` — RBAC / ABAC / ReBAC, policy engines.

---

## Pipeline overview (5 phases, ≥6 AskUserQuestion calls)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | App flows, stack, storage, MFA | 4× |
| 2 | [phase-2-identity-provider.md](phase-2-identity-provider.md) | Pick + configure IdPs; env scaffold | 1× |
| 3 | [phase-3-session-strategy.md](phase-3-session-strategy.md) | Server-session vs JWT; cookie flags | 1× |
| 4 | [phase-4-authorization.md](phase-4-authorization.md) | RBAC / ABAC / ReBAC; permission matrix | 1× |
| 5 | [phase-5-threats.md](phase-5-threats.md) | CSRF / XSS / timing / enumeration | 1× |

Minimum AskUserQuestion count across a full session: **8** (4 in Phase 1 +
1 each in Phases 2–5). Exceeds the ≥6 hub-and-spoke contract.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `FLOWS` | Phase 1 | subset of {email+password, magic-link, OAuth, passkey, SSO} |
| `STACK` | Phase 1 | Next / Remix / SvelteKit / Astro / Rust axum / FastAPI / other |
| `STORAGE` | Phase 1 | Postgres / SQLite / MySQL / Supabase / managed-auth |
| `MFA` | Phase 1 | none / TOTP / passkey / WebAuthn-as-2FA |
| `PROVIDERS` | Phase 2 | list of OAuth/OIDC providers with env-var names |
| `SESSION` | Phase 3 | server-session OR JWT + cookie config |
| `AUTHZ` | Phase 4 | RBAC / ABAC / ReBAC + policy engine (or none) |
| `THREATS` | Phase 5 | mitigation checklist, per threat class |

---

## Final report (emit after Phase 5)

```
=== AUTH-SETUP REPORT ===
App:        <first 80 chars of intake>...
Stack:      <STACK> + <STORAGE>
Flows:      <FLOWS list>  MFA: <MFA>
Providers:  <PROVIDERS with env var names>
Session:    <SESSION summary line>
Authz:      <AUTHZ model + engine if any>
Threats:    <N mitigations selected>
Libraries:  <pick per language, one line>
Env vars:   <count> new entries for secrets/<file>.env
Next:       run `compose-solution` or hand off to project code-implementer
```

---

## Rules (apply throughout)

- **Pure-click contract.** Only the Phase 1 intake paragraph is typed.
  Every other decision is `AskUserQuestion`.
- **RULE 0.8 Secrets SSoT.** The skill emits env VARIABLE NAMES only
  (`GOOGLE_CLIENT_ID`, `APPLE_TEAM_ID`, ...). It NEVER echoes a token
  value, never writes to `secrets/*.env`, never suggests hard-coding.
  Storage path is `<repo>/secrets/auth.env` per `domain-has-secrets.md`.
- **NO DOWNGRADE.** If the chosen combination is unsafe (e.g.
  passkey-only without a recovery flow) the skill returns 2–3 constructive
  alternatives, never "not supported".
- **Fail-closed default.** Every authz / session / threat decision
  defaults to the safer option when the user is unsure.
- **Surgical scope.** Reads the four auth blocks; writes nothing outside
  its own phase files. Production scaffolding is delegated.

---

## References

- `_blocks/auth-oauth2-oidc.md`, `_blocks/auth-passkeys.md`,
  `_blocks/auth-sessions.md`, `_blocks/auth-authorization.md`.
- `_blocks/domain-has-secrets.md` — storage path + loading convention.
- `_blocks/rule-pre-dev-gate.md` — analogue check before inventing.
- Evidence grade [E2] — pipeline mirrors OWASP ASVS v4.0.3 chapters 2–4.
