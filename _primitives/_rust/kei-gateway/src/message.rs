//! Normalised message event types (port of Hermes `gateway/platforms/base.py:831-908`).
//!
//! Every adapter produces [`MessageEvent`]; every consumer reads it. This is the
//! single contract between platform-specific I/O and the agent runner.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Supported messaging platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Telegram,
    Discord,
    Slack,
    WhatsApp,
    Signal,
    Cli,
    /// Catch-all for embedded / webhook adapters that don't have a first-class enum.
    Generic,
}

impl Platform {
    /// Stable string token used in session keys (mirrors Hermes `Platform.value`).
    pub fn as_str(self) -> &'static str {
        match self {
            Platform::Telegram => "telegram",
            Platform::Discord => "discord",
            Platform::Slack => "slack",
            Platform::WhatsApp => "whatsapp",
            Platform::Signal => "signal",
            Platform::Cli => "cli",
            Platform::Generic => "generic",
        }
    }
}

/// Whether a chat is a 1-1 DM, a group, or a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Dm,
    Group,
    Channel,
}

impl ChatType {
    pub fn as_str(self) -> &'static str {
        match self {
            ChatType::Dm => "dm",
            ChatType::Group => "group",
            ChatType::Channel => "channel",
        }
    }
}

/// Where a message came from. Drives session-key derivation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionSource {
    pub platform: Platform,
    pub chat_type: ChatType,
    /// The parent chat / room / channel ID. Optional for fallback DMs.
    pub chat_id: Option<String>,
    /// The user who sent the message (group isolation key).
    pub user_id: Option<String>,
    /// Alternate user ID — e.g. WhatsApp LID-vs-JID flip. Takes precedence.
    pub user_id_alt: Option<String>,
    /// Thread / topic / reply-tree ID (Telegram forum topic, Discord thread, Slack thread).
    pub thread_id: Option<String>,
}

impl SessionSource {
    /// Build a DM source on a platform with a single participant identifier.
    pub fn dm(platform: Platform, chat_id: impl Into<String>) -> Self {
        Self {
            platform,
            chat_type: ChatType::Dm,
            chat_id: Some(chat_id.into()),
            user_id: None,
            user_id_alt: None,
            thread_id: None,
        }
    }
}

/// Type of inbound message content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Text,
    Photo,
    Voice,
    Document,
    Video,
    Audio,
    Sticker,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Text
    }
}

/// A normalised inbound message event. All adapters produce this shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEvent {
    pub text: String,
    #[serde(default)]
    pub message_type: MessageType,
    pub source: SessionSource,
    /// Platform-native message ID (for replies, audit, /restart bookkeeping).
    pub message_id: Option<String>,
    /// Local file paths to downloaded media (for vision tools).
    #[serde(default)]
    pub media_urls: Vec<String>,
    /// Parallel array describing each `media_urls[i]` MIME / extension hint.
    #[serde(default)]
    pub media_types: Vec<String>,
    /// Message ID this message replies to (for context injection).
    pub reply_to_message_id: Option<String>,
    /// Per-channel ephemeral system prompt (Discord channel_prompts equivalent).
    pub channel_prompt: Option<String>,
    /// Internal flag — synthetic events bypass user-authorisation checks.
    #[serde(default)]
    pub internal: bool,
    pub timestamp: DateTime<Utc>,
}

impl MessageEvent {
    pub fn new(text: impl Into<String>, source: SessionSource) -> Self {
        Self {
            text: text.into(),
            message_type: MessageType::Text,
            source,
            message_id: None,
            media_urls: Vec::new(),
            media_types: Vec::new(),
            reply_to_message_id: None,
            channel_prompt: None,
            internal: false,
            timestamp: Utc::now(),
        }
    }

    /// True if `text` starts with `/` (slash command convention).
    pub fn is_command(&self) -> bool {
        self.text.starts_with('/')
    }
}
