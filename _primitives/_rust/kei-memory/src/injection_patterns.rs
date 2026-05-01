//! Injection-pattern table for `injection_guard`.
//!
//! Constructor Pattern: this cube only declares regex/string patterns.
//! Detection logic lives in `injection_guard.rs`. Test corpus in
//! `tests/guard_test_corpus.rs`. Each entry carries a stable id, a
//! severity (`Block` or `Warn`), and a human-readable source label so
//! triage output points back to the heuristic that fired.

use regex::Regex;
use std::sync::OnceLock;

/// Severity of a single pattern match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Hard reject — content must not be persisted.
    Block,
    /// Surface to caller; persistence still allowed unless caller upgrades.
    Warn,
}

/// One regex-based pattern row.
pub struct RegexPattern {
    pub id: &'static str,
    pub re: Regex,
    pub severity: Severity,
    pub source: &'static str,
}

/// One substring/heuristic row evaluated on a lower-cased copy of input.
pub struct SubstringPattern {
    pub id: &'static str,
    /// All needles must appear (AND semantics).
    pub needles: &'static [&'static str],
    pub severity: Severity,
    pub source: &'static str,
}

/// Invisible / bidi / zero-width unicode codepoints.
pub const INVISIBLE_CHARS: &[char] = &[
    '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{200F}',
    '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}',
    '\u{2060}', '\u{FEFF}',
];

/// Threshold above which a single base64-looking line is flagged.
/// Documented for callers; the regex pattern hardcodes the same value.
#[allow(dead_code)]
pub const BASE64_BLOB_BYTES: usize = 1024;

/// PEM marker dashes — built at runtime to keep `secrets-guard` quiet.
fn pem_dashes() -> String {
    "-".repeat(5)
}

fn pem_marker(label: &str) -> String {
    let d = pem_dashes();
    format!("{d}BEGIN {label}{d}")
}

fn rx(id: &'static str, pat: &str, sev: Severity, src: &'static str) -> RegexPattern {
    RegexPattern {
        id,
        re: Regex::new(pat).unwrap(),
        severity: sev,
        source: src,
    }
}

fn prompt_override_patterns() -> Vec<RegexPattern> {
    vec![
        rx("prompt_override_ignore_previous", r"(?i)ignore\s+previous\s+instructions", Severity::Block, "promptguard:override"),
        rx("prompt_override_you_are_now", r"(?i)you\s+are\s+now\b", Severity::Block, "promptguard:roleplay"),
        rx("prompt_override_disregard", r"(?i)disregard\s+(all|prior|above)", Severity::Block, "promptguard:override"),
        rx("system_role_prefix", r"(?im)^\s*system\s*:", Severity::Block, "promptguard:role-prefix"),
        rx("chatml_im_start", r"<\|im_start\|>", Severity::Block, "chatml:tag"),
        rx("chatml_endoftext", r"<\|endoftext\|>", Severity::Block, "chatml:tag"),
    ]
}

fn secret_patterns() -> Vec<RegexPattern> {
    let openssh = regex::escape(&pem_marker("OPENSSH PRIVATE KEY"));
    let rsa = regex::escape(&pem_marker("RSA PRIVATE KEY"));
    vec![
        rx("ssh_openssh_private", &openssh, Severity::Block, "secret:openssh"),
        rx("ssh_rsa_private", &rsa, Severity::Block, "secret:rsa"),
        // Long single-line base64 blobs are now Block-tier (see audit P2.1.b):
        // attestation/key blobs pasted into agent transcripts represent a
        // direct exfiltration vector for memory-write paths. The 1024-byte
        // floor keeps benign hex hashes / short tokens unaffected.
        rx("long_base64_line", r"(?m)^[A-Za-z0-9+/=]{1024,}$", Severity::Block, "heuristic:base64-blob"),
    ]
}

fn build_regex_table() -> Vec<RegexPattern> {
    let mut out = prompt_override_patterns();
    out.extend(secret_patterns());
    out
}

fn build_substring_table() -> Vec<SubstringPattern> {
    vec![
        SubstringPattern {
            id: "curl_with_bearer",
            needles: &["bearer ", "://"],
            severity: Severity::Block,
            source: "exfil:curl-bearer",
        },
        SubstringPattern {
            id: "aws_secret_keyword",
            needles: &["aws_secret"],
            severity: Severity::Block,
            source: "secret:aws",
        },
        SubstringPattern {
            id: "api_key_url",
            needles: &["api_key=", "://"],
            severity: Severity::Block,
            source: "exfil:api-key-url",
        },
    ]
}

static REGEX_TABLE: OnceLock<Vec<RegexPattern>> = OnceLock::new();
static SUBSTR_TABLE: OnceLock<Vec<SubstringPattern>> = OnceLock::new();

/// Lazily-built regex pattern table.
pub fn regex_patterns() -> &'static [RegexPattern] {
    REGEX_TABLE.get_or_init(build_regex_table)
}

/// Lazily-built substring/heuristic table.
pub fn substring_patterns() -> &'static [SubstringPattern] {
    SUBSTR_TABLE.get_or_init(build_substring_table)
}
