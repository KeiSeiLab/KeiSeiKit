// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`build_webauthn`] — thin wrapper around `webauthn_rs::WebauthnBuilder`.
//!
//! Centralises relying-party (RP) configuration so [`crate::WebauthnProvider`]
//! constructors stay short. No logic of our own — defaults come from
//! webauthn-rs upstream (FIDO MDS optional, ResidentKey by default).

use crate::error::{Error, Result};
use url::Url;
use webauthn_rs::prelude::{Webauthn, WebauthnBuilder};

/// Build a [`Webauthn`] instance for a given relying party.
///
/// - `rp_id`     — registrable domain suffix (e.g. `"example.com"`). Must
///                 match the `rp_origin` host (or be a parent suffix of it).
/// - `rp_origin` — full origin URL, scheme + host + optional port (e.g.
///                 `"https://example.com"` or `"http://localhost:8080"`).
/// - `rp_name`   — human-readable RP name shown in authenticator UI.
///
/// Returns [`Error::Url`] if `rp_origin` does not parse, or
/// [`Error::WebauthnRs`] if webauthn-rs rejects the (rp_id, rp_origin) pair.
pub fn build_webauthn(rp_id: &str, rp_origin: &str, rp_name: &str) -> Result<Webauthn> {
    let origin = Url::parse(rp_origin).map_err(Error::Url)?;
    let builder = WebauthnBuilder::new(rp_id, &origin)?.rp_name(rp_name);
    Ok(builder.build()?)
}
