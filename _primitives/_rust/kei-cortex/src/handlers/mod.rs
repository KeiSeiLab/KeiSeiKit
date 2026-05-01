//! HTTP handler modules — one file per endpoint family.

pub mod chat;
pub mod chat_cost;
pub mod chat_events;
pub mod chat_memory_nudge;
pub mod chat_stream;
mod chat_stream_ctx;
pub mod chat_token;
pub mod fs_list;
pub mod health;
pub mod ledger;
pub mod memory;
pub mod pet;
pub mod portrait;
pub mod stt;
pub mod summary;
pub mod term;
mod term_pty;
pub mod tool_apply;
mod tool_apply_atomic;
pub mod tts;
pub mod usage;
pub mod voice_id;
