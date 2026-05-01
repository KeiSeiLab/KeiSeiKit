//! Binary / blob rules for `injection_check`.
//!
//! Constructor Pattern: invisible-codepoint scan + base64-blob length
//! heuristic. No regex; per-char iteration only.

use crate::injection_check::InjectionFinding;

const INVISIBLE_CHARS: &[char] = &[
    '\u{200B}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{200F}',
    '\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}',
    '\u{2060}', '\u{FEFF}',
];

const BASE64_BLOB_MIN: usize = 1024;

/// Detect invisible / bidi unicode codepoints anywhere in `content`.
pub(crate) fn scan_invisible(content: &str) -> Option<InjectionFinding> {
    for ch in content.chars() {
        if INVISIBLE_CHARS.contains(&ch) {
            return Some(InjectionFinding {
                pattern: "invisible_unicode",
                source: "unicode:bidi",
            });
        }
    }
    None
}

/// Detect a single line >= 1024 chars composed of base64 alphabet.
pub(crate) fn scan_base64_blob(content: &str) -> Option<InjectionFinding> {
    for line in content.lines() {
        if line.len() >= BASE64_BLOB_MIN && line.chars().all(is_base64_char) {
            return Some(InjectionFinding {
                pattern: "long_base64_line",
                source: "heuristic:base64-blob",
            });
        }
    }
    None
}

fn is_base64_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='
}
