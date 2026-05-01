//! Non-streaming generate — `mlx_lm.generate --model X --prompt P`.
//!
//! Constructor Pattern: this cube builds the argv, calls the Runner,
//! and parses the canonical mlx_lm stdout footer:
//!
//! ```text
//! <generated text>
//! ==========
//! Prompt: 12 tokens, 132.4 tokens-per-sec
//! Generation: 64 tokens, 78.9 tokens-per-sec
//! ```
//!
//! The footer regex is permissive — minor mlx_lm version drift in
//! punctuation (`tokens-per-sec` vs `tokens per second`) is tolerated.

use crate::error::Error;
use crate::platform::is_supported;
use crate::runner::Runner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerateOpts {
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub model_id: String,
    pub prompt: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_per_sec: Option<f32>,
}

/// Run a single non-streaming generation.
pub fn generate(
    runner: &dyn Runner,
    bin: &str,
    model_id: &str,
    prompt: &str,
    opts: &GenerateOpts,
) -> Result<Response, Error> {
    let support = is_supported();
    if !support.supported {
        return Err(Error::NotSupported(
            support.reason.unwrap_or_else(|| "unsupported".into()),
        ));
    }
    let argv = build_argv(model_id, prompt, opts);
    let args: Vec<&str> = argv.iter().map(|s| s.as_str()).collect();
    let out = runner.run(bin, &args).map_err(|e| Error::SpawnFailed(e.to_string()))?;
    if !out.is_success() {
        return Err(Error::NonZeroExit { code: out.code, stderr: out.stderr });
    }
    parse_response(&out.stdout, model_id, prompt)
}

/// Build argv for `mlx_lm.generate`. Visible for tests.
pub fn build_argv(model_id: &str, prompt: &str, opts: &GenerateOpts) -> Vec<String> {
    let mut v = vec![
        "--model".to_string(),
        model_id.to_string(),
        "--prompt".to_string(),
        prompt.to_string(),
    ];
    if let Some(n) = opts.max_tokens {
        v.push("--max-tokens".into());
        v.push(n.to_string());
    }
    if let Some(t) = opts.temperature {
        v.push("--temp".into());
        v.push(format!("{t:.4}"));
    }
    v
}

/// Split stdout into `(generation_text, footer_lines)` and decode the
/// footer. The `==========` separator is the canonical mlx_lm divider.
pub fn parse_response(stdout: &str, model_id: &str, prompt: &str) -> Result<Response, Error> {
    let (text, footer) = split_text_and_footer(stdout);
    let (pt, gt, tps) = parse_footer(&footer);
    Ok(Response {
        model_id: model_id.to_string(),
        prompt: prompt.to_string(),
        text,
        prompt_tokens: pt,
        generation_tokens: gt,
        tokens_per_sec: tps,
    })
}

fn split_text_and_footer(stdout: &str) -> (String, String) {
    if let Some((before, after)) = stdout.split_once("==========") {
        (before.trim().to_string(), after.trim().to_string())
    } else {
        (stdout.trim().to_string(), String::new())
    }
}

fn parse_footer(footer: &str) -> (Option<u32>, Option<u32>, Option<f32>) {
    let pt_re = regex::Regex::new(r"(?i)Prompt:\s*([0-9]+)\s*tokens").ok();
    let gt_re = regex::Regex::new(r"(?i)Generation:\s*([0-9]+)\s*tokens").ok();
    let tps_re =
        regex::Regex::new(r"(?i)([0-9]+\.?[0-9]*)\s*tokens[\s-]+per[\s-]+sec").ok();
    let pt = pt_re
        .as_ref()
        .and_then(|r| r.captures(footer))
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());
    let gt = gt_re
        .as_ref()
        .and_then(|r| r.captures(footer))
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());
    let tps = tps_re
        .as_ref()
        .and_then(|r| r.captures(footer))
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse().ok());
    (pt, gt, tps)
}
