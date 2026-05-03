// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Pure helpers extracted from [`crate::provider`]. Each one is a
//! single-responsibility check used inside `verify()` — split out so
//! the provider file stays under the 200-LOC Constructor Pattern bound
//! and so the security-critical predicates are unit-testable in
//! isolation (no HTTP, no async).

use crate::client::{TokenResponse, UserInfo};
use crate::error::Error;
use crate::id_token::extract_sub as extract_id_token_sub;
use kei_runtime_core::traits::auth::AuthChallenge;
use subtle::ConstantTimeEq;

/// Pull `(code, state, expected_state, code_verifier)` out of an
/// [`AuthChallenge::OAuthCode`] for `provider == "google"`.
pub(crate) fn unpack_challenge<'a>(
    c: &'a AuthChallenge,
) -> kei_runtime_core::Result<(&'a str, &'a str, &'a str, Option<&'a str>)> {
    match c {
        AuthChallenge::OAuthCode {
            provider, code, state, expected_state, code_verifier,
        } if provider == "google" => Ok((
            code.as_str(),
            state.as_str(),
            expected_state.as_str(),
            code_verifier.as_deref(),
        )),
        AuthChallenge::OAuthCode { provider, .. } => Err(kei_runtime_core::Error::Auth(
            format!("wrong provider for google: {provider}"),
        )),
        _ => Err(kei_runtime_core::Error::from(Error::MissingState)),
    }
}

/// Constant-time CSRF-state compare. Returns
/// [`kei_runtime_core::Error::CsrfStateMismatch`] on disagreement.
pub(crate) fn check_state(got: &str, expected: &str) -> kei_runtime_core::Result<()> {
    let ok: bool = got.as_bytes().ct_eq(expected.as_bytes()).into();
    if !ok {
        Err(kei_runtime_core::Error::CsrfStateMismatch)
    } else {
        Ok(())
    }
}

/// Reject userinfo where `email_verified` is absent / false.
///
/// CVE-2023-7028 class fix: Google Workspace admins can mint accounts
/// with arbitrary unverified email aliases. Trusting `email` without
/// the verified flag is account-takeover-equivalent.
pub(crate) fn enforce_email_verified(info: &UserInfo) -> kei_runtime_core::Result<()> {
    if !info.email_verified {
        return Err(kei_runtime_core::Error::from(Error::EmailNotVerified));
    }
    Ok(())
}

/// If `token.id_token` is `Some`, decode its claims and require
/// `id_token.sub == info.sub`. Skipped (Ok) when absent. Signature
/// verification is a follow-up; this is defence-in-depth against a
/// forged userinfo response.
pub(crate) fn cross_check_id_token_sub(
    token: &TokenResponse,
    info: &UserInfo,
) -> kei_runtime_core::Result<()> {
    let Some(id_token) = token.id_token.as_deref() else {
        return Ok(());
    };
    let id_sub = extract_id_token_sub(id_token)
        .map_err(kei_runtime_core::Error::from)?;
    if id_sub != info.sub {
        return Err(kei_runtime_core::Error::from(Error::IdSubMismatch));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ui(email_verified: bool, sub: &str) -> UserInfo {
        UserInfo {
            sub: sub.into(),
            email: "x@y.z".into(),
            email_verified,
            name: "X".into(),
        }
    }

    #[test]
    fn enforce_email_verified_passes_when_true() {
        assert!(enforce_email_verified(&ui(true, "abc")).is_ok());
    }

    #[test]
    fn enforce_email_verified_rejects_false() {
        let err = enforce_email_verified(&ui(false, "abc")).unwrap_err();
        assert!(format!("{err}").contains("not verified"));
    }

    #[test]
    fn cross_check_no_id_token_is_ok() {
        let tok = TokenResponse { access_token: "t".into(), expires_in: 0, id_token: None };
        assert!(cross_check_id_token_sub(&tok, &ui(true, "abc")).is_ok());
    }

    #[test]
    fn check_state_constant_time_ok() {
        assert!(check_state("abc", "abc").is_ok());
        assert!(check_state("abc", "abd").is_err());
    }
}
