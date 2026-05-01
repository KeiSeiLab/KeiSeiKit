//! Orchestrates the full inbound → agent → delivery loop.
//!
//! Hermes equivalent: `gateway/run.py:_handle_message_event`.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::adapters::base::OutboundMessage;
use crate::agent_cache::AgentCache;
use crate::guard::SessionGuard;
use crate::message::MessageEvent;
use crate::router::{DeliveryRouter, DeliveryTarget};
use crate::session_key::{build_session_key, SessionKeyOpts};
use crate::session_store::SessionStore;

/// Type-erased agent runner.
///
/// Real impl supplies `Arc<dyn AgentRunFn>`; the gateway only sees the trait.
#[async_trait::async_trait]
pub trait AgentRunFn: Send + Sync {
    /// Process an inbound event and return the agent's outbound text. Empty
    /// string means "[SILENT] — no delivery".
    async fn run(&self, session_key: &str, event: &MessageEvent) -> Result<String>;
}

/// Configuration for [`GatewayRunner`].
#[derive(Clone)]
pub struct RunnerConfig {
    pub session_opts: SessionKeyOpts,
    /// Channel buffer for inbound events.
    pub inbound_buffer: usize,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            session_opts: SessionKeyOpts::default(),
            inbound_buffer: 256,
        }
    }
}

/// Top-level gateway runtime.
///
/// Wires:
/// - inbound mpsc channel (adapters push events here)
/// - per-session [`SessionGuard`]
/// - [`AgentCache`] for warm agents
/// - [`SessionStore`] for durable session bookkeeping
/// - [`DeliveryRouter`] for outbound dispatch
pub struct GatewayRunner {
    config: RunnerConfig,
    guard: SessionGuard,
    agent_cache: AgentCache,
    sessions: SessionStore,
    router: DeliveryRouter,
    agent_runner: Arc<dyn AgentRunFn>,
}

impl GatewayRunner {
    pub fn new(
        config: RunnerConfig,
        guard: SessionGuard,
        agent_cache: AgentCache,
        sessions: SessionStore,
        router: DeliveryRouter,
        agent_runner: Arc<dyn AgentRunFn>,
    ) -> Self {
        Self {
            config,
            guard,
            agent_cache,
            sessions,
            router,
            agent_runner,
        }
    }

    /// Process a single inbound event end-to-end.
    pub async fn handle_inbound(&self, event: MessageEvent) -> Result<()> {
        let key = build_session_key(&event.source, self.config.session_opts);
        let _lock = self.guard.acquire(&key).await;
        let _data = self
            .sessions
            .get_or_create(&key, || format!("session_{}", chrono::Utc::now().timestamp()))
            .await?;
        let response = self.agent_runner.run(&key, &event).await?;
        self.sessions.record_turn(&key).await?;
        if response.is_empty() {
            return Ok(());
        }
        let target = origin_target(&event, &key);
        self.router.deliver(target, OutboundMessage::text(response)).await?;
        Ok(())
    }

    /// Spawn the inbound consume loop on the current Tokio runtime. Returns the
    /// sender half so adapters can push events.
    pub fn start(self: Arc<Self>) -> mpsc::Sender<MessageEvent> {
        let (tx, mut rx) = mpsc::channel::<MessageEvent>(self.config.inbound_buffer);
        let me = self.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let Err(e) = me.handle_inbound(event).await {
                    eprintln!("[kei-gateway] inbound failed: {e:#}");
                }
            }
        });
        tx
    }

    /// Helper for tests: drop the agent cache. Mostly for debug.
    pub async fn purge_idle_agents(&self) -> usize {
        self.agent_cache.evict_idle().await
    }
}

/// Build an [`DeliveryTarget::Origin`] from the inbound event source.
fn origin_target(event: &MessageEvent, fallback_key: &str) -> DeliveryTarget {
    DeliveryTarget::Origin {
        platform: event.source.platform,
        chat_id: event
            .source
            .chat_id
            .clone()
            .unwrap_or_else(|| fallback_key.to_string()),
        thread_id: event.source.thread_id.clone(),
    }
}
