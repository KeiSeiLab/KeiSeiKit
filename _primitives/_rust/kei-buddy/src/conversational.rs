// SPDX-License-Identifier: Apache-2.0
//! LLM-driven conversational onboarding step (replaces rigid FSM after Intro/AskLanguage).
//! Constructor Pattern: one responsibility — one LLM call, one structured response.

use serde_json::{json, Value};

use crate::{
    error::BuddyError,
    extractor::LlmExtractor,
    retrieval::RetrievalContext,
    state::OnboardState,
    strings::Lang,
    transition::StepOutput,
};

const PROMPT_TEMPLATE: &str = concat!(
    "You are KeiBuddy, a personal-assistant chatbot doing first-meeting onboarding.\n",
    "Be a warm, natural conversationalist. Reply in the user's language ({lang}).\n",
    "Don't act like a survey form — extract info from natural conversation.\n\n",
    "USER PROFILE (so far — fields with values are KNOWN, don't re-ask):\n{persona}\n\n",
    "RECENT CONVERSATION (latest first; \"u:\" user, \"b:\" you):\n{history}\n\n",
    "RELATED KNOWLEDGE (relevant atoms from user's graph):\n{atoms}\n\n",
    "USER JUST SAID:\n\"{text}\"\n\n",
    "YOUR JOB: Update persona fields you newly learned. Decide what to ask next.\n\n",
    "Output ONLY this JSON object, no prose, no markdown fences:\n",
    "{{\"slot_updates\":{{\"name\":\"<str>\",\"tone\":\"<str>\",\"interests\":[],",
    "\"hobbies\":[],\"schedule\":{{}},\"language\":\"<str>\"}},",
    "\"response_text\":\"<reply in {lang}, ≤300 chars>\",",
    "\"done\":false,\"focus\":\"name|tone|interests|hobbies|topics|schedule|free|done\"}}"
);

/// Drive one turn of free-form onboarding with a single LLM call.
pub async fn conversational_step<E: LlmExtractor + ?Sized>(
    state: &OnboardState,
    persona: &Value,
    context: &RetrievalContext,
    user_text: &str,
    extractor: &E,
    lang: Lang,
) -> Result<StepOutput, BuddyError> {
    let prompt = build_prompt(persona, context, user_text, lang);
    let raw = extractor.extract(&prompt, "").await?;
    match parse_llm_response(&raw, state) {
        Some(out) => Ok(out),
        None => Ok(fallback_output(state, lang)),
    }
}

fn build_prompt(persona: &Value, ctx: &RetrievalContext, text: &str, lang: Lang) -> String {
    let lc = lang.code();
    let ps = serde_json::to_string_pretty(persona).unwrap_or_default();
    let hs = if ctx.history.is_empty() { "(no prior messages)".into() } else { ctx.history.join("\n") };
    let atoms = if ctx.atoms.is_empty() { "(none)".into() } else { ctx.atoms.join("\n") };
    PROMPT_TEMPLATE
        .replace("{lang}", lc)
        .replace("{persona}", &ps)
        .replace("{history}", &hs)
        .replace("{atoms}", &atoms)
        .replace("{text}", text)
}

fn parse_llm_response(raw: &Value, current: &OnboardState) -> Option<StepOutput> {
    let obj = raw.as_object()?;
    let response_text = obj.get("response_text")?.as_str()?.to_owned();
    if response_text.is_empty() { return None; }
    let done = obj.get("done").and_then(|v| v.as_bool()).unwrap_or(false);
    let focus = obj.get("focus").and_then(|v| v.as_str()).unwrap_or("free");
    let slot_updates = obj.get("slot_updates").cloned().unwrap_or_else(|| json!({}));
    let next_state = if done { OnboardState::Ready } else { focus_to_state(focus, current) };
    Some(StepOutput { next_state, response_text, persona_patch: slot_updates })
}

fn focus_to_state(focus: &str, current: &OnboardState) -> OnboardState {
    match focus {
        "name"     => OnboardState::AskName,
        "tone"     => OnboardState::AskTone,
        "interests"=> OnboardState::AskInterests,
        "hobbies"  => OnboardState::AskHobbies,
        "topics"   => OnboardState::TopicSpecifics,
        "schedule" => OnboardState::AskSchedule,
        "done"     => OnboardState::Ready,
        _          => current.clone(),
    }
}

fn fallback_output(state: &OnboardState, lang: Lang) -> StepOutput {
    let text = match lang {
        Lang::Ru => "Хм, дай-ка подумаю — можешь перефразировать?",
        Lang::En => "Hmm, let me think — could you rephrase?",
    };
    StepOutput { next_state: state.clone(), response_text: text.to_owned(), persona_patch: json!({}) }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::MockExtractor;
    use serde_json::json;

    fn empty_ctx() -> RetrievalContext {
        RetrievalContext { history: vec![], atoms: vec![] }
    }

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    #[test]
    fn parses_well_formed_llm_response() {
        rt().block_on(async {
            let mock = MockExtractor::new(json!({
                "slot_updates": { "name": "Alice" },
                "response_text": "Nice to meet you! What are your interests?",
                "done": false,
                "focus": "interests"
            }));
            let out = conversational_step(
                &OnboardState::AskName, &json!({}), &empty_ctx(),
                "I'm Alice", &mock, Lang::En,
            ).await.unwrap();
            assert_eq!(out.next_state, OnboardState::AskInterests);
            assert!(!out.response_text.is_empty());
            assert_eq!(out.persona_patch["name"].as_str(), Some("Alice"));
        });
    }

    #[test]
    fn done_true_transitions_to_ready() {
        rt().block_on(async {
            let mock = MockExtractor::new(json!({
                "slot_updates": {},
                "response_text": "All set!",
                "done": true,
                "focus": "done"
            }));
            let out = conversational_step(
                &OnboardState::AskSchedule, &json!({}), &empty_ctx(),
                "mornings at 9", &mock, Lang::En,
            ).await.unwrap();
            assert_eq!(out.next_state, OnboardState::Ready);
        });
    }

    #[test]
    fn invalid_json_falls_back_gracefully() {
        rt().block_on(async {
            let mock = MockExtractor::new(json!("not a json object at all"));
            let out = conversational_step(
                &OnboardState::AskInterests, &json!({}), &empty_ctx(),
                "hello", &mock, Lang::En,
            ).await.unwrap();
            assert_eq!(out.next_state, OnboardState::AskInterests);
            assert!(!out.response_text.is_empty());
        });
    }
}
