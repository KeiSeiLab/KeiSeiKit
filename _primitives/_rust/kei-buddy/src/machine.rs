// SPDX-License-Identifier: Apache-2.0
//! Onboarding state-machine: `handle_step` (12-arm FSM match).
//! Helpers → machine_helpers.rs. Tests → machine_tests.rs.
//!
//! LOC exception: file is allowed up to 260 LOC (Constructor Pattern §thresholds).

use serde_json::{json, Value};

use crate::error::BuddyError;
use crate::extractor::{
    LlmExtractor, prompt_list, prompt_name, prompt_now_or_later,
    prompt_schedule, prompt_tone, prompt_topic_specifics, TONES,
};
use crate::machine_helpers::{
    build_topic_state, clamp_hour, describe_schedule, extract_string, finish_topic,
    format_list, parse_source_selection, str_list,
};
use crate::machine_lang::{
    ask_schedule_lang, backfill_language, build_ready_response, handle_ask_language,
    step_topic_research,
};
use crate::state::OnboardState;
use crate::strings::{Lang, Strings};
use crate::transition::StepOutput;

/// Advance the onboarding FSM by one user message.
/// Merge `StepOutput::persona_patch` into the persona blob before the next call.
/// `__topic_state` in the patch tracks the per-topic loop; keep it in blob.
pub async fn handle_step<E: LlmExtractor>(
    state: &OnboardState,
    user_text: &str,
    persona: &Value,
    extractor: &E,
) -> Result<StepOutput, BuddyError> {
    // Back-compat migration: chats that started before language selection was
    // added will have no `language` key.  Treat them as Russian so existing
    // in-progress threads keep their original language.
    // Skipped for Intro / AskLanguage (language not yet chosen) and Ready
    // (onboarding complete, no need to persist migration patch).
    let migration_patch = match state {
        OnboardState::Intro | OnboardState::AskLanguage | OnboardState::Ready => None,
        _ => backfill_language(persona),
    };
    let lang = Lang::from_persona(persona);

    let mut out = step_dispatch(state, user_text, persona, extractor, lang).await?;

    // Merge migration patch when present.
    if let Some(mp) = migration_patch {
        if let (Some(obj), Some(mp_obj)) = (
            out.persona_patch.as_object_mut(),
            mp.as_object(),
        ) {
            for (k, v) in mp_obj {
                obj.entry(k).or_insert_with(|| v.clone());
            }
        }
    }
    Ok(out)
}

async fn step_dispatch<E: LlmExtractor>(
    state: &OnboardState,
    user_text: &str,
    persona: &Value,
    extractor: &E,
    lang: Lang,
) -> Result<StepOutput, BuddyError> {
    match state {
        OnboardState::Intro => Ok(StepOutput {
            next_state: OnboardState::AskLanguage,
            response_text: Strings::intro_ask_language().to_owned(),
            persona_patch: json!({}),
        }),

        OnboardState::AskLanguage => Ok(handle_ask_language(user_text).unwrap_or_else(|| {
            StepOutput {
                next_state: OnboardState::AskLanguage,
                response_text: Strings::invalid_language().to_owned(),
                persona_patch: json!({}),
            }
        })),

        OnboardState::AskName => {
            let v = extractor.extract(prompt_name(), user_text).await?;
            let name: String = v["name"]
                .as_str()
                .unwrap_or(user_text.trim())
                .chars().take(40).collect();
            let step2 = match lang { Lang::En => "Step 2/5.", Lang::Ru => "Шаг 2/5." };
            let ok = match lang { Lang::En => "Got it,", Lang::Ru => "Отлично," };
            Ok(StepOutput {
                next_state: OnboardState::AskTone,
                response_text: format!(
                    "{ok} *{name}*.\n\n*{step2}* {}",
                    Strings::ask_tone(lang)
                ),
                persona_patch: json!({ "name": name }),
            })
        }

        OnboardState::AskTone => {
            let v = extractor.extract(prompt_tone(), user_text).await?;
            let raw = v["tone"].as_str().unwrap_or("").to_lowercase();
            let tone = if TONES.contains(&raw.as_str()) { raw } else { "friendly".to_owned() };
            let step3 = match lang { Lang::En => "Step 3/5.", Lang::Ru => "Шаг 3/5." };
            let ok = match lang { Lang::En => "Tone:", Lang::Ru => "Тон:" };
            Ok(StepOutput {
                next_state: OnboardState::AskInterests,
                response_text: format!(
                    "{ok} *{tone}*.\n\n*{step3}* {}",
                    Strings::ask_interests(lang)
                ),
                persona_patch: json!({ "tone": tone }),
            })
        }

        OnboardState::AskInterests => {
            let prompt = prompt_list("interests");
            let v = extractor.extract(&prompt, user_text).await?;
            let interests = str_list(&v["items"]);
            let step4 = match lang { Lang::En => "Step 4/5.", Lang::Ru => "Шаг 4/5." };
            let label = match lang { Lang::En => "Interests:", Lang::Ru => "Интересы:" };
            Ok(StepOutput {
                next_state: OnboardState::AskHobbies,
                response_text: format!(
                    "{label} {}.\n\n*{step4}* {}",
                    format_list(&interests),
                    Strings::ask_hobbies(lang)
                ),
                persona_patch: json!({ "interests": interests }),
            })
        }

        OnboardState::AskHobbies => step_ask_hobbies(user_text, persona, extractor, lang).await,

        OnboardState::TopicSpecifics => {
            let v = extractor.extract(prompt_topic_specifics(), user_text).await?;
            let specifics = str_list(&v["aspects"]);
            let cur_name = extract_string(&persona["current_topic"], "name");
            let understood = match lang { Lang::En => "Got it on", Lang::Ru => "Понял по" };
            Ok(StepOutput {
                next_state: OnboardState::TopicNowLater,
                response_text: format!(
                    "{understood} *{cur_name}*: {}.\n\n{}",
                    format_list(&specifics),
                    Strings::topic_now_later(lang)
                ),
                persona_patch: json!({ "current_topic_specifics": specifics }),
            })
        }

        OnboardState::TopicNowLater => {
            let v = extractor.extract(prompt_now_or_later(), user_text).await?;
            let defer = v["decision"].as_str().unwrap_or("later") != "now";
            let cur_name = extract_string(&persona["current_topic"], "name");
            let body = build_now_later_msg(lang, &cur_name, defer);
            Ok(StepOutput {
                next_state: OnboardState::TopicResearch,
                response_text: format!("{body}\n\n{}", Strings::topic_research(lang)),
                persona_patch: json!({ "current_topic_defer": defer }),
            })
        }

        OnboardState::TopicResearch => step_topic_research(user_text, persona, extractor, lang).await,

        OnboardState::TopicSources => {
            let cur = &persona["current_topic"];
            let cur_name = extract_string(cur, "name");
            let kind_interest = extract_string(cur, "kind").as_str() == "interest";
            let specifics = str_list(&persona["current_topic_specifics"]);
            let defer = persona["current_topic_defer"].as_bool().unwrap_or(true);
            let proposed: Vec<Value> = persona["current_topic_proposed"].as_array().cloned().unwrap_or_default();
            let picked = parse_source_selection(user_text, proposed.len());
            Ok(finish_topic(persona, &cur_name, kind_interest, &specifics, defer, true, &proposed, &picked))
        }

        OnboardState::AskSchedule => {
            let v = extractor.extract(prompt_schedule(), user_text).await?;
            let morning = clamp_hour(&v["morning"]);
            let evening = clamp_hour(&v["evening"]);
            let tz = v["timezone"].as_str().filter(|s| s.len() <= 64).unwrap_or("UTC").to_owned();
            let tone = persona["tone"].as_str().unwrap_or("friendly");
            let interests = str_list(&persona["interests"]);
            let sched_str = describe_schedule(morning, evening, &tz);
            Ok(build_ready_response(lang, tone, &interests, &sched_str, morning, evening, &tz))
        }

        OnboardState::Ready => Ok(StepOutput {
            next_state: OnboardState::Ready,
            response_text: String::new(),
            persona_patch: json!({}),
        }),
    }
}

