# Phase 5 — Threats & mitigations

Close the pipeline with a per-threat checklist. The user picks which
mitigations to commit to; the skill emits them into the final report so
they get tracked as acceptance criteria.

## 5a — Threat-class click (AskUserQuestion, multi-select, pre-checked)

Pre-select every item by default — opting OUT requires a click, opting IN
is the cheap path (fail-closed bias).

```json
{
  "questions": [
    {
      "question": "Confirm the threat mitigations to enforce (pre-checked; deselect only if you have a compensating control)?",
      "header": "Threats",
      "multiSelect": true,
      "options": [
        {"label": "CSRF — SameSite + token",
         "description": "SameSite=Lax default; double-submit token for cross-origin POSTs; reject on mismatch"},
        {"label": "XSS — HttpOnly + CSP",
         "description": "HttpOnly on every auth cookie; strict CSP (no inline script); sanitise every rendered string; NEVER put session or JWT in localStorage"},
        {"label": "Session fixation — rotate on login",
         "description": "New session_id issued at every privilege change (login, logout-all, password/passkey change, MFA step-up)"},
        {"label": "Account enumeration — uniform responses",
         "description": "Same timing + wording for 'user not found' and 'wrong password'; signup and reset flows respond identically regardless of address existence"},
        {"label": "Timing attacks — constant-time compare",
         "description": "Use subtle.timingSafeEqual / crypto.constant_time_compare on password hash, token, session_id lookups"},
        {"label": "Password policy — argon2id + HIBP",
         "description": "argon2id hashing (memory≥64MB, t≥3); reject passwords found in HaveIBeenPwned k-anonymity API; min length 12, no max"},
        {"label": "Brute-force — rate limit + lockout",
         "description": "Per-account exponential backoff; per-IP sliding window; CAPTCHA after N failures; unlock via email or time"},
        {"label": "Email-link security",
         "description": "One-time tokens (random 32B, SHA-256 in DB); ≤15 min TTL; single-use; bound to email address at issue time"},
        {"label": "OAuth state + nonce",
         "description": "state (CSRF) + nonce (replay) on every authorize request; reject on mismatch; see _blocks/auth-oauth2-oidc.md"},
        {"label": "Passkey recovery path",
         "description": "Backup codes OR email magic-link OR OAuth fallback; user opts out only after explicit warning"},
        {"label": "Logging without leakage",
         "description": "Never log raw password, TOTP secret, session_id, or access_token; log userID + action + result only"},
        {"label": "Dependency hygiene",
         "description": "Auth library at latest patched version; CVE scan in CI; pin via lock file"}
      ]
    }
  ]
}
```

Store the confirmed subset as `THREATS`. Any item the user deselects must
have a one-line justification recorded in the final report.

## 5b — Emit threat-by-threat implementation hints (inline)

For each item in `THREATS`, print ONE implementation line. Examples:

- `CSRF` → "middleware double-submit: cookie `__Host-csrf` + `X-CSRF-Token`
  header; `subtle.timingSafeEqual` on compare."
- `Timing attacks` → "Rust `subtle::ConstantTimeEq`; Node
  `crypto.timingSafeEqual`; Python `hmac.compare_digest`."
- `Passkey recovery` → "register 10 single-use codes at passkey creation;
  store argon2id hashes; mark consumed on use."

Keep each line short — full guidance lives in the upstream blocks, not in
this phase.

## 5c — Final report assembly

After Phase 5 completes, emit the final report template from SKILL.md with
all variables filled. Add at the bottom:

```
Deselected threats (with justification):
- <threat name>: <one-line justification> 
(or "none" if THREATS covers every default)
```

## Verify-criterion

- `THREATS` has ≥8 of the 12 defaults selected; each deselection carries a
  justification line.
- Every selected threat has an implementation hint printed (5b).
- Final report (5c) emits the full `=== AUTH-SETUP REPORT ===` block from
  SKILL.md.
