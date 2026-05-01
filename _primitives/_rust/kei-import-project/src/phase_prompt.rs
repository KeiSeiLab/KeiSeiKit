//! phase_prompt — generates agent-prompt JSON per migration phase.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::plan_parser::ParsedPhase;
use anyhow::Result;
use serde::Serialize;

// ─────────────────────────── public types ──────────────────────────────────

/// The fully-rendered prompt spec for one migration phase.
#[derive(Debug, Clone, Serialize)]
pub struct PhasePrompt {
    pub agent_type: String,
    pub phase_id: String,
    pub trait_family: String,
    pub modules: Vec<String>,
    pub prompt_text: String,
    pub estimated_tokens_in: u64,
    pub estimated_tokens_out: u64,
}

// ─────────────────────────── public API ────────────────────────────────────

/// Build a `PhasePrompt` from a `ParsedPhase`.
pub fn build_phase_prompt(phase: &ParsedPhase) -> PhasePrompt {
    let modules: Vec<String> = phase.modules.iter().map(|m| m.name.clone()).collect();
    let prompt_text = render_prompt_text(phase, &modules);
    let estimated_tokens_in = estimate_tokens_in(&prompt_text);
    let estimated_tokens_out = estimate_tokens_out(&modules);
    PhasePrompt {
        agent_type: "code-implementer-rust".to_owned(),
        phase_id: phase.id.clone(),
        trait_family: phase.trait_family.clone(),
        modules,
        prompt_text,
        estimated_tokens_in,
        estimated_tokens_out,
    }
}

/// Render all prompts as a JSON array string.
pub fn render_json(prompts: &[PhasePrompt]) -> Result<String> {
    Ok(serde_json::to_string_pretty(prompts)?)
}

// ─────────────────────────── internals ─────────────────────────────────────

fn render_prompt_text(phase: &ParsedPhase, modules: &[String]) -> String {
    let module_list = build_module_list(phase);
    let first_module = modules.first().map(|s| s.as_str()).unwrap_or("unknown");
    let family_lower = phase.trait_family.to_lowercase().replace("backend", "-backend");
    format!(
        "You MUST NOT invoke git/gh/bash beyond cargo check/test. \
Implement the {trait_family} trait for the following foreign modules \
ported into KeiSeiKit substrate.\n\n\
Modules to port (priority {priority}):\n\
{module_list}\n\n\
Tasks:\n\
1. Read existing trait definition in kei-runtime-core/src/traits/{family_lower}.rs\n\
2. For each module, generate the impl skeleton (use kei-import-project \
skeleton --module <m> --trait-name {trait_family} as starting point)\n\
3. Wire actual logic — replace unimplemented!() bodies\n\
4. Run cargo check + cargo test per crate\n\
5. End report with === STATUS-TRUTH MARKER === per RULE 0.16\n\n\
Verify gate:\n\
- cargo check --workspace PASS\n\
- cargo test -p {first_module} PASS\n\
- All new files ≤200 LOC, fns ≤30 LOC",
        trait_family = phase.trait_family,
        priority = phase.priority,
        module_list = module_list,
        family_lower = family_lower,
        first_module = first_module,
    )
}

fn build_module_list(phase: &ParsedPhase) -> String {
    phase
        .modules
        .iter()
        .map(|m| format!("- {} (confidence {:.2})", m.name, m.confidence))
        .collect::<Vec<_>>()
        .join("\n")
}

fn estimate_tokens_in(prompt: &str) -> u64 {
    // Rough heuristic: ~4 chars per token [ESTIMATE-HTC: no benchmark, standard GPT-4 rule of thumb]
    (prompt.len() as u64).saturating_div(4).max(50)
}

fn estimate_tokens_out(modules: &[String]) -> u64 {
    // Each module ~800 tokens of Rust skeleton [ESTIMATE-HTC: based on skeleton.rs typical output]
    (modules.len() as u64) * 800
}

// ─────────────────────────── unit tests ────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan_parser::ParsedModule;

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
        let phase = make_phase(
            "P0.1",
            "MemoryBackend",
            0,
            &[("mem-sled", 0.85), ("mem-pg", 0.78)],
        );
        let p = build_phase_prompt(&phase);
        assert!(p.prompt_text.contains("MemoryBackend"), "missing trait family");
        assert!(p.prompt_text.contains("MUST NOT invoke git"), "missing git ban");
        assert!(p.prompt_text.contains("STATUS-TRUTH MARKER"), "missing RULE 0.16 ref");
        assert!(p.prompt_text.contains("cargo check --workspace"), "missing cargo check");
    }

    #[test]
    fn prompt_contains_all_modules() {
        let phase =
            make_phase("P1.1", "ComputeProvider", 1, &[("compute-vultr", 0.75), ("compute-do", 0.72)]);
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
    fn render_json_produces_valid_array() {
        let phase1 =
            make_phase("P0.1", "MemoryBackend", 0, &[("mem-sled", 0.85)]);
        let phase2 =
            make_phase("P1.1", "ComputeProvider", 1, &[("compute-vultr", 0.75)]);
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
        assert!(p3.estimated_tokens_out > p1.estimated_tokens_out);
    }
}
