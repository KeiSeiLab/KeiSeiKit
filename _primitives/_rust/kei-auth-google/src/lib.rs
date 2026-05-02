// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-auth-google — `AuthProvider` impl for Google OAuth 2.0 + OIDC.
//!
//! Wave 7 atomar primitive. Implements
//! [`kei_runtime_core::traits::auth::AuthProvider`] over Google's
//! authorization-code flow (RFC 6749 §4.1) plus the OIDC userinfo endpoint
//! to surface a stable `sub`/`email` for [`AuthSession::user_id`].
//!
//! ## Endpoints
//!
//! | role        | URL                                                  |
//! |-------------|------------------------------------------------------|
//! | authorize   | `https://accounts.google.com/o/oauth2/v2/auth`       |
//! | token       | `https://oauth2.googleapis.com/token`                |
//! | userinfo    | `https://openidconnect.googleapis.com/v1/userinfo`   |
//!
//! ## Quick start
//!
//! ```ignore
//! use kei_auth_google::{GoogleAuthClient, GoogleAuthProvider};
//! use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};
//!
//! # async fn ex() -> kei_runtime_core::Result<()> {
//! let client = GoogleAuthClient::from_env()?;
//! let provider = GoogleAuthProvider::new(client, None)?;
//! let challenge = AuthChallenge::OAuthCode {
//!     provider: "google".into(),
//!     code: "<code from redirect>".into(),
//!     state: "<csrf state from callback>".into(),
//!     expected_state: "<csrf state you generated>".into(),
//!     code_verifier: Some("<pkce verifier stored in session>".into()),
//! };
//! let session = provider.verify(&challenge).await?;
//! # let _ = session;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod pkce;
pub mod provider;

pub use client::{GoogleAuthClient, TokenResponse, UserInfo};
pub use error::{Error, Result};
pub use pkce::pkce_challenge;
pub use provider::GoogleAuthProvider;
