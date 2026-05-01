//! Session-key determinism + cross-platform invariants.

use kei_gateway::message::{ChatType, Platform, SessionSource};
use kei_gateway::session_key::{build_session_key, hash_session_key, SessionKeyOpts};

fn dm(platform: Platform, chat_id: &str) -> SessionSource {
    SessionSource::dm(platform, chat_id)
}

#[test]
fn same_source_yields_same_key_on_repeat_call() {
    let s = dm(Platform::Telegram, "user-77");
    let k1 = build_session_key(&s, SessionKeyOpts::default());
    let k2 = build_session_key(&s, SessionKeyOpts::default());
    let k3 = build_session_key(&s, SessionKeyOpts::default());
    assert_eq!(k1, k2);
    assert_eq!(k2, k3);
}

#[test]
fn different_platforms_with_same_chat_id_produce_distinct_keys() {
    // Two unrelated DMs should never collapse onto the same session.
    let tg = dm(Platform::Telegram, "777");
    let dc = dm(Platform::Discord, "777");
    let opts = SessionKeyOpts::default();
    let k_tg = build_session_key(&tg, opts);
    let k_dc = build_session_key(&dc, opts);
    assert_ne!(k_tg, k_dc);
    assert!(k_tg.contains("telegram"));
    assert!(k_dc.contains("discord"));
}

#[test]
fn same_user_same_dm_chat_yields_same_key() {
    // Two messages from the same DM chat must land on the same session.
    let s1 = dm(Platform::Telegram, "111");
    let s2 = dm(Platform::Telegram, "111");
    assert_eq!(
        build_session_key(&s1, SessionKeyOpts::default()),
        build_session_key(&s2, SessionKeyOpts::default()),
    );
}

#[test]
fn group_per_user_disabled_collapses_users_into_one_session() {
    let mut s = SessionSource {
        platform: Platform::Slack,
        chat_type: ChatType::Group,
        chat_id: Some("G1".into()),
        user_id: Some("alice".into()),
        user_id_alt: None,
        thread_id: None,
    };
    let opts_off = SessionKeyOpts {
        group_per_user: false,
        thread_per_user: false,
        agent_name: "main",
    };
    let k_alice = build_session_key(&s, opts_off);
    s.user_id = Some("bob".into());
    let k_bob = build_session_key(&s, opts_off);
    assert_eq!(k_alice, k_bob); // shared session
}

#[test]
fn group_per_user_enabled_isolates_each_user() {
    let mut s = SessionSource {
        platform: Platform::Slack,
        chat_type: ChatType::Group,
        chat_id: Some("G1".into()),
        user_id: Some("alice".into()),
        user_id_alt: None,
        thread_id: None,
    };
    let k_alice = build_session_key(&s, SessionKeyOpts::default());
    s.user_id = Some("bob".into());
    let k_bob = build_session_key(&s, SessionKeyOpts::default());
    assert_ne!(k_alice, k_bob);
}

#[test]
fn whatsapp_alias_normalisation_collapses_lid_jid_flip() {
    // Same physical user appearing under two suffixes should land on one session.
    let s_jid = dm(Platform::WhatsApp, "5511999@s.whatsapp.net");
    let s_lid = dm(Platform::WhatsApp, "5511999@lid");
    assert_eq!(
        build_session_key(&s_jid, SessionKeyOpts::default()),
        build_session_key(&s_lid, SessionKeyOpts::default()),
    );
}

#[test]
fn hash_session_key_is_64_char_hex_and_deterministic() {
    let h1 = hash_session_key("agent:main:telegram:dm:1");
    let h2 = hash_session_key("agent:main:telegram:dm:1");
    assert_eq!(h1, h2);
    assert_eq!(h1.len(), 64);
    assert!(h1.chars().all(|c| c.is_ascii_hexdigit()));
}
