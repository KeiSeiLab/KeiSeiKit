// SPDX-License-Identifier: Apache-2.0
//! Error type for kei-buddy operations.
//!
//! Three categories cover the three integration layers that will be
//! wired in follow-up tasks: state-machine, memory store, transport.

use thiserror::Error;

/// Top-level error type for the KeiBuddy crate.
///
/// Variants will gain `#[from]` impls once the concrete dependencies
/// (kei-memory-sqlite, kei-notify-telegram, kei-cortex) are wired.
#[derive(Debug, Error)]
pub enum BuddyError {
    /// Error originating in the onboarding state machine.
    #[error("state machine error: {0}")]
    StateMachine(String),

    /// Error originating in the memory persistence layer.
    #[error("memory error: {0}")]
    Memory(String),

    /// Error originating in the Telegram (or other) transport layer.
    #[error("transport error: {0}")]
    Transport(String),
}

impl From<rusqlite::Error> for BuddyError {
    fn from(e: rusqlite::Error) -> Self {
        BuddyError::Memory(e.to_string())
    }
}

impl From<kei_memory_sqlite::Error> for BuddyError {
    fn from(e: kei_memory_sqlite::Error) -> Self {
        BuddyError::Memory(e.to_string())
    }
}
