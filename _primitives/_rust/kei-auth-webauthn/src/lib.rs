// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-auth-webauthn — WebAuthn passkey [`AuthProvider`] (Wave 7 atomar).
//!
//! Layout:
//! - [`error`]: local `Error`/`Result` mapping into the runtime-core error.
//! - [`builder`]: thin wrapper around `webauthn_rs::WebauthnBuilder`.
//! - [`provider`]: [`WebauthnProvider`] — DNA-bearing trait impl + ceremony
//!   helpers.
//!
//! ## Mechanism
//!
//! WebAuthn registration + authentication are two-leg ceremonies. The server
//! issues a **challenge** (`CreationChallengeResponse` / `RequestChallengeResponse`)
//! which the browser/authenticator answers with a credential
//! (`RegisterPublicKeyCredential` / `PublicKeyCredential`). Between leg 1 and
//! leg 2 the server MUST round-trip the challenge state
//! (`PasskeyRegistration` / `PasskeyAuthentication`) — typically through a
//! signed cookie or session store.
//!
//! This primitive is **stateless**: it owns no session store, no credential
//! store. It exposes pure ceremony APIs and delegates persistence to the
//! caller. That is the atomar contract.
//!
//! ## Trait-extension convention
//!
//! [`AuthChallenge`](kei_runtime_core::traits::auth::AuthChallenge) does not
//! have a `Webauthn` variant. To remain `AuthProvider`-compatible without
//! mutating the trait, this primitive overloads the existing
//! [`AuthChallenge::SshKeySig`] variant as the wire carrier:
//!
//! - `key_id`   — base64-encoded credential id (during `verify`), or
//!                the literal string `"register"` / `"authenticate"`
//!                (during `issue_challenge`).
//! - `signature`— base64-encoded JSON of the
//!                [`RegisterPublicKeyCredential`] / [`PublicKeyCredential`]
//!                returned by the browser ceremony.
//!
//! Because the WebAuthn ceremony state (`PasskeyRegistration` /
//! `PasskeyAuthentication`) does not fit cleanly into the trait's
//! `Result<()>` / `Result<AuthSession>` shape, **callers MUST use the
//! explicit ceremony helpers on [`WebauthnProvider`]**:
//!
//! - [`WebauthnProvider::start_registration`]
//! - [`WebauthnProvider::finish_registration`]
//! - [`WebauthnProvider::start_authentication`]
//! - [`WebauthnProvider::finish_authentication`]
//!
//! The trait methods [`AuthProvider::issue_challenge`] and
//! [`AuthProvider::verify`] return `Provider`-tagged errors that point the
//! caller at these helpers. This is documented as a deliberate trait
//! mis-fit until the runtime-core trait grows a `Webauthn` variant in a
//! future revision (see DNA-CONVENTION.md trait-extension policy).
//!
//! ## DNA
//!
//! ```ignore
//! DnaBuilder::new("primitive")
//!     .caps(["PR", "AP", "WN"])
//!     .scope("keiseikit.dev/primitives/kei-auth-webauthn")
//!     .body(b"webauthn-rs-v0.5")
//!     .build()?
//! ```
//!
//! - `PR` — Provider role tag
//! - `AP` — AuthProvider trait tag
//! - `WN` — WebauthN backend tag
//!
//! ## Quick start
//!
//! ```ignore
//! use kei_auth_webauthn::WebauthnProvider;
//!
//! # async fn ex() -> kei_auth_webauthn::Result<()> {
//! let provider = WebauthnProvider::new(
//!     "example.com",
//!     "https://example.com",
//!     "Example",
//! )?;
//!
//! // Registration leg 1 — issue challenge, persist `reg_state` in session.
//! let user_id = uuid::Uuid::new_v4();
//! let (challenge, reg_state) =
//!     provider.start_registration(user_id, "alice", "Alice")?;
//! let _ = (challenge, reg_state);
//!
//! // Registration leg 2 (after browser response) — finish + store passkey.
//! // let passkey = provider.finish_registration(&reg_state, &browser_resp)?;
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod error;
pub mod provider;

pub use builder::build_webauthn;
pub use error::{Error, Result};
pub use provider::WebauthnProvider;

// Re-export the upstream types callers need to build storage + session
// shapes around the ceremony helpers.
pub use webauthn_rs::prelude::{
    AuthenticationResult, CreationChallengeResponse, Passkey, PasskeyAuthentication,
    PasskeyRegistration, PublicKeyCredential, RegisterPublicKeyCredential,
    RequestChallengeResponse, Webauthn,
};
