//! Pure conversion: Slack Socket Mode `message` event payload → gateway
//! [`MessageEvent`].
//!
//! Kept free of network calls so the conversion can be unit-tested by feeding
//! deserialised JSON fixtures (no live Slack API).

use serde::Deserialize;

use crate::message::{ChatType, MessageEvent, Platform, SessionSource};

// ---------------------------------------------------------------------------
// Payload shapes
// ---------------------------------------------------------------------------

/// Subset of the Slack `event_callback` envelope we care about.
#[derive(Debug, Deserialize)]
pub struct EventCallback {
    pub event: SlackEvent,
}

/// Inner event — we only handle `message` subtypes; anything else is ignored.
#[derive(Debug, Deserialize)]
pub struct SlackEvent {
    #[serde(rename = "type")]
    pub kind: String,
    /// Channel ID where the message was posted.
    pub channel: Option<String>,
    /// Unique message timestamp (also serves as message ID).
    pub ts: Option<String>,
    /// Thread parent timestamp (set for threaded replies).
    pub thread_ts: Option<String>,
    /// User ID of the sender. Absent for bot messages.
    pub user: Option<String>,
    /// Plain-text content of the message.
    pub text: Option<String>,
    /// Channel type: `"im"` = DM, `"channel"` = public, etc.
    pub channel_type: Option<String>,
}

// ---------------------------------------------------------------------------
// Conversion
// ---------------------------------------------------------------------------

/// Convert a raw Socket Mode event callback JSON `value` into a
/// [`MessageEvent`]. Returns `None` for non-text events or bot messages.
pub fn event_to_message(callback: &EventCallback) -> Option<MessageEvent> {
    let ev = &callback.event;
    if ev.kind != "message" {
        return None;
    }
    // Skip bot messages (no user field means bot/app posted it).
    let user = ev.user.as_deref()?;
    let text = ev.text.as_deref().filter(|t| !t.is_empty())?.to_string();
    let channel = ev.channel.as_deref()?.to_string();
    let ts = ev.ts.as_deref()?.to_string();

    let chat_type = classify_channel_type(ev.channel_type.as_deref());
    let source = SessionSource {
        platform: Platform::Slack,
        chat_type,
        chat_id: Some(channel),
        user_id: Some(user.to_string()),
        user_id_alt: None,
        thread_id: ev.thread_ts.clone(),
    };
    let mut event = MessageEvent::new(text, source);
    event.message_id = Some(ts);
    if let Some(tts) = &ev.thread_ts {
        // If thread_ts != ts, this message is a reply.
        if tts != event.message_id.as_deref().unwrap_or("") {
            event.reply_to_message_id = Some(tts.clone());
        }
    }
    Some(event)
}

fn classify_channel_type(channel_type: Option<&str>) -> ChatType {
    match channel_type {
        Some("im") | Some("mpim") => ChatType::Dm,
        Some("channel") => ChatType::Channel,
        _ => ChatType::Group,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "slack")]
mod tests {
    use super::*;

    const DM_MESSAGE_JSON: &str = r#"{
        "event": {
            "type": "message",
            "channel": "D01234567",
            "channel_type": "im",
            "user": "U001",
            "text": "hello bot",
            "ts": "1710000001.000100"
        }
    }"#;

    const CHANNEL_REPLY_JSON: &str = r#"{
        "event": {
            "type": "message",
            "channel": "C09876543",
            "channel_type": "channel",
            "user": "U002",
            "text": "a reply",
            "ts": "1710000002.000200",
            "thread_ts": "1710000001.000100"
        }
    }"#;

    #[test]
    fn dm_message_extracts_event() {
        let cb: EventCallback = serde_json::from_str(DM_MESSAGE_JSON).expect("parse");
        let ev = event_to_message(&cb).expect("should produce event");
        assert_eq!(ev.text, "hello bot");
        assert_eq!(ev.source.platform, Platform::Slack);
        assert_eq!(ev.source.chat_type, ChatType::Dm);
        assert_eq!(ev.source.chat_id.as_deref(), Some("D01234567"));
        assert_eq!(ev.source.user_id.as_deref(), Some("U001"));
        assert_eq!(ev.message_id.as_deref(), Some("1710000001.000100"));
        assert!(ev.reply_to_message_id.is_none());
    }

    #[test]
    fn channel_reply_identifies_parent_thread() {
        let cb: EventCallback = serde_json::from_str(CHANNEL_REPLY_JSON).expect("parse");
        let ev = event_to_message(&cb).expect("should produce event");
        assert_eq!(ev.source.thread_id.as_deref(), Some("1710000001.000100"));
        assert_eq!(ev.reply_to_message_id.as_deref(), Some("1710000001.000100"));
    }
}
