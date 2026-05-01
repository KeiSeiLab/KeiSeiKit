//! Platform adapters (per-protocol I/O).
//!
//! `base` defines the [`PlatformAdapter`] trait. Concrete adapters are
//! feature-gated; only `cli` is fully implemented in MVP.

pub mod base;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "telegram")]
pub mod telegram;

#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "slack")]
pub mod slack;
