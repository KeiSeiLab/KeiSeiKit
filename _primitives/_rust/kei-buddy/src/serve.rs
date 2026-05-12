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
    /// If `Some`, only these chat_ids are processed; others are warn-logged + ignored.
    /// `None` (or empty) means accept all chat_ids.
    pub allowed_chat_ids: Option<Vec<i64>>,
    /// Optional OpenAI-compatible LLM proxy. If set together with `llm_api_key`,
    /// `run_serve` instantiates `OpenAiExtractor`; otherwise falls back to
    /// `MockExtractor` with a warning.
    pub llm_proxy_url: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
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
    /// Whitelist of chat_ids; `None` or empty = accept all.
    pub allowed_chat_ids: Arc<Option<Vec<i64>>>,
}

impl<E: LlmExtractor + Send + Sync + 'static> Clone for BuddyContext<E> {
    fn clone(&self) -> Self {
        Self {
            secret: self.secret.clone(),
            bot_token: self.bot_token.clone(),
            store: Arc::clone(&self.store),
            extractor: Arc::clone(&self.extractor),
            http: self.http.clone(),
            allowed_chat_ids: Arc::clone(&self.allowed_chat_ids),
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
    fn chat_allowed(&self, chat_id: i64) -> bool {
        match self.allowed_chat_ids.as_ref() {
            Some(list) if !list.is_empty() => list.contains(&chat_id),
            _ => true,
        }
    }

    async fn handle_text(&self, chat_id: i64, text: String) {
        if !self.chat_allowed(chat_id) {
            warn!(chat_id, "chat_id not in whitelist; ignoring");
            return;
        }
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
    let allowed_chat_ids = Arc::new(cfg.allowed_chat_ids);
    let http = reqwest::Client::new();

    #[cfg(feature = "extractor-openai")]
    {
        if let (Some(proxy), Some(key)) = (cfg.llm_proxy_url, cfg.llm_api_key) {
            let model = cfg
                .llm_model
                .unwrap_or_else(|| "gpt-4o-mini".to_string());
            tracing::info!(model = %model, "using OpenAiExtractor (LiteLLM-compatible)");
            let extractor = Arc::new(crate::extractor::openai::OpenAiExtractor::new_with_model(
                proxy, key, model,
            ));
            return start_listener(cfg.port, BuddyContext {
                secret: cfg.webhook_secret,
                bot_token: cfg.bot_token,
                store,
                extractor,
                http,
                allowed_chat_ids,
            }).await;
        }
    }

    warn!("no LLM extractor configured — using MockExtractor (state machine will advance but field-extraction returns empty)");
    let extractor = Arc::new(crate::extractor::MockExtractor::new(json!({})));
    start_listener(cfg.port, BuddyContext {
        secret: cfg.webhook_secret,
        bot_token: cfg.bot_token,
        store,
        extractor,
        http,
        allowed_chat_ids,
    }).await
}

async fn start_listener<E>(port: u16, ctx: BuddyContext<E>) -> anyhow::Result<()>
where
    E: LlmExtractor + Send + Sync + 'static,
{
    let router = build_router(ctx);
    let addr = format!("0.0.0.0:{}", port);
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
