// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-notify-sms — Twilio Programmable Messaging [`NotifyChannel`].
//!
//! Wave 8 atomar SMS sibling of `kei-notify-email`. Targets Twilio's
//! `/2010-04-01/Accounts/{ACCOUNT_SID}/Messages.json` REST surface with
//! HTTP Basic auth (`ACCOUNT_SID:AUTH_TOKEN`). Bodies are formatted as
//! `[severity-emoji] subject — body_text` and UTF-8-safe truncated to
//! 1500 bytes to stay comfortably under Twilio's 1600-char hard limit.
//!
//! The channel's `min_severity()` returns [`NotifySeverity::Warn`]: SMS
//! is intrusive and metered, so info / success notifications are dropped
//! by the trait-default delivery filter. Override by wrapping in a
//! custom [`NotifyChannel`] if you really want every Info SMS.
//!
//! ## Quick start
//!
//! ```ignore
//! use kei_notify_sms::SmsChannel;
//! use kei_runtime_core::traits::notify::NotifyChannel;
//!
//! # async fn ex() -> kei_runtime_core::Result<()> {
//! let channel = SmsChannel::from_env(None)?;
//! // ... build a Notification with severity >= Warn ...
//! # Ok(())
//! # }
//! ```
//!
//! ## Branding axes vs `kei-notify-email`
//!
//! | axis             | kei-notify-email     | kei-notify-sms             |
//! |------------------|----------------------|----------------------------|
//! | DNA caps         | `["PR", "AP", "EM"]` | `["PR", "AP", "SM"]`       |
//! | DNA scope        | `.../kei-notify-email` | `.../kei-notify-sms`     |
//! | DNA body         | `b"smtp-email-v1"`   | `b"twilio-sms-v1"`         |
//! | min_severity     | Info (default)       | Warn (override)            |
//! | supports_batching| `true`               | `false`                    |

pub mod channel;
pub mod error;
pub mod payload;

pub use channel::SmsChannel;
pub use error::{Error, Result};
pub use payload::{build_body, severity_emoji};
