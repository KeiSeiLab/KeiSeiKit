//! Slack adapter (P6 — real slack-morphism impl).
//!
//! `connect()` = api.test. `send()` = chat.postMessage. `recv_loop()` =
//! Socket Mode WebSocket via apps.connections.open. Submodules: `convert`,
//! `dedup`.

mod convert;
mod dedup;
#[cfg(test)]
mod tests;

use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use slack_morphism::prelude::*;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as WsMsg;

use crate::adapters::base::{OutboundMessage, PlatformAdapter, SendResult};
use crate::message::{MessageEvent, Platform};

use self::convert::{event_to_message, EventCallback};
use self::dedup::{message_digest, DedupCache};

/// Slack bot adapter. `bot_token` authenticates REST calls; `app_token`
/// (prefix `xapp-`) enables Socket Mode for `recv_loop`.
pub struct SlackAdapter {
    bot_token: SlackApiToken,
    app_token: Option<SlackApiToken>,
    pub allowed_channels: Vec<String>,
    dedup: Arc<DedupCache>,
    client: Arc<SlackHyperClient>,
}

impl SlackAdapter {
    /// Construct from a bot token (read from env per RULE 0.8 — never hardcode).
    pub fn new(bot_token: impl Into<String>) -> Result<Self> {
        let connector = SlackClientHyperConnector::new()
            .context("build Slack hyper connector (TLS init)")?;
        Ok(Self {
            bot_token: SlackApiToken::new(SlackApiTokenValue::new(bot_token.into())),
            app_token: None,
            allowed_channels: Vec::new(),
            dedup: Arc::new(DedupCache::default()),
            client: Arc::new(SlackClient::new(connector)),
        })
    }

    /// Enable Socket Mode for `recv_loop`. `app_token` must start with `xapp-`.
    pub fn with_app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = Some(SlackApiToken::new(SlackApiTokenValue::new(app_token.into())));
        self
    }

    /// Restrict inbound to specific channel IDs. Builder-style.
    pub fn with_allowed_channels(mut self, channels: Vec<String>) -> Self {
        self.allowed_channels = channels;
        self
    }
}

#[async_trait]
impl PlatformAdapter for SlackAdapter {
    fn platform(&self) -> Platform {
        Platform::Slack
    }

    async fn connect(&self) -> Result<()> {
        self.client
            .open_session(&self.bot_token)
            .api_test(&SlackApiTestRequest::new())
            .await
            .context("Slack api.test — check bot token / network")?;
        Ok(())
    }

    async fn send(&self, msg: OutboundMessage) -> Result<SendResult> {
        let channel_str = msg
            .chat_id
            .as_deref()
            .context("OutboundMessage missing chat_id")?;
        let channel = SlackChannelId::new(channel_str.to_string());
        let content = SlackMessageContent::new().with_text(msg.text.clone());
        let mut req = SlackApiChatPostMessageRequest::new(channel, content);
        if let Some(thread_ts) = msg.thread_id.as_deref() {
            req = req.with_thread_ts(SlackTs::new(thread_ts.to_string()));
        }
        let session = self.client.open_session(&self.bot_token);
        match session.chat_post_message(&req).await {
            Ok(resp) => Ok(SendResult::ok(Some(resp.ts.0))),
            Err(e) => Ok(SendResult::err(format!("slack chat.postMessage: {e}"))),
        }
    }

    async fn recv_loop(&self, tx: mpsc::Sender<MessageEvent>) -> Result<()> {
        let app_token = self
            .app_token
            .as_ref()
            .context("SlackAdapter::recv_loop requires an app_token for Socket Mode")?;
        run_socket_loop(
            self.client.clone(),
            app_token.clone(),
            tx,
            self.dedup.clone(),
            self.allowed_channels.clone(),
        )
        .await
    }
}

async fn run_socket_loop(
    client: Arc<SlackHyperClient>,
    app_token: SlackApiToken,
    tx: mpsc::Sender<MessageEvent>,
    dedup: Arc<DedupCache>,
    allow: Vec<String>,
) -> Result<()> {
    let conn = client
        .open_session(&app_token)
        .apps_connections_open(&SlackApiAppsConnectionOpenRequest::new())
        .await
        .context("apps.connections.open — check xapp- token, Socket Mode enabled")?;
    let ws_url = conn.url.0.as_str().to_string();
    let (mut ws, _) = connect_async(ws_url.as_str())
        .await
        .context("WebSocket connect to Slack Socket Mode URL")?;
    while let Some(frame) = ws.next().await {
        match frame {
            Ok(WsMsg::Text(raw)) => dispatch_text(&raw, &tx, &dedup, &allow).await,
            Ok(WsMsg::Close(_)) => break,
            Err(e) => {
                eprintln!("[kei-gateway:slack] ws error: {e}");
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn dispatch_text(
    raw: &str,
    tx: &mpsc::Sender<MessageEvent>,
    dedup: &DedupCache,
    allow: &[String],
) {
    let Ok(cb) = serde_json::from_str::<EventCallback>(raw) else {
        return;
    };
    if let Some(event) = filter_and_dedup(&cb, dedup, allow) {
        let _ = tx.send(event).await;
    }
}

/// Apply allow-list and dedup. Returns `Some(event)` only when both pass.
pub(crate) fn filter_and_dedup(
    callback: &EventCallback,
    dedup: &DedupCache,
    allow: &[String],
) -> Option<MessageEvent> {
    let event = event_to_message(callback)?;
    let channel = event.source.chat_id.as_deref()?;
    if !allow.is_empty() && !allow.iter().any(|c| c == channel) {
        return None;
    }
    let ts = event.message_id.as_deref().unwrap_or("");
    let digest = message_digest(channel, ts, &event.text);
    if dedup.observe(digest) {
        return None;
    }
    Some(event)
}
