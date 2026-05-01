// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! kei-notify-discord — Discord webhook [`NotifyChannel`].
//!
//! Sibling of `kei-notify-email` (SMTP) and `kei-notify-slack` (incoming
//! webhook). Wraps the Discord public webhook surface
//! (`https://discord.com/api/webhooks/<id>/<token>`) into the
//! `NotifyChannel` trait surface from `kei-runtime-core`.
//!
//! ## Severity → embed color (Discord decimal RGB)
//!
//! | severity   | hex      | decimal   |
//! |------------|----------|-----------|
//! | Info       | `#3498DB` | `3447003`  |
//! | Success    | `#2ECC71` | `3066993`  |
//! | Warn       | `#F1C40F` | `15844367` |
//! | Error      | `#E74C3C` | `15158332` |
//!
//! ## Quick start
//!
//! ```ignore
//! use kei_notify_discord::DiscordChannel;
//! use kei_runtime_core::traits::notify::NotifyChannel;
//!
//! # async fn ex(notification: kei_runtime_core::traits::notify::Notification)
//! #     -> kei_runtime_core::Result<()>
//! # {
//! let channel = DiscordChannel::from_env(None)?;
//! channel.send(&notification).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Branding axes (sibling notify primitives)
//!
//! | axis            | kei-notify-email          | kei-notify-discord          |
//! |-----------------|---------------------------|-----------------------------|
//! | channel_name    | `email`                   | `discord`                   |
//! | DNA caps        | `["PR", "AP", "EM"]`      | `["PR", "AP", "DC"]`        |
//! | DNA scope       | `.../kei-notify-email`    | `.../kei-notify-discord`    |
//! | DNA body        | `b"smtp-v1"` (illustr.)   | `b"discord-webhook-v1"`     |
//! | env (URL/host)  | `SMTP_HOST` etc.          | `DISCORD_WEBHOOK_URL`       |

pub mod channel;
pub mod error;
pub mod payload;

pub use channel::DiscordChannel;
pub use error::{Error, Result};
pub use payload::build_payload;
