//! `kei-tty` — terminal UI client for the local kei-cortex daemon.
//!
//! Constructor pattern:
//!   * [`types`] — wire types (SSE `ChatEvent` enum + request body).
//!   * [`client`] — async HTTP/SSE client (`chat_stream`).
//!   * [`app`]    — TUI state machine (`App` + tokio::select! loop).
//!   * [`ui`]     — ratatui frame rendering (read-only over `&App`).
//!   * [`keys`]   — keyboard event → state-transition mapping.
//!
//! Each module is independently testable. The crate has both a `lib` (for
//! integration tests) and a `bin` (`main.rs`) entry point.

pub mod app;
pub mod client;
pub mod keys;
pub mod runner;
pub mod types;
pub mod ui;
