//! Integration tests for phase_prompt.
//! All fixtures are synthetic — no real plan files touched.

use kei_import_project::phase_prompt::{build_phase_prompt, render_json};
use kei_import_project::plan_parser::{ParsedModule, ParsedPhase};

fn make_phase(id: &str, family: &str, priority: u8, mods: &[(&str, f64)]) -> ParsedPhase {
    ParsedPhase {
        id: id.to_owned(),
        trait_family: family.to_owned(),
        priority,
        status: "scaffolding".to_owned(),
        modules: mods
            .iter()
            .map(|(n, c)| ParsedModule { name: n.to_string(), confidence: *c })
            .collect(),
    }
}

#[test]
fn prompt_contains_required_sections() {
    let phase =
        make_phase("P0.1", "MemoryBackend", 0, &[("mem-sled", 0.85), ("mem-pg", 0.78)]);
    let p = build_phase_prompt(&phase);
    assert!(p.prompt_text.contains("MemoryBackend"), "missing trait family");
    assert!(p.prompt_text.contains("MUST NOT invoke git"), "missing git ban");
    assert!(p.prompt_text.contains("STATUS-TRUTH MARKER"), "missing RULE 0.16 ref");
    assert!(p.prompt_text.contains("cargo check --workspace"), "missing cargo check");
}

#[test]
fn prompt_contains_all_modules() {
    let phase = make_phase(
        "P1.1",
        "ComputeProvider",
        1,
        &[("compute-vultr", 0.75), ("compute-do", 0.72)],
    );
    let p = build_phase_prompt(&phase);
    assert!(p.prompt_text.contains("compute-vultr"), "missing first module");
    assert!(p.prompt_text.contains("compute-do"), "missing second module");
    assert_eq!(p.modules.len(), 2);
}

#[test]
fn agent_type_defaults_to_code_implementer_rust() {
    let phase = make_phase("P2.1", "NotifyChannel", 2, &[("notify-tg", 0.60)]);
    let p = build_phase_prompt(&phase);
    assert_eq!(p.agent_type, "code-implementer-rust");
}

#[test]
fn render_json_produces_valid_array_with_correct_phase_ids() {
    let phase1 = make_phase("P0.1", "MemoryBackend", 0, &[("mem-sled", 0.85)]);
    let phase2 = make_phase("P1.1", "ComputeProvider", 1, &[("compute-vultr", 0.75)]);
    let prompts = vec![build_phase_prompt(&phase1), build_phase_prompt(&phase2)];
    let json = render_json(&prompts).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["phase_id"], "P0.1");
    assert_eq!(arr[1]["phase_id"], "P1.1");
}

#[test]
fn estimated_tokens_scale_with_module_count() {
    let one_mod = make_phase("P0.1", "MemoryBackend", 0, &[("mem-a", 0.80)]);
    let three_mod = make_phase(
        "P0.2",
        "MemoryBackend",
        0,
        &[("mem-a", 0.80), ("mem-b", 0.75), ("mem-c", 0.70)],
    );
    let p1 = build_phase_prompt(&one_mod);
    let p3 = build_phase_prompt(&three_mod);
    assert!(
        p3.estimated_tokens_out > p1.estimated_tokens_out,
        "3-module phase should have higher token estimate than 1-module phase"
    );
}
