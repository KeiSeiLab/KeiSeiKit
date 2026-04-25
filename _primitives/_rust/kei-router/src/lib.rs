//! kei-router — two routing concerns under one crate:
//!
//! 1. **NL query → tool-call dispatch** (LBM port; original purpose).
//!    Public API: [`Router::new`] / [`Router::route`] / [`Router::add_dynamic`].
//!
//! 2. **Multi-provider LLM abstraction** (v0.40 Wave 32).
//!    Public API: [`LlmRouter`] / [`Provider`] trait / per-provider impls.
//!    See INTEGRATION.md for orchestrator wiring guide.
//!
//! Constructor Pattern: one cube = one file. Tool router and LLM router are
//! independent cubes — they share crate metadata only.

// (1) tool-call routing (existing — unchanged)
pub mod extract;
pub mod keywords;
pub mod kw_tables;
pub mod router;
pub mod rules;

pub use extract::{extract_params, Extracted};
pub use router::{Method, RouteResult, Router};
pub use rules::{DynRule, KeywordRule};

// (2) LLM provider abstraction (v0.40 Wave 32)
pub mod provider;
pub mod providers;
pub mod llm_router;

pub use llm_router::LlmRouter;
pub use provider::{Error as LlmError, Message, Provider, StreamEvent, Tool};
pub use providers::{AnthropicProvider, KimiProvider, OpenAiProvider};
