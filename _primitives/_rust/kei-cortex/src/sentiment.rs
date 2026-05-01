//! Keyword-based sentiment classifier.
//!
//! Given accumulated assistant text, score each of the 9 allowed tags by
//! counting case-insensitive whole-word keyword matches. Return the top
//! tag + confidence in `[0.0, 1.0]`. Default to `neutral @ 0.5` if no
//! keyword matches. Pure and synchronous — trivial to unit-test.
//!
//! Wave 45 TODO — Cross-language SSoT for emotion tags:
//!   The set below (`TAGS`) is duplicated in three places:
//!     1. this file — Rust `&[&str]` (sentiment scoring keys).
//!     2. `_ts_packages/cortex-ui/src/lib/emotions.ts` — TS enum.
//!     3. `_ts_packages/cortex-ui/src/lib/chat/types.ts` — TS string union.
//!   When a tag is added/renamed all three must change in lock-step.
//!   Wave 45 should introduce a build-time codegen step (one canonical
//!   source — likely a `_schemas/emotions.json` — and emit the Rust slice
//!   + the two TS bindings from it). This is out of scope for Wave 44d
//!   because it requires a TS-build hook + workspace build.rs plumbing.

/// The exact nine tags permitted on the wire.
pub const TAGS: [&str; 9] = [
    "happy", "sad", "angry", "think", "surprised",
    "awkward", "question", "curious", "neutral",
];

/// Classification result.
#[derive(Debug, Clone, PartialEq)]
pub struct Sentiment {
    pub tag: &'static str,
    pub confidence: f32,
}

/// Classify `text` into one of `TAGS`.
///
/// Confidence = `top_score / total_matches`, clamped to `[0.0, 1.0]`.
/// No keywords hit → `neutral @ 0.5`.
pub fn classify(text: &str) -> Sentiment {
    let lower = text.to_lowercase();
    let scores = score_all(&lower);
    pick_top(&scores)
}

/// Score every tag over the lower-cased text.
fn score_all(lower: &str) -> [(&'static str, u32); 9] {
    let mut out: [(&'static str, u32); 9] = [
        ("happy", 0), ("sad", 0), ("angry", 0), ("think", 0),
        ("surprised", 0), ("awkward", 0), ("question", 0),
        ("curious", 0), ("neutral", 0),
    ];
    for (tag, count) in out.iter_mut() {
        *count = count_keywords(lower, keywords_for(tag));
    }
    out
}

/// Keyword list per tag. Chosen to be cheap and reasonable — not exhaustive.
fn keywords_for(tag: &str) -> &'static [&'static str] {
    match tag {
        "happy"     => &["happy", "glad", "great", "awesome", "love", "yay", "wonderful", "delighted"],
        "sad"       => &["sad", "sorry", "upset", "unfortunate", "regret", "miss"],
        "angry"     => &["angry", "mad", "furious", "annoyed", "irritated", "frustrated"],
        "think"     => &["think", "consider", "suppose", "perhaps", "maybe", "hmm"],
        "surprised" => &["surprised", "wow", "whoa", "really", "unexpected", "shocked"],
        "awkward"   => &["awkward", "uh", "um", "weird", "strange", "uncomfortable"],
        "question"  => &["?", "what", "why", "how", "when", "where", "who"],
        "curious"   => &["curious", "wonder", "interesting", "fascinating", "intrigued"],
        _           => &[],
    }
}

/// Count case-insensitive whole-word occurrences of each keyword.
fn count_keywords(lower: &str, kws: &[&str]) -> u32 {
    let mut n: u32 = 0;
    for kw in kws {
        n += count_one(lower, kw);
    }
    n
}

/// Whole-word match for alpha keywords; raw substring match for punctuation ("?").
fn count_one(lower: &str, kw: &str) -> u32 {
    if kw.chars().all(|c| c.is_ascii_punctuation()) {
        return lower.matches(kw).count() as u32;
    }
    let mut n: u32 = 0;
    for token in lower.split(|c: char| !c.is_ascii_alphanumeric()) {
        if token == kw {
            n += 1;
        }
    }
    n
}

/// Reduce score table to the top tag + normalised confidence.
fn pick_top(scores: &[(&'static str, u32); 9]) -> Sentiment {
    let total: u32 = scores.iter().map(|(_, v)| *v).sum();
    if total == 0 {
        return Sentiment { tag: "neutral", confidence: 0.5 };
    }
    let (best_tag, best_count) = scores
        .iter()
        .max_by_key(|(_, v)| *v)
        .copied()
        .unwrap_or(("neutral", 0));
    let conf = (best_count as f32) / (total as f32);
    Sentiment {
        tag: best_tag,
        confidence: conf.clamp(0.0, 1.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_is_neutral_half() {
        let s = classify("");
        assert_eq!(s.tag, "neutral");
        assert!((s.confidence - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn happy_keyword_wins() {
        let s = classify("I am so happy and glad today");
        assert_eq!(s.tag, "happy");
        assert!(s.confidence > 0.5);
    }

    #[test]
    fn question_mark_triggers_question() {
        let s = classify("Hello?");
        assert_eq!(s.tag, "question");
    }

    #[test]
    fn confidence_is_clamped() {
        let s = classify("sad sad sad sad");
        assert!(s.confidence <= 1.0 && s.confidence >= 0.0);
        assert_eq!(s.tag, "sad");
    }
}
