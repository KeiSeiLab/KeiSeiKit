// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

//! Smoke tests for kei-auth-magiclink.
//!
//! Covers the token codec only — provider tests live behind the same
//! `MagicLinkProvider::new` constructor and exercise the trait surface
//! via direct calls (not network).

use kei_auth_magiclink::{build_token, parse_token, Error};
use kei_auth_magiclink::MagicLinkProvider;
use kei_runtime_core::dna::{DnaBuilder, HasDna};
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};

const KEY: &[u8] = b"0123456789abcdef0123456789abcdef"; // 32 bytes
const KEY2: &[u8] = b"FEDCBA9876543210FEDCBA9876543210"; // different 32 bytes

fn future_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    now + 60_000 // 60s in the future
}

fn parent_dna() -> kei_runtime_core::Dna {
    DnaBuilder::new("user")
        .cap("EM")
        .scope("keiseikit.dev/test/parent")
        .body(b"test-user")
        .build()
        .expect("parent dna build ok")
}

#[test]
fn build_and_parse_roundtrip() {
    let exp = future_ms();
    let token = build_token("alice@example.com", exp, KEY);
    let (email, decoded_exp) = parse_token(&token, KEY, 0).expect("parse ok");
    assert_eq!(email, "alice@example.com");
    assert_eq!(decoded_exp, exp);
}

#[test]
fn expired_token_rejected() {
    // Token expired at t=1000ms; we evaluate at t=2000ms.
    let token = build_token("bob@example.com", 1_000, KEY);
    let err = parse_token(&token, KEY, 2_000).expect_err("must reject");
    match err {
        Error::TokenExpired { expires_unix_ms, now_unix_ms } => {
            assert_eq!(expires_unix_ms, 1_000);
            assert_eq!(now_unix_ms, 2_000);
        }
        other => panic!("expected TokenExpired, got {other:?}"),
    }
}

#[test]
fn tampered_hmac_rejected() {
    let token = build_token("carol@example.com", future_ms(), KEY);
    // Flip last char of the tag part (a base64url char). If the random
    // pick happened to flip to itself, force a different one.
    let mut chars: Vec<char> = token.chars().collect();
    let last = *chars.last().unwrap();
    let replacement = if last == 'A' { 'B' } else { 'A' };
    *chars.last_mut().unwrap() = replacement;
    let tampered: String = chars.into_iter().collect();
    let err = parse_token(&tampered, KEY, 0).expect_err("must reject");
    // Could be BadSignature OR TokenMalformed (if last byte broke base64).
    assert!(
        matches!(err, Error::BadSignature | Error::TokenMalformed(_)),
        "expected BadSignature or TokenMalformed, got {err:?}"
    );
}

#[test]
fn malformed_two_parts_rejected() {
    let bad = "AAAA.BBBB";
    let err = parse_token(bad, KEY, 0).expect_err("must reject");
    match err {
        Error::TokenMalformed(_) => {}
        other => panic!("expected TokenMalformed, got {other:?}"),
    }
}

#[test]
fn unicode_email_roundtrip() {
    let exp = future_ms();
    let email = "пользователь@пример.рф";
    let token = build_token(email, exp, KEY);
    let (decoded, _) = parse_token(&token, KEY, 0).expect("parse ok");
    assert_eq!(decoded, email);
}

#[test]
fn unknown_hmac_key_rejected() {
    let token = build_token("dave@example.com", future_ms(), KEY);
    let err = parse_token(&token, KEY2, 0).expect_err("must reject");
    assert!(
        matches!(err, Error::BadSignature),
        "expected BadSignature, got {err:?}"
    );
}

#[test]
fn provider_verify_full_flow() {
    let parent = parent_dna();
    let provider = MagicLinkProvider::new(parent.clone(), KEY.to_vec(), 900)
        .expect("provider new ok");
    assert_eq!(provider.provider_name(), "magiclink");
    assert!(provider.is_passwordless());
    assert_eq!(provider.ttl_secs(), 900);

    // Build a valid token via the same key.
    let token = build_token("eve@example.com", future_ms(), KEY);
    let challenge = AuthChallenge::MagicLink { email: token.clone() };

    let rt = tokio::runtime::Runtime::new().expect("runtime");
    let session = rt
        .block_on(provider.verify(&challenge))
        .expect("verify ok");
    assert_eq!(session.user_id, "eve@example.com");
    assert_eq!(session.parent_dna.as_str(), parent.as_str());
    assert!(session.expires_unix_ms > 0);
    // Provider DNA must round-trip via HasDna.
    assert_eq!(provider.dna().role(), "primitive");
    assert_eq!(provider.dna().caps(), "PR-AP-ML");
}

#[test]
fn provider_short_key_rejected() {
    let parent = parent_dna();
    let err = MagicLinkProvider::new(parent, b"short".to_vec(), 900)
        .expect_err("must reject short key");
    assert!(
        matches!(err, Error::KeyMissing(_)),
        "expected KeyMissing, got {err:?}"
    );
}

#[test]
fn provider_build_magic_url_shape() {
    let parent = parent_dna();
    let provider = MagicLinkProvider::new(parent, KEY.to_vec(), 900)
        .expect("provider new ok");
    let url = provider.build_magic_url("https://app.example.com/", "frank@example.com");
    assert!(
        url.starts_with("https://app.example.com/auth/magic?token="),
        "url = {url}"
    );
    // Trailing slash on base_url MUST be stripped.
    assert!(!url.contains("//auth/magic"));
}
