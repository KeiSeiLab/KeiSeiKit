// SPDX-License-Identifier: Apache-2.0
//! `run_serve` — store construction + HTTP listener bootstrap.
//! Extracted from serve.rs to keep both files ≤ 200 LOC.

use std::sync::Arc;

use serde_json::json;
use tracing::warn;

use crate::{
    chat_log::ChatLog,
    contacts::Contacts,
    extractor::LlmExtractor,
    serve::{BuddyContext, ServeConfig},
    store::SqliteBuddyStore,
    topics::Topics,
    voice::VoiceHandler,
};

/// Start the HTTP server (entry-point called from the binary).
pub async fn run_serve(cfg: ServeConfig) -> anyhow::Result<()> {
    init_tracing();
    let store = Arc::new(SqliteBuddyStore::from_path(&cfg.db_path)?);
    let allowed_chat_ids = Arc::new(cfg.allowed_chat_ids);
    let http = reqwest::Client::new();
    let chat_log = Arc::new(ChatLog::from_path(&cfg.chat_log_db_path)?);
    let topics = Arc::new(Topics::from_path(&cfg.topics_db_path)?);
    let contacts = Arc::new(Contacts::from_path(&cfg.contacts_db_path)?);
    let voice = build_voice_handler(cfg.stt_backend.as_deref(), &cfg.bot_token);

    #[cfg(feature = "extractor-openai")]
    {
        if let (Some(proxy), Some(key)) = (cfg.llm_proxy_url, cfg.llm_api_key) {
            let model = cfg.llm_model.unwrap_or_else(|| "gpt-4o-mini".to_string());
            tracing::info!(model = %model, "using OpenAiExtractor (LiteLLM-compatible)");
            let extractor = Arc::new(crate::extractor::openai::OpenAiExtractor::new_with_model(
                proxy, key, model,
            ));
            return start_listener(cfg.port, BuddyContext {
                secret: cfg.webhook_secret,
                bot_token: cfg.bot_token,
                store, extractor, http, allowed_chat_ids, chat_log, topics, contacts, voice,
            }).await;
        }
    }

    warn!("no LLM extractor configured — using MockExtractor");
    let extractor = Arc::new(crate::extractor::MockExtractor::new(json!({})));
    start_listener(cfg.port, BuddyContext {
        secret: cfg.webhook_secret,
        bot_token: cfg.bot_token,
        store, extractor, http, allowed_chat_ids, chat_log, topics, contacts, voice,
    }).await
}

fn build_voice_handler(stt_backend: Option<&str>, bot_token: &str) -> Option<Arc<VoiceHandler>> {
    let name = stt_backend?;
    std::env::set_var("KEI_STT_BACKEND", name);
    match kei_stt::from_env() {
        Ok(stt) => Some(Arc::new(VoiceHandler::new(bot_token.to_string(), Arc::from(stt)))),
        Err(e) => {
            tracing::warn!(backend = name, error = %e, "STT init failed; voice disabled");
            None
        }
    }
}

async fn start_listener<E>(port: u16, ctx: BuddyContext<E>) -> anyhow::Result<()>
where
    E: LlmExtractor + Send + Sync + 'static,
{
    let router = crate::serve::build_router(ctx);
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(addr = %addr, "kei-buddy listening");
    axum::serve(listener, router).await?;
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};
    let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
}
