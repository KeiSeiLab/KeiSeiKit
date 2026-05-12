// SPDX-License-Identifier: Apache-2.0
//! Command execution helpers — one function per slash-command.
//! Called by `commands::execute_command`; not public API.

use std::sync::Arc;

use crate::{
    chat_log::ChatLog,
    contacts::Contacts,
    contacts_sync::{sync_from_apple, sync_from_google},
    topics::Topics,
};

pub(crate) async fn exec_topics(chat_id: i64, topics: &Topics) -> String {
    match topics.list_topics(chat_id).await {
        Err(e) => format!("ошибка при получении тем: {e}"),
        Ok(list) if list.is_empty() => "тем пока нет".to_string(),
        Ok(list) => {
            let mut out = String::new();
            for (i, unit) in list.iter().take(10).enumerate() {
                let slug = unit.source_path.split('/').last().unwrap_or(&unit.source_path);
                out.push_str(&format!("{}. {} ({})\n", i + 1, unit.title, slug));
            }
            out.trim_end().to_string()
        }
    }
}

pub(crate) async fn exec_contacts(contacts: &Contacts) -> String {
    match contacts.search_contacts("", 10).await {
        Err(_) => "контакты пусты".to_string(),
        Ok(list) if list.is_empty() => "контакты пусты".to_string(),
        Ok(list) => format_people(&list),
    }
}

pub(crate) async fn exec_whois(name: &str, contacts: &Contacts) -> String {
    if name.is_empty() {
        return "использование: /whois <имя>".to_string();
    }
    match contacts.search_contacts(name, 5).await {
        Err(e) => format!("ошибка поиска: {e}"),
        Ok(list) if list.is_empty() => format!("не найдено никого по запросу '{name}'"),
        Ok(list) => {
            let mut out = format_people(&list);
            append_common_connections(&mut out, &list, contacts).await;
            out
        }
    }
}

async fn append_common_connections(
    out: &mut String,
    list: &[kei_social_store::people::Person],
    contacts: &Contacts,
) {
    if list.len() <= 1 {
        return;
    }
    let top_id = list[0].id;
    for hit in list.iter().skip(1).take(4) {
        if let Ok(cc) = contacts.common_connections(top_id, hit.id).await {
            if !cc.is_empty() {
                let ids: Vec<String> = cc.iter().map(|id| id.to_string()).collect();
                out.push_str(&format!(
                    "\nобщие знакомые ({} и {}): {}",
                    list[0].name,
                    hit.name,
                    ids.join(", ")
                ));
            }
        }
    }
}

pub(crate) async fn exec_find(query: &str, chat_id: i64, chat_log: &ChatLog) -> String {
    if query.is_empty() {
        return "использование: /find <текст>".to_string();
    }
    match chat_log.search(query, Some(chat_id), 10).await {
        Err(e) => format!("ошибка поиска в переписке: {e}"),
        Ok(msgs) if msgs.is_empty() => "ничего не найдено в переписке".to_string(),
        Ok(msgs) => {
            let mut out = String::new();
            for (i, msg) in msgs.iter().enumerate() {
                let snippet = truncate(&msg.content, 80);
                out.push_str(&format!("{}. [{}] {}…\n", i + 1, msg.role, snippet));
            }
            out.trim_end().to_string()
        }
    }
}

pub(crate) fn format_people(list: &[kei_social_store::people::Person]) -> String {
    let mut out = String::new();
    for (i, p) in list.iter().enumerate() {
        let detail = if !p.email.is_empty() {
            p.email.clone()
        } else if !p.organization.is_empty() {
            p.organization.clone()
        } else {
            String::new()
        };
        if detail.is_empty() {
            out.push_str(&format!("{}. {}\n", i + 1, p.name));
        } else {
            out.push_str(&format!("{}. {} — {}\n", i + 1, p.name, detail));
        }
    }
    out.trim_end().to_string()
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub(crate) async fn exec_sync_google(contacts: &Arc<Contacts>) -> String {
    let token = match std::env::var("GOOGLE_OAUTH_ACCESS_TOKEN") {
        Ok(t) if !t.is_empty() => t,
        _ => return "не настроено: GOOGLE_OAUTH_ACCESS_TOKEN не задан".to_string(),
    };
    let r = sync_from_google(&token, contacts).await;
    format!(
        "Google: загружено {}, добавлено {}, пропущено {}\nошибок: {}",
        r.fetched, r.added, r.skipped, r.errors.len()
    )
}

pub(crate) async fn exec_sync_apple(contacts: &Arc<Contacts>) -> String {
    let apple_id = std::env::var("APPLE_ID").unwrap_or_default();
    let app_pw = std::env::var("APPLE_APP_PASSWORD").unwrap_or_default();
    let url = std::env::var("APPLE_CARDDAV_URL").unwrap_or_default();
    if apple_id.is_empty() || app_pw.is_empty() || url.is_empty() {
        return "не настроено: APPLE_ID / APPLE_APP_PASSWORD / APPLE_CARDDAV_URL не заданы"
            .to_string();
    }
    let r = sync_from_apple(&apple_id, &app_pw, &url, contacts).await;
    format!(
        "Apple: загружено {}, добавлено {}, пропущено {}\nошибок: {}",
        r.fetched, r.added, r.skipped, r.errors.len()
    )
}
