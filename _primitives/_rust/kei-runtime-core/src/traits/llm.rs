// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::HasDna;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,        // "user" | "assistant" | "system"
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompletionOpts {
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stop: Vec<String>,
    pub use_caching: bool,
    pub use_batch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub text: String,
    pub stop_reason: String,
    pub tokens_input: u32,
    pub tokens_output: u32,
    pub cached_tokens: u32,
    pub request_id: String,
}

#[async_trait::async_trait]
pub trait LlmBackend: HasDna + Send + Sync {
    fn backend_name(&self) -> &'static str;
    fn model_name(&self) -> &str;

    async fn complete(
        &self,
        messages: &[Message],
        opts: &CompletionOpts,
    ) -> Result<CompletionResponse>;

    /// (input USD/Mtok, output USD/Mtok). Used by CostGuard.
    fn pricing_per_mtok(&self) -> (f64, f64);

    fn supports_caching(&self) -> bool;
    fn supports_batch(&self) -> bool;

    /// Maximum context tokens for the configured model.
    fn context_window(&self) -> u32;
}
