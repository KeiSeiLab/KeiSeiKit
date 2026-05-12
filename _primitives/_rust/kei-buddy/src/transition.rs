// SPDX-License-Identifier: Apache-2.0
//! Input struct for state-machine transitions.
//!
//! `TransitionInput` carries the raw user message and any structured
//! fields that an LLM extractor has already parsed from it.
//! In the scaffold phase the `extracted_fields` value is always `null`.

use serde::{Deserialize, Serialize};

/// Input to `OnboardState::next()`.
///
/// `user_text` is the verbatim Telegram message body.
/// `extracted_fields` will hold the result of an LLM-extract call
/// (e.g. `extractName`, `extractTone`, `extractList` from
/// `chat-onboard-extract.ts`) once kei-cortex integration is wired.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionInput {
    /// Raw message text from the user, UTF-8, already trimmed.
    pub user_text: String,

    /// Structured fields extracted by an LLM call.
    /// `serde_json::Value::Null` until kei-cortex integration is added.
    pub extracted_fields: serde_json::Value,
}
