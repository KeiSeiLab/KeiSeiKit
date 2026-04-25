# AUTH — Passkeys (WebAuthn / FIDO2)

Phishing-resistant, passwordless authentication via public-key credentials bound to the Relying Party. For federated login see `auth-oauth2-oidc.md`; for session issuance after passkey assertion see `auth-sessions.md`.

## When to include

- Greenfield auth: passkeys as PRIMARY login (password-optional or password-less).
- Existing password login: passkeys as stronger step-up or second factor that also replaces the password.
- Any consumer product — Apple, Google, Microsoft all ship platform authenticators (Touch ID / Face ID / Windows Hello) and sync passkeys across devices via iCloud Keychain / Google Password Manager / Microsoft Authenticator as of 2024–2026.

## What it declares

- **Two ceremonies:**
  - **Registration** — server sends `PublicKeyCredentialCreationOptions` (random `challenge`, `rp.id`, `rp.name`, `user.id` opaque, `pubKeyCredParams` prefer ES256=-7 and RS256=-257, `authenticatorSelection`, `attestation: "none"` unless regulated). Client returns `attestationObject` + `clientDataJSON`. Server verifies and stores `credentialID`, `publicKey`, `signCount`, `transports`, `backupEligible`, `backupState`.
  - **Assertion (login)** — server sends `PublicKeyCredentialRequestOptions` (fresh random `challenge`, `rpId`, `allowCredentials` list or empty for discoverable). Client returns `signature` + `authenticatorData` + `clientDataJSON`. Server verifies signature with stored `publicKey`, checks `signCount` strictly > stored, origin, `rpId` hash.
- **RP ID** = eTLD+1 or a subdomain of it (`example.com` covers `app.example.com`; a passkey for `app.example.com` does NOT work on `example.com`). Pick RP ID carefully at launch — changing it invalidates every existing credential.
- **Resident / discoverable credentials** (`residentKey: "required"` + `userVerification: "required"`) enable username-less login ("Sign in" button with no email field). Requires passkey-capable authenticator.
- **Platform vs cross-platform:** `authenticatorAttachment: "platform"` = Touch ID / Face ID / Windows Hello (synced, convenient). `"cross-platform"` = roaming security keys (YubiKey, Titan). Leave unset to accept both.
- **Challenge**: 16+ random bytes per ceremony, single-use, time-boxed (≤5 min), bound to server session, rejected on replay.
- **Libraries:** SimpleWebAuthn (TS — reference implementation, covers both server + browser), webauthn-rs (Rust, `Webauthn` builder + `passkey` feature), fido2-rs (low-level), py_webauthn (Python). NEVER roll CBOR / COSE parsing by hand.
- **Recovery path REQUIRED** before enabling passkey-only — lose device, lose account. Ship at least one of: email magic-link fallback, passkey backup codes, OAuth federation as recovery. User opts out of recovery only after explicit warning.

## References

- W3C WebAuthn Level 3 (2024-ready), FIDO2 CTAP 2.1, passkeys.dev [E1 — W3C/FIDO specs].
- `auth-sessions.md` for cookie issuance after `verifyAuthenticationResponse` succeeds.
- Evidence grade [E2] — Apple/Google/Microsoft production since 2023–2024; SimpleWebAuthn 10.x stable.
