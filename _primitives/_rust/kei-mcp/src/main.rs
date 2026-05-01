//! kei-mcp binary — stdio JSON-RPC loop.
//!
//! Flow:
//!   1. Read stdin line-by-line (each line is one JSON-RPC request).
//!   2. Cap each line at `MAX_MESSAGE_BYTES`; oversize lines emit a
//!      `-32700 parse error` reply and the loop continues.
//!   3. Parse each line; on parse error emit a `-32700 parse error` reply.
//!   4. Dispatch to the matching async handler; serialise the response
//!      back as one stdout line.
//!   5. On stdin EOF, exit cleanly (graceful shutdown).
//!
//! Discipline:
//!   - stdout: ONLY JSON-RPC frames (line-delimited).
//!   - stderr: ALL logging / warnings.
//!   - One request per line, one response per line. No batching for now.

use anyhow::Context;
use kei_mcp::{
    dispatch, read_capped_line, JsonRpcRequest, JsonRpcResponse, ReadOutcome, ServerContext,
    MAX_MESSAGE_BYTES,
};
use serde_json::Value;
use std::path::PathBuf;
use tokio::io::{AsyncWriteExt, BufReader};

/// JSON-RPC `parse error` code per the spec.
const PARSE_ERROR: i32 = -32700;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let ctx = build_context();
    eprintln!(
        "kei-mcp v{} ready (atoms_root={}, skills_root={})",
        ctx.server_version,
        ctx.atoms_root.display(),
        ctx.skills_root.display(),
    );

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut stdout = tokio::io::stdout();

    loop {
        match read_capped_line(&mut reader).await? {
            ReadOutcome::Eof => break,
            ReadOutcome::Empty => continue,
            ReadOutcome::Line(line) => {
                let response = handle_one_line(&line, &ctx).await;
                write_response(&mut stdout, &response).await?;
            }
            ReadOutcome::Oversize => {
                let response = oversize_error_response();
                write_response(&mut stdout, &response).await?;
            }
        }
    }

    eprintln!("kei-mcp: stdin EOF, shutting down");
    Ok(())
}

/// Default atom-registry root: `$KEI_MCP_ATOMS_ROOT` or `_primitives/_rust`.
/// Default skills root: `$KEI_MCP_SKILLS_ROOT` or `skills/`.
fn build_context() -> ServerContext {
    let atoms_root = std::env::var("KEI_MCP_ATOMS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("_primitives/_rust"));
    let skills_root = std::env::var("KEI_MCP_SKILLS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("skills"));
    ServerContext::new(atoms_root, skills_root)
}

/// Parse + dispatch one stdin line. On parse failure produce a JSON-RPC
/// parse-error reply with `id: null`.
async fn handle_one_line(line: &str, ctx: &ServerContext) -> JsonRpcResponse {
    match serde_json::from_str::<JsonRpcRequest>(line) {
        Ok(req) => dispatch(req, ctx).await,
        Err(e) => parse_error_response(e),
    }
}

fn parse_error_response(e: serde_json::Error) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".into(),
        id: Some(Value::Null),
        result: None,
        error: Some(kei_mcp::JsonRpcError {
            code: PARSE_ERROR,
            message: format!("parse error: {e}"),
            data: None,
        }),
    }
}

/// Build a `-32700` reply for an oversize line. We deliberately do NOT
/// echo any `id` (we never parsed one), per the spec's parse-error rules.
fn oversize_error_response() -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".into(),
        id: Some(Value::Null),
        result: None,
        error: Some(kei_mcp::JsonRpcError {
            code: PARSE_ERROR,
            message: format!(
                "parse error: line exceeded {} bytes",
                MAX_MESSAGE_BYTES
            ),
            data: None,
        }),
    }
}

async fn write_response(
    stdout: &mut tokio::io::Stdout,
    response: &JsonRpcResponse,
) -> anyhow::Result<()> {
    let mut line = serde_json::to_string(response).context("serialising response")?;
    line.push('\n');
    stdout.write_all(line.as_bytes()).await?;
    stdout.flush().await?;
    Ok(())
}
