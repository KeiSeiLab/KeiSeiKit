//! Pet self-reflection — analyse user correction signals, propose persona
//! tune changes.
//!
//! Pipeline: caller accumulates `CorrectionSignal`s over some window (a
//! session, a day, since last tune). `propose_tune` groups them by topic
//! and emits a minimal, idempotent set of `ProposedChange`s against the
//! current `PetManifest`. Persistence and user-approval UX are the
//! caller's concern — this module is pure data + pure logic.

use crate::schema::{Directness, Initiative, PetManifest, Tone};
use std::collections::HashMap;

// ─────────────────────────── public types ────────────────────────────

#[derive(Debug, Clone)]
pub struct CorrectionSignal {
    pub timestamp: i64,
    /// Topic label. Two shapes:
    ///   * flat topic, e.g. `"too_verbose"`, `"too_formal"`,
    ///     `"not_proactive_enough"`.
    ///   * namespaced topic, e.g. `"forbidden_topic:diagnosis"` — the
    ///     prefix before `:` is the category, the suffix is the payload.
    pub topic: String,
    pub severity: u8,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProposedChange {
    SetDirectness(String),
    AddForbiddenTopic(String),
    SetInitiative(String),
    SetTonePrimary(String),
}

// ─────────────────────────── thresholds ──────────────────────────────

const TOO_VERBOSE_THRESHOLD: usize = 3;
const FORBIDDEN_TOPIC_THRESHOLD: usize = 2;
const NOT_PROACTIVE_THRESHOLD: usize = 3;
const TOO_FORMAL_THRESHOLD: usize = 3;

// ─────────────────────────── public api ──────────────────────────────

/// Produce an ordered, idempotent set of proposed changes.
///
/// Order: directness → forbidden topics (by first-seen order) →
/// initiative → tone. Idempotent: a change is only emitted when the
/// manifest is NOT already in the desired state.
pub fn propose_tune(
    manifest: &PetManifest,
    signals: &[CorrectionSignal],
) -> Vec<ProposedChange> {
    let counts = tally(signals);
    let forbidden_topics = tally_forbidden(signals);

    let mut out = Vec::new();
    maybe_directness(&counts, manifest, &mut out);
    emit_forbidden(&forbidden_topics, manifest, &mut out);
    maybe_initiative(&counts, manifest, &mut out);
    maybe_tone(&counts, manifest, &mut out);
    out
}

// ─────────────────────────── tallying ────────────────────────────────

fn tally(signals: &[CorrectionSignal]) -> HashMap<&str, usize> {
    let mut out: HashMap<&str, usize> = HashMap::new();
    for sig in signals {
        if sig.topic.contains(':') {
            continue;
        }
        *out.entry(sig.topic.as_str()).or_insert(0) += 1;
    }
    out
}

/// Collect `forbidden_topic:<payload>` signals preserving first-seen
/// order, with per-payload counts.
fn tally_forbidden(signals: &[CorrectionSignal]) -> Vec<(String, usize)> {
    let mut order: Vec<String> = Vec::new();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for sig in signals {
        let Some(payload) = sig.topic.strip_prefix("forbidden_topic:") else {
            continue;
        };
        let payload = payload.to_string();
        if !counts.contains_key(&payload) {
            order.push(payload.clone());
        }
        *counts.entry(payload).or_insert(0) += 1;
    }
    order.into_iter().map(|p| { let c = counts[&p]; (p, c) }).collect()
}

// ─────────────────────────── emitters ────────────────────────────────

fn maybe_directness(
    counts: &HashMap<&str, usize>,
    manifest: &PetManifest,
    out: &mut Vec<ProposedChange>,
) {
    let n = counts.get("too_verbose").copied().unwrap_or(0);
    if n < TOO_VERBOSE_THRESHOLD {
        return;
    }
    // "direct" maps to Directness::Hard (the terse end of the scale).
    if manifest.edge.directness == Directness::Hard {
        return;
    }
    out.push(ProposedChange::SetDirectness("direct".to_string()));
}

fn emit_forbidden(
    forbidden: &[(String, usize)],
    manifest: &PetManifest,
    out: &mut Vec<ProposedChange>,
) {
    for (topic, count) in forbidden {
        if *count < FORBIDDEN_TOPIC_THRESHOLD {
            continue;
        }
        if manifest.forbidden.topics.iter().any(|t| t == topic) {
            continue;
        }
        out.push(ProposedChange::AddForbiddenTopic(topic.clone()));
    }
}

fn maybe_initiative(
    counts: &HashMap<&str, usize>,
    manifest: &PetManifest,
    out: &mut Vec<ProposedChange>,
) {
    let n = counts.get("not_proactive_enough").copied().unwrap_or(0);
    if n < NOT_PROACTIVE_THRESHOLD {
        return;
    }
    // "proactive" maps to Initiative::TapOnShoulder (most proactive rung).
    if manifest.edge.initiative == Initiative::TapOnShoulder {
        return;
    }
    out.push(ProposedChange::SetInitiative("proactive".to_string()));
}

fn maybe_tone(
    counts: &HashMap<&str, usize>,
    manifest: &PetManifest,
    out: &mut Vec<ProposedChange>,
) {
    let n = counts.get("too_formal").copied().unwrap_or(0);
    if n < TOO_FORMAL_THRESHOLD {
        return;
    }
    if manifest.voice.tone_primary == Tone::Warm {
        return;
    }
    out.push(ProposedChange::SetTonePrimary("warm".to_string()));
}
