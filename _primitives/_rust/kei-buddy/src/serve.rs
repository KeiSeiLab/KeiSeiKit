// SPDX-License-Identifier: Apache-2.0
//! `BuddyContext` + axum router. Store bootstrap lives in `serve_runner`.
//! Constructor Pattern: one responsibility. No bot token logging.

use std::sync::Arc;

use async_trait::async_trait;
use axum::{routing, Json, Router};
use serde_json::{json, Value};
use tracing::{error, warn};

use kei_telegram_webhook::{WebhookContext, WebhookEvent};

use crate::{
    chat_log::ChatLog,
    commands::{execute_command, parse_command, CommandStores},
    contacts::Contacts,
    conversational::conversational_step,
    error::BuddyError,
    extractor::LlmExtractor,
    machine::handle_step,
    persona_merge::deep_merge,
    retrieval::retrieve_context,
    serve_telegram::send_message,
    state::OnboardState,
    store::{BuddyStore, SqliteBuddyStore},
    strings::Lang,
    topic_classify::classify_and_store_topic,
    topics::Topics,
    voice::VoiceHandler,
};

pub use crate::serve_runner::run_serve;

/// Configuration passed from the binary to `run_serve`.
pub struct ServeConfig {
    pub port: u16,
    pub db_path: String,
    pub bot_token: String,
    pub webhook_secret: String,
    pub allowed_chat_ids: Option<Vec<i64>>,
    pub llm_proxy_url: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
    pub chat_log_db_path: String,
    pub topics_db_path: String,
    pub contacts_db_path: String,
    /// STT backend name (e.g. "whisper-local"). `None` → voice messages ignored.
    pub stt_backend: Option<String>,
}

/// Axum state — implements `WebhookContext`. `Arc<E>` allows cheap `Clone`.
pub struct BuddyContext<E: LlmExtractor + Send + Sync + 'static> {
    pub secret: String,
    pub bot_token: String,
    pub store: Arc<SqliteBuddyStore>,
    pub extractor: Arc<E>,
    pub http: reqwest::Client,
    pub allowed_chat_ids: Arc<Option<Vec<i64>>>,
    pub chat_log: Arc<ChatLog>,
    pub topics: Arc<Topics>,
    pub contacts: Arc<Contacts>,
    /// Optional voice handler; `None` = voice messages ignored.
    pub voice: Option<Arc<VoiceHandler>>,
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
            chat_log: Arc::clone(&self.chat_log),
            topics: Arc::clone(&self.topics),
            contacts: Arc::clone(&self.contacts),
            voice: self.voice.as_ref().map(Arc::clone),
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
                let me = self.clone();
                tokio::spawn(async move { me.handle_text(chat_id, text).await });
            }
            WebhookEvent::Voice { chat_id, file_id, mime_type, .. } => {
                let me = self.clone();
                tokio::spawn(async move {
                    me.handle_voice(chat_id, file_id, mime_type).await
                });
            }
            other => {
                warn!(event = ?other, "ignoring unhandled webhook event");
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

    async fn handle_voice(&self, chat_id: i64, file_id: String, mime_type: String) {
        let Some(h) = self.voice.as_ref() else {
            warn!(chat_id, "voice message: no STT backend; ignoring");
            return;
        };
        if !self.chat_allowed(chat_id) {
            warn!(chat_id, "chat_id not in whitelist; ignoring voice");
            return;
        }
        match h.transcribe_file(&file_id, &mime_type).await {
            Ok(t) => self.handle_text(chat_id, t).await,
            Err(e) => error!(chat_id, error=%e, "voice transcription failed"),
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
        if let Err(e) = self.chat_log.log_user(chat_id, text).await {
            error!(chat_id, error = %e, "chat_log failure");
        }
        if let Some(cmd) = parse_command(text) {
            return self.dispatch_command(cmd, chat_id).await;
        }
        self.run_fsm(chat_id, text).await
    }

    async fn dispatch_command(
        &self, cmd: crate::commands::Command<'_>, chat_id: i64,
    ) -> Result<(), BuddyError> {
        let stores = CommandStores {
            chat_log: &self.chat_log,
            contacts: &self.contacts,
            topics: &self.topics,
        };
        let response = execute_command(cmd, chat_id, &stores).await;
        let _ = send_message(&self.bot_token, chat_id, &response, &self.http).await;
        if let Err(e) = self.chat_log.log_bot(chat_id, &response).await {
            error!(chat_id, error = %e, "chat_log failure");
        }
        Ok(())
    }

    async fn run_fsm(&self, chat_id: i64, text: &str) -> Result<(), BuddyError> {
        let state = self.store.load_state(chat_id).await?.unwrap_or(OnboardState::Intro);
        let was_ready = state == OnboardState::Ready;
        let persona = self.store.load_persona(chat_id).await?.unwrap_or_else(|| json!({}));
        let output = if matches!(state, OnboardState::Intro | OnboardState::AskLanguage) {
            handle_step(&state, text, &persona, self.extractor.as_ref()).await?
        } else {
            let lang = Lang::from_persona(&persona);
            let ctx = retrieve_context(&self.chat_log, &self.topics, chat_id, text, 10, 5).await;
            conversational_step(&state, &persona, &ctx, text, self.extractor.as_ref(), lang).await?
        };
        self.store.save_state(chat_id, &output.next_state).await?;
        self.apply_persona_patch(chat_id, output.persona_patch).await?;
        if was_ready || output.next_state == OnboardState::Ready {
            classify_and_store_topic(self.extractor.as_ref(), self.topics.as_ref(), chat_id, text).await;
        }
        send_message(&self.bot_token, chat_id, &output.response_text, &self.http).await?;
        if let Err(e) = self.chat_log.log_bot(chat_id, &output.response_text).await {
            error!(chat_id, error = %e, "chat_log failure");
        }
        Ok(())
    }

    async fn apply_persona_patch(&self, chat_id: i64, patch: Value) -> Result<(), BuddyError> {
        if patch == json!({}) {
            return Ok(());
        }
        let base = self.store.load_persona(chat_id).await?.unwrap_or_else(|| json!({}));
        let merged = deep_merge(base, patch);
        self.store.save_persona(chat_id, &merged).await
    }
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "crate": "kei-buddy", "version": env!("CARGO_PKG_VERSION") }))
}

/// Build the axum Router.
pub fn build_router<E: LlmExtractor + Send + Sync + 'static>(ctx: BuddyContext<E>) -> Router {
    Router::new()
        .route("/webhook", routing::post(kei_telegram_webhook::handle_webhook::<BuddyContext<E>>))
        .route("/health", routing::get(health))
        .with_state(ctx)
}
