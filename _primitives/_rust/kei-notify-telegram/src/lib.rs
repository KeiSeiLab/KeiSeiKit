// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! kei-notify-telegram — `NotifyChannel` impl over the Telegram Bot API.
//!
//! Endpoint: `POST {base}/bot{TOKEN}/sendMessage` with JSON body
//! `{"chat_id": <i64 or "@channel">, "text": "...", "parse_mode": "HTML"}`.
//! Severity is rendered as a leading emoji in the body so a single
//! channel-route surfaces all four `NotifySeverity` levels visually.
//!
//! Constructor Pattern: 3 source files, each <200 LOC, one
//! responsibility per cube:
//!   * `error.rs`   — error type + `From<reqwest::Error>` +
//!                    bridge into `kei_runtime_core::Error::Provider`
//!   * `payload.rs` — body composition (severity emoji + HTML escape)
//!   * `channel.rs` — `TelegramChannel` (env + with_config constructors,
//!                    `NotifyChannel::send` over reqwest)
//!
//! Auth: bot token from `TELEGRAM_BOT_TOKEN`, target from
//! `TELEGRAM_CHAT_ID` (numeric `i64` or `@channel_username`). Base URL
//! defaults to `https://api.telegram.org` and is overridable via
//! constructor so wiremock tests can point at a localhost mock.

pub mod channel;
pub mod error;
pub mod payload;

pub use channel::TelegramChannel;
pub use error::{Error, Result};
pub use payload::{build_text, severity_emoji};
