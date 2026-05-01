// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! `TelegramChannel` — `NotifyChannel` impl that POSTs to the
//! Telegram Bot API `sendMessage` endpoint.
//!
//! Constructor surface:
//!   * [`TelegramChannel::from_env`]    — reads `TELEGRAM_BOT_TOKEN`
//!     + `TELEGRAM_CHAT_ID`, defaults base URL to
//!     `https://api.telegram.org`.
//!   * [`TelegramChannel::with_config`] — explicit base URL, token,
//!     chat_id (used in wiremock tests).
//!
//! Wire format: `POST {base}/bot{token}/sendMessage` with JSON
//! `{"chat_id": <i64|String>, "text": "...", "parse_mode": "HTML"}`.
//! Response is `{"ok": true, "result": {...}}` on success or
//! `{"ok": false, "description": "..."}` on failure — the latter is
//! surfaced as `Error::Api(description)`.

use crate::error::{Error, Result};
use crate::payload::build_text;
use kei_runtime_core::traits::notify::{Notification, NotifyChannel, NotifySeverity};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use serde::Deserialize;
use serde_json::{json, Value};

const DEFAULT_BASE_URL: &str = "https://api.telegram.org";

pub struct TelegramChannel {
    dna: Dna,
    parent: Option<Dna>,
    http: reqwest::Client,
    base_url: String,
    bot_token: String,
    chat_id: String,
}

impl TelegramChannel {
    /// Build a channel from explicit base URL + bot token + chat id.
    /// `base_url` is trimmed of trailing slashes so callers may pass
    /// either `https://api.telegram.org` or `.../`.
    pub fn with_config(
        base_url: impl Into<String>,
        bot_token: impl Into<String>,
        chat_id: impl Into<String>,
        parent: Option<Dna>,
    ) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "TG"])
            .scope("keiseikit.dev/primitives/kei-notify-telegram")
            .body(b"telegram-bot-v1")
            .build()?;
        let base_url = base_url.into().trim_end_matches('/').to_string();
        Ok(Self {
            dna,
            parent,
            http: reqwest::Client::new(),
            base_url,
            bot_token: bot_token.into(),
            chat_id: chat_id.into(),
        })
    }

    /// Build a channel from env vars.
    /// Required: `TELEGRAM_BOT_TOKEN`, `TELEGRAM_CHAT_ID`.
    /// Optional: `TELEGRAM_API_BASE_URL` (default `https://api.telegram.org`).
    pub fn from_env(parent: Option<Dna>) -> Result<Self> {
        let token = std::env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| Error::MissingEnv("TELEGRAM_BOT_TOKEN".into()))?;
        let chat_id = std::env::var("TELEGRAM_CHAT_ID")
            .map_err(|_| Error::MissingEnv("TELEGRAM_CHAT_ID".into()))?;
        let base_url = std::env::var("TELEGRAM_API_BASE_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        Self::with_config(base_url, token, chat_id, parent)
    }

    pub fn base_url(&self) -> &str { &self.base_url }
    pub fn chat_id(&self) -> &str { &self.chat_id }

    /// Render `chat_id` as JSON: numeric `i64` if it parses, else `String`
    /// (covers the `@channel_username` form Telegram accepts).
    fn chat_id_value(&self) -> Value {
        match self.chat_id.parse::<i64>() {
            Ok(n) => Value::from(n),
            Err(_) => Value::String(self.chat_id.clone()),
        }
    }
}

impl HasDna for TelegramChannel {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    ok: bool,
    #[serde(default)]
    description: Option<String>,
}

#[async_trait::async_trait]
impl NotifyChannel for TelegramChannel {
    fn channel_name(&self) -> &'static str { "telegram" }
    fn supports_batching(&self) -> bool { false }
    fn min_severity(&self) -> NotifySeverity { NotifySeverity::Info }

    async fn send(&self, n: &Notification) -> kei_runtime_core::Result<()> {
        let url = format!("{}/bot{}/sendMessage", self.base_url, self.bot_token);
        let body = json!({
            "chat_id": self.chat_id_value(),
            "text": build_text(n),
            "parse_mode": "HTML",
        });
        let resp = self.http.post(&url).json(&body).send().await
            .map_err(Error::from).map_err(kei_runtime_core::Error::from)?;
        let status = resp.status();
        if !status.is_success() {
            // 5xx / 4xx that did not produce a JSON body — surface as Api error
            // with the raw text so debugging is possible without a packet capture.
            let text = resp.text().await.unwrap_or_default();
            return Err(kei_runtime_core::Error::from(Error::Api(format!(
                "http {} body {}", status.as_u16(), text
            ))));
        }
        let parsed: ApiResponse = resp.json().await
            .map_err(Error::from).map_err(kei_runtime_core::Error::from)?;
        if !parsed.ok {
            let desc = parsed.description.unwrap_or_else(|| "no description".into());
            return Err(kei_runtime_core::Error::from(Error::Api(desc)));
        }
        Ok(())
    }
}
