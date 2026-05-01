//! CLI command handlers — one async fn per subcommand.
//!
//! Each handler returns `Result<(), ApiError>`; `main` maps the error to an
//! exit code via `ApiError::exit_code`.

use std::io::Write;

use futures::StreamExt;

use crate::api::{build_options, ChatReq, GenerateReq, Message};
use crate::cli::{BaseUrlOpt, ChatOpt, GenerateOpt, PullOpt};
use crate::client::Client;
use crate::error::ApiError;
use crate::health;

pub async fn run_tags(opt: &BaseUrlOpt) -> Result<(), ApiError> {
    let client = Client::new(opt.base_url.clone());
    let resp = client.tags().await?;
    println_json(&resp)
}

pub async fn run_generate(opt: &GenerateOpt) -> Result<(), ApiError> {
    let client = Client::new(opt.base.base_url.clone());
    let req = GenerateReq {
        model: opt.model.clone(),
        prompt: opt.prompt.clone(),
        stream: opt.stream,
        options: build_options(opt.temperature, opt.max_tokens),
    };
    if opt.stream {
        let mut s = client.generate_stream(&req).await?;
        stream_to_stdout(&mut s).await
    } else {
        let resp = client.generate(&req).await?;
        println_json(&resp)
    }
}

pub async fn run_chat(opt: &ChatOpt) -> Result<(), ApiError> {
    let client = Client::new(opt.base.base_url.clone());
    let messages = parse_messages(&opt.messages)?;
    let req = ChatReq {
        model: opt.model.clone(),
        messages,
        stream: opt.stream,
        options: build_options(opt.temperature, opt.max_tokens),
    };
    if opt.stream {
        let mut s = client.chat_stream(&req).await?;
        stream_to_stdout(&mut s).await
    } else {
        let resp = client.chat(&req).await?;
        println_json(&resp)
    }
}

pub async fn run_pull(opt: &PullOpt) -> Result<(), ApiError> {
    let client = Client::new(opt.base.base_url.clone());
    let mut s = client.pull_stream(&opt.model).await?;
    while let Some(chunk) = s.next().await {
        let bytes = chunk.map_err(|e| ApiError::Transport(e.to_string()))?;
        let text = String::from_utf8_lossy(&bytes);
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            eprintln!("{line}");
        }
    }
    println!("{{\"status\":\"done\",\"model\":\"{}\"}}", opt.model);
    Ok(())
}

pub async fn run_health(opt: &BaseUrlOpt) -> Result<(), ApiError> {
    let client = Client::new(opt.base_url.clone());
    let snap = health::snapshot(&client).await;
    println_json(&snap)?;
    if !snap.running {
        return Err(ApiError::DaemonNotRunning {
            url: opt.base_url.clone(),
            source: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "no response"),
        });
    }
    Ok(())
}

fn parse_messages(arg: &str) -> Result<Vec<Message>, ApiError> {
    if let Some(path) = arg.strip_prefix('@') {
        let raw = std::fs::read_to_string(path).map_err(|e| ApiError::DecodeError(e.to_string()))?;
        return serde_json::from_str(&raw).map_err(|e| ApiError::DecodeError(e.to_string()));
    }
    serde_json::from_str(arg).map_err(|e| ApiError::DecodeError(e.to_string()))
}

fn println_json<T: serde::Serialize>(t: &T) -> Result<(), ApiError> {
    let s = serde_json::to_string_pretty(t).map_err(|e| ApiError::DecodeError(e.to_string()))?;
    println!("{s}");
    Ok(())
}

async fn stream_to_stdout<S>(stream: &mut S) -> Result<(), ApiError>
where
    S: futures::stream::Stream<Item = Result<crate::stream::Chunk, ApiError>> + Unpin,
{
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        let line = serde_json::json!({
            "delta": chunk.delta,
            "done": chunk.done,
            "eval_count": chunk.eval_count,
            "eval_duration_ns": chunk.eval_duration_ns,
        });
        writeln!(lock, "{line}").map_err(|e| ApiError::Transport(e.to_string()))?;
        if chunk.done {
            break;
        }
    }
    Ok(())
}
