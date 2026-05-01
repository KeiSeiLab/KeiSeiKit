// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Apple Silicon only. On other platforms, complete() returns
//! Error::Provider("MLX requires macOS Apple Silicon").
//! Non-streaming. Flattens chat to single-prompt.

use crate::error::{Error as BrError, Result as BrResult};
use kei_llm_mlx::generate::{generate, GenerateOpts};
use kei_llm_mlx::platform::is_supported;
use kei_llm_mlx::runner::Runner;
use kei_runtime_core::traits::llm::{
    CompletionOpts, CompletionResponse, LlmBackend, Message,
};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::sync::Arc;

pub struct MlxBridge {
    dna: Dna,
    parent: Option<Dna>,
    runner: Arc<dyn Runner + Send + Sync>,
    bin: String,
    model_id: String,
    context_window: u32,
}

impl MlxBridge {
    pub fn new(
        runner: Arc<dyn Runner + Send + Sync>,
        parent: Option<Dna>,
        bin: String,
        model_id: String,
        context_window: u32,
    ) -> BrResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "MX"])
            .scope("keiseikit.dev/primitives/kei-llm-bridge-mlx")
            .body(model_id.as_bytes())
            .build()?;
        Ok(Self { dna, parent, runner, bin, model_id, context_window })
    }
}

impl HasDna for MlxBridge {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait::async_trait]
impl LlmBackend for MlxBridge {
    fn backend_name(&self) -> &'static str { "mlx-bridge" }
    fn model_name(&self) -> &str { &self.model_id }

    async fn complete(&self, messages: &[Message], opts: &CompletionOpts) -> kei_runtime_core::Result<CompletionResponse> {
        if !is_supported().supported {
            return Err(BrError::WrongPlatform.into());
        }
        let prompt = flatten_messages(messages);
        let mlx_opts = GenerateOpts {
            max_tokens: opts.max_tokens,
            temperature: opts.temperature,
        };
        // kei-llm-mlx::generate is sync. Wrap in spawn_blocking for async trait.
        let runner = self.runner.clone();
        let bin = self.bin.clone();
        let model_id = self.model_id.clone();
        let resp = tokio::task::spawn_blocking(move || {
            generate(runner.as_ref(), &bin, &model_id, &prompt, &mlx_opts)
        })
        .await
        .map_err(|e| BrError::Mlx(format!("join: {e}")))?
        .map_err(BrError::from)?;

        let in_tokens = resp.prompt_tokens.unwrap_or_else(|| estimate_tokens(&resp.prompt));
        let out_tokens = resp.generation_tokens.unwrap_or_else(|| estimate_tokens(&resp.text));
        Ok(CompletionResponse {
            text: resp.text,
            stop_reason: "stop".into(),
            tokens_input: in_tokens,
            tokens_output: out_tokens,
            cached_tokens: 0,
            request_id: resp.model_id,
        })
    }

    fn pricing_per_mtok(&self) -> (f64, f64) { (0.0, 0.0) }
    fn supports_caching(&self) -> bool { false }
    fn supports_batch(&self) -> bool { false }
    fn context_window(&self) -> u32 { self.context_window }
}

fn flatten_messages(messages: &[Message]) -> String {
    let mut buf = String::new();
    for m in messages {
        if !buf.is_empty() { buf.push_str("\n\n"); }
        match m.role.as_str() {
            "system" => buf.push_str(&format!("[SYSTEM] {}", m.content)),
            "user" => buf.push_str(&format!("[USER] {}", m.content)),
            "assistant" => buf.push_str(&format!("[ASSISTANT] {}", m.content)),
            other => buf.push_str(&format!("[{}] {}", other.to_uppercase(), m.content)),
        }
    }
    buf.push_str("\n\n[ASSISTANT] ");
    buf
}

fn estimate_tokens(s: &str) -> u32 {
    let words = s.split_whitespace().count() as f64;
    (words * 1.3).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_llm_mlx::runner::{RunOutput, Runner};

    struct StubRunner;
    impl Runner for StubRunner {
        fn run(&self, _bin: &str, _args: &[&str]) -> anyhow::Result<RunOutput> {
            Ok(RunOutput { stdout: String::new(), stderr: String::new(), code: Some(0) })
        }
    }

    #[test]
    fn dna_has_mx_cap() {
        let runner: Arc<dyn Runner + Send + Sync> = Arc::new(StubRunner);
        let b = MlxBridge::new(runner, None, "mlx_lm.generate".into(), "mlx-community/Qwen2.5-72B-4bit".into(), 32_768).unwrap();
        assert!(b.dna().caps().contains("MX"));
        assert_eq!(b.backend_name(), "mlx-bridge");
    }

    #[test]
    fn pricing_zero_local() {
        let runner: Arc<dyn Runner + Send + Sync> = Arc::new(StubRunner);
        let b = MlxBridge::new(runner, None, "bin".into(), "any".into(), 8192).unwrap();
        assert_eq!(b.pricing_per_mtok(), (0.0, 0.0));
        assert_eq!(b.context_window(), 8192);
    }

    #[test]
    fn flatten_preserves_roles() {
        let msgs = vec![
            Message { role: "user".into(), content: "ping".into() },
        ];
        let p = flatten_messages(&msgs);
        assert!(p.contains("[USER] ping"));
        assert!(p.ends_with("[ASSISTANT] "));
    }
}
