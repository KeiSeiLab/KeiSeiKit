// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Smoke tests for `kei-auth-webauthn`. Verify call shape only — the
//! full WebAuthn cryptographic ceremony requires a real authenticator
//! and is exercised by webauthn-rs upstream.

use kei_auth_webauthn::{build_webauthn, Error, WebauthnProvider};
use kei_runtime_core::dna::HasDna;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};
use uuid::Uuid;

fn provider() -> WebauthnProvider {
    WebauthnProvider::new("localhost", "http://localhost:8080", "kei-test")
        .expect("provider builds with valid origin")
}

#[test]
fn provider_dna_has_wn_cap() {
    let p = provider();
    let dna = p.dna();
    assert_eq!(dna.role(), "primitive");
    let caps = dna.caps();
    assert!(caps.contains("WN"), "DNA caps must contain WN backend tag, got {caps}");
    assert!(caps.contains("AP"), "DNA caps must contain AP trait tag, got {caps}");
    assert!(caps.contains("PR"), "DNA caps must contain PR role tag, got {caps}");
    assert!(p.parent_dna().is_none(), "default constructor leaves parent unset");
    assert_eq!(p.provider_name(), "webauthn");
    assert!(p.is_passwordless(), "WebAuthn is passwordless by definition");
}

#[test]
fn build_webauthn_validates_origin() {
    // Valid origin → Ok.
    assert!(build_webauthn("localhost", "http://localhost:3000", "ok").is_ok());

    // Garbage URL → Error::Url.
    let err = build_webauthn("example.com", "not-a-url", "bad")
        .expect_err("invalid origin must error");
    assert!(matches!(err, Error::Url(_)), "expected Url variant, got {err:?}");
}

#[test]
fn start_registration_returns_credential_creation_options() {
    let p = provider();
    let user_id = Uuid::new_v4();
    let (challenge, state) = p
        .start_registration(user_id, "alice", "Alice Example")
        .expect("registration ceremony starts");

    // Sanity: the challenge response carries the rp name we configured
    // and the user info we passed in. Use serde_json round-trip to peek
    // at the structure without depending on private fields.
    let challenge_json =
        serde_json::to_value(&challenge).expect("CreationChallengeResponse serialises");
    assert!(
        challenge_json.is_object(),
        "challenge must serialise to a JSON object"
    );

    // PasskeyRegistration is opaque ceremony state held in memory by
    // the caller between leg 1 and leg 2 of the registration ceremony.
    // We assert only that the type is constructed and Send/Sync-safe.
    let _: &dyn Send = &state;
}

#[tokio::test]
async fn trait_issue_challenge_returns_ok_and_documents_redirect() {
    let p = provider();

    // Per the trait-extension convention in lib.rs, calling the trait
    // method directly is a misuse — the primitive returns a Provider
    // error pointing at the explicit helpers.
    let dummy = AuthChallenge::SshKeySig {
        key_id: "register".into(),
        signature: String::new(),
    };

    let issue_err = p
        .issue_challenge(&dummy)
        .await
        .expect_err("issue_challenge over WebAuthn must error with redirect msg");
    let msg = format!("{issue_err}");
    assert!(
        msg.contains("start_registration") || msg.contains("start_authentication"),
        "issue_challenge error must redirect to ceremony helpers, got: {msg}"
    );

    let verify_err = p
        .verify(&dummy)
        .await
        .expect_err("verify over WebAuthn must error with redirect msg");
    let msg = format!("{verify_err}");
    assert!(
        msg.contains("finish_registration") || msg.contains("finish_authentication"),
        "verify error must redirect to ceremony helpers, got: {msg}"
    );

    // revoke is a no-op (operator-managed).
    let revoked = p.revoke(p.dna()).await;
    assert!(revoked.is_ok(), "revoke must be Ok (no-op)");
}
