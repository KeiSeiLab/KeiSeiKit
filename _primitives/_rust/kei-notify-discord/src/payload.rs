// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Payload builder: `Notification` → Discord webhook JSON.
//!
//! Discord webhook expects:
//! ```json
//! {
//!   "content": "...",
//!   "embeds": [{"title": "...", "description": "...", "color": <int>}]
//! }
//! ```
//!
//! Color is decimal RGB (NOT hex). Mapping is fixed by severity.

use kei_runtime_core::traits::notify::{Notification, NotifySeverity};
use serde_json::{json, Value};

/// Discord embed color (decimal RGB) per severity.
pub fn color_for(severity: NotifySeverity) -> u32 {
    match severity {
        NotifySeverity::Info => 3_447_003,     // #3498DB blue
        NotifySeverity::Success => 3_066_993,  // #2ECC71 green
        NotifySeverity::Warn => 15_844_367,    // #F1C40F orange
        NotifySeverity::Error => 15_158_332,   // #E74C3C red
    }
}

/// Build the JSON body POSTed to a Discord webhook.
///
/// `content` carries the bare subject line (so non-embed-rendering clients
/// still see something), and `embeds[0]` carries the structured body
/// (title = subject, description = body_text, color = severity-mapped).
pub fn build_payload(notification: &Notification) -> Value {
    json!({
        "content": notification.subject,
        "embeds": [{
            "title": notification.subject,
            "description": notification.body_text,
            "color": color_for(notification.severity),
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_runtime_core::DnaBuilder;

    fn fixture(severity: NotifySeverity) -> Notification {
        let dna = DnaBuilder::new("notification")
            .cap("ND")
            .scope("test")
            .body(b"n")
            .build()
            .unwrap();
        let parent = DnaBuilder::new("primitive")
            .cap("PR")
            .scope("test")
            .body(b"p")
            .build()
            .unwrap();
        Notification {
            dna,
            parent_dna: parent,
            subject: "subject-line".into(),
            body_text: "body-line".into(),
            body_html: None,
            severity,
            tags: vec![],
        }
    }

    #[test]
    fn warn_color_orange() {
        let v = build_payload(&fixture(NotifySeverity::Warn));
        assert_eq!(v["embeds"][0]["color"], 15_844_367);
    }

    #[test]
    fn error_color_red() {
        let v = build_payload(&fixture(NotifySeverity::Error));
        assert_eq!(v["embeds"][0]["color"], 15_158_332);
    }

    #[test]
    fn embed_uses_subject_as_title() {
        let v = build_payload(&fixture(NotifySeverity::Info));
        assert_eq!(v["embeds"][0]["title"], "subject-line");
        assert_eq!(v["embeds"][0]["description"], "body-line");
        assert_eq!(v["content"], "subject-line");
        assert_eq!(v["embeds"][0]["color"], 3_447_003);
    }
}
