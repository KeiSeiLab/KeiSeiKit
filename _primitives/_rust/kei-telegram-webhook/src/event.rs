// SPDX-License-Identifier: Apache-2.0
//! `WebhookEvent` — typed summary of an inbound Telegram update.

use crate::update::{Update, User};

/// Typed classification of a Telegram `Update`.
#[derive(Debug, Clone, PartialEq)]
pub enum WebhookEvent {
    /// Incoming text message.
    Text {
        chat_id: i64,
        from: Option<User>,
        text: String,
    },
    /// Incoming voice or audio message — carries a Telegram file_id for download.
    Voice {
        chat_id: i64,
        from: Option<User>,
        file_id: String,
        mime_type: String,
    },
    /// Inline-keyboard button press.
    Callback {
        chat_id: i64,
        from: Option<User>,
        data: String,
    },
    /// Any update type not modelled above.
    Other,
}

/// Extract a typed [`WebhookEvent`] from a raw [`Update`].
///
/// Classification priority: voice/audio before text, text before callback.
pub fn classify(update: Update) -> WebhookEvent {
    if let Some(msg) = update.message {
        let chat_id = msg.chat.id;
        let from = msg.from.clone();
        if let Some(v) = msg.voice {
            return WebhookEvent::Voice { chat_id, from, file_id: v.file_id, mime_type: v.mime_type };
        }
        if let Some(a) = msg.audio {
            return WebhookEvent::Voice { chat_id, from, file_id: a.file_id, mime_type: a.mime_type };
        }
        if let Some(text) = msg.text {
            return WebhookEvent::Text { chat_id, from: msg.from, text };
        }
    }
    if let Some(cb) = update.callback_query {
        if let Some(data) = cb.data {
            let chat_id = cb.message.as_ref().map(|m| m.chat.id).unwrap_or(0);
            return WebhookEvent::Callback { chat_id, from: cb.from, data };
        }
    }
    WebhookEvent::Other
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::update::{Audio, CallbackQuery, Chat, Message, Update, User, Voice};

    fn make_user() -> User {
        User {
            id: 42,
            username: Some("alice".into()),
            first_name: Some("Alice".into()),
        }
    }

    fn text_msg(chat_id: i64, text: &str) -> Message {
        Message {
            message_id: 10,
            date: 1_700_000_000,
            chat: Chat { id: chat_id, r#type: Some("private".into()) },
            from: Some(make_user()),
            text: Some(text.into()),
            voice: None,
            audio: None,
        }
    }

    #[test]
    fn classify_text_message() {
        let update = Update {
            update_id: 1,
            message: Some(text_msg(99, "hello")),
            callback_query: None,
        };
        assert_eq!(
            classify(update),
            WebhookEvent::Text { chat_id: 99, from: Some(make_user()), text: "hello".into() }
        );
    }

    #[test]
    fn classify_callback_query() {
        let update = Update {
            update_id: 2,
            message: None,
            callback_query: Some(CallbackQuery {
                id: "cb1".into(),
                from: Some(make_user()),
                message: Some(Message {
                    message_id: 20,
                    date: 1_700_000_001,
                    chat: Chat { id: 77, r#type: None },
                    from: None,
                    text: None,
                    voice: None,
                    audio: None,
                }),
                data: Some("action:start".into()),
            }),
        };
        assert_eq!(
            classify(update),
            WebhookEvent::Callback { chat_id: 77, from: Some(make_user()), data: "action:start".into() }
        );
    }

    #[test]
    fn classify_other_returns_other() {
        let update = Update { update_id: 3, message: None, callback_query: None };
        assert_eq!(classify(update), WebhookEvent::Other);
    }

    #[test]
    fn classify_voice_message() {
        let update = Update {
            update_id: 4,
            message: Some(Message {
                message_id: 30,
                date: 1_700_000_002,
                chat: Chat { id: 55, r#type: Some("private".into()) },
                from: Some(make_user()),
                text: None,
                voice: Some(Voice {
                    file_id: "voice_file_abc".into(),
                    duration: 5,
                    mime_type: "audio/ogg".into(),
                }),
                audio: None,
            }),
            callback_query: None,
        };
        assert_eq!(
            classify(update),
            WebhookEvent::Voice {
                chat_id: 55,
                from: Some(make_user()),
                file_id: "voice_file_abc".into(),
                mime_type: "audio/ogg".into(),
            }
        );
    }

    #[test]
    fn classify_audio_message_maps_to_voice_variant() {
        let update = Update {
            update_id: 5,
            message: Some(Message {
                message_id: 31,
                date: 1_700_000_003,
                chat: Chat { id: 66, r#type: Some("private".into()) },
                from: Some(make_user()),
                text: None,
                voice: None,
                audio: Some(Audio {
                    file_id: "audio_file_xyz".into(),
                    duration: 120,
                    mime_type: "audio/mpeg".into(),
                }),
            }),
            callback_query: None,
        };
        assert_eq!(
            classify(update),
            WebhookEvent::Voice {
                chat_id: 66,
                from: Some(make_user()),
                file_id: "audio_file_xyz".into(),
                mime_type: "audio/mpeg".into(),
            }
        );
    }
}
