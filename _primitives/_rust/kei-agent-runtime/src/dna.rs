//! Layer G — DNA identity for agent invocations.
//!
//! DNA format:  `<role>::<caps-bitmap>::<scope-hash>::<body-hash>-<nonce>`
//! where
//!   - `role`        — role slug, e.g. `edit-local`
//!   - `caps-bitmap` — hyphen-separated 2-char atom codes (ordered, from
//!                     the resolved capability list)
//!   - `scope-hash`  — 8-char truncated SHA-256 of canonicalised scope fields
//!                     (32-bit; widened from 16-bit to push birthday collision
//!                     threshold from ~256 to ~65k agents per role+caps group)
//!   - `body-hash`   — 8-char truncated SHA-256 of `task.body.text` (32-bit)
//!   - `nonce`       — 8-char hex from `rand::random::<u32>()` (full 32-bit
//!                     entropy; was 16-bit pre-2026-04 H4/M4/S3 widening)
//!
//! Constructor Pattern: one cube = DNA identity primitive only. No I/O.
//!
//! Round-trip: `compose` → `render` → `parse` → equal.
//! Parse accepts both shipped DNA strings and hand-written ones; it enforces
//! the 5-segment shape but tolerates arbitrary (non-empty) segment content
//! so future schema extensions don't break old ledger rows. For rolling
//! upgrade, 4-hex legacy hash/nonce values still parse silently — the
//! fallback is a successful parse path, not an error.
//!
//! Wire-format SSoT lives in `kei_shared::dna` — `render()` delegates to
//! `kei_shared::compose_dna` so the format string exists in one place.
//! Strict parser primitives from `kei_shared` (`parse_dna`, `ParsedDna`,
//! `is_hex8`) are re-exported for callers that want width validation;
//! the in-crate lenient `Dna::parse` stays for rolling-upgrade support.

use crate::capability::TaskSpec;
use crate::role::ResolvedRole;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Re-export of the strict wire-format parser from `kei_shared::dna`.
/// Callers needing 8-hex width validation (e.g. kei-dna-index) use these;
/// rolling-upgrade callers use the lenient [`Dna::parse`] below.
pub use kei_shared::dna::{is_hex8, parse_dna, ParsedDna};

/// Capability-name → 2-char atom code lookup.
///
/// Stable, extensible — additions allowed; removals NOT. `compose` emits
/// `?\?` for unknown names so missing entries are visibly flagged rather
/// than silently dropped.
pub const CAP_CODES: &[(&str, &str)] = &[
    ("policy::no-git-ops", "NG"),
    ("scope::files-whitelist", "FW"),
    ("scope::files-denylist", "FD"),
    ("quality::constructor-pattern", "CP"),
    ("quality::cargo-check-green", "CG"),
    ("quality::tests-green", "TG"),
    ("safety::no-dep-bump", "ND"),
    ("output::report-format", "RF"),
    ("output::severity-grade", "SG"),
    ("tools::deny-tools", "DT"),
    ("tools::bash-allowlist", "BA"),
];

/// Agent DNA — composition fingerprint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dna {
    pub role: String,
    pub caps_bitmap: String,
    pub scope_hash: String,
    pub body_hash: String,
    pub nonce: String,
}

/// Error during lenient rolling-upgrade DNA parsing.
///
/// Distinct from [`kei_shared::dna::DnaError`]: this variant is lenient
/// (accepts legacy 4-hex segment widths), and shape-failure is the only
/// error class. Segment-content validation is deferred to callers that
/// care about widths — they can re-parse with `kei_shared::parse_dna`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DnaError {
    #[error("DNA string must have 4 `::` segments and `<body>-<nonce>` tail")]
    Shape,
    #[error("DNA segment `{0}` is empty")]
    EmptySegment(&'static str),
}

impl Dna {
    /// Build DNA from a task + already-resolved role.
    pub fn compose(task: &TaskSpec, resolved: &ResolvedRole) -> Self {
        let caps_bitmap = build_caps_bitmap(&resolved.required);
        let scope_hash = short_sha256(&canonical_scope(task));
        let body_hash = short_sha256(&task.body.text);
        let nonce = nonce_hex();
        Self {
            role: task.task.role.clone(),
            caps_bitmap,
            scope_hash,
            body_hash,
            nonce,
        }
    }

    /// Render to the canonical wire format. Delegates the format-string
    /// SSoT to `kei_shared::dna::compose_dna`.
    pub fn render(&self) -> String {
        kei_shared::dna::compose_dna(
            &self.role,
            &self.caps_bitmap,
            &self.scope_hash,
            &self.body_hash,
            &self.nonce,
        )
    }

    /// Parse a DNA string. Lenient on segment content, strict on shape.
    /// Accepts both 8-hex (current) and 4-hex (legacy pre-widening) values
    /// for `scope_hash`, `body_hash`, `nonce` — both widths parse silently.
    pub fn parse(s: &str) -> Result<Self, DnaError> {
        let parts: Vec<&str> = s.splitn(4, "::").collect();
        if parts.len() != 4 {
            return Err(DnaError::Shape);
        }
        let (role, caps_bitmap, scope_hash) = (parts[0], parts[1], parts[2]);
        let (body_hash, nonce) = parts[3].rsplit_once('-').ok_or(DnaError::Shape)?;
        ensure_non_empty(role, caps_bitmap, scope_hash, body_hash, nonce)?;
        Ok(Self {
            role: role.into(),
            caps_bitmap: caps_bitmap.into(),
            scope_hash: scope_hash.into(),
            body_hash: body_hash.into(),
            nonce: nonce.into(),
        })
    }
}

fn ensure_non_empty(
    role: &str,
    caps_bitmap: &str,
    scope_hash: &str,
    body_hash: &str,
    nonce: &str,
) -> Result<(), DnaError> {
    for (name, value) in [
        ("role", role),
        ("caps_bitmap", caps_bitmap),
        ("scope_hash", scope_hash),
        ("body_hash", body_hash),
        ("nonce", nonce),
    ] {
        if value.is_empty() {
            return Err(DnaError::EmptySegment(name));
        }
    }
    Ok(())
}

fn build_caps_bitmap(caps: &[String]) -> String {
    caps.iter()
        .map(|c| code_for(c).to_string())
        .collect::<Vec<_>>()
        .join("-")
}

fn code_for(cap_name: &str) -> &'static str {
    CAP_CODES
        .iter()
        .find(|(n, _)| *n == cap_name)
        .map(|(_, c)| *c)
        .unwrap_or("??")
}

fn canonical_scope(task: &TaskSpec) -> String {
    let mut wl = task.scope.files_whitelist.clone();
    wl.sort();
    let mut dl = task.scope.files_denylist.clone();
    dl.sort();
    format!("wl={}\ndl={}", wl.join(","), dl.join(","))
}

fn short_sha256(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    // 4 bytes = 8 hex chars = 32-bit truncation (widened from 16-bit).
    format!(
        "{:02X}{:02X}{:02X}{:02X}",
        digest[0], digest[1], digest[2], digest[3]
    )
}

fn nonce_hex() -> String {
    // 32-bit nonce (widened from 16-bit). Birthday collision threshold
    // ~65k DNAs sharing the same role+caps+scope+body triple.
    let r: u32 = rand::random();
    format!("{r:08x}")
}
