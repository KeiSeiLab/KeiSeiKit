// SPDX-License-Identifier: Apache-2.0
//! Onboarding state-machine: `handle_step` (11-arm FSM match).
//! Helpers → machine_helpers.rs. Tests → machine_tests.rs.

use serde_json::{json, Value};

use crate::error::BuddyError;
use crate::extractor::{
    LlmExtractor, prompt_list, prompt_name, prompt_now_or_later, prompt_propose_sources,
    prompt_schedule, prompt_tone, prompt_topic_specifics, prompt_yes_no, TONES,
};
use crate::machine_helpers::{
    ask_schedule, build_topic_state, clamp_hour, describe_schedule, extract_string, finish_topic,
    format_list, parse_source_selection, str_list,
};
use crate::state::OnboardState;
use crate::transition::StepOutput;

const INTRO: &str = "👋 Привет! Я KeiBuddie — твой персональный AI-компаньон от KeiSei.\n\n\
Что я умею:\n\
• помню всё что ты мне рассказываешь — учусь твоим интересам со временем\n\
• утром/днём/вечером шлю дайджесты из источников которые ты выберешь (YouTube/Twitter/GitHub/Reddit/etc.)\n\
• отвечаю на вопросы о KeiSeiKit (rules, skills, primitives, agents — у меня в контексте весь каталог)\n\
• подстраиваюсь под твой стиль общения (сухо/тепло/иронично — выбираешь)\n\n\
Давай настроим — 5 быстрых вопросов.";

/// Advance the onboarding FSM by one user message.
/// Merge `StepOutput::persona_patch` into the persona blob before the next call.
/// `__topic_state` in the patch tracks the per-topic loop; keep it in blob.
pub async fn handle_step<E: LlmExtractor>(
    state: &OnboardState,
    user_text: &str,
    persona: &Value,
    extractor: &E,
) -> Result<StepOutput, BuddyError> {
    match state {
        OnboardState::Intro => Ok(StepOutput {
            next_state: OnboardState::AskName,
            response_text: format!("{INTRO}\n\n*Шаг 1/5.* Как тебя называть?"),
            persona_patch: json!({}),
        }),

        OnboardState::AskName => {
            let v = extractor.extract(prompt_name(), user_text).await?;
            let name: String = v["name"]
                .as_str()
                .unwrap_or(user_text.trim())
                .chars().take(40).collect();
            Ok(StepOutput {
                next_state: OnboardState::AskTone,
                response_text: format!(
                    "Отлично, *{name}*. Запомнил.\n\n\
                    *Шаг 2/5.* Какой стиль общения тебе ближе? Опиши своими словами — например, \
                    \"по-дружески\", \"сухо и по делу\", \"с иронией\". \
                    Или просто слово: `friendly`, `calm`, `stoic`, `sarcastic`, `professional`."
                ),
                persona_patch: json!({ "name": name }),
            })
        }

        OnboardState::AskTone => {
            let v = extractor.extract(prompt_tone(), user_text).await?;
            let raw = v["tone"].as_str().unwrap_or("").to_lowercase();
            let tone = if TONES.contains(&raw.as_str()) { raw } else { "friendly".to_owned() };
            Ok(StepOutput {
                next_state: OnboardState::AskInterests,
                response_text: format!(
                    "Тон: *{tone}*. Принято.\n\n\
                    *Шаг 3/5.* Какие у тебя интересы? Просто перечисли — \
                    как удобно (через запятую, списком, или одним абзацем)."
                ),
                persona_patch: json!({ "tone": tone }),
            })
        }

        OnboardState::AskInterests => {
            let prompt = prompt_list("interests");
            let v = extractor.extract(&prompt, user_text).await?;
            let interests = str_list(&v["items"]);
            Ok(StepOutput {
                next_state: OnboardState::AskHobbies,
                response_text: format!(
                    "Интересы: {}.\n\n\
                    *Шаг 4/5.* А хобби? Чем конкретно занимаешься в свободное время.",
                    format_list(&interests)
                ),
                persona_patch: json!({ "interests": interests }),
            })
        }

        OnboardState::AskHobbies => step_ask_hobbies(user_text, persona, extractor).await,

        OnboardState::TopicSpecifics => {
            let v = extractor.extract(prompt_topic_specifics(), user_text).await?;
            let specifics = str_list(&v["aspects"]);
            let cur_name = extract_string(&persona["current_topic"], "name");
            Ok(StepOutput {
                next_state: OnboardState::TopicNowLater,
                response_text: format!(
                    "Понял по *{cur_name}*: {}.\n\n\
                    Хочешь *обсудить это сейчас* или *сохранить на потом*?",
                    format_list(&specifics)
                ),
                persona_patch: json!({ "current_topic_specifics": specifics }),
            })
        }

        OnboardState::TopicNowLater => {
            let v = extractor.extract(prompt_now_or_later(), user_text).await?;
            let defer = v["decision"].as_str().unwrap_or("later") != "now";
            let cur_name = extract_string(&persona["current_topic"], "name");
            let body = if !defer { format!("Окей, обсудим *{cur_name}* подробно когда закончим настройку. Запомнил.") }
                       else { format!("Отложил *{cur_name}* на потом.") };
            Ok(StepOutput {
                next_state: OnboardState::TopicResearch,
                response_text: format!("{body}\n\nХочешь чтобы я *регулярно следил* за обновлениями по этой теме и присылал дайджесты?"),
                persona_patch: json!({ "current_topic_defer": defer }),
            })
        }

        OnboardState::TopicResearch => step_topic_research(user_text, persona, extractor).await,

        OnboardState::TopicSources => {
            let cur = &persona["current_topic"];
            let (cur_name, kind_interest) = (extract_string(cur, "name"), extract_string(cur, "kind").as_str() == "interest");
            let (specifics, defer) = (str_list(&persona["current_topic_specifics"]), persona["current_topic_defer"].as_bool().unwrap_or(true));
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
            Ok(StepOutput {
                next_state: OnboardState::Ready,
                response_text: format!(
                    "Готово! ✨ Я настроен.\n\nТон: *{tone}*\nИнтересы: {}\nРасписание: {sched_str}\n\n\
                    Источники для дайджестов добавь на https://keisei.app/keibuddy \
                    (10 платформ — YouTube, Twitter, GitHub и др.).\n\n\
                    Теперь можешь писать мне о чём угодно — буду помнить и подстраиваться. \
                    Скажи что-нибудь!",
                    format_list(&interests)
                ),
                persona_patch: json!({
                    "schedule": { "morning": morning, "evening": evening, "timezone": tz }
                }),
            })
        }

        OnboardState::Ready => Ok(StepOutput {
            next_state: OnboardState::Ready,
            response_text: String::new(),
            persona_patch: json!({}),
        }),
    }
}

