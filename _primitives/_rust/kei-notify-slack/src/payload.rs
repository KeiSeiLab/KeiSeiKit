// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Slack incoming-webhook payload builder.
//!
//! Pure function — takes a [`Notification`] and emits a `serde_json::Value`
//! shaped for the Slack `chat.postMessage`-compatible incoming-webhook
//! contract:
//!
//! ```json
//! {
//!   "text": "<subject>",
//!   "attachments": [
//!     { "color": "good|warning|danger|#3b82f6", "title": "...", "text": "..." }
//!   ]
//! }
//! ```
//!
//! Colour mapping (Slack conventions + one CSS hex for Info):
//! - `Info`    → `#3b82f6`  (blue)
//! - `Success` → `good`     (green)
//! - `Warn`    → `warning`  (yellow)
//! - `Error`   → `danger`   (red)

use kei_runtime_core::traits::notify::{Notification, NotifySeverity};
use serde_json::{json, Value};

/// Map [`NotifySeverity`] to the Slack attachment `color` value.
pub fn severity_color(s: NotifySeverity) -> &'static str {
    match s {
        NotifySeverity::Info => "#3b82f6",
        NotifySeverity::Success => "good",
        NotifySeverity::Warn => "warning",
        NotifySeverity::Error => "danger",
    }
}

/// Build the JSON body for a Slack incoming-webhook POST.
///
/// `text` is set to the subject so Slack's notification preview is sane;
/// the full subject + body lives in the single attachment so severity
/// colouring renders.
pub fn build_payload(n: &Notification) -> Value {
    json!({
        "text": n.subject,
        "attachments": [
            {
                "color": severity_color(n.severity),
                "title": n.subject,
                "text": n.body_text,
            }
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_runtime_core::DnaBuilder;

    fn sample_notification(severity: NotifySeverity) -> Notification {
        let dna = DnaBuilder::new("notification")
            .cap("NT")
            .scope("test")
            .body(b"sample")
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
            subject: "subj".into(),
            body_text: "body".into(),
            body_html: None,
            severity,
            tags: Vec::new(),
        }
    }

    #[test]
    fn info_color_is_blue_hex() {
        let n = sample_notification(NotifySeverity::Info);
        let v = build_payload(&n);
        assert_eq!(v["attachments"][0]["color"], "#3b82f6");
    }

    #[test]
    fn error_color_is_danger() {
        let n = sample_notification(NotifySeverity::Error);
        let v = build_payload(&n);
        assert_eq!(v["attachments"][0]["color"], "danger");
    }

    #[test]
    fn attachment_shape_has_title_and_text() {
        let n = sample_notification(NotifySeverity::Warn);
        let v = build_payload(&n);
        assert_eq!(v["text"], "subj");
        let att = &v["attachments"][0];
        assert_eq!(att["color"], "warning");
        assert_eq!(att["title"], "subj");
        assert_eq!(att["text"], "body");
        // Must be a single-element array (one severity-coloured block).
        assert_eq!(v["attachments"].as_array().unwrap().len(), 1);
    }
}
