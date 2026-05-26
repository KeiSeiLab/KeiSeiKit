//! MCP method handlers — one file per method-family.
//!
//! `dispatch` is the single entry point; it parses the method string,
//! delegates to the matching family handler, and folds the result back into
//! a JSON-RPC envelope.
//!
//! `dispatch` is `async` because `tools/call` shells out to atom binaries
//! through `tokio::process::Command` with a 60s timeout (MISS-4). The other
//! method handlers are pure / synchronous; they are awaited as cheap no-op
//! futures by the matcher below.

pub mod initialize;
pub mod prompts;
pub mod resources;
pub mod safe_tools;
pub mod tools;

use crate::protocol::{err, JsonRpcRequest, JsonRpcResponse, Method, ServerContext, METHOD_NOT_FOUND};

/// Async dispatcher — picks the right handler based on the method.
///
/// Each handler returns a complete `JsonRpcResponse`, including its own
/// `id` echo and either `result` or `error`. Unknown methods produce a
/// JSON-RPC `-32601 method not found` error.
pub async fn dispatch(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    match Method::parse(&req.method) {
        Method::Initialize => initialize::handle(req, ctx),
        Method::ToolsList => tools::list(req, ctx),
        Method::ToolsCall => tools::call(req, ctx).await,
        Method::ResourcesList => resources::list(req, ctx),
        Method::ResourcesRead => resources::read(req, ctx),
        Method::PromptsList => prompts::list(req, ctx),
        Method::PromptsGet => prompts::get(req, ctx),
        Method::Other(m) => err(req.id, METHOD_NOT_FOUND, format!("method not found: {m}")),
    }
}
