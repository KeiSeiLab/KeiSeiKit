// SPDX-License-Identifier: Apache-2.0
//! Onboarding state-machine enum.
//!
//! Ported from `keisei-marketplace/src/lib/keibuddy/chat-onboard.ts`.
//! Each variant corresponds to one `Step` in the TypeScript source.
//! Transitions are stubs; real logic arrives in a follow-up task.

use serde::{Deserialize, Serialize};

use crate::transition::TransitionInput;

/// 11-state onboarding finite-state machine.
///
/// Mirrors the TypeScript `Step` union type exactly:
/// `intro | ask_name | ask_tone | ask_interests | ask_hobbies |
///  topic_specifics | topic_now_later | topic_research |
///  topic_sources | ask_schedule | ready`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardState {
    /// Initial greeting — bot explains itself.
    Intro,
    /// Collecting user's display name.
    AskName,
    /// Collecting preferred communication tone.
    AskTone,
    /// Collecting list of interests.
    AskInterests,
    /// Collecting list of hobbies.
    AskHobbies,
    /// Per-topic: "what specifically interests you here?"
    TopicSpecifics,
    /// Per-topic: "discuss now or save for later?"
    TopicNowLater,
    /// Per-topic: "want ongoing source monitoring?"
    TopicResearch,
    /// Per-topic: "here are proposed sources, which to add?"
    TopicSources,
    /// Collecting digest schedule (morning/evening hours + timezone).
    AskSchedule,
    /// Onboarding complete; regular conversation mode.
    Ready,
}

impl OnboardState {
    /// Advance to the next state given user input.
    ///
    /// **Stub** — returns `self.clone()` until transition logic is ported.
    /// Real implementation will extract fields via kei-cortex and follow
    /// the per-topic queue logic from `chat-onboard.ts::handleStep`.
    pub fn next(&self, _input: &TransitionInput) -> Self {
        // TODO: port transition logic from chat-onboard.ts::handleStep
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: every variant round-trips through JSON serialisation.
    #[test]
    fn all_variants_serde_roundtrip() {
        let variants = [
            OnboardState::Intro,
            OnboardState::AskName,
            OnboardState::AskTone,
            OnboardState::AskInterests,
            OnboardState::AskHobbies,
            OnboardState::TopicSpecifics,
            OnboardState::TopicNowLater,
            OnboardState::TopicResearch,
            OnboardState::TopicSources,
            OnboardState::AskSchedule,
            OnboardState::Ready,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant)
                .unwrap_or_else(|e| panic!("serialize {:?}: {e}", variant));
            let back: OnboardState = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("deserialize {:?} from {json:?}: {e}", variant));
            assert_eq!(variant, &back, "round-trip failed for {:?}", variant);
        }
    }
}
