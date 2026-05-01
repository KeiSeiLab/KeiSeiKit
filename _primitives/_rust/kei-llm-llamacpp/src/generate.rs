//! Generate — non-streaming `llama-cli` invocation.
//!
//! Shells out to `llama-cli -m <path> -p <prompt> -n <n>` (plus optional
//! `--temp`) and parses the trailing timing footer. Output:
//! `Response { text, eval_tokens, eval_ms, tokens_per_sec }`.
//!
//! Footer format (llama.cpp >= b3000): line of the form
//!   "llama_perf_context_print: eval time = 1234.56 ms / 12 runs ..."
//! We tolerate older builds that emit "llama_print_timings" with the
//! same fields.

use crate::error::{Error, Result};
use crate::runner::Runner;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// User-facing options for a generate call.
#[derive(Debug, Clone)]
pub struct GenerateOpts {
    pub max_tokens: u32,
    pub temperature: Option<f32>,
}

impl Default for GenerateOpts {
    fn default() -> Self {
        Self { max_tokens: 128, temperature: None }
    }
}

/// Parsed result from a non-streaming `llama-cli` run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    pub text: String,
    pub eval_tokens: u32,
    pub eval_ms: f64,
    pub tokens_per_sec: f64,
}

/// Build the argv passed to `llama-cli` for a non-streaming call.
pub fn build_args(model: &Path, prompt: &str, opts: &GenerateOpts) -> Vec<String> {
    let mut args = vec![
        "-m".into(),
        model.to_string_lossy().into_owned(),
        "-p".into(),
        prompt.to_string(),
        "-n".into(),
        opts.max_tokens.to_string(),
        "--no-display-prompt".into(),
    ];
    if let Some(t) = opts.temperature {
        args.push("--temp".into());
        args.push(t.to_string());
    }
    args
}

/// Run a non-streaming generate via the supplied Runner.
pub async fn generate<R: Runner + ?Sized>(
    runner: &R,
    bin: &str,
    model: &Path,
    prompt: &str,
    opts: &GenerateOpts,
) -> Result<Response> {
    if !model.exists() {
        return Err(Error::ModelNotFound { path: model.to_path_buf() });
    }
    let args = build_args(model, prompt, opts);
    let out = runner.run(bin, &args).await?;
    if out.code != 0 {
        return Err(Error::NonZeroExit { code: out.code, stderr: out.stderr });
    }
    parse_stdout(&out.stdout, &out.stderr)
}

/// Parse the captured stdout+stderr into a Response. The "answer text"
/// is the stdout up to the timing footer; timings come from stderr (or
/// stdout for older builds).
pub fn parse_stdout(stdout: &str, stderr: &str) -> Result<Response> {
    let (text, _) = split_text_and_footer(stdout);
    let combined = format!("{stdout}\n{stderr}");
    let (eval_tokens, eval_ms) = parse_timings(&combined)?;
    let tps = if eval_ms > 0.0 {
        (eval_tokens as f64) * 1000.0 / eval_ms
    } else {
        0.0
    };
    Ok(Response {
        text: text.trim().to_string(),
        eval_tokens,
        eval_ms,
        tokens_per_sec: tps,
    })
}

/// Split the model output into (answer_text, footer_block). The footer
/// starts at the first line beginning with "llama_perf" or
/// "llama_print_timings".
fn split_text_and_footer(stdout: &str) -> (String, String) {
    let mut text = String::new();
    let mut footer = String::new();
    let mut in_footer = false;
    for line in stdout.lines() {
        if !in_footer && (line.starts_with("llama_perf") || line.starts_with("llama_print_timings"))
        {
            in_footer = true;
        }
        if in_footer {
            footer.push_str(line);
            footer.push('\n');
        } else {
            text.push_str(line);
            text.push('\n');
        }
    }
    (text, footer)
}

/// Pull `(eval_tokens, eval_ms)` out of the timing footer.
/// Format: `eval time = 1234.56 ms / 12 runs`.
fn parse_timings(combined: &str) -> Result<(u32, f64)> {
    let re = Regex::new(r"eval time\s*=\s*([\d.]+)\s*ms\s*/\s*(\d+)\s*runs?").map_err(|e| {
        Error::ParseFailed { reason: format!("regex compile: {e}") }
    })?;
    let cap = re
        .captures(combined)
        .ok_or_else(|| Error::ParseFailed { reason: "no eval-time footer".into() })?;
    let ms: f64 = cap[1]
        .parse()
        .map_err(|e| Error::ParseFailed { reason: format!("ms parse: {e}") })?;
    let tokens: u32 = cap[2]
        .parse()
        .map_err(|e| Error::ParseFailed { reason: format!("tokens parse: {e}") })?;
    Ok((tokens, ms))
}
