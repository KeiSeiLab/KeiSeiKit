// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`SlackChannel`] — DNA-bearing [`NotifyChannel`] backed by a Slack
//! incoming webhook.
//!
//! Constructors:
//! - [`SlackChannel::from_env`]: reads `SLACK_WEBHOOK_URL` from the env.
//! - [`SlackChannel::with_url`]: takes an explicit URL (used by tests).
//!
//! `send` POSTs the [`build_payload`] JSON to the configured webhook and
//! treats any non-200 status as [`Error::Api`].

use crate::error::{Error, Result as SlackResult};
use crate::payload::build_payload;
use async_trait::async_trait;
use kei_runtime_core::traits::notify::{Notification, NotifyChannel, NotifySeverity};
use kei_runtime_core::{Dna, DnaBuilder, HasDna, Result as CoreResult};
use reqwest::Client;
use std::time::Duration;

/// Env var holding the Slack incoming-webhook URL.
pub const ENV_WEBHOOK_URL: &str = "SLACK_WEBHOOK_URL";
/// Per-request timeout. Slack webhooks normally answer in <1s.
pub const DEFAULT_TIMEOUT_SECS: u64 = 15;

/// Slack incoming-webhook NotifyChannel.
#[derive(Debug, Clone)]
pub struct SlackChannel {
    dna: Dna,
    parent: Option<Dna>,
    http: Client,
    webhook_url: String,
}

impl SlackChannel {
    /// Build a fresh channel using the URL from `SLACK_WEBHOOK_URL`.
    pub fn from_env(parent: Option<Dna>) -> SlackResult<Self> {
        let url = std::env::var(ENV_WEBHOOK_URL)
            .map_err(|e| Error::Dna(format!("env {ENV_WEBHOOK_URL}: {e}")))?;
        Self::with_url(parent, url)
    }

    /// Build a channel against an explicit webhook URL (for `wiremock`).
    pub fn with_url(parent: Option<Dna>, webhook_url: impl Into<String>) -> SlackResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "SK"])
            .scope("keiseikit.dev/primitives/kei-notify-slack")
            .body(b"slack-webhook-v1")
            .build()?;
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .map_err(Error::from)?;
        Ok(Self { dna, parent, http, webhook_url: webhook_url.into() })
    }

    /// Direct webhook accessor (read-only; useful for assertions in tests).
    pub fn webhook_url(&self) -> &str {
        &self.webhook_url
    }

    /// Crate-local send returning the local error (for tests that want
    /// to assert on `Error::Api`).
    pub async fn send_raw(&self, n: &Notification) -> SlackResult<()> {
        let body = build_payload(n);
        let resp = self.http.post(&self.webhook_url).json(&body).send().await?;
        let status = resp.status();
        if status.as_u16() != 200 {
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::Api(format!(
                "slack webhook returned {}: {}",
                status.as_u16(),
                text
            )));
        }
        Ok(())
    }
}

impl HasDna for SlackChannel {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait]
impl NotifyChannel for SlackChannel {
    fn channel_name(&self) -> &'static str {
        "slack"
    }

    async fn send(&self, n: &Notification) -> CoreResult<()> {
        self.send_raw(n).await.map_err(Into::into)
    }

    fn supports_batching(&self) -> bool {
        false
    }

    fn min_severity(&self) -> NotifySeverity {
        NotifySeverity::Info
    }
}
