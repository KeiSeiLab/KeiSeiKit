//! [`PlatformAdapter`] trait — the contract every messaging adapter implements.
//!
//! Hermes equivalent: `gateway/platforms/base.py:BasePlatformAdapter`.

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::message::{MessageEvent, Platform};

/// Outbound message envelope. Adapters serialise this onto their wire format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundMessage {
    pub text: String,
    pub chat_id: Option<String>,
    pub thread_id: Option<String>,
    pub reply_to_message_id: Option<String>,
    /// Local file paths to attach (vision / voice / document delivery).
    #[serde(default)]
    pub attachments: Vec<String>,
}

impl OutboundMessage {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            chat_id: None,
            thread_id: None,
            reply_to_message_id: None,
            attachments: Vec::new(),
        }
    }

    /// Bind concrete `chat_id` / `thread_id` (called by the router).
    pub fn with_target(mut self, chat_id: String, thread_id: Option<String>) -> Self {
        self.chat_id = Some(chat_id);
        self.thread_id = thread_id;
        self
    }
}

/// Adapter delivery result.
#[derive(Debug, Clone)]
pub struct SendResult {
    pub success: bool,
    pub platform_message_id: Option<String>,
    pub error: Option<String>,
    pub at: DateTime<Utc>,
}

impl SendResult {
    pub fn ok(message_id: Option<String>) -> Self {
        Self {
            success: true,
            platform_message_id: message_id,
            error: None,
            at: Utc::now(),
        }
    }

    pub fn err(error: impl Into<String>) -> Self {
        Self {
            success: false,
            platform_message_id: None,
            error: Some(error.into()),
            at: Utc::now(),
        }
    }

    /// Synthetic success for `DeliveryTarget::Local` (file-only routes).
    pub fn local() -> Self {
        Self {
            success: true,
            platform_message_id: None,
            error: None,
            at: Utc::now(),
        }
    }
}

/// The trait every messaging-platform adapter implements.
///
/// `connect` runs once on startup; `send` is the per-outbound hot path;
/// `recv_loop` is a long-running task that pushes inbound events onto the
/// gateway's mpsc channel.
#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Stable platform identifier.
    fn platform(&self) -> Platform;

    /// One-time setup: open sockets, log in, fetch credentials.
    async fn connect(&self) -> Result<()>;

    /// Send `msg` over the wire. Returns delivery confirmation or error.
    async fn send(&self, msg: OutboundMessage) -> Result<SendResult>;

    /// Long-running receive loop. Each inbound message becomes a
    /// [`MessageEvent`] pushed onto `tx`. Returns when the underlying
    /// transport closes.
    async fn recv_loop(&self, tx: mpsc::Sender<MessageEvent>) -> Result<()>;
}
