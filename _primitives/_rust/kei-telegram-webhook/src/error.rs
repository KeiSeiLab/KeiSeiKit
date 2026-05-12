// SPDX-License-Identifier: Apache-2.0
//! `WebhookError` — error types for the webhook handler.

use thiserror::Error;

/// Errors that can arise while processing an inbound Telegram update.
#[derive(Debug, Error)]
pub enum WebhookError {
    /// The `X-Telegram-Bot-Api-Secret-Token` header is missing.
    #[error("missing X-Telegram-Bot-Api-Secret-Token header")]
    MissingSecretToken,

    /// The provided secret token does not match the configured value.
    #[error("invalid secret token")]
    InvalidSecretToken,

    /// The JSON payload could not be deserialized into an `Update`.
    #[error("failed to deserialize update: {0}")]
    DeserializeError(String),
}
