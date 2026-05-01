// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`DiscordChannel`] — `NotifyChannel` impl backed by a Discord webhook.
//!
//! `channel_name = "discord"`. `supports_batching = false` — Discord
//! webhooks accept one message per POST (no native digest). DNA carries
//! caps `["PR", "AP", "DC"]` per the Wave 8 atomar branding axes.

use crate::error::{Error, Result};
use crate::payload::build_payload;
use async_trait::async_trait;
use kei_runtime_core::traits::notify::{Notification, NotifyChannel};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use reqwest::{Client, StatusCode};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Discord webhook NotifyChannel.
pub struct DiscordChannel {
    dna: Dna,
    parent: Option<Dna>,
    http: Client,
    webhook_url: String,
}

impl DiscordChannel {
    /// Build from the `DISCORD_WEBHOOK_URL` env var (returned wrapped in
    /// `Error::Config` if unset). Use [`DiscordChannel::with_url`] for
    /// wiremock tests or explicit-URL configurations.
    pub fn from_env(parent: Option<Dna>) -> Result<Self> {
        let url = std::env::var("DISCORD_WEBHOOK_URL")
            .map_err(|_| Error::Config("DISCORD_WEBHOOK_URL unset".into()))?;
        Self::with_url(url, parent)
    }

    /// Explicit-URL constructor — the wiremock test path.
    pub fn with_url(webhook_url: impl Into<String>, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "DC"])
            .scope("keiseikit.dev/primitives/kei-notify-discord")
            .body(b"discord-webhook-v1")
            .build()?;
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .map_err(Error::from)?;
        Ok(Self {
            dna,
            parent,
            http,
            webhook_url: webhook_url.into(),
        })
    }

    /// Borrow the configured webhook URL.
    pub fn webhook_url(&self) -> &str {
        &self.webhook_url
    }
}

impl HasDna for DiscordChannel {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait]
impl NotifyChannel for DiscordChannel {
    fn channel_name(&self) -> &'static str { "discord" }

    fn supports_batching(&self) -> bool { false }

    async fn send(&self, n: &Notification) -> kei_runtime_core::Result<()> {
        let body = build_payload(n);
        let resp = self
            .http
            .post(&self.webhook_url)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::from(e)))?;
        let status = resp.status();
        // Discord returns 204 No Content for webhooks, sometimes 200.
        if status == StatusCode::OK || status == StatusCode::NO_CONTENT {
            return Ok(());
        }
        let text = resp.text().await.unwrap_or_default();
        Err(kei_runtime_core::Error::from(Error::Api(format!(
            "discord webhook http {status}: {text}"
        ))))
    }
}
