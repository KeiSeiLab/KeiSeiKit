// SPDX-License-Identifier: Apache-2.0
//! kei-buddy — KeiBuddy personal-assistant Telegram bot scaffold.
//!
//! Concept-level crate. This file declares the public module surface.
//! No business logic lives here; see individual modules.
//!
//! Module layout (Constructor Pattern — one file, one responsibility):
//!   * `state`      — `OnboardState` enum + `next()` stub
//!   * `transition` — `TransitionInput` input struct
//!   * `error`      — `BuddyError` error type
//!
//! Follow-up tasks will add:
//!   * LLM extraction via kei-cortex
//!   * Memory persistence via kei-memory-sqlite
//!   * Telegram webhook driver (kei-notify-telegram)

pub mod error;
pub mod state;
pub mod transition;

pub use error::BuddyError;
pub use state::OnboardState;
pub use transition::TransitionInput;
