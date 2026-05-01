//! Telegram adapter (P4.1.b — real teloxide impl).
//!
//! Long-poll only (no webhook). `connect()` calls `getMe` for sanity. `send()`
//! issues `sendMessage`. `recv_loop()` consumes `update_listeners::polling_default`
//! and pushes deduped, normalised [`MessageEvent`]s onto the inbound channel.
//!
//! Constructor pattern: this `mod.rs` orchestrates only. Conversion lives in
//! [`convert`], dedup in [`dedup`].

mod convert;
mod dedup;

use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use teloxide::prelude::*;
use teloxide::types::{ChatId, MessageId, Recipient, ReplyParameters};
use teloxide::update_listeners::{self, AsUpdateStream, UpdateListener};
use tokio::pin;
use tokio::sync::mpsc;

use crate::adapters::base::{OutboundMessage, PlatformAdapter, SendResult};
use crate::message::{MessageEvent, Platform};

use self::convert::update_to_event;
use self::dedup::{message_digest, DedupCache};

/// Telegram bot adapter.
pub struct TelegramAdapter {
    bot: Bot,
    /// Optional allow-list of chat IDs (string form). Empty = allow all.
    pub allowed_chats: Vec<String>,
    dedup: Arc<DedupCache>,
}

impl TelegramAdapter {
    /// Construct an adapter from a bot token (typically read from
    /// `~/.claude/secrets/.env` per RULE 0.8 — never hardcode).
    pub fn new(bot_token: impl Into<String>) -> Self {
        Self {
            bot: Bot::new(bot_token.into()),
            allowed_chats: Vec::new(),
            dedup: Arc::new(DedupCache::default()),
        }
    }

    /// Restrict inbound to a specific allow-list. Builder-style.
    pub fn with_allowed_chats(mut self, chats: Vec<String>) -> Self {
        self.allowed_chats = chats;
        self
    }
}

#[async_trait]
impl PlatformAdapter for TelegramAdapter {
    fn platform(&self) -> Platform {
        Platform::Telegram
    }

    async fn connect(&self) -> Result<()> {
        let me = self
            .bot
            .get_me()
            .await
            .context("teloxide getMe — check bot token / network")?;
        if me.user.is_bot {
            Ok(())
        } else {
            anyhow::bail!("getMe returned a non-bot account; refusing to start")
        }
    }

    async fn send(&self, msg: OutboundMessage) -> Result<SendResult> {
        let chat_id = msg
            .chat_id
            .as_deref()
            .context("OutboundMessage missing chat_id")?;
        let parsed: i64 = chat_id
            .parse()
            .context("Telegram chat_id must be a numeric i64")?;
        let mut req = self.bot.send_message(Recipient::Id(ChatId(parsed)), &msg.text);
        if let Some(reply_id) = msg.reply_to_message_id.as_deref() {
            if let Ok(rid) = reply_id.parse::<i32>() {
                req = req.reply_parameters(ReplyParameters::new(MessageId(rid)));
            }
        }
        match req.await {
            Ok(sent) => Ok(SendResult::ok(Some(sent.id.0.to_string()))),
            Err(e) => Ok(SendResult::err(format!("teloxide send: {e}"))),
        }
    }

    async fn recv_loop(&self, tx: mpsc::Sender<MessageEvent>) -> Result<()> {
        let listener = update_listeners::polling_default(self.bot.clone()).await;
        run_listener(listener, tx, self.dedup.clone(), self.allowed_chats.clone()).await
    }
}

/// Drive the listener stream, dedupe, then forward to the gateway runner.
async fn run_listener<L>(
    mut listener: L,
    tx: mpsc::Sender<MessageEvent>,
    dedup: Arc<DedupCache>,
    allow: Vec<String>,
) -> Result<()>
where
    L: UpdateListener + Send,
    for<'a> <L as AsUpdateStream<'a>>::StreamErr: std::fmt::Display,
{
    let stream = listener.as_stream();
    pin!(stream);
    while let Some(item) = stream.next().await {
        let update = match item {
            Ok(u) => u,
            Err(e) => {
                eprintln!("[kei-gateway:telegram] listener error: {e}");
                continue;
            }
        };
        if let Some(event) = filter_and_dedup(&update, &dedup, &allow) {
            if tx.send(event).await.is_err() {
                break;
            }
        }
    }
    Ok(())
}

/// Apply allow-list + dedup, returning a [`MessageEvent`] only if both pass.
fn filter_and_dedup(
    update: &teloxide::types::Update,
    dedup: &DedupCache,
    allow: &[String],
) -> Option<MessageEvent> {
    let event = update_to_event(update)?;
    let chat_id = event.source.chat_id.as_deref()?;
    if !allow.is_empty() && !allow.iter().any(|c| c == chat_id) {
        return None;
    }
    let parsed_chat: i64 = chat_id.parse().ok()?;
    let parsed_msg: i32 = event.message_id.as_deref()?.parse().ok()?;
    let digest = message_digest(parsed_chat, parsed_msg, &event.text);
    if dedup.observe(digest) {
        return None;
    }
    Some(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DM_TEXT_JSON: &str = r#"{
        "update_id": 1,
        "message": {
            "message_id": 7,
            "date": 1710000000,
            "chat": { "id": 555, "type": "private", "first_name": "Bob" },
            "from": { "id": 555, "is_bot": false, "first_name": "Bob" },
            "text": "ping"
        }
    }"#;

    #[test]
    fn dedup_blocks_duplicate_updates() {
        let upd: teloxide::types::Update = serde_json::from_str(DM_TEXT_JSON).unwrap();
        let dedup = DedupCache::default();
        let allow: Vec<String> = vec![];
        assert!(filter_and_dedup(&upd, &dedup, &allow).is_some());
        assert!(filter_and_dedup(&upd, &dedup, &allow).is_none());
    }

    #[test]
    fn allow_list_filters_unknown_chat() {
        let upd: teloxide::types::Update = serde_json::from_str(DM_TEXT_JSON).unwrap();
        let dedup = DedupCache::default();
        let allow: Vec<String> = vec!["999".into()];
        assert!(filter_and_dedup(&upd, &dedup, &allow).is_none());
    }

    #[test]
    fn allow_list_admits_listed_chat() {
        let upd: teloxide::types::Update = serde_json::from_str(DM_TEXT_JSON).unwrap();
        let dedup = DedupCache::default();
        let allow: Vec<String> = vec!["555".into()];
        let ev = filter_and_dedup(&upd, &dedup, &allow).expect("admitted");
        assert_eq!(ev.text, "ping");
    }
}
