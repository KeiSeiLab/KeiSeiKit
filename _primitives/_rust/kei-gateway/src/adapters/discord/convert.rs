//! Pure conversion: serenity `Message` → gateway [`MessageEvent`].
//!
//! Kept free of network calls so the conversion can be unit-tested by
//! constructing `Message` values from JSON fixtures (no live Discord API).
//! Guild presence is inferred from `msg.guild_id` — absent means DM.

use serenity::model::channel::Message;

use crate::message::{ChatType, MessageEvent, Platform, SessionSource};

/// Convert a serenity `Message` to a [`MessageEvent`].
///
/// Returns `None` if the message has no text content (attachments-only, system
/// messages, etc.) — the recv loop drops those silently.
pub fn message_to_event(msg: &Message) -> Option<MessageEvent> {
    let text = extract_text(msg)?;
    let source = build_source(msg);
    let mut event = MessageEvent::new(text, source);
    event.message_id = Some(msg.id.get().to_string());
    if let Some(ref_msg) = &msg.referenced_message {
        event.reply_to_message_id = Some(ref_msg.id.get().to_string());
    }
    Some(event)
}

fn extract_text(msg: &Message) -> Option<String> {
    let t = msg.content.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn build_source(msg: &Message) -> SessionSource {
    let channel_id = msg.channel_id.get().to_string();
    let chat_type = classify_message(msg);
    let user_id = Some(msg.author.id.get().to_string());
    SessionSource {
        platform: Platform::Discord,
        chat_type,
        chat_id: Some(channel_id),
        user_id,
        user_id_alt: None,
        thread_id: None,
    }
}

/// DM if `guild_id` is absent; guild text channel otherwise.
///
/// We cannot distinguish announcement vs regular vs thread without a cache
/// lookup, so all guild messages are `Group`. Session keys handle the rest.
fn classify_message(msg: &Message) -> ChatType {
    if msg.guild_id.is_none() {
        ChatType::Dm
    } else {
        ChatType::Group
    }
}

#[cfg(test)]
#[cfg(feature = "discord")]
mod tests {
    use super::*;

    fn guild_msg_json(content: &str, channel_id: u64, author_id: u64, msg_id: u64) -> String {
        serde_json::json!({
            "id": msg_id.to_string(),
            "channel_id": channel_id.to_string(),
            "guild_id": "999",
            "author": {
                "id": author_id.to_string(),
                "username": "testuser",
                "discriminator": "0001",
                "bot": false
            },
            "content": content,
            "timestamp": "2024-01-01T00:00:00+00:00",
            "edited_timestamp": null,
            "tts": false,
            "mention_everyone": false,
            "mentions": [],
            "mention_roles": [],
            "attachments": [],
            "embeds": [],
            "pinned": false,
            "type": 0
        })
        .to_string()
    }

    fn dm_msg_json(content: &str, channel_id: u64, author_id: u64, msg_id: u64) -> String {
        serde_json::json!({
            "id": msg_id.to_string(),
            "channel_id": channel_id.to_string(),
            "author": {
                "id": author_id.to_string(),
                "username": "dmuser",
                "discriminator": "0002",
                "bot": false
            },
            "content": content,
            "timestamp": "2024-01-01T00:00:00+00:00",
            "edited_timestamp": null,
            "tts": false,
            "mention_everyone": false,
            "mentions": [],
            "mention_roles": [],
            "attachments": [],
            "embeds": [],
            "pinned": false,
            "type": 0
        })
        .to_string()
    }

    #[test]
    fn guild_text_message_extracts_event() {
        let json = guild_msg_json("hello discord", 111, 222, 333);
        let msg: Message = serde_json::from_str(&json).expect("parse");
        let ev = message_to_event(&msg).expect("event");
        assert_eq!(ev.text, "hello discord");
        assert_eq!(ev.source.platform, Platform::Discord);
        assert_eq!(ev.source.chat_type, ChatType::Group);
        assert_eq!(ev.source.chat_id.as_deref(), Some("111"));
        assert_eq!(ev.source.user_id.as_deref(), Some("222"));
        assert_eq!(ev.message_id.as_deref(), Some("333"));
    }

    #[test]
    fn dm_message_classified_as_dm() {
        let json = dm_msg_json("direct", 444, 555, 666);
        let msg: Message = serde_json::from_str(&json).expect("parse");
        let ev = message_to_event(&msg).expect("event");
        assert_eq!(ev.source.chat_type, ChatType::Dm);
        assert_eq!(ev.source.chat_id.as_deref(), Some("444"));
    }

    #[test]
    fn empty_content_returns_none() {
        let json = guild_msg_json("", 111, 222, 333);
        let msg: Message = serde_json::from_str(&json).expect("parse");
        assert!(message_to_event(&msg).is_none());
    }

    #[test]
    fn whitespace_only_content_returns_none() {
        let json = guild_msg_json("   ", 111, 222, 333);
        let msg: Message = serde_json::from_str(&json).expect("parse");
        assert!(message_to_event(&msg).is_none());
    }
}
