// SPDX-License-Identifier: Apache-2.0
//! kei-buddy — KeiBuddy personal-assistant Telegram bot scaffold.
//!
//! Module layout (Constructor Pattern — one file, one responsibility):
//!   * `state`      — `OnboardState` enum
//!   * `transition` — `StepOutput` output struct
//!   * `extractor`  — `LlmExtractor` trait + `MockExtractor` + `OpenAiExtractor` (feature-gated)
//!   * `machine`    — `handle_step` — the 11-arm onboarding FSM
//!   * `error`      — `BuddyError` error type
//!   * `schema`     — buddy-specific SQLite DDL
//!   * `store`      — `BuddyStore` trait + `SqliteBuddyStore` impl

pub mod chat_log;
pub mod contacts;
pub mod error;
pub mod extractor;
pub mod machine;
pub(crate) mod machine_helpers;
pub mod persona_merge;
pub mod schema;
pub mod state;
pub mod store;
pub(crate) mod store_ops;
pub mod topics;
pub mod transition;

#[cfg(feature = "serve")]
pub mod serve;
#[cfg(feature = "serve")]
pub mod serve_telegram;

pub use chat_log::ChatLog;
pub use contacts::Contacts;
pub use error::BuddyError;
pub use extractor::LlmExtractor;
pub use machine::handle_step;
pub use state::OnboardState;
pub use store::{BuddyStore, SqliteBuddyStore};
pub use topics::Topics;
pub use transition::StepOutput;
