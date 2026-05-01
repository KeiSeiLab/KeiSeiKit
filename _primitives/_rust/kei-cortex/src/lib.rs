//! kei-cortex — local HTTP daemon exposing cortex state for UI consumption.
//!
//! Constructor Pattern: one module = one responsibility. This crate wires up:
//! `auth` (bearer-token lifecycle), `config` (CLI/env binding), `error`
//! (typed JSON responses), `state` (shared handler state), `routes` (router
//! + middleware), `handlers` (endpoint implementations).
//!
//! The daemon is intended to serve a single user on `127.0.0.1:9797` and
//! is fronted by a bearer token read from `~/.keisei/cortex.token`. CORS is
//! locked to a single origin provided at startup.

pub mod agent;
pub mod anthropic;
pub mod anthropic_config;
pub mod anthropic_invoker;
pub mod anthropic_sse;
pub mod auth;
pub mod config;
pub mod context;
pub mod elevenlabs;
pub mod error;
pub mod fal;
pub mod handlers;
pub mod persona;
pub mod whisper_local;
pub mod rig_clone;
pub mod routes;
pub mod sentiment;
pub mod state;
pub mod tool;
pub mod validate;

pub use config::AppConfig;
pub use error::AppError;
pub use routes::build_router;
pub use state::AppState;
