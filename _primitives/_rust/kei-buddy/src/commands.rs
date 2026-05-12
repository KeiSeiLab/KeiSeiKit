// SPDX-License-Identifier: Apache-2.0
//! Slash-command public API: types, parser, and dispatcher.
//! Execution helpers live in `command_exec` (≤200 LOC split).
//! `process_text` in serve.rs calls `parse_command` BEFORE `handle_step`.

use std::sync::Arc;

use crate::{chat_log::ChatLog, command_exec as exec, contacts::Contacts, topics::Topics};

/// Recognised slash-commands. `None` = not a command → fall through to FSM.
pub enum Command<'a> {
    Whois(&'a str),
    Find(&'a str),
    Topics,
    Contacts,
    Help,
}

/// Shared store references passed to `execute_command`.
pub struct CommandStores<'a> {
    pub chat_log: &'a Arc<ChatLog>,
    pub contacts: &'a Arc<Contacts>,
    pub topics: &'a Arc<Topics>,
}

const HELP_TEXT: &str = "Доступные команды:\n\
    /whois <имя> — найти контакт\n\
    /find <текст> — поиск по переписке\n\
    /topics — список тем\n\
    /contacts — список контактов\n\
    /help — это сообщение";

/// Parse a raw user text into a Command, or None if it is not a slash-command.
pub fn parse_command(text: &str) -> Option<Command<'_>> {
    let t = text.trim();
    if !t.starts_with('/') {
        return None;
    }
    let rest = &t[1..]; // drop leading '/'
    if rest.eq_ignore_ascii_case("help") {
        return Some(Command::Help);
    }
    if rest.eq_ignore_ascii_case("topics") {
        return Some(Command::Topics);
    }
    if rest.eq_ignore_ascii_case("contacts") {
        return Some(Command::Contacts);
    }
    let lower = rest.to_lowercase();
    if lower.starts_with("whois") {
        return Some(Command::Whois(rest[5..].trim()));
    }
    if lower.starts_with("find") {
        return Some(Command::Find(rest[4..].trim()));
    }
    None
}

/// Execute a parsed command. Returns a human-readable response string.
/// All errors become human-readable messages (Russian, English fallback).
pub async fn execute_command(
    cmd: Command<'_>,
    chat_id: i64,
    stores: &CommandStores<'_>,
) -> String {
    match cmd {
        Command::Help => HELP_TEXT.to_string(),
        Command::Topics => exec::exec_topics(chat_id, stores.topics).await,
        Command::Contacts => exec::exec_contacts(stores.contacts).await,
        Command::Whois(name) => exec::exec_whois(name, stores.contacts).await,
        Command::Find(query) => exec::exec_find(query, chat_id, stores.chat_log).await,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn make_stores<'a>(
        cl: &'a Arc<ChatLog>,
        co: &'a Arc<Contacts>,
        to: &'a Arc<Topics>,
    ) -> CommandStores<'a> {
        CommandStores { chat_log: cl, contacts: co, topics: to }
    }

    #[test]
    fn parse_help_no_args() {
        assert!(matches!(parse_command("/help"), Some(Command::Help)));
    }

    #[test]
    fn parse_whois_with_name() {
        let cmd = parse_command("/whois Denis");
        assert!(matches!(cmd, Some(Command::Whois("Denis"))));
    }

    #[test]
    fn parse_non_command_returns_none() {
        assert!(parse_command("hello there").is_none());
    }

    #[tokio::test]
    async fn execute_help_returns_help_text() {
        let cl = Arc::new(ChatLog::from_memory().unwrap());
        let co = Arc::new(Contacts::from_memory().unwrap());
        let to = Arc::new(Topics::from_memory().unwrap());
        let stores = make_stores(&cl, &co, &to);
        let resp = execute_command(Command::Help, 1, &stores).await;
        assert!(resp.contains("/whois"));
        assert!(resp.contains("/find"));
    }

    #[tokio::test]
    async fn execute_topics_lists_added() {
        let cl = Arc::new(ChatLog::from_memory().unwrap());
        let co = Arc::new(Contacts::from_memory().unwrap());
        let to = Arc::new(Topics::from_memory().unwrap());
        to.add_topic(42, "rust-lang", "Rust Language", "content").await.unwrap();
        let stores = make_stores(&cl, &co, &to);
        let resp = execute_command(Command::Topics, 42, &stores).await;
        assert!(resp.contains("Rust Language"));
    }

    #[tokio::test]
    async fn execute_find_returns_matches() {
        let cl = Arc::new(ChatLog::from_memory().unwrap());
        let co = Arc::new(Contacts::from_memory().unwrap());
        let to = Arc::new(Topics::from_memory().unwrap());
        cl.log_user(99, "unique_search_word here").await.unwrap();
        let stores = make_stores(&cl, &co, &to);
        let resp = execute_command(Command::Find("unique_search_word"), 99, &stores).await;
        assert!(resp.contains("unique_search_word"));
    }

    #[tokio::test]
    async fn execute_contacts_empty_handled() {
        let cl = Arc::new(ChatLog::from_memory().unwrap());
        let co = Arc::new(Contacts::from_memory().unwrap());
        let to = Arc::new(Topics::from_memory().unwrap());
        let stores = make_stores(&cl, &co, &to);
        let resp = execute_command(Command::Contacts, 1, &stores).await;
        assert!(resp.contains("пусты") || resp.contains("контакт"));
    }
}
