// SPDX-License-Identifier: Apache-2.0
//! `WebhookContext` — trait that consumer state types must implement.
//!
//! This trait is what the handler needs from the application's `axum::State`.
//! Consumers clone their state into every handler call (axum requirement).

use async_trait::async_trait;

use crate::event::WebhookEvent;

/// Contract between the handler and the consuming application.
///
/// Implement this on your axum `State` type, then pass `State<S>` to the
/// router. The handler calls [`WebhookContext::secret_token`] for HMAC-free
/// constant-time comparison and [`WebhookContext::on_event`] for dispatch.
#[async_trait]
pub trait WebhookContext: Clone + Send + Sync + 'static {
    /// Return the secret token that was passed to `setWebhook`.
    fn secret_token(&self) -> &str;

    /// Handle a classified inbound event. Errors are logged but not surfaced
    /// to Telegram — the handler always returns 200 on successful validation.
    async fn on_event(&self, event: WebhookEvent);
}
