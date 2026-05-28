//! kei-mcp — Model Context Protocol server.
//!
//! Exposes the atom registry (and skills as resources) over stdio JSON-RPC,
//! per the MCP spec at <https://modelcontextprotocol.io/>. JSON-RPC 2.0,
//! line-delimited, one request → one response. Stateless beyond the
//! `initialize` handshake.
//!
//! Library shape exists so integration tests can drive `dispatch` directly
//! without spawning the binary.

pub mod error;
pub mod framing;
pub mod handlers;
pub mod protocol;

pub use error::Error;
pub use framing::{read_capped_line, ReadOutcome, MAX_MESSAGE_BYTES};
pub use handlers::dispatch;
pub use protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, Method, ServerContext};
