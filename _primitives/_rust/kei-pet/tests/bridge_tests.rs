//! Integration tests for `bridge::compose_prompt_with_pet`.
//!
//! Fixtures load from `examples/full.toml` via `include_str!` — this is the
//! only reliable way to test against a known-good manifest until a shared
//! `templates` module exists.

use kei_pet::{parse, AgentSpawnRequest, compose_prompt_with_pet};

const FULL: &str = include_str!("../examples/full.toml");

fn base_req(with_pet: bool) -> AgentSpawnRequest {
    let pet = if with_pet {
        Some(parse(FULL).expect("examples/full.toml must validate"))
    } else {
        None
    };
    AgentSpawnRequest {
        role: "code-implementer".to_string(),
        pet_manifest: pet,
        task_body: "Refactor the foo module into three cubes.".to_string(),
        base_prompt: "You are a senior Rust engineer.".to_string(),
    }
}

#[test]
fn compose_prompt_without_pet_returns_base_plus_body() {
    let req = base_req(false);
    let out = compose_prompt_with_pet(&req);

    // Must contain both the base prompt and the task body verbatim.
    assert!(
        out.contains("You are a senior Rust engineer."),
        "base prompt missing from composed output:\n{out}"
    );
    assert!(
        out.contains("Refactor the foo module into three cubes."),
        "task body missing from composed output:\n{out}"
    );

    // Must NOT contain the persona-overlay header.
    assert!(
        !out.contains("## Persona overlay"),
        "persona overlay section leaked in without a manifest:\n{out}"
    );
}

#[test]
fn compose_prompt_with_pet_includes_voice_tone_string() {
    let req = base_req(true);
    let out = compose_prompt_with_pet(&req);

    // full.toml: tone_primary = "dry" → overlay emits "Primary tone: dry."
    assert!(
        out.contains("Primary tone: dry"),
        "expected primary tone 'dry' in overlay output:\n{out}"
    );
    // Header must appear exactly once — overlay was injected.
    assert!(
        out.contains("## Persona overlay"),
        "persona overlay header missing when manifest present:\n{out}"
    );
}

#[test]
fn pet_forbidden_topics_appear_in_system_prompt() {
    let req = base_req(true);
    let out = compose_prompt_with_pet(&req);

    // full.toml: forbidden.topics = ["politics", "crypto-hype"]
    assert!(
        out.contains("politics"),
        "forbidden topic 'politics' not surfaced by overlay:\n{out}"
    );
    assert!(
        out.contains("crypto-hype"),
        "forbidden topic 'crypto-hype' not surfaced by overlay:\n{out}"
    );
    // And the "Never engage with" lead-in from overlay.rs must be present.
    assert!(
        out.contains("Never engage with:"),
        "forbidden-topics lead-in phrase missing:\n{out}"
    );
}
