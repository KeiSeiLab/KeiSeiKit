// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! HTML body composition for Telegram Bot API `sendMessage`.
//!
//! Format: `<b>{subject}</b>\n\n{severity_emoji} {body_text}`.
//! The subject is wrapped in `<b>...</b>` so Telegram's HTML
//! parse_mode renders it bold; the body is severity-prefixed so
//! readers see the level at a glance.
//!
//! HTML escaping: Telegram's HTML parse_mode requires `<`, `>`, `&`
//! to be escaped in plain content (the only tags it understands are
//! `b/i/u/s/code/pre/a` etc.) Without escaping, a stray `<` in either
//! field can either error 400 or, worse, render as a literal tag.

use kei_runtime_core::traits::notify::{Notification, NotifySeverity};

/// Map a severity to its display emoji. Pure mapping, no allocation.
pub fn severity_emoji(s: NotifySeverity) -> &'static str {
    match s {
        NotifySeverity::Info => "ℹ️",
        NotifySeverity::Success => "✅",
        NotifySeverity::Warn => "⚠️",
        NotifySeverity::Error => "🚨",
    }
}

/// Compose the HTML-formatted message body.
///
/// Subject is rendered bold via `<b>...</b>`; body is separated by a
/// blank line and prefixed with the severity emoji. Both fields are
/// HTML-escaped against Telegram's HTML parse_mode rules.
pub fn build_text(n: &Notification) -> String {
    let subject = html_escape(&n.subject);
    let body = html_escape(&n.body_text);
    let emoji = severity_emoji(n.severity);
    format!("<b>{subject}</b>\n\n{emoji} {body}")
}

/// Minimal HTML escape for Telegram parse_mode=HTML.
/// Telegram requires `<`, `>`, `&` escaped in non-tag content.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_runtime_core::traits::notify::{Notification, NotifySeverity};
    use kei_runtime_core::{DnaBuilder, Dna};

    fn dummy_dna() -> Dna {
        DnaBuilder::new("test")
            .cap("TG")
            .scope("test/scope")
            .body(b"test")
            .build()
            .unwrap()
    }

    fn notif(sev: NotifySeverity, subject: &str, body: &str) -> Notification {
        Notification {
            dna: dummy_dna(),
            parent_dna: dummy_dna(),
            subject: subject.into(),
            body_text: body.into(),
            body_html: None,
            severity: sev,
            tags: vec![],
        }
    }

    #[test]
    fn emoji_info() {
        assert_eq!(severity_emoji(NotifySeverity::Info), "ℹ️");
        let t = build_text(&notif(NotifySeverity::Info, "S", "B"));
        assert!(t.contains("ℹ️ B"));
    }

    #[test]
    fn emoji_success() {
        assert_eq!(severity_emoji(NotifySeverity::Success), "✅");
        let t = build_text(&notif(NotifySeverity::Success, "S", "B"));
        assert!(t.contains("✅ B"));
    }

    #[test]
    fn emoji_warn() {
        assert_eq!(severity_emoji(NotifySeverity::Warn), "⚠️");
        let t = build_text(&notif(NotifySeverity::Warn, "S", "B"));
        assert!(t.contains("⚠️ B"));
    }

    #[test]
    fn emoji_error() {
        assert_eq!(severity_emoji(NotifySeverity::Error), "🚨");
        let t = build_text(&notif(NotifySeverity::Error, "S", "B"));
        assert!(t.contains("🚨 B"));
    }

    #[test]
    fn html_escapes_lt_gt_amp() {
        let n = notif(
            NotifySeverity::Info,
            "1 < 2 & 3 > 0",
            "<script>x & y</script>",
        );
        let t = build_text(&n);
        assert!(t.contains("1 &lt; 2 &amp; 3 &gt; 0"), "subject escape: {t}");
        assert!(t.contains("&lt;script&gt;x &amp; y&lt;/script&gt;"), "body escape: {t}");
        // Bold tag itself must remain literal (it's our wrapping, not user content).
        assert!(t.starts_with("<b>") && t.contains("</b>"));
    }
}
