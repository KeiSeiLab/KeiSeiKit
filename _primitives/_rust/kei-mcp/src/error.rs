//! MCP server errors.

/// Errors returned by public library APIs in kei-mcp.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid utf-8 in input: {0}")]
    InvalidUtf8(String),

    #[error("stdin: {0}")]
    Stdin(String),
}
