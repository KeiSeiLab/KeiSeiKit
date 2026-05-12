// SPDX-License-Identifier: Apache-2.0
//! Language-aware helpers for `machine::handle_step`.
//!
//! Extracted from machine_helpers.rs (Constructor Pattern: one file ≤ 200 LOC).
//! Covers: language selection, migration back-fill, schedule/ready response builders,
//! and the async TopicResearch arm (needs LlmExtractor).

use serde_json::{json, Value};

use crate::error::BuddyError;
use crate::extractor::{LlmExtractor, prompt_yes_no};
use crate::machine_helpers::{extract_string, finish_topic, format_list, str_list};
use crate::state::OnboardState;
use crate::strings::{Lang, Strings};
use crate::transition::StepOutput;

// ─── language selection ───────────────────────────────────────────────────────

/// Handle the `AskLanguage` state.
///
/// Returns `Some(StepOutput)` when the input is a recognised language choice
/// (advances to AskName).  Returns `None` on invalid input (caller loops).
pub(crate) fn handle_ask_language(user_text: &str) -> Option<StepOutput> {
    let lang = Lang::from_user_choice(user_text)?;
    Some(StepOutput {
        next_state: OnboardState::AskName,
        response_text: format!(
            "{}\n\n*Step 1/5.* {}",
            Strings::language_set(lang),
            Strings::ask_name(lang)
        ),
        persona_patch: json!({ "language": lang.code() }),
    })
}

/// Back-compat migration: inject `"language": "ru"` when the persona has no
/// language key, so threads started before this commit keep Russian prompts.
pub(crate) fn backfill_language(persona: &Value) -> Option<Value> {
    if persona.get("language").is_none() {
        Some(json!({ "language": "ru" }))
    } else {
        None
    }
}

// ─── schedule helpers ─────────────────────────────────────────────────────────

pub(crate) fn ask_schedule_lang(extra_patch: &Value, prefix: &str, lang: Lang) -> StepOutput {
    let intro = if prefix.is_empty() {
        String::new()
    } else {
        format!("{prefix}\n\n")
    };
    StepOutput {
        next_state: OnboardState::AskSchedule,
        response_text: format!("{intro}{}", Strings::ask_schedule(lang)),
        persona_patch: extra_patch.clone(),
    }
}

// ─── ready-response builder ───────────────────────────────────────────────────

pub(crate) fn build_ready_response(
    lang: Lang,
    tone: &str,
    interests: &[String],
    sched_str: &str,
    morning: Option<u8>,
    evening: Option<u8>,
    tz: &str,
) -> StepOutput {
    let ready = Strings::ready(lang);
    let (tone_lbl, int_lbl, sched_lbl) = match lang {
        Lang::En => ("Tone", "Interests", "Schedule"),
        Lang::Ru => ("Тон", "Интересы", "Расписание"),
    };
    let sources_hint = match lang {
        Lang::En => "Add digest sources at https://keisei.app/keibuddy \
            (10 platforms — YouTube, Twitter, GitHub, and more).\n\n\
            Now you can write to me about anything — I'll remember and adapt. Say something!",
        Lang::Ru => "Источники для дайджестов добавь на https://keisei.app/keibuddy \
            (10 платформ — YouTube, Twitter, GitHub и др.).\n\n\
            Теперь можешь писать мне о чём угодно — буду помнить и подстраиваться. \
            Скажи что-нибудь!",
    };
    StepOutput {
        next_state: OnboardState::Ready,
        response_text: format!(
            "{ready}\n\n{tone_lbl}: *{tone}*\n{int_lbl}: {}\n{sched_lbl}: {sched_str}\n\n{sources_hint}",
            format_list(interests)
        ),
        persona_patch: json!({
            "schedule": { "morning": morning, "evening": evening, "timezone": tz }
        }),
    }
}

// ─── TopicResearch arm ────────────────────────────────────────────────────────

/// Prompt used to propose sources to the user. Returns `{name, url, why}` triples.
fn propose_sources_prompt(topic: &str, lang: Lang) -> String {
    let lang_hint = match lang {
        Lang::En => "Respond in English.",
        Lang::Ru => "Respond in Russian.",
    };
    format!(
        "You are a research-sources proposer. The user wants to follow a topic.\n\
         Output a JSON object with one field \"sources\": an array of 3-5 objects,\n\
         each with {{\"name\":\"...\",\"url\":\"...\",\"why\":\"...\"}}.\n\
         Pick concrete, reputable sources for the topic.\n\
         URLs must be real, well-known site root URLs.\n\
         {lang_hint} Output ONLY the JSON, no prose, no markdown fences.\n\
         Topic: {topic}"
    )
}

/// TopicResearch arm: gather research consent and propose sources via LLM.
pub(crate) async fn step_topic_research<E: LlmExtractor>(
    user_text: &str,
    persona: &Value,
    extractor: &E,
    lang: Lang,
) -> Result<StepOutput, BuddyError> {
    let v = extractor.extract(prompt_yes_no(), user_text).await?;
    let want_research = v["yes"].as_bool().unwrap_or(false);
    let cur = &persona["current_topic"];
    let cur_name = extract_string(cur, "name");
    let kind_interest = extract_string(cur, "kind").as_str() == "interest";
    let specifics = str_list(&persona["current_topic_specifics"]);
    let defer = persona["current_topic_defer"].as_bool().unwrap_or(true);
    if !want_research {
        return Ok(finish_topic(persona, &cur_name, kind_interest, &specifics, defer, false, &[], &[]));
    }
    let src_prompt = propose_sources_prompt(&cur_name, lang);
    let sv = match extractor.extract(&src_prompt, "").await {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("propose_sources LLM call failed for {cur_name:?}: {e}");
            Value::Object(serde_json::Map::new())
        }
    };
    let proposed: Vec<Value> = sv["sources"].as_array().cloned().unwrap_or_default();
    if proposed.is_empty() {
        let fallback = build_sources_fallback(lang);
        return Ok(StepOutput {
            next_state: OnboardState::TopicSources,
            response_text: fallback,
            persona_patch: json!({ "current_topic_proposed": [] }),
        });
    }
    let list = build_sources_list(&proposed);
    let propose_lbl = match lang {
        Lang::En => "Proposed sources for",
        Lang::Ru => "Предлагаю источники по",
    };
    Ok(StepOutput {
        next_state: OnboardState::TopicSources,
        response_text: format!(
            "{propose_lbl} *{cur_name}*:\n\n{list}\n\n{}",
            Strings::topic_sources_intro(lang)
        ),
        persona_patch: json!({ "current_topic_proposed": proposed }),
    })
}

fn build_sources_list(sources: &[Value]) -> String {
    sources.iter().enumerate()
        .map(|(i, s)| {
            let name = s["name"].as_str().unwrap_or("?");
            let url = s["url"].as_str().unwrap_or("?");
            let why = s["why"].as_str().unwrap_or("");
            format!("{}. *{name}* — {url}\n   _{why}_", i + 1)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_sources_fallback(lang: Lang) -> String {
    match lang {
        Lang::En => "I couldn't propose sources automatically. \
            Could you suggest one yourself? \
            (e.g. \"plus rss https://example.com/feed\")"
            .to_owned(),
        Lang::Ru => "Не смог подобрать источники автоматически. \
            Можешь предложить сам? \
            (например, \"плюс rss https://example.com/feed\")"
            .to_owned(),
    }
}

