//! Persona version history + manifest diff.
//!
//! `PersonaVersion` records a single snapshot of a `PetManifest` with a
//! monotonic version number and an optional parent pointer — forming a linked
//! history chain. `diff` produces a minimal set of human-readable `Change`
//! entries between two manifests (voice tone, edge directness/initiative,
//! humor style, forbidden topics, identity languages). Persistence (file
//! layout, serialization target) is the caller's concern; this module is
//! pure data.

use crate::schema::{
    Directness, HumorStyle, Initiative, PetManifest, Tone,
};
use serde::{Deserialize, Serialize};

// ─────────────────────────── public types ────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaVersion {
    pub version: u32,
    pub parent_version: Option<u32>,
    pub manifest: PetManifest,
    /// Unix seconds (UTC). Caller fills via `chrono::Utc::now().timestamp()`
    /// or equivalent; the struct is agnostic to the clock source.
    pub created_at: i64,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    VoiceTonePrimaryChanged { from: String, to: String },
    EdgeDirectnessChanged { from: String, to: String },
    EdgeInitiativeChanged { from: String, to: String },
    ForbiddenTopicAdded(String),
    ForbiddenTopicRemoved(String),
    LanguageAdded(String),
    LanguageRemoved(String),
    HumorStyleChanged { from: String, to: String },
}

// ───────────────────────────── diff api ──────────────────────────────

/// Minimal ordered diff between two manifests.
///
/// Field order: voice → edge → humor → forbidden (topics) → identity
/// (languages). Added/Removed entries emitted in source-vector order.
pub fn diff(old: &PetManifest, new: &PetManifest) -> Vec<Change> {
    let mut out = Vec::new();
    diff_voice(old, new, &mut out);
    diff_edge(old, new, &mut out);
    diff_humor(old, new, &mut out);
    diff_forbidden(old, new, &mut out);
    diff_languages(old, new, &mut out);
    out
}

/// Fork a new version off `parent`. `created_at` is left at 0 — caller
/// should overwrite with a real timestamp before persisting.
pub fn fork_version(
    parent: &PersonaVersion,
    reason: &str,
    new_manifest: PetManifest,
) -> PersonaVersion {
    PersonaVersion {
        version: parent.version + 1,
        parent_version: Some(parent.version),
        manifest: new_manifest,
        created_at: 0,
        reason: reason.to_string(),
    }
}

// ─────────────────────────── sub-diff helpers ────────────────────────

fn diff_voice(old: &PetManifest, new: &PetManifest, out: &mut Vec<Change>) {
    if old.voice.tone_primary != new.voice.tone_primary {
        out.push(Change::VoiceTonePrimaryChanged {
            from: tone_str(old.voice.tone_primary).to_string(),
            to: tone_str(new.voice.tone_primary).to_string(),
        });
    }
}

fn diff_edge(old: &PetManifest, new: &PetManifest, out: &mut Vec<Change>) {
    if old.edge.directness != new.edge.directness {
        out.push(Change::EdgeDirectnessChanged {
            from: directness_str(old.edge.directness).to_string(),
            to: directness_str(new.edge.directness).to_string(),
        });
    }
    if old.edge.initiative != new.edge.initiative {
        out.push(Change::EdgeInitiativeChanged {
            from: initiative_str(old.edge.initiative).to_string(),
            to: initiative_str(new.edge.initiative).to_string(),
        });
    }
}

fn diff_humor(old: &PetManifest, new: &PetManifest, out: &mut Vec<Change>) {
    if old.voice.humor_style != new.voice.humor_style {
        out.push(Change::HumorStyleChanged {
            from: humor_style_str(old.voice.humor_style).to_string(),
            to: humor_style_str(new.voice.humor_style).to_string(),
        });
    }
}

fn diff_forbidden(old: &PetManifest, new: &PetManifest, out: &mut Vec<Change>) {
    for t in &new.forbidden.topics {
        if !old.forbidden.topics.contains(t) {
            out.push(Change::ForbiddenTopicAdded(t.clone()));
        }
    }
    for t in &old.forbidden.topics {
        if !new.forbidden.topics.contains(t) {
            out.push(Change::ForbiddenTopicRemoved(t.clone()));
        }
    }
}

fn diff_languages(old: &PetManifest, new: &PetManifest, out: &mut Vec<Change>) {
    for l in &new.identity.languages {
        if !old.identity.languages.contains(l) {
            out.push(Change::LanguageAdded(l.clone()));
        }
    }
    for l in &old.identity.languages {
        if !new.identity.languages.contains(l) {
            out.push(Change::LanguageRemoved(l.clone()));
        }
    }
}

// ─────────────────────────── enum → kebab-case ───────────────────────

fn tone_str(t: Tone) -> &'static str {
    match t {
        Tone::Warm => "warm",
        Tone::Dry => "dry",
        Tone::Sarcastic => "sarcastic",
        Tone::Neutral => "neutral",
        Tone::Supportive => "supportive",
    }
}

fn directness_str(d: Directness) -> &'static str {
    match d {
        Directness::Soft => "soft",
        Directness::Balanced => "balanced",
        Directness::Hard => "hard",
    }
}

fn initiative_str(i: Initiative) -> &'static str {
    match i {
        Initiative::Wait => "wait",
        Initiative::Suggest => "suggest",
        Initiative::TapOnShoulder => "tap-on-shoulder",
    }
}

fn humor_style_str(h: HumorStyle) -> &'static str {
    match h {
        HumorStyle::None => "none",
        HumorStyle::Puns => "puns",
        HumorStyle::Dark => "dark",
        HumorStyle::Absurd => "absurd",
        HumorStyle::EngineeringMeta => "engineering-meta",
        HumorStyle::DarkMeta => "dark+meta",
    }
}
