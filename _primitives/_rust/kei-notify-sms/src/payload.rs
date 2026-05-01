// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! SMS body composition: severity-emoji prefix + subject + em-dash +
//! body_text, then UTF-8-safe truncation to 1500 bytes (Twilio's hard
//! per-segment limit is 1600; we keep 100 bytes of headroom).

use kei_runtime_core::traits::notify::{Notification, NotifySeverity};

/// Hard byte cap. Twilio's `Body` parameter accepts up to 1600 chars; we
/// truncate at 1500 BYTES to stay safely below that on any UTF-8 string.
const MAX_BYTES: usize = 1500;

/// Map a [`NotifySeverity`] to a single-glyph prefix. Plain ASCII so the
/// SMS encoding (GSM-7 vs UCS-2) doesn't flip on emoji presence.
pub fn severity_emoji(s: NotifySeverity) -> &'static str {
    match s {
        NotifySeverity::Info => "[i]",
        NotifySeverity::Success => "[+]",
        NotifySeverity::Warn => "[!]",
        NotifySeverity::Error => "[x]",
    }
}

/// Compose the wire body from a `Notification`. Format:
///
/// ```text
/// [<emoji>] <subject> — <body_text>
/// ```
///
/// truncated to 1500 bytes on a UTF-8 character boundary.
pub fn build_body(n: &Notification) -> String {
    let prefix = severity_emoji(n.severity);
    let raw = format!("{} {} — {}", prefix, n.subject, n.body_text);
    truncate_utf8(&raw, MAX_BYTES)
}

/// Truncate `s` to at most `max_bytes` bytes without splitting a UTF-8
/// codepoint. Walks back from `max_bytes` to the nearest char boundary.
fn truncate_utf8(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    let mut cut = max_bytes;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    s[..cut].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_runtime_core::traits::notify::{Notification, NotifySeverity};
    use kei_runtime_core::DnaBuilder;

    fn n(sev: NotifySeverity, subject: &str, body: &str) -> Notification {
        let dna = DnaBuilder::new("notification")
            .cap("NF")
            .scope("test")
            .body(b"test")
            .build()
            .unwrap();
        let parent = DnaBuilder::new("primitive")
            .cap("PR")
            .scope("test-parent")
            .body(b"parent")
            .build()
            .unwrap();
        Notification {
            dna,
            parent_dna: parent,
            subject: subject.into(),
            body_text: body.into(),
            body_html: None,
            severity: sev,
            tags: vec![],
        }
    }

    #[test]
    fn warn_emoji() {
        let body = build_body(&n(NotifySeverity::Warn, "boot", "ok"));
        assert!(body.starts_with("[!]"), "expected [!] prefix, got {body}");
    }

    #[test]
    fn error_emoji() {
        let body = build_body(&n(NotifySeverity::Error, "fail", "stack"));
        assert!(body.starts_with("[x]"), "expected [x] prefix, got {body}");
    }

    #[test]
    fn truncates_at_1500_bytes() {
        let long = "A".repeat(2000);
        let out = build_body(&n(NotifySeverity::Warn, "sub", &long));
        assert!(out.len() <= 1500, "got {} bytes", out.len());
        assert!(out.is_char_boundary(out.len()));
    }

    #[test]
    fn utf8_safe_truncation() {
        // Multibyte chars near the boundary must not split. Use 4-byte
        // emoji repeated to push the truncation point onto a boundary
        // that would otherwise split a codepoint.
        let stuffed = "🎯".repeat(500); // 500 * 4 = 2000 bytes
        let out = build_body(&n(NotifySeverity::Warn, "x", &stuffed));
        assert!(out.len() <= 1500);
        // Any prefix produced is a valid UTF-8 string with no replacement
        // markers from a mid-codepoint cut.
        assert!(!out.contains('\u{FFFD}'));
    }
}
