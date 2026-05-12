// SPDX-License-Identifier: Apache-2.0
//! kei-telegram-webhook — inbound Telegram Bot API webhook handler.
//!
//! Consumers mount [`handler::handle_webhook`] inside their own [`axum::Router`].
//! This crate does NOT own an `axum::Server`.
//!
//! Module layout (Constructor Pattern — one file, one responsibility):
//!   * `update`  — lean `Update` / `Message` / `User` / `Chat` / `CallbackQuery` structs
//!   * `event`   — `WebhookEvent` enum + `classify` function
//!   * `context` — `WebhookContext` trait (secret_token + on_event)
//!   * `handler` — axum handler `handle_webhook<S>`
//!   * `error`   — `WebhookError` via thiserror

pub mod context;
pub mod error;
pub mod event;
pub mod handler;
pub mod update;

pub use context::WebhookContext;
pub use error::WebhookError;
pub use event::{classify, WebhookEvent};
pub use handler::handle_webhook;
pub use update::{CallbackQuery, Chat, Message, Update, User};
