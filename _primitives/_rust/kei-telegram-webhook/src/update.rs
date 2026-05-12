// SPDX-License-Identifier: Apache-2.0
//! Lean Telegram `Update` struct hierarchy.
//!
//! Only the fields KeiBuddy needs are modelled.
//! All optional fields use `#[serde(default)]` so missing JSON keys deserialize cleanly.

use serde::{Deserialize, Serialize};

/// Telegram `Voice` attachment (OGG-Opus from the mic).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct Voice {
    pub file_id: String,
    #[serde(default)]
    pub duration: i64,
    #[serde(default)]
    pub mime_type: String,
}

/// Telegram `Audio` attachment (music/audio file).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct Audio {
    pub file_id: String,
    #[serde(default)]
    pub duration: i64,
    #[serde(default)]
    pub mime_type: String,
}

/// Top-level Telegram update payload.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(default)]
    pub message: Option<Message>,
    #[serde(default)]
    pub callback_query: Option<CallbackQuery>,
}

/// Incoming text message (or voice/audio message).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub chat: Chat,
    #[serde(default)]
    pub from: Option<User>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub voice: Option<Voice>,
    #[serde(default)]
    pub audio: Option<Audio>,
}

/// Telegram user or bot.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
}

/// Chat where a message was sent.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Chat {
    pub id: i64,
    #[serde(default)]
    pub r#type: Option<String>,
}

/// Inline-keyboard button callback.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CallbackQuery {
    pub id: String,
    #[serde(default)]
    pub from: Option<User>,
    #[serde(default)]
    pub message: Option<Message>,
    #[serde(default)]
    pub data: Option<String>,
}
