//! kei-forge — local web wizard for scaffolding new atoms per the locked
//! SUBSTRATE-SCHEMA.md contract.
//!
//! Architecture (Constructor Pattern, one responsibility per file):
//! - [`server`]     — axum router + handlers
//! - [`middleware`] — DNS-rebinding + CSRF defences
//! - [`headers`]    — CSP / nosniff / frame-deny / referrer headers
//! - [`html`]       — static HTML form (JSON-over-fetch)
//! - [`form`]       — request deserialization + validation
//! - [`generate`]   — pure-Rust atom templating (no shell-out)
//!
//! Public entry point is [`server::app`], which returns the fully-wired
//! `axum::Router` ready to be served by any bind target (production =
//! 127.0.0.1:8747; tests = random ephemeral port).

pub mod form;
pub mod generate;
pub mod headers;
pub mod html;
pub mod middleware;
pub mod server;
