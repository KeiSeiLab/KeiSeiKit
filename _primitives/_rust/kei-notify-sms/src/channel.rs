// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`SmsChannel`] — `NotifyChannel` impl for Twilio Programmable Messaging.
//!
//! Hits exactly one endpoint:
//! `POST /2010-04-01/Accounts/{ACCOUNT_SID}/Messages.json` with form body
//! `To=...&From=...&Body=...` and HTTP Basic auth
//! (`ACCOUNT_SID:AUTH_TOKEN`). Twilio answers 201 Created with a JSON
//! payload containing the message `sid` on success and a `{code, message}`
//! pair on 4xx.

use crate::error::{Error, Result};
use crate::payload::build_body;
use async_trait::async_trait;
use kei_runtime_core::traits::notify::{Notification, NotifyChannel, NotifySeverity};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_BASE_URL: &str = "https://api.twilio.com";

/// Twilio Programmable Messaging SMS channel.
///
/// Construct via [`SmsChannel::from_env`] (reads `TWILIO_*` env vars) or
/// [`SmsChannel::with_config`] (explicit, used by `wiremock` tests).
pub struct SmsChannel {
    dna: Dna,
    parent: Option<Dna>,
    http: Client,
    base_url: String,
    account_sid: String,
    auth_token: String,
    from_number: String,
    to_number: String,
}

/// Twilio 4xx error envelope. Both fields are always present in 4xx/5xx
/// responses; we use `Option` defensively so a malformed body still maps
/// to `Error::Api` instead of `Error::Http`.
#[derive(Debug, Deserialize)]
struct TwilioApiError {
    code: Option<i64>,
    message: Option<String>,
}

impl SmsChannel {
    /// Build from process env: `TWILIO_ACCOUNT_SID`, `TWILIO_AUTH_TOKEN`,
    /// `TWILIO_FROM_NUMBER`, `TWILIO_TO_NUMBER`. The base URL defaults to
    /// `https://api.twilio.com` (override via [`Self::with_config`]).
    pub fn from_env(parent: Option<Dna>) -> Result<Self> {
        let sid = need_env("TWILIO_ACCOUNT_SID")?;
        let tok = need_env("TWILIO_AUTH_TOKEN")?;
        let from = need_env("TWILIO_FROM_NUMBER")?;
        let to = need_env("TWILIO_TO_NUMBER")?;
        Self::with_config(DEFAULT_BASE_URL, sid, tok, from, to, parent)
    }

    /// Explicit-config constructor. `base_url` lets `wiremock` tests
    /// retarget to a local mock; production callers pass
    /// `"https://api.twilio.com"` (or use [`Self::from_env`]).
    pub fn with_config(
        base_url: impl Into<String>,
        account_sid: impl Into<String>,
        auth_token: impl Into<String>,
        from_number: impl Into<String>,
        to_number: impl Into<String>,
        parent: Option<Dna>,
    ) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "SM"])
            .scope("keiseikit.dev/primitives/kei-notify-sms")
            .body(b"twilio-sms-v1")
            .build()?;
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;
        Ok(Self {
            dna,
            parent,
            http,
            base_url: base_url.into(),
            account_sid: account_sid.into(),
            auth_token: auth_token.into(),
            from_number: from_number.into(),
            to_number: to_number.into(),
        })
    }

    fn endpoint(&self) -> String {
        format!(
            "{}/2010-04-01/Accounts/{}/Messages.json",
            self.base_url, self.account_sid
        )
    }
}

fn need_env(key: &str) -> Result<String> {
    std::env::var(key).map_err(|_| Error::MissingEnv(key.to_string()))
}

impl HasDna for SmsChannel {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait]
impl NotifyChannel for SmsChannel {
    fn channel_name(&self) -> &'static str {
        "sms"
    }

    fn supports_batching(&self) -> bool {
        false
    }

    /// SMS is intrusive and metered. Drop anything below `Warn` by
    /// default; callers who really want Info SMS can wrap this channel
    /// in their own delegating impl.
    fn min_severity(&self) -> NotifySeverity {
        NotifySeverity::Warn
    }

    async fn send(&self, n: &Notification) -> kei_runtime_core::Result<()> {
        let body = build_body(n);
        let form = [
            ("To", self.to_number.as_str()),
            ("From", self.from_number.as_str()),
            ("Body", body.as_str()),
        ];
        let resp = self
            .http
            .post(self.endpoint())
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&form)
            .send()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::from(e)))?;

        let status = resp.status();
        if status == StatusCode::CREATED {
            return Ok(());
        }
        let raw = resp
            .text()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::from(e)))?;
        let detail = match serde_json::from_str::<TwilioApiError>(&raw) {
            Ok(parsed) => format!(
                "twilio {} code={} message={}",
                status,
                parsed.code.map(|c| c.to_string()).unwrap_or_else(|| "?".into()),
                parsed.message.unwrap_or_else(|| "?".into())
            ),
            Err(_) => format!("twilio {status}: {raw}"),
        };
        Err(kei_runtime_core::Error::from(Error::Api(detail)))
    }
}
