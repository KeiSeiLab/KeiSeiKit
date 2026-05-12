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
/// Classification priority: `message` before `callback_query`.
pub fn classify(update: Update) -> WebhookEvent {
    if let Some(msg) = update.message {
        if let Some(text) = msg.text {
            return WebhookEvent::Text {
                chat_id: msg.chat.id,
                from: msg.from,
                text,
            };
        }
    }
    if let Some(cb) = update.callback_query {
        if let Some(data) = cb.data {
            let chat_id = cb.message.as_ref().map(|m| m.chat.id).unwrap_or(0);
            return WebhookEvent::Callback {
                chat_id,
                from: cb.from,
                data,
            };
        }
    }
    WebhookEvent::Other
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::update::{CallbackQuery, Chat, Message, Update, User};

    fn make_user() -> User {
        User {
            id: 42,
            username: Some("alice".into()),
            first_name: Some("Alice".into()),
        }
    }

    #[test]
    fn classify_text_message() {
        let update = Update {
            update_id: 1,
            message: Some(Message {
                message_id: 10,
                date: 1_700_000_000,
                chat: Chat { id: 99, r#type: Some("private".into()) },
                from: Some(make_user()),
                text: Some("hello".into()),
            }),
            callback_query: None,
        };
        let event = classify(update);
        assert_eq!(
            event,
            WebhookEvent::Text {
                chat_id: 99,
                from: Some(make_user()),
                text: "hello".into(),
            }
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
                }),
                data: Some("action:start".into()),
            }),
        };
        let event = classify(update);
        assert_eq!(
            event,
            WebhookEvent::Callback {
                chat_id: 77,
                from: Some(make_user()),
                data: "action:start".into(),
            }
        );
    }

    #[test]
    fn classify_other_returns_other() {
        // Update with no message and no callback_query (e.g. edited_message not modelled).
        let update = Update {
            update_id: 3,
            message: None,
            callback_query: None,
        };
        assert_eq!(classify(update), WebhookEvent::Other);
    }
}
