//! DNA parser — thin wrapper over `kei_shared::dna`.
//!
//! Format: `<role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>`
//! Example: `edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-e9bf468d`
//!
//! Wire-format SSoT lives in `kei_shared::dna`. This module re-exports
//! `ParsedDna` and exposes `split_dna` that maps `kei_shared::DnaError`
//! into `crate::Error::MalformedDna` so callers keep a single error type.

use crate::error::{Error, Result};

pub use kei_shared::dna::ParsedDna;

/// Parse a DNA string into its five fields. Hex widths are validated.
/// Errors are wrapped in [`Error::MalformedDna`] with the raw DNA included
/// for debuggability, matching the pre-extraction contract.
pub fn split_dna(dna: &str) -> Result<ParsedDna> {
    kei_shared::dna::parse_dna(dna)
        .map_err(|e| Error::MalformedDna(format!("{e}: {dna}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_canonical() {
        let dna = "edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-e9bf468d";
        let p = split_dna(dna).unwrap();
        assert_eq!(p.role, "edit-local");
        assert_eq!(p.caps, "NG-FW-FD-CP-CG-TG-ND-RF");
        assert_eq!(p.scope_sha, "5435F821");
        assert_eq!(p.body_sha, "AC73A6A3");
        assert_eq!(p.nonce, "e9bf468d");
    }

    #[test]
    fn rejects_short_scope() {
        let dna = "r::c::12::AC73A6A3-e9bf468d";
        assert!(split_dna(dna).is_err());
    }

    #[test]
    fn rejects_non_hex_nonce() {
        let dna = "r::c::12345678::AC73A6A3-ZZZZZZZZ";
        assert!(split_dna(dna).is_err());
    }

    #[test]
    fn rejects_missing_body_separator() {
        let dna = "r::c::12345678::AC73A6A3e9bf468d";
        assert!(split_dna(dna).is_err());
    }

    #[test]
    fn rejects_empty_role() {
        let dna = "::c::12345678::AC73A6A3-e9bf468d";
        assert!(split_dna(dna).is_err());
    }
}
