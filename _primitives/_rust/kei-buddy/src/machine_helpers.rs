// SPDX-License-Identifier: Apache-2.0
//! Pure helper functions for `machine::handle_step`.
//!
//! Constructor Pattern split: helpers extracted so `machine.rs` stays
//! within its 250-LOC exception budget.

use serde_json::{json, Value};

use crate::state::OnboardState;
use crate::transition::StepOutput;

// ─── string helpers ───────────────────────────────────────────────────────────

pub(crate) fn format_list(items: &[String]) -> String {
    if items.is_empty() {
        return "_не указано_".to_owned();
    }
    items.iter().map(|i| format!("`{i}`")).collect::<Vec<_>>().join(", ")
}

pub(crate) fn str_list(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_owned)).collect())
        .unwrap_or_default()
}

pub(crate) fn extract_string(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or("").to_owned()
}

// ─── topic-state helpers ──────────────────────────────────────────────────────

pub(crate) fn topic_queue(persona: &Value) -> Vec<Value> {
    persona["__topic_state"]["queue"]
        .as_array()
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn topic_index(persona: &Value) -> usize {
    persona["__topic_state"]["index"].as_u64().unwrap_or(0) as usize
}

pub(crate) fn build_topic_state(queue: &[Value], index: usize, extra: Value) -> Value {
    let mut obj = extra.as_object().cloned().unwrap_or_default();
    obj.insert("queue".to_owned(), json!(queue));
    obj.insert("index".to_owned(), json!(index));
    json!({ "__topic_state": obj })
}

// ─── source-selection parser ──────────────────────────────────────────────────

pub(crate) fn parse_source_selection(text: &str, total: usize) -> Vec<usize> {
    let lower = text.trim().to_lowercase();
    if ["all", "все", "yes", "да"].contains(&lower.as_str()) {
        return (0..total).collect();
    }
    if ["none", "нет", "skip", "пропусти"].contains(&lower.as_str()) {
        return vec![];
    }
    let mut seen = std::collections::HashSet::new();
    let mut out = vec![];
    for part in text.split(|c: char| c == ',' || c == ';' || c.is_whitespace()) {
        if let Ok(n) = part.trim().parse::<usize>() {
            let idx = n.saturating_sub(1);
            if idx < total && seen.insert(idx) {
                out.push(idx);
            }
        }
    }
    out
}

// ─── schedule helpers ─────────────────────────────────────────────────────────

pub(crate) fn clamp_hour(v: &Value) -> Option<u8> {
    match v {
        Value::Number(n) => n.as_u64().filter(|&h| h <= 23).map(|h| h as u8),
        Value::String(s) => s.parse::<u64>().ok().filter(|&h| h <= 23).map(|h| h as u8),
        _ => None,
    }
}

pub(crate) fn describe_schedule(morning: Option<u8>, evening: Option<u8>, tz: &str) -> String {
    if morning.is_none() && evening.is_none() {
        return "_без расписания_".to_owned();
    }
    let mut parts = vec![];
    if let Some(h) = morning {
        parts.push(format!("утро {h}:00"));
    }
    if let Some(h) = evening {
        parts.push(format!("вечер {h}:00"));
    }
    format!("{} ({tz})", parts.join(", "))
}

// ─── topic finisher ───────────────────────────────────────────────────────────

/// Save the completed topic record and advance to next topic or ask_schedule.
pub(crate) fn finish_topic(
    persona: &Value,
    name: &str,
    _is_interest: bool,
    specifics: &[String],
    defer: bool,
    research: bool,
    proposed: &[Value],
    picked: &[usize],
) -> StepOutput {
    let status = if defer { "отложено" } else { "обсудим" };
    let src_line = build_source_line(research, picked, proposed);
    let summary = format!("✓ *{name}* — {status}; {src_line}.");

    let queue = topic_queue(persona);
    let index = topic_index(persona);
    let mut done: Vec<Value> = persona["topics_done"].as_array().cloned().unwrap_or_default();
    done.push(json!({
        "name": name, "specifics": specifics, "defer": defer,
        "research": research, "picked": picked
    }));

    if queue.is_empty() {
        return ask_schedule(&json!({ "topics_done": done }), &summary);
    }
    let next_topic = &queue[0];
    let next_name = next_topic["name"].as_str().unwrap_or("?");
    let ts = build_topic_state(&queue[1..], index + 1, json!({}));
    let mut patch = ts;
    patch["topics_done"] = json!(done);
    patch["current_topic"] = next_topic.clone();
    StepOutput {
        next_state: OnboardState::TopicSpecifics,
        response_text: format!(
            "{summary}\n\nСледующая тема: *{next_name}*.\n\nЧто именно в ней тебе интересно?"
        ),
        persona_patch: patch,
    }
}

fn build_source_line(research: bool, picked: &[usize], proposed: &[Value]) -> String {
    if research && !picked.is_empty() {
        let handles: Vec<_> = picked.iter()
            .filter_map(|&i| proposed.get(i))
            .filter_map(|s| s["handle_or_url"].as_str())
            .map(|h| format!("`{h}`"))
            .collect();
        format!("Источники: {}", handles.join(", "))
    } else if research {
        "Источники: _не выбрано_".to_owned()
    } else {
        "_без мониторинга_".to_owned()
    }
}

pub(crate) fn ask_schedule(extra_patch: &Value, prefix: &str) -> StepOutput {
    let intro = if prefix.is_empty() {
        String::new()
    } else {
        format!("{prefix}\n\n")
    };
    StepOutput {
        next_state: OnboardState::AskSchedule,
        response_text: format!(
            "{intro}Темы разобрали. ⏰ Когда удобно получать дайджесты? Напиши свободно — \
            например, \"утром часов в 8, вечером в 10, я в Бали\" или \"вечером в 9\". \
            Если не нужно — напиши \"нет\"."
        ),
        persona_patch: extra_patch.clone(),
    }
}
