// SPDX-License-Identifier: Apache-2.0
//! `run_serve` — axum router builder + BuddyContext impl.
//!
//! Constructor Pattern: one responsibility — compose crate pieces into HTTP server.
//! Each function ≤ 30 LOC. No logging of bot tokens.

use std::sync::Arc;

use async_trait::async_trait;
use axum::{routing, Json, Router};
use serde_json::{json, Value};
use tracing::{error, warn};

use kei_telegram_webhook::{WebhookContext, WebhookEvent};

use crate::{
    error::BuddyError,
    extractor::LlmExtractor,
    machine::handle_step,
    persona_merge::deep_merge,
    serve_telegram::send_message,
    state::OnboardState,
    store::{BuddyStore, SqliteBuddyStore},
};

/// Configuration passed from the binary to `run_serve`.
pub struct ServeConfig {
    pub port: u16,
    pub db_path: String,
    pub bot_token: String,
    pub webhook_secret: String,
}

/// Axum state — implements `WebhookContext` for the webhook handler.
///
/// `Arc<E>` provides cheap `Clone` without requiring `E: Clone`.
pub struct BuddyContext<E: LlmExtractor + Send + Sync + 'static> {
    pub secret: String,
    pub bot_token: String,
    pub store: Arc<SqliteBuddyStore>,
    pub extractor: Arc<E>,
    pub http: reqwest::Client,
}

impl<E: LlmExtractor + Send + Sync + 'static> Clone for BuddyContext<E> {
    fn clone(&self) -> Self {
        Self {
            secret: self.secret.clone(),
            bot_token: self.bot_token.clone(),
            store: Arc::clone(&self.store),
            extractor: Arc::clone(&self.extractor),
            http: self.http.clone(),
        }
    }
}

#[async_trait]
impl<E: LlmExtractor + Send + Sync + 'static> WebhookContext for BuddyContext<E> {
    fn secret_token(&self) -> &str {
        &self.secret
    }

    async fn on_event(&self, event: WebhookEvent) {
        match event {
            WebhookEvent::Text { chat_id, text, .. } => {
                self.handle_text(chat_id, text).await;
            }
            other => {
                warn!(event = ?other, "ignoring non-text webhook event");
            }
        }
    }
}

impl<E: LlmExtractor + Send + Sync + 'static> BuddyContext<E> {
    async fn handle_text(&self, chat_id: i64, text: String) {
        if let Err(e) = self.process_text(chat_id, &text).await {
            error!(chat_id, error = %e, "failed to process text event");
        }
    }

    async fn process_text(&self, chat_id: i64, text: &str) -> Result<(), BuddyError> {
        let state = self
            .store
            .load_state(chat_id)
            .await?
            .unwrap_or(OnboardState::Intro);
        let persona = self
            .store
            .load_persona(chat_id)
            .await?
            .unwrap_or_else(|| serde_json::json!({}));
        let output = handle_step(&state, text, &persona, self.extractor.as_ref()).await?;
        self.store.save_state(chat_id, &output.next_state).await?;
        self.apply_persona_patch(chat_id, output.persona_patch).await?;
        send_message(&self.bot_token, chat_id, &output.response_text, &self.http).await?;
        Ok(())
    }

    async fn apply_persona_patch(&self, chat_id: i64, patch: Value) -> Result<(), BuddyError> {
        if patch == json!({}) {
            return Ok(());
        }
        let base = self
            .store
            .load_persona(chat_id)
            .await?
            .unwrap_or_else(|| json!({}));
        let merged = deep_merge(base, patch);
        self.store.save_persona(chat_id, &merged).await
    }
}

/// Health-check handler.
async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "crate": "kei-buddy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Build the axum Router.
pub fn build_router<E>(ctx: BuddyContext<E>) -> Router
where
    E: LlmExtractor + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/webhook",
            routing::post(kei_telegram_webhook::handle_webhook::<BuddyContext<E>>),
        )
        .route("/health", routing::get(health))
        .with_state(ctx)
}

/// Start the HTTP server.
pub async fn run_serve(cfg: ServeConfig) -> anyhow::Result<()> {
    init_tracing();
    let store = Arc::new(SqliteBuddyStore::from_path(&cfg.db_path)?);
    let extractor = Arc::new(crate::extractor::MockExtractor::new(json!({})));
    let ctx = BuddyContext {
        secret: cfg.webhook_secret,
        bot_token: cfg.bot_token,
        store,
        extractor,
        http: reqwest::Client::new(),
    };
    let router = build_router(ctx);
    let addr = format!("0.0.0.0:{}", cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(addr = %addr, "kei-buddy listening");
    axum::serve(listener, router).await?;
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};
    let _ = fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}
