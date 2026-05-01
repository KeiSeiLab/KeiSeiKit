// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`WebauthnProvider`] — DNA-bearing [`AuthProvider`] impl that wraps
//! a configured `webauthn_rs::Webauthn` instance.
//!
//! See `crate` doc for the trait-extension convention (`AuthChallenge`
//! has no `Webauthn` variant; the trait methods point callers at the
//! explicit ceremony helpers below).

use crate::builder::build_webauthn;
use crate::error::{Error, Result};
use async_trait::async_trait;
use kei_runtime_core::dna::{Dna, DnaBuilder, HasDna};
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};
use uuid::Uuid;
use webauthn_rs::prelude::{
    AuthenticationResult, CreationChallengeResponse, Passkey, PasskeyAuthentication,
    PasskeyRegistration, PublicKeyCredential, RegisterPublicKeyCredential,
    RequestChallengeResponse, Webauthn,
};

/// WebAuthn passkey AuthProvider. Stateless — owns no session store and no
/// credential store; the caller is responsible for round-tripping the
/// ceremony state ([`PasskeyRegistration`] / [`PasskeyAuthentication`])
/// between leg 1 and leg 2.
pub struct WebauthnProvider {
    dna: Dna,
    parent: Option<Dna>,
    webauthn: Webauthn,
}

impl WebauthnProvider {
    /// Construct a new provider with no parent DNA.
    ///
    /// See [`build_webauthn`] for `rp_id` / `rp_origin` / `rp_name`
    /// semantics.
    pub fn new(rp_id: &str, rp_origin: &str, rp_name: &str) -> Result<Self> {
        Self::with_parent(rp_id, rp_origin, rp_name, None)
    }

    /// Construct with an explicit parent DNA (for genealogy attribution).
    pub fn with_parent(
        rp_id: &str,
        rp_origin: &str,
        rp_name: &str,
        parent: Option<Dna>,
    ) -> Result<Self> {
        let webauthn = build_webauthn(rp_id, rp_origin, rp_name)?;
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "WN"])
            .scope("keiseikit.dev/primitives/kei-auth-webauthn")
            .body(b"webauthn-rs-v0.5")
            .build()?;
        Ok(Self {
            dna,
            parent,
            webauthn,
        })
    }

    /// Borrow the configured `Webauthn` instance (escape hatch for
    /// callers who need an upstream API the helpers don't expose).
    pub fn webauthn(&self) -> &Webauthn {
        &self.webauthn
    }

    /// Begin a passkey-registration ceremony.
    ///
    /// Returns the challenge to ship to the browser (`navigator.credentials.create`)
    /// AND the server-side state to round-trip back into
    /// [`Self::finish_registration`].
    pub fn start_registration(
        &self,
        user_unique_id: Uuid,
        user_name: &str,
        user_display_name: &str,
    ) -> Result<(CreationChallengeResponse, PasskeyRegistration)> {
        Ok(self.webauthn.start_passkey_registration(
            user_unique_id,
            user_name,
            user_display_name,
            None, // no exclude-credentials list at v0.1
        )?)
    }

    /// Complete a passkey-registration ceremony.
    ///
    /// `state` is the [`PasskeyRegistration`] returned by
    /// [`Self::start_registration`]; `response` is the
    /// [`RegisterPublicKeyCredential`] decoded from the browser response.
    /// On success returns the [`Passkey`] the caller MUST persist —
    /// no storage happens inside this primitive.
    pub fn finish_registration(
        &self,
        state: &PasskeyRegistration,
        response: &RegisterPublicKeyCredential,
    ) -> Result<Passkey> {
        Ok(self.webauthn.finish_passkey_registration(response, state)?)
    }

    /// Begin a passkey-authentication ceremony.
    ///
    /// `allow_credentials` is the user's known passkey set (loaded from
    /// the caller's credential store). Returns the challenge for the
    /// browser AND the server-side state.
    pub fn start_authentication(
        &self,
        allow_credentials: &[Passkey],
    ) -> Result<(RequestChallengeResponse, PasskeyAuthentication)> {
        Ok(self.webauthn.start_passkey_authentication(allow_credentials)?)
    }

    /// Complete a passkey-authentication ceremony.
    pub fn finish_authentication(
        &self,
        state: &PasskeyAuthentication,
        response: &PublicKeyCredential,
    ) -> Result<AuthenticationResult> {
        Ok(self.webauthn.finish_passkey_authentication(response, state)?)
    }
}

impl HasDna for WebauthnProvider {
    fn dna(&self) -> &Dna {
        &self.dna
    }

    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait]
impl AuthProvider for WebauthnProvider {
    fn provider_name(&self) -> &'static str {
        "webauthn"
    }

    /// Trait-shaped no-op. WebAuthn ceremonies do not fit
    /// `Result<()>`; callers MUST drive registration via
    /// [`Self::start_registration`] / [`Self::finish_registration`] and
    /// authentication via [`Self::start_authentication`] /
    /// [`Self::finish_authentication`].
    async fn issue_challenge(&self, _c: &AuthChallenge) -> kei_runtime_core::Result<()> {
        Err(Error::TraitMisuse(
            "WebauthnProvider::start_registration / start_authentication".into(),
        )
        .into())
    }

    /// Trait-shaped error. See [`Self::issue_challenge`] — drive
    /// `verify` via the explicit helpers; this method has no access to
    /// the round-tripped ceremony state.
    async fn verify(&self, _c: &AuthChallenge) -> kei_runtime_core::Result<AuthSession> {
        Err(Error::TraitMisuse(
            "WebauthnProvider::finish_registration / finish_authentication".into(),
        )
        .into())
    }

    /// Passkey revocation is operator-managed: delete the [`Passkey`]
    /// row from the caller's credential store. No-op here.
    async fn revoke(&self, _session: &Dna) -> kei_runtime_core::Result<()> {
        Ok(())
    }

    fn is_passwordless(&self) -> bool {
        true
    }
}
