//! Pure conversion: teloxide `Update` → gateway [`MessageEvent`].
//!
//! Kept free of network calls so the conversion can be unit-tested by feeding
//! deserialised JSON fixtures (no live Telegram API).

use teloxide::types::{ChatKind, Message, PublicChatKind, Update, UpdateKind};

use crate::message::{ChatType, MessageEvent, Platform, SessionSource};

/// Result of converting an `Update`. `None` = update was not a text message we
/// care about (edits, callback queries, polls, etc).
pub fn update_to_event(update: &Update) -> Option<MessageEvent> {
    let msg = extract_message(update)?;
    let text = extract_text(msg)?.to_string();
    let source = build_source(msg);
    let mut event = MessageEvent::new(text, source);
    event.message_id = Some(msg.id.0.to_string());
    if let Some(reply) = msg.reply_to_message() {
        event.reply_to_message_id = Some(reply.id.0.to_string());
    }
    Some(event)
}

fn extract_message(update: &Update) -> Option<&Message> {
    match &update.kind {
        UpdateKind::Message(m) => Some(m),
        _ => None,
    }
}

fn extract_text(msg: &Message) -> Option<&str> {
    msg.text()
}

fn build_source(msg: &Message) -> SessionSource {
    let chat_id = msg.chat.id.0.to_string();
    let chat_type = classify_chat(&msg.chat.kind);
    let user_id = msg.from.as_ref().map(|u| u.id.0.to_string());
    let thread_id = msg.thread_id.as_ref().map(|t| t.0.0.to_string());
    SessionSource {
        platform: Platform::Telegram,
        chat_type,
        chat_id: Some(chat_id),
        user_id,
        user_id_alt: None,
        thread_id,
    }
}

fn classify_chat(kind: &ChatKind) -> ChatType {
    match kind {
        ChatKind::Private(_) => ChatType::Dm,
        ChatKind::Public(p) => match p.kind {
            PublicChatKind::Channel(_) => ChatType::Channel,
            _ => ChatType::Group,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// JSON fixture of a private (DM) text message.
    /// Field shape mirrors a real Telegram `getUpdates` response trimmed to the
    /// fields teloxide deserialises.
    const DM_TEXT_JSON: &str = r#"{
        "update_id": 100,
        "message": {
            "message_id": 42,
            "date": 1710000000,
            "chat": { "id": 123, "type": "private", "first_name": "Alice" },
            "from": { "id": 123, "is_bot": false, "first_name": "Alice" },
            "text": "hello bot"
        }
    }"#;

    #[test]
    fn dm_text_extracts_message_event() {
        let upd: Update = serde_json::from_str(DM_TEXT_JSON).expect("parse");
        let ev = update_to_event(&upd).expect("text event");
        assert_eq!(ev.text, "hello bot");
        assert_eq!(ev.source.platform, Platform::Telegram);
        assert_eq!(ev.source.chat_type, ChatType::Dm);
        assert_eq!(ev.source.chat_id.as_deref(), Some("123"));
        assert_eq!(ev.source.user_id.as_deref(), Some("123"));
        assert_eq!(ev.message_id.as_deref(), Some("42"));
    }

    #[test]
    fn non_message_update_returns_none() {
        // Edited message — not handled.
        let json = r#"{"update_id": 1, "edited_message": {
            "message_id": 1, "date": 0,
            "chat": {"id": 1, "type": "private", "first_name": "X"},
            "from": {"id": 1, "is_bot": false, "first_name": "X"},
            "text": "edit"
        }}"#;
        let upd: Update = serde_json::from_str(json).expect("parse");
        assert!(update_to_event(&upd).is_none());
    }
}
