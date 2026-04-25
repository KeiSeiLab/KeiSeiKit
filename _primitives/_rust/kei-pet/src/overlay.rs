//! Render a validated `PetManifest` → system-prompt overlay string.
//!
//! Used by any agent spawn / routine render: prepend this text to the agent's
//! base system prompt. Deterministic — same manifest → same overlay, byte-for-byte.

use crate::schema::*;
use std::fmt::Write;

/// Build the overlay prefix that a `PetManifest` contributes to a system prompt.
pub fn system_prompt(m: &PetManifest) -> String {
    let mut s = String::with_capacity(1024);

    writeln!(
        s,
        "You are {}, a companion to {}.{}",
        m.identity.pet_name,
        m.identity.user_name,
        addressing_hint(m.identity.addressing, &m.identity.user_name),
    ).unwrap();

    // Voice
    write!(s, "Primary tone: {}.", tone_str(m.voice.tone_primary)).unwrap();
    if !m.voice.tone_secondary.is_empty() {
        write!(s, " Blended with: ").unwrap();
        let blended: Vec<&str> = m.voice.tone_secondary.iter().copied().map(tone_str).collect();
        write!(s, "{}.", blended.join(", ")).unwrap();
    }
    writeln!(s).unwrap();

    writeln!(
        s,
        "Humor: {} at {} frequency.",
        humor_style_str(m.voice.humor_style),
        humor_freq_str(m.voice.humor_frequency),
    ).unwrap();

    // Edge
    writeln!(s, "{}", profanity_line(&m.edge)).unwrap();
    writeln!(
        s,
        "Directness: {}. Initiative: {}.",
        directness_str(m.edge.directness),
        initiative_str(m.edge.initiative),
    ).unwrap();

    // Interests
    if !m.interests.is_empty() {
        writeln!(s, "\n{}'s interests (treat as peer at the declared depth — no basics explain-back):", m.identity.user_name).unwrap();
        for i in &m.interests {
            writeln!(s, "  - {} ({})", i.topic, depth_str(i.depth)).unwrap();
        }
    }

    // Forbidden
    if !m.forbidden.topics.is_empty() || !m.forbidden.tone_patterns.is_empty() {
        writeln!(s).unwrap();
        if !m.forbidden.topics.is_empty() {
            writeln!(s, "Never engage with: {}.", m.forbidden.topics.join(", ")).unwrap();
        }
        if !m.forbidden.tone_patterns.is_empty() {
            writeln!(s, "Never use: {}.", m.forbidden.tone_patterns.join(", ")).unwrap();
        }
    }

    // Language preference
    if m.identity.languages.len() > 1 {
        let first = &m.identity.languages[0];
        let rest = m.identity.languages[1..].join(", ");
        writeln!(
            s,
            "\nLanguage: prefer {}, code-switch to {} for domain terms.",
            first, rest,
        ).unwrap();
    } else if let Some(only) = m.identity.languages.first() {
        writeln!(s, "\nLanguage: {}.", only).unwrap();
    }

    s
}

fn addressing_hint(a: Addressing, user: &str) -> String {
    match a {
        Addressing::ByName => format!(" Address {user} by name."),
        Addressing::Nickname => format!(" Address {user} by an affectionate nickname (ask once; reuse thereafter)."),
        Addressing::Formal => format!(" Address {user} formally (вы / Mr./Ms. / sir/madam as language dictates)."),
        Addressing::NoAddress => String::new(),
    }
}

fn tone_str(t: Tone) -> &'static str {
    match t {
        Tone::Warm => "warm",
        Tone::Dry => "dry",
        Tone::Sarcastic => "sarcastic",
        Tone::Neutral => "neutral",
        Tone::Supportive => "supportive",
    }
}

fn humor_style_str(h: HumorStyle) -> &'static str {
    match h {
        HumorStyle::None => "no humor",
        HumorStyle::Puns => "wordplay",
        HumorStyle::Dark => "dark humor",
        HumorStyle::Absurd => "absurdist humor",
        HumorStyle::EngineeringMeta => "engineering meta-humor",
        HumorStyle::DarkMeta => "dark + engineering-meta humor",
    }
}

fn humor_freq_str(f: HumorFrequency) -> &'static str {
    match f {
        HumorFrequency::Rare => "rare",
        HumorFrequency::Medium => "medium",
        HumorFrequency::Often => "often",
    }
}

fn directness_str(d: Directness) -> &'static str {
    match d {
        Directness::Soft => "soft, hedge when uncertain",
        Directness::Balanced => "balanced, plain but polite",
        Directness::Hard => "hard — say what you see, no hedging",
    }
}

fn initiative_str(i: Initiative) -> &'static str {
    match i {
        Initiative::Wait => "wait until asked",
        Initiative::Suggest => "suggest occasionally",
        Initiative::TapOnShoulder => "tap on the shoulder when you spot a problem",
    }
}

fn depth_str(d: Depth) -> &'static str {
    match d {
        Depth::Shallow => "shallow",
        Depth::Intermediate => "intermediate",
        Depth::Expert => "expert",
    }
}

fn profanity_line(e: &Edge) -> String {
    match e.profanity {
        Profanity::Never => "Profanity: never.".to_string(),
        Profanity::Accent => format!(
            "Profanity: rare, used only for strong accent in {}.",
            e.profanity_languages.join(", ")
        ),
        Profanity::Casual => format!(
            "Profanity: casual in {}.",
            e.profanity_languages.join(", ")
        ),
        Profanity::MirrorUser => format!(
            "Profanity: mirror the user's style in {}.",
            e.profanity_languages.join(", ")
        ),
    }
}
