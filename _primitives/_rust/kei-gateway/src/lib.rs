//! P4.1 — Unified messaging gateway.
//!
//! Cross-platform message ingress (Telegram / Discord / Slack / CLI / WhatsApp /
//! Signal / Generic) → normalised [`MessageEvent`] → session-keyed agent run →
//! response delivery via [`DeliveryRouter`].
//!
//! MVP scope: only the CLI adapter is fully implemented. Telegram / Discord /
//! Slack adapters are feature-gated stubs (Hermes-equivalent surface, todo!()
//! bodies). Full impls land in P4.1.b.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub mod adapters;
pub mod agent_cache;
pub mod guard;
pub mod message;
pub mod router;
pub mod runner;
pub mod session_key;
pub mod session_store;

pub use adapters::base::{OutboundMessage, PlatformAdapter, SendResult};
pub use agent_cache::{AgentCache, CachedAgent};
pub use guard::SessionGuard;
pub use message::{MessageEvent, MessageType, Platform, SessionSource};
pub use router::{DeliveryRouter, DeliveryTarget};
pub use runner::GatewayRunner;
pub use session_key::{build_session_key, SessionKeyOpts};
pub use session_store::{SessionData, SessionStore};
