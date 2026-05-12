// SPDX-License-Identifier: Apache-2.0
//! Output struct for `machine::handle_step`.
//!
//! `TransitionInput` is removed — callers pass `(OnboardState, &str, extractor)` directly.
//! `StepOutput` carries everything the webhook layer needs after one state step.

use serde::{Deserialize, Serialize};

use crate::state::OnboardState;

/// Result of a single state-machine step.
///
/// Mirrors the TypeScript `NextMsg` type from `chat-onboard.ts`:
///   `{ reply, next, scratchpad, finalize? }`
///
/// `persona_patch` is a partial JSON object the caller must merge into
/// the persistent persona blob. An empty object `{}` means no change.
///
/// `__topic_state` key inside `persona_patch` carries queue/index for
/// the topic decomposition loop — the caller must pass it back in on
/// the next turn by including it in the persona blob they load.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepOutput {
    /// Next state to persist.
    pub next_state: OnboardState,
    /// Bot reply text (Markdown, Russian).
    pub response_text: String,
    /// Partial persona update; merge into existing persona blob.
    pub persona_patch: serde_json::Value,
}
