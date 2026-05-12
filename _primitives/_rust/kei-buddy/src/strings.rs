// SPDX-License-Identifier: Apache-2.0
//! Localization table for all onboarding prompt strings.
//! Add new languages by extending the `Lang` enum + each match arm.

use serde_json::Value;

/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lang {
    En,
    Ru,
}

impl Lang {
    /// Infer language from a stored persona blob (`persona["language"]`).
    /// Falls back to `En` when the key is absent or unrecognised.
    pub fn from_persona(persona: &Value) -> Self {
        match persona.get("language").and_then(|v| v.as_str()) {
            Some("ru") => Lang::Ru,
            _ => Lang::En,
        }
    }

    /// Parse a user's free-form language choice.
    /// Returns `None` when the text is not a recognised choice.
    pub fn from_user_choice(text: &str) -> Option<Lang> {
        let t = text.trim().to_lowercase();
        match t.as_str() {
            "en" | "english" | "1" | "англ" | "🇬🇧" | "🇺🇸" => Some(Lang::En),
            "ru" | "русский" | "rus" | "2" | "рус" | "🇷🇺" => Some(Lang::Ru),
            _ => None,
        }
    }

    /// BCP-47 / ISO 639-1 code.
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ru => "ru",
        }
    }
}

/// Static onboarding prompt strings, keyed by language.
pub struct Strings;

impl Strings {
    /// Always bilingual — shown before language is known.
    pub fn intro_ask_language() -> &'static str {
        "Hi! I'm KeiBuddy, your personal AI assistant from KeiSei. \
         Please choose your language:\n• English (en)\n• Русский (ru)\n\n\
         Привет! Я KeiBuddy, твой персональный AI-компаньон от KeiSei. \
         Выбери язык:\n• English (en)\n• Русский (ru)"
    }

    pub fn ask_name(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "What's your name? (I'll use it to address you.)",
            Lang::Ru => "Как тебя называть?",
        }
    }

    pub fn ask_tone(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "How should I talk to you? Describe it in your own words — e.g. \
                \"friendly\", \"dry and to the point\", \"with irony\". \
                Or just a word: `friendly`, `calm`, `stoic`, `sarcastic`, `professional`.",
            Lang::Ru => "Какой стиль общения тебе ближе? Опиши своими словами — например, \
                \"по-дружески\", \"сухо и по делу\", \"с иронией\". \
                Или просто слово: `friendly`, `calm`, `stoic`, `sarcastic`, `professional`.",
        }
    }

    pub fn ask_interests(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "What are your interests? Just list them — \
                any format works (comma-separated, bullet points, or a paragraph).",
            Lang::Ru => "Какие у тебя интересы? Просто перечисли — \
                как удобно (через запятую, списком, или одним абзацем).",
        }
    }

    pub fn ask_hobbies(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "What about hobbies? What do you actually do in your free time?",
            Lang::Ru => "А хобби? Чем конкретно занимаешься в свободное время.",
        }
    }

    /// Dynamic — topic name is interpolated by the caller.
    pub fn topic_specifics_prefix(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Now let's dig into the topics. First up",
            Lang::Ru => "Теперь разберём по темам. Поехали — сначала",
        }
    }

    pub fn topic_specifics_question(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "What *specifically* interests you here? Give me details \
                (e.g. for AI: \"agents, model training, papers\"; \
                for surfing: \"technique, boards, spot reports\").",
            Lang::Ru => "*Что именно* в этой теме тебе интересно? Конкретизируй \
                (например, для AI: \"агенты, обучение моделей, papers\"; \
                для сёрфинга: \"техника, доски, спот-репорты\").",
        }
    }

    pub fn topic_now_later(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Would you like to *discuss this now* or *save it for later*?",
            Lang::Ru => "Хочешь *обсудить это сейчас* или *сохранить на потом*?",
        }
    }

    pub fn topic_research(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Should I *regularly monitor* updates on this topic and send you digests?",
            Lang::Ru => "Хочешь чтобы я *регулярно следил* за обновлениями по этой теме и присылал дайджесты?",
        }
    }

    pub fn topic_sources_intro(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Which ones do you want to add? Write the numbers separated by commas \
                (`1,3,5`), `all`, or `none`. \
                You can add your own — just write \"plus <platform> <handle>\".",
            Lang::Ru => "Какие добавить? Напиши номера через запятую (`1,3,5`), `все`, или `нет`. \
                Можешь добавить свои — просто напиши \"плюс <платформа> <handle>\".",
        }
    }

    pub fn ask_schedule(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Topics covered! ⏰ When would you like to receive digests? \
                Write freely — e.g. \"mornings around 8, evenings at 10, I'm in Bali\" \
                or \"evenings at 9\". If you don't need them, write \"no\".",
            Lang::Ru => "Темы разобрали. ⏰ Когда удобно получать дайджесты? Напиши свободно — \
                например, \"утром часов в 8, вечером в 10, я в Бали\" или \"вечером в 9\". \
                Если не нужно — напиши \"нет\".",
        }
    }

    pub fn ready(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "All set! ✨ I'm now your assistant.",
            Lang::Ru => "Готово! ✨ Я настроен.",
        }
    }

    /// Confirmation shown right after language is chosen.
    pub fn language_set(lang: Lang) -> &'static str {
        match lang {
            Lang::En => "Language set to English.",
            Lang::Ru => "Язык установлен: русский.",
        }
    }

    /// Shown when input doesn't match any language choice — always bilingual.
    pub fn invalid_language() -> &'static str {
        "Please answer 'en' for English or 'ru' for Russian.\n\
         Пожалуйста, ответь 'en' или 'ru'."
    }
}