// ─── arm helpers ─────────────────────────────────────────────────────────────

fn build_now_later_msg(lang: Lang, cur_name: &str, defer: bool) -> String {
    match (lang, defer) {
        (Lang::En, false) => format!("Ok, we'll discuss *{cur_name}* in detail after setup. Noted."),
        (Lang::En, true)  => format!("Saved *{cur_name}* for later."),
        (Lang::Ru, false) => format!("Окей, обсудим *{cur_name}* подробно когда закончим настройку. Запомнил."),
        (Lang::Ru, true)  => format!("Отложил *{cur_name}* на потом."),
    }
}

async fn step_ask_hobbies<E: LlmExtractor>(
    user_text: &str,
    persona: &Value,
    extractor: &E,
    lang: Lang,
) -> Result<StepOutput, BuddyError> {
    let prompt = prompt_list("hobbies");
    let v = extractor.extract(&prompt, user_text).await?;
    let hobbies = str_list(&v["items"]);
    let interests = str_list(&persona["interests"]);
    let queue: Vec<Value> = interests
        .iter().map(|n| json!({"name": n, "kind": "interest"}))
        .chain(hobbies.iter().map(|n| json!({"name": n, "kind": "hobby"})))
        .collect();
    let hobbies_label = match lang { Lang::En => "Hobbies:", Lang::Ru => "Хобби:" };
    if queue.is_empty() {
        return Ok(ask_schedule_lang(
            &json!({ "hobbies": hobbies }),
            &format!("{hobbies_label} {}.", format_list(&hobbies)),
            lang,
        ));
    }
    let next_topic = queue[0].clone();
    let topic_name = next_topic["name"].as_str().unwrap_or("?").to_owned();
    let ts = build_topic_state(&queue[1..], 0, json!({}));
    let mut patch = ts;
    patch["hobbies"] = json!(hobbies);
    patch["current_topic"] = next_topic;
    let prefix_str = Strings::topic_specifics_prefix(lang);
    let question_str = Strings::topic_specifics_question(lang);
    Ok(StepOutput {
        next_state: OnboardState::TopicSpecifics,
        response_text: format!(
            "{hobbies_label} {}.\n\n{prefix_str} *{topic_name}*.\n\n{question_str}",
            format_list(&hobbies)
        ),
        persona_patch: patch,
    })
}

// Tests live in machine_tests.rs (Constructor Pattern: separate test module).
#[cfg(test)]
#[path = "machine_tests.rs"]
mod machine_tests;
