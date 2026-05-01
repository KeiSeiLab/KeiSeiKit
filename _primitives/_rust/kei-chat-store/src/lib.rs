//! kei-chat-store — SQLite + FTS5 session archive for Claude chats.

pub mod schema;
pub mod search;
pub mod sessions;
pub mod stats;
pub mod store;

pub use sessions::{ChatMessage, ChatSession};
pub use store::Store;
