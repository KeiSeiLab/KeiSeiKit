//! Deterministic session-key construction.
//!
//! Port of Hermes `gateway/session.py:build_session_key` (572-637) with the
//! KeiSei addition of an opt-in blake3 hash for keys exceeding a length floor —
//! storage layers can index either form.

use crate::message::{ChatType, Platform, SessionSource};

/// Tunables forwarded from `GatewayConfig`.
///
/// Mirrors Hermes group_sessions_per_user / thread_sessions_per_user toggles.
#[derive(Debug, Clone, Copy)]
pub struct SessionKeyOpts {
    /// In group chats, prefix with the participant ID so each user gets an
    /// isolated session in the same room.
    pub group_per_user: bool,
    /// In threads, ALSO isolate per user (Hermes default = false: threads are
    /// shared across all participants).
    pub thread_per_user: bool,
    /// Optional logical agent name. Defaults to `"main"`.
    pub agent_name: &'static str,
}

impl Default for SessionKeyOpts {
    fn default() -> Self {
        Self {
            group_per_user: true,
            thread_per_user: false,
            agent_name: "main",
        }
    }
}

/// Build a deterministic session key from a [`SessionSource`].
///
/// Format: `agent:<name>:<platform>:<chat_type>[:<chat_id>][:<thread_id>][:<user_id>]`.
///
/// See Hermes session.py:572-637 for the canonical rules.
pub fn build_session_key(source: &SessionSource, opts: SessionKeyOpts) -> String {
    let platform = source.platform.as_str();
    let agent = opts.agent_name;

    if source.chat_type == ChatType::Dm {
        return build_dm_key(source, platform, agent);
    }

    build_group_key(source, platform, agent, opts)
}

/// DM key — chat_id+thread_id+platform-specific normalisation.
fn build_dm_key(source: &SessionSource, platform: &str, agent: &str) -> String {
    let dm_chat_id = canonicalise_dm_chat_id(source);

    if let Some(cid) = dm_chat_id {
        return match &source.thread_id {
            Some(tid) => format!("agent:{agent}:{platform}:dm:{cid}:{tid}"),
            None => format!("agent:{agent}:{platform}:dm:{cid}"),
        };
    }

    if let Some(tid) = &source.thread_id {
        return format!("agent:{agent}:{platform}:dm:{tid}");
    }

    format!("agent:{agent}:{platform}:dm")
}

/// Group / channel key — supports per-user isolation and thread shaping.
fn build_group_key(
    source: &SessionSource,
    platform: &str,
    agent: &str,
    opts: SessionKeyOpts,
) -> String {
    let mut parts: Vec<String> = vec![format!("agent:{agent}"), platform.to_string()];
    parts.push(source.chat_type.as_str().to_string());
    if let Some(cid) = &source.chat_id {
        parts.push(cid.clone());
    }
    if let Some(tid) = &source.thread_id {
        parts.push(tid.clone());
    }
    if should_isolate_user(source, opts) {
        if let Some(pid) = canonicalise_participant(source) {
            parts.push(pid);
        }
    }
    parts.join(":")
}

/// Threads default to shared sessions; per-user only when explicitly enabled.
fn should_isolate_user(source: &SessionSource, opts: SessionKeyOpts) -> bool {
    if source.thread_id.is_some() && !opts.thread_per_user {
        return false;
    }
    opts.group_per_user
}

/// WhatsApp JID/LID canonicalisation — mirrors Hermes
/// `canonical_whatsapp_identifier` (no-op for other platforms).
fn canonicalise_dm_chat_id(source: &SessionSource) -> Option<String> {
    let raw = source.chat_id.as_deref()?;
    Some(canonicalise(raw, source.platform))
}

fn canonicalise_participant(source: &SessionSource) -> Option<String> {
    let raw = source
        .user_id_alt
        .as_deref()
        .or(source.user_id.as_deref())?;
    Some(canonicalise(raw, source.platform))
}

fn canonicalise(raw: &str, platform: Platform) -> String {
    if platform == Platform::WhatsApp {
        // Strip everything past `@` (LID-vs-JID flip safety).
        if let Some((user, _)) = raw.split_once('@') {
            return user.to_string();
        }
    }
    raw.to_string()
}

/// blake3 hash a key (hex-encoded). Useful for fixed-length DB indices.
pub fn hash_session_key(key: &str) -> String {
    blake3::hash(key.as_bytes()).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dm(platform: Platform, chat_id: &str) -> SessionSource {
        SessionSource::dm(platform, chat_id)
    }

    #[test]
    fn dm_key_includes_chat_id() {
        let s = dm(Platform::Telegram, "12345");
        let k = build_session_key(&s, SessionKeyOpts::default());
        assert_eq!(k, "agent:main:telegram:dm:12345");
    }

    #[test]
    fn dm_thread_appends_thread_id() {
        let mut s = dm(Platform::Telegram, "12345");
        s.thread_id = Some("topic7".into());
        let k = build_session_key(&s, SessionKeyOpts::default());
        assert_eq!(k, "agent:main:telegram:dm:12345:topic7");
    }

    #[test]
    fn whatsapp_dm_strips_at_suffix() {
        let s = dm(Platform::WhatsApp, "5511999@s.whatsapp.net");
        let k = build_session_key(&s, SessionKeyOpts::default());
        assert_eq!(k, "agent:main:whatsapp:dm:5511999");
    }

    #[test]
    fn group_per_user_appends_user_id() {
        let s = SessionSource {
            platform: Platform::Discord,
            chat_type: ChatType::Group,
            chat_id: Some("guild-42".into()),
            user_id: Some("alice".into()),
            user_id_alt: None,
            thread_id: None,
        };
        let k = build_session_key(&s, SessionKeyOpts::default());
        assert_eq!(k, "agent:main:discord:group:guild-42:alice");
    }

    #[test]
    fn group_thread_shared_by_default() {
        let s = SessionSource {
            platform: Platform::Slack,
            chat_type: ChatType::Channel,
            chat_id: Some("C9".into()),
            user_id: Some("u1".into()),
            user_id_alt: None,
            thread_id: Some("t1".into()),
        };
        let k = build_session_key(&s, SessionKeyOpts::default());
        // thread_per_user=false → user_id NOT appended
        assert_eq!(k, "agent:main:slack:channel:C9:t1");
    }

    #[test]
    fn hash_is_deterministic() {
        let h1 = hash_session_key("agent:main:telegram:dm:1");
        let h2 = hash_session_key("agent:main:telegram:dm:1");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64); // blake3 hex = 64 chars
    }
}