// ─── arm helpers ─────────────────────────────────────────────────────────────

async fn step_ask_hobbies<E: LlmExtractor>(
    user_text: &str,
    persona: &Value,
    extractor: &E,
) -> Result<StepOutput, BuddyError> {
    let prompt = prompt_list("hobbies");
    let v = extractor.extract(&prompt, user_text).await?;
    let hobbies = str_list(&v["items"]);
    let interests = str_list(&persona["interests"]);
    let queue: Vec<Value> = interests
        .iter().map(|n| json!({"name": n, "kind": "interest"}))
        .chain(hobbies.iter().map(|n| json!({"name": n, "kind": "hobby"})))
        .collect();
    if queue.is_empty() {
        return Ok(ask_schedule(
            &json!({ "hobbies": hobbies }),
            &format!("Хобби: {}.", format_list(&hobbies)),
        ));
    }
    let next_topic = queue[0].clone();
    let topic_name = next_topic["name"].as_str().unwrap_or("?").to_owned();
    let ts = build_topic_state(&queue[1..], 0, json!({}));
    let mut patch = ts;
    patch["hobbies"] = json!(hobbies);
    patch["current_topic"] = next_topic;
    Ok(StepOutput {
        next_state: OnboardState::TopicSpecifics,
        response_text: format!(
            "Хобби: {}.\n\nТеперь разберём по темам. Поехали — сначала *{topic_name}*.\n\n\
            *Что именно* в этой теме тебе интересно? Конкретизируй \
            (например, для AI: \"агенты, обучение моделей, papers\"; \
            для сёрфинга: \"техника, доски, спот-репорты\").",
            format_list(&hobbies)
        ),
        persona_patch: patch,
    })
}

async fn step_topic_research<E: LlmExtractor>(
    user_text: &str,
    persona: &Value,
    extractor: &E,
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
    // TODO(phase2): proposeTopicSources — real production wires OpenAiExtractor here.
    // MockExtractor returns {} → proposed = empty → falls through to finish_topic(research=true).
    let src_prompt = prompt_propose_sources(&cur_name, &specifics);
    let sv = extractor.extract(&src_prompt, "").await?;
    let proposed: Vec<Value> = sv["sources"].as_array().cloned().unwrap_or_default();
    if proposed.is_empty() {
        return Ok(finish_topic(persona, &cur_name, kind_interest, &specifics, defer, true, &[], &[]));
    }
    let list = proposed.iter().enumerate()
        .map(|(i, s)| format!(
            "{}. `{}` *{}* — {}",
            i + 1,
            s["platform"].as_str().unwrap_or("?"),
            s["handle_or_url"].as_str().unwrap_or("?"),
            s["why"].as_str().unwrap_or("")
        ))
        .collect::<Vec<_>>().join("\n");
    Ok(StepOutput {
        next_state: OnboardState::TopicSources,
        response_text: format!(
            "Предлагаю источники по *{cur_name}*:\n\n{list}\n\n\
            Какие добавить? Напиши номера через запятую (`1,3,5`), `все`, или `нет`. \
            Можешь добавить свои — просто напиши \"плюс <платформа> <handle>\"."
        ),
        persona_patch: json!({ "current_topic_proposed": proposed }),
    })
}

// Tests live in machine_tests.rs (Constructor Pattern: separate test module).
#[cfg(test)]
#[path = "machine_tests.rs"]
mod machine_tests;
