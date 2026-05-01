// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-notify-slack — Slack incoming-webhook impl of
//! [`kei_runtime_core::NotifyChannel`].
//!
//! Layout (Constructor Pattern, ≤200 LOC per file):
//! - [`error`]: local `Error`/`Result` mapped into runtime-core error.
//! - [`payload`]: pure `build_payload` function (severity → attachment colour).
//! - [`channel`]: [`SlackChannel`] — DNA-bearing trait impl.
//!
//! Auth: webhook URL read from env `SLACK_WEBHOOK_URL`. URL is overridable
//! via [`SlackChannel::with_url`] for `wiremock` tests.

pub mod channel;
pub mod error;
pub mod payload;

pub use channel::SlackChannel;
pub use error::{Error, Result};
pub use payload::build_payload;
