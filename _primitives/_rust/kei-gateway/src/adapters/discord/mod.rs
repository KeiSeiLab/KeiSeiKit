//! Discord adapter (P5 — serenity gateway impl).
//! Gateway-mode only. Conversion lives in [`convert`], dedup in [`dedup`].

mod convert;
mod dedup;

use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use serenity::async_trait as serenity_async_trait;
use serenity::client::{ClientBuilder, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::GatewayIntents;
use serenity::model::id::{ChannelId, MessageId};
use serenity::prelude::Context as SerenityContext;
use tokio::sync::mpsc;

use crate::adapters::base::{OutboundMessage, PlatformAdapter, SendResult};
use crate::message::{MessageEvent, Platform};

use self::convert::message_to_event;
use self::dedup::{message_digest, DedupCache};

/// Discord bot adapter.
pub struct DiscordAdapter {
    bot_token: String,
    /// Optional allow-list of channel IDs (string form). Empty = allow all.
    pub allowed_channels: Vec<String>,
    dedup: Arc<DedupCache>,
}

impl DiscordAdapter {
    /// Construct an adapter from a bot token (from `~/.claude/secrets/.env`, RULE 0.8).
    pub fn new(bot_token: impl Into<String>) -> Self {
        Self {
            bot_token: bot_token.into(),
            allowed_channels: Vec::new(),
            dedup: Arc::new(DedupCache::default()),
        }
    }

    /// Restrict inbound to a channel allow-list. Builder-style.
    pub fn with_allowed_channels(mut self, channels: Vec<String>) -> Self {
        self.allowed_channels = channels;
        self
    }
}

#[async_trait]
impl PlatformAdapter for DiscordAdapter {
    fn platform(&self) -> Platform {
        Platform::Discord
    }

    async fn connect(&self) -> Result<()> {
        let http = serenity::http::Http::new(&self.bot_token);
        let me = http
            .get_current_user()
            .await
            .context("serenity get_current_user — check bot token / network")?;
        if me.bot {
            Ok(())
        } else {
            anyhow::bail!("get_current_user returned a non-bot account; refusing to start")
        }
    }

    async fn send(&self, msg: OutboundMessage) -> Result<SendResult> {
        let chat_id = msg
            .chat_id
            .as_deref()
            .context("OutboundMessage missing chat_id")?;
        let channel_id: u64 = chat_id
            .parse()
            .context("Discord chat_id must be a numeric u64 snowflake")?;
        let channel = ChannelId::new(channel_id);
        let http = serenity::http::Http::new(&self.bot_token);
        let result = if let Some(reply_id) = msg.reply_to_message_id.as_deref() {
            send_with_reply(&http, channel, &msg.text, reply_id).await
        } else {
            channel.say(&http, &msg.text).await
        };
        match result {
            Ok(sent) => Ok(SendResult::ok(Some(sent.id.get().to_string()))),
            Err(e) => Ok(SendResult::err(format!("serenity send: {e}"))),
        }
    }

    async fn recv_loop(&self, tx: mpsc::Sender<MessageEvent>) -> Result<()> {
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let handler = InboundHandler {
            tx,
            dedup: self.dedup.clone(),
            allowed_channels: self.allowed_channels.clone(),
        };
        let mut client = ClientBuilder::new(&self.bot_token, intents)
            .event_handler(handler)
            .await
            .context("serenity ClientBuilder — check bot token")?;
        client.start().await.context("serenity client stopped")?;
        Ok(())
    }
}

/// Send a message with an optional reply reference.
async fn send_with_reply(
    http: &serenity::http::Http,
    channel: ChannelId,
    text: &str,
    reply_id_str: &str,
) -> Result<Message, serenity::Error> {
    if let Ok(rid) = reply_id_str.parse::<u64>() {
        let msg_ref = (channel, MessageId::new(rid));
        channel
            .send_message(
                http,
                serenity::builder::CreateMessage::new()
                    .content(text)
                    .reference_message(msg_ref),
            )
            .await
    } else {
        channel.say(http, text).await
    }
}

/// serenity `EventHandler` that forwards text messages to the gateway runner.
struct InboundHandler {
    tx: mpsc::Sender<MessageEvent>,
    dedup: Arc<DedupCache>,
    allowed_channels: Vec<String>,
}

#[serenity_async_trait]
impl EventHandler for InboundHandler {
    async fn message(&self, _ctx: SerenityContext, msg: Message) {
        if let Some(event) = filter_and_dedup(&msg, &self.dedup, &self.allowed_channels) {
            let _ = self.tx.send(event).await;
        }
    }
}

/// Apply allow-list + dedup, returning a [`MessageEvent`] only if both pass.
fn filter_and_dedup(
    msg: &Message,
    dedup: &DedupCache,
    allow: &[String],
) -> Option<MessageEvent> {
    let event = message_to_event(msg)?;
    let channel_id_str = event.source.chat_id.as_deref()?;
    if !allow.is_empty() && !allow.iter().any(|c| c == channel_id_str) {
        return None;
    }
    let channel_id: u64 = channel_id_str.parse().ok()?;
    let message_id: u64 = msg.id.get();
    let digest = message_digest(channel_id, message_id, &event.text);
    if dedup.observe(digest) {
        return None;
    }
    Some(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msg(text: &str, ch: u64, author: u64, id: u64) -> Message {
        let s = format!(
            r#"{{"id":"{id}","channel_id":"{ch}","guild_id":"999","author":{{"id":"{author}","username":"u","discriminator":"0001","bot":false}},"content":{text:?},"timestamp":"2024-01-01T00:00:00+00:00","edited_timestamp":null,"tts":false,"mention_everyone":false,"mentions":[],"mention_roles":[],"attachments":[],"embeds":[],"pinned":false,"type":0}}"#
        );
        serde_json::from_str(&s).unwrap()
    }

    #[test]
    fn dedup_blocks_duplicate_message() {
        let msg = make_msg("ping", 100, 200, 300);
        let d = DedupCache::default();
        let a: Vec<String> = vec![];
        assert!(filter_and_dedup(&msg, &d, &a).is_some());
        assert!(filter_and_dedup(&msg, &d, &a).is_none());
    }

    #[test]
    fn allow_list_filters_unknown_channel() {
        let msg = make_msg("ping", 100, 200, 300);
        let allow = vec!["999".to_string()];
        assert!(filter_and_dedup(&msg, &DedupCache::default(), &allow).is_none());
    }

    #[test]
    fn allow_list_admits_listed_channel() {
        let msg = make_msg("ping", 100, 200, 300);
        let allow = vec!["100".to_string()];
        let ev = filter_and_dedup(&msg, &DedupCache::default(), &allow).expect("admitted");
        assert_eq!(ev.text, "ping");
    }
}
