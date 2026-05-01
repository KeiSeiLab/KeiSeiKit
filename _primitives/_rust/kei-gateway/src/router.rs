//! Outbound delivery router.
//!
//! Mirrors the Hermes `_resolve_delivery_target` / `_deliver_result` flow
//! (cron/scheduler.py:150-484): a job's output is dispatched to the source
//! channel, a configured home channel, or an explicit `platform:chat_id` ref.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::adapters::base::{OutboundMessage, PlatformAdapter, SendResult};
use crate::message::Platform;

/// Where to deliver a response.
#[derive(Debug, Clone)]
pub enum DeliveryTarget {
    /// Reply on the same channel the message came from.
    Origin {
        platform: Platform,
        chat_id: String,
        thread_id: Option<String>,
    },
    /// Local-only — write to file, no platform send.
    Local,
    /// Explicit destination override.
    Explicit {
        platform: Platform,
        chat_id: String,
        thread_id: Option<String>,
    },
}

/// Routes [`OutboundMessage`] to the right [`PlatformAdapter`].
#[derive(Default, Clone)]
pub struct DeliveryRouter {
    adapters: HashMap<Platform, Arc<dyn PlatformAdapter>>,
}

impl DeliveryRouter {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Register an adapter for `platform`. Replaces any existing entry.
    pub fn register(&mut self, platform: Platform, adapter: Arc<dyn PlatformAdapter>) {
        self.adapters.insert(platform, adapter);
    }

    /// Number of adapters wired in. Test / observability helper.
    pub fn adapter_count(&self) -> usize {
        self.adapters.len()
    }

    /// Dispatch `msg` to the resolved `target`. Returns the adapter's result,
    /// or a synthetic local-only success when `target == DeliveryTarget::Local`.
    pub async fn deliver(
        &self,
        target: DeliveryTarget,
        msg: OutboundMessage,
    ) -> Result<SendResult> {
        match target {
            DeliveryTarget::Local => Ok(SendResult::local()),
            DeliveryTarget::Origin {
                platform,
                chat_id,
                thread_id,
            }
            | DeliveryTarget::Explicit {
                platform,
                chat_id,
                thread_id,
            } => {
                let adapter = self.adapters.get(&platform).cloned().ok_or_else(|| {
                    anyhow!(
                        "no adapter registered for platform {}",
                        platform.as_str()
                    )
                })?;
                let final_msg = msg.with_target(chat_id, thread_id);
                adapter.send(final_msg).await
            }
        }
    }
}
