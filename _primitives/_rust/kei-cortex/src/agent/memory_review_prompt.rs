//! Background-review prompt template.
//!
//! Constructor Pattern: a single immutable string + a small builder
//! adapting it to KeiSei voice. Ported from Hermes
//! `run_agent.py:3147-3156`. The template intentionally bounds the
//! review agent to two questions and one short-circuit phrase
//! (`"Nothing to save."`) — that bound is what makes the background
//! pass cheap and reliably terminating.

/// Verbatim review-prompt body. Adapted from Hermes
/// `_MEMORY_REVIEW_PROMPT` with KeiSei wording (third-person "the user"
/// rather than second-person, matches the kei-cortex persona surface).
pub const REVIEW_PROMPT: &str = r#"Review the conversation above and consider saving to memory if appropriate.

Focus on:
1. Has the user revealed things about themselves — their persona, desires, preferences, or personal details worth remembering?
2. Has the user expressed expectations about how you should behave, their work style, or ways they want you to operate?

If something stands out, save it using the memory tool.
If nothing is worth saving, respond with exactly the phrase:
Nothing to save.
and stop. Do not produce any other output."#;

/// Short-circuit phrase the review agent emits when the conversation
/// contains nothing memory-worthy. The scheduler watches for this
/// exact string (case-insensitive, trimmed) to skip persistence work.
pub const NOTHING_TO_SAVE: &str = "Nothing to save.";

/// True when the agent's reply is the recognised short-circuit.
pub fn is_nothing_to_save(reply: &str) -> bool {
    let trimmed = reply.trim();
    trimmed.eq_ignore_ascii_case(NOTHING_TO_SAVE)
        || trimmed.eq_ignore_ascii_case("nothing to save")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_circuit_recognised() {
        assert!(is_nothing_to_save("Nothing to save."));
        assert!(is_nothing_to_save("nothing to save"));
        assert!(is_nothing_to_save("  Nothing to save.\n"));
    }

    #[test]
    fn other_replies_not_short_circuit() {
        assert!(!is_nothing_to_save("Saved a note about Bali."));
        assert!(!is_nothing_to_save(""));
    }

    #[test]
    fn template_mentions_both_focus_questions() {
        assert!(REVIEW_PROMPT.contains("persona"));
        assert!(REVIEW_PROMPT.contains("expectations"));
        assert!(REVIEW_PROMPT.contains("Nothing to save"));
    }
}
