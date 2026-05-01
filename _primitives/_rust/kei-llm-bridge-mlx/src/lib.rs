// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! kei-llm-bridge-mlx — bridges kei-llm-mlx → LlmBackend.
//! Apple Silicon only — non-Mac platforms get a runtime error.

pub mod bridge;
pub mod error;

pub use bridge::MlxBridge;
pub use error::{Error, Result};
