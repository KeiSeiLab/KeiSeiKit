// SPDX-License-Identifier: Apache-2.0
//! LLM extraction abstraction for onboarding free-form answers.
//!
//! Mirrors `chat-onboard-extract.ts`. Three layers:
//!   * `LlmExtractor` trait — async extraction over a prompt + user text.
//!   * `MockExtractor`      — returns canned JSON; used in tests.
//!   * `OpenAiExtractor`    — real HTTP to LiteLLM proxy (behind `extractor-openai` feature).

use async_trait::async_trait;
use serde_json::Value;

use crate::error::BuddyError;

/// Valid communication tones (mirrors TS `TONES` const).
pub const TONES: &[&str] = &["friendly", "calm", "stoic", "sarcastic", "professional"];

// ─── trait ───────────────────────────────────────────────────────────────────

/// Abstract LLM extraction: given a system prompt + user text, returns JSON.
///
/// Implementations must return a `serde_json::Value::Object` on success.
/// On soft failure (model returned garbage) they should return a sensible
/// default object rather than `Err`.
#[async_trait]
pub trait LlmExtractor: Send + Sync {
    async fn extract(
        &self,
        system: &str,
        user_text: &str,
    ) -> Result<Value, BuddyError>;
}

// ─── mock ────────────────────────────────────────────────────────────────────

/// Test extractor: returns `response` verbatim.
pub struct MockExtractor {
    pub response: Value,
}

impl MockExtractor {
    pub fn new(response: Value) -> Self {
        Self { response }
    }
}

#[async_trait]
impl LlmExtractor for MockExtractor {
    async fn extract(&self, _system: &str, _user_text: &str) -> Result<Value, BuddyError> {
        Ok(self.response.clone())
    }
}

// ─── per-step system prompts ──────────────────────────────────────────────────

pub fn prompt_name() -> &'static str {
    r#"Extract the user's preferred name/handle to address them by.
Return JSON: {"name":"<value>"}.
If user wrote multiple options, pick the first. Strip honorifics. Max 40 chars.
If unclear, copy the entire input verbatim. Output JSON only, no prose."#
}

pub fn prompt_tone() -> &'static str {
    r#"Map the user's free-form description of their preferred conversation style to ONE of:
friendly, calm, stoic, sarcastic, professional.
Return JSON: {"tone":"<one>"}.
Hints: warm/cheerful/тёплый/болтливый → friendly; quiet/measured/спокойный → calm;
brief/factual/сухой/коротко → stoic; ironic/witty/иронично/саркастически → sarcastic;
expert/business/деловой → professional.
Default friendly if ambiguous. Output JSON only."#
}

pub fn prompt_list(kind: &str) -> String {
    format!(
        r#"Extract a list of the user's {kind} from their free-form text.
Return JSON: {{"items":["...","..."]}}.
Each item: 1-4 words, lowercased except proper nouns. Max 10 items.
Drop filler words ("и", "вот", "всё", "such as", etc).
If user said none/no/нет/skip, return empty array.
Output JSON only."#
    )
}

pub fn prompt_schedule() -> &'static str {
    r#"Extract digest schedule from free text.
Return JSON: {"morning":<0-23 or null>,"evening":<0-23 or null>,"timezone":"<IANA tz>"}.
morning/evening = hour the user wants morning/evening digest delivered.
If user said no/нет/skip → both null.
timezone: prefer IANA name (Asia/Bali, Europe/Moscow, America/Los_Angeles).
Bali → Asia/Makassar. Moscow → Europe/Moscow. London → Europe/London. NY → America/New_York.
If only city given, infer the IANA tz. Default UTC if completely unclear.
Output JSON only."#
}

pub fn prompt_now_or_later() -> &'static str {
    r#"Map user reply to "now" or "later". Return JSON: {"decision":"now"|"later"}.
Now: yes/да/обсудим/давай/готов/let's/sure/now/сейчас.
Later: no/нет/потом/позже/save/skip/save for later/сохрани.
Default later if ambiguous. Output JSON only."#
}

pub fn prompt_yes_no() -> &'static str {
    r#"Map user reply to boolean. Return JSON: {"yes":true|false}.
Yes: yes/да/да давай/sure/please/конечно/хочу/нужно.
No: no/нет/не надо/skip/пропусти.
Default false. Output JSON only."#
}

pub fn prompt_topic_specifics() -> &'static str {
    r#"Extract specific sub-aspects of a topic the user mentioned.
Return JSON: {"aspects":["...","..."]}.  Max 5 aspects.
Each aspect: 2-6 words, lowercase except proper nouns.
If user said empty/skip/none — return []. Output JSON only."#
}

pub fn prompt_propose_sources(topic: &str, aspects: &[String]) -> String {
    format!(
        r#"You suggest 3-7 high-signal sources for keeping up with a topic.
Return JSON: {{"sources":[{{"platform":"...","handle_or_url":"...","why":"..."}}]}}.
Allowed platforms: youtube, twitter, github, reddit, rss, telegram.
For youtube/twitter/telegram use @handle. For github use owner/repo.
For reddit use r/subreddit. For rss use full https URL.
Pick well-known authoritative sources only — no obscure or made-up ones.
Each `why` ≤ 60 chars. Output JSON only.
Topic: {topic}
Aspects: {aspects}"#,
        topic = topic,
        aspects = aspects.join(", ")
    )
}

// ─── OpenAiExtractor ─────────────────────────────────────────────────────────

#[cfg(feature = "extractor-openai")]
pub mod openai {
    use super::*;

    const DEFAULT_MODEL: &str = "claude-haiku-4-5-20251001";

    /// Real HTTP extractor hitting a LiteLLM-compatible proxy.
    pub struct OpenAiExtractor {
        pub proxy_url: String,
        pub api_key: String,
        client: reqwest::Client,
    }

    impl OpenAiExtractor {
        pub fn new(proxy_url: String, api_key: String) -> Self {
            Self {
                proxy_url,
                api_key,
                client: reqwest::Client::new(),
            }
        }
    }

    #[async_trait]
    impl LlmExtractor for OpenAiExtractor {
        async fn extract(&self, system: &str, user_text: &str) -> Result<Value, BuddyError> {
            let body = serde_json::json!({
                "model": DEFAULT_MODEL,
                "temperature": 0,
                "max_tokens": 200,
                "messages": [
                    {"role": "system", "content": system},
                    {"role": "user", "content": &user_text[..user_text.len().min(500)]}
                ]
            });
            let resp = self
                .client
                .post(format!("{}/v1/chat/completions", self.proxy_url))
                .bearer_auth(&self.api_key)
                .json(&body)
                .timeout(std::time::Duration::from_secs(15))
                .send()
                .await
                .map_err(|e| BuddyError::Transport(e.to_string()))?;
            if !resp.status().is_success() {
                return Ok(Value::Object(serde_json::Map::new()));
            }
            let data: Value = resp
                .json()
                .await
                .map_err(|e| BuddyError::Transport(e.to_string()))?;
            let text = data["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .trim()
                .to_owned();
            let cleaned = text
                .trim_start_matches("```json")
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim();
            Ok(serde_json::from_str(cleaned)
                .unwrap_or_else(|_| Value::Object(serde_json::Map::new())))
        }
    }
}
