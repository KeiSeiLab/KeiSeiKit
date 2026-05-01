// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`Dna`] — newtype wrapping the validated wire string from
//! `kei_shared::dna`. Construction goes through [`DnaBuilder`] which
//! computes scope_sha + body_sha deterministically and rolls a fresh
//! random nonce per call.

use kei_shared::dna::{compose_dna, parse_dna, DnaError, ParsedDna};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A validated DNA serial. Always parseable by [`parse_dna`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dna(String);

impl Dna {
    /// Wrap an existing string. Errors if not parseable.
    pub fn parse(s: impl Into<String>) -> Result<Self, DnaError> {
        let s: String = s.into();
        let _ = parse_dna(&s)?;
        Ok(Dna(s))
    }

    /// Borrow the wire-format string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Re-parse into `ParsedDna` view (cheap; the wire format is the SSoT).
    pub fn parsed(&self) -> ParsedDna {
        // Safe: we only construct `Dna` via parse, so re-parse cannot fail.
        parse_dna(&self.0).expect("Dna invariant: always parseable")
    }

    pub fn role(&self) -> String {
        self.parsed().role
    }

    pub fn caps(&self) -> String {
        self.parsed().caps
    }

    pub fn scope_sha(&self) -> String {
        self.parsed().scope_sha
    }

    pub fn body_sha(&self) -> String {
        self.parsed().body_sha
    }

    pub fn nonce(&self) -> String {
        self.parsed().nonce
    }
}

impl std::fmt::Display for Dna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Build a fresh DNA from semantic inputs.
///
/// `scope` and `body` are arbitrary bytes whose first 8 hex chars of
/// SHA-256 are baked into the wire format. `caps` is a `-`-joined
/// uppercase tag list (see DNA-CONVENTION.md glossary).
pub struct DnaBuilder {
    role: String,
    caps: Vec<String>,
    scope_input: Vec<u8>,
    body_input: Vec<u8>,
}

impl DnaBuilder {
    pub fn new(role: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            caps: Vec::new(),
            scope_input: Vec::new(),
            body_input: Vec::new(),
        }
    }

    pub fn cap(mut self, tag: impl Into<String>) -> Self {
        self.caps.push(tag.into());
        self
    }

    pub fn caps<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.caps.extend(tags.into_iter().map(Into::into));
        self
    }

    pub fn scope(mut self, scope: impl AsRef<[u8]>) -> Self {
        self.scope_input = scope.as_ref().to_vec();
        self
    }

    pub fn body(mut self, body: impl AsRef<[u8]>) -> Self {
        self.body_input = body.as_ref().to_vec();
        self
    }

    pub fn build(self) -> Result<Dna, DnaError> {
        let caps_str = self.caps.join("-");
        let scope_sha = sha256_hex8(&self.scope_input);
        let body_sha = sha256_hex8(&self.body_input);
        let nonce = random_hex8_lower();
        let s = compose_dna(&self.role, &caps_str, &scope_sha, &body_sha, &nonce);
        Dna::parse(s)
    }
}

fn sha256_hex8(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let digest = hasher.finalize();
    // First 4 bytes = 8 uppercase hex chars (matches DnaError::HexWidth check).
    format!(
        "{:02X}{:02X}{:02X}{:02X}",
        digest[0], digest[1], digest[2], digest[3]
    )
}

fn random_hex8_lower() -> String {
    let mut buf = [0u8; 4];
    rand::thread_rng().fill_bytes(&mut buf);
    format!("{:02x}{:02x}{:02x}{:02x}", buf[0], buf[1], buf[2], buf[3])
}

/// Trait every registerable entity must implement.
///
/// Foundational rule: an entity without a DNA cannot be registered. The
/// `kei-runtime-core::Registry` refuses to insert anything that doesn't
/// satisfy this trait.
pub trait HasDna {
    fn dna(&self) -> &Dna;
    fn parent_dna(&self) -> Option<&Dna>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_parseable_dna() {
        let d = DnaBuilder::new("vm-managed")
            .caps(["HZ", "CX22", "NB"])
            .scope("keiseikit.dev/vms/hetzner/nbg1")
            .body(br#"{"tier":"cx22","cloud_init_sha":"abc"}"#)
            .build()
            .expect("build ok");
        assert_eq!(d.role(), "vm-managed");
        assert_eq!(d.caps(), "HZ-CX22-NB");
        assert_eq!(d.scope_sha().len(), 8);
        assert_eq!(d.body_sha().len(), 8);
        assert_eq!(d.nonce().len(), 8);
    }

    #[test]
    fn nonces_differ_across_builds() {
        let s = "scope";
        let b = b"body";
        let d1 = DnaBuilder::new("user").cap("EM").scope(s).body(b).build().unwrap();
        let d2 = DnaBuilder::new("user").cap("EM").scope(s).body(b).build().unwrap();
        assert_eq!(d1.scope_sha(), d2.scope_sha());
        assert_eq!(d1.body_sha(), d2.body_sha());
        assert_ne!(d1.nonce(), d2.nonce(), "nonces must differ");
    }

    #[test]
    fn parse_round_trip() {
        let d = DnaBuilder::new("sleep-run").cap("ST").scope("x").body("y").build().unwrap();
        let s = d.as_str().to_string();
        let r = Dna::parse(&s).unwrap();
        assert_eq!(d, r);
    }
}
