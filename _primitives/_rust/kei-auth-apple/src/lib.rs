// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-auth-apple — Sign in with Apple impl of [`kei_runtime_core::AuthProvider`].
//!
//! Layout:
//! - [`error`]: local `Error`/`Result` mapping into the runtime-core error.
//! - [`client`]: thin async OAuth code-exchange client (mockable URL).
//! - [`jwt`]: unverified base64-url id_token claim decoder.
//! - [`provider`]: [`AppleAuthProvider`] — DNA-bearing trait impl.
//!
//! Endpoints:
//! - Authorize: `https://appleid.apple.com/auth/authorize`
//! - Token:     `https://appleid.apple.com/auth/token`
//!
//! Auth required (env):
//! - `APPLE_OAUTH_CLIENT_ID`        — services-id reverse domain (e.g. `com.example.web`).
//! - `APPLE_CLIENT_SECRET_JWT`      — pre-built ES256 client_secret JWT.
//! - `APPLE_OAUTH_REDIRECT_URI`     — registered redirect URI.
//!
//! KNOWN LIMITATION (v0.1):
//! - Apple requires `client_secret` to be an ES256-signed JWT over
//!   `(team_id, bundle_id, key_id)`. Producing that JWT is OUT OF SCOPE for
//!   this atomic cube; the caller MUST supply a pre-built JWT in
//!   `APPLE_CLIENT_SECRET_JWT`. Signing the JWT will live in a future sister
//!   crate `kei-auth-apple-jwt`.
//! - The id_token returned by Apple is a JWT signed with Apple's JWKS. v0.1
//!   decodes the claims segment WITHOUT signature verification. Full JWKS
//!   validation also lives in the future `kei-auth-apple-jwt` cube.

pub mod client;
pub mod error;
pub mod jwt;
pub mod provider;

pub use client::{AppleAuthClient, TokenResponse};
pub use error::{Error, Result};
pub use jwt::{decode_id_token_unverified, IdTokenClaims};
pub use provider::AppleAuthProvider;
