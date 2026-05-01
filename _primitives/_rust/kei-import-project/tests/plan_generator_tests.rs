//! Integration tests for plan_generator.
//! Uses synthetic MapEntry vectors — no real repo walk needed.

use kei_import_project::{
    map_cmd::MapEntry,
    matcher::MatchScore,
    plan_generator::{build_plan, render_markdown, PhaseStatus},
    trait_patterns::TraitKind,
};

// ─────────────────────────── helpers ───────────────────────────────────────

fn make_entry(name: &str, kind: TraitKind, confidence: f64) -> MapEntry {
    MapEntry {
        module: name.to_owned(),
        kind: "RustCrate".to_owned(),
        source_files: 3,
        best_match: Some(MatchScore {
            kind,
            confidence,
            matched_methods: vec!["store".to_owned()],
            matched_keywords: vec!["memory".to_owned()],
        }),
        all_matches: vec![],
    }
}

fn make_no_match(name: &str) -> MapEntry {
    MapEntry {
        module: name.to_owned(),
        kind: "RustCrate".to_owned(),
        source_files: 2,
        best_match: None,
        all_matches: vec![],
    }
}

// ─────────────────────────── test 1: happy path ────────────────────────────

#[test]
fn happy_path_three_families_produces_three_or_more_phases() {
    let entries = vec![
        make_entry("mem-sled", TraitKind::MemoryBackend, 0.85),
        make_entry("mem-pg", TraitKind::MemoryBackend, 0.78),
        make_entry("auth-oauth", TraitKind::AuthProvider, 0.70),
        make_entry("auth-magic", TraitKind::AuthProvider, 0.65),
        make_entry("notify-tg", TraitKind::NotifyChannel, 0.60),
        make_entry("notify-slack", TraitKind::NotifyChannel, 0.55),
        make_entry("compute-vultr", TraitKind::ComputeProvider, 0.75),
        make_entry("compute-do", TraitKind::ComputeProvider, 0.72),
    ];
    let plan = build_plan("my-project", "https://example.com/repo", &entries, 0.5);
    assert!(
        plan.phases.len() >= 3,
        "expected ≥3 phases, got {}",
        plan.phases.len()
    );
    assert_eq!(plan.project_name, "my-project");
    assert_eq!(plan.source_repo, "https://example.com/repo");
}

// ─────────────────────────── test 2: priority ordering ─────────────────────

#[test]
fn memory_backend_before_llm_backend() {
    let entries = vec![
        make_entry("llm-openai", TraitKind::LlmBackend, 0.80),
        make_entry("mem-redis", TraitKind::MemoryBackend, 0.80),
    ];
    let plan = build_plan("proj", "path", &entries, 0.5);
    let ids: Vec<&str> = plan.phases.iter().map(|p| p.id.as_str()).collect();
    // MemoryBackend is priority 0 → P0.x; LlmBackend is priority 2 → P2.x
    let mem_pos = ids.iter().position(|id| id.starts_with("P0")).unwrap_or(999);
    let llm_pos = ids.iter().position(|id| id.starts_with("P2")).unwrap_or(999);
    assert!(
        mem_pos < llm_pos,
        "expected MemoryBackend (P0.x) before LlmBackend (P2.x), got {ids:?}"
    );
}

// ─────────────────────────── test 3: threshold filter ──────────────────────

#[test]
fn modules_below_threshold_go_to_unmatched() {
    let entries = vec![
        make_entry("above-thresh", TraitKind::MemoryBackend, 0.85),
        make_entry("below-thresh", TraitKind::LlmBackend, 0.20), // below 0.3 wip floor
        make_no_match("no-match-module"),
    ];
    let plan = build_plan("proj", "path", &entries, 0.5);
    assert!(
        plan.unmatched_modules.contains(&"below-thresh".to_owned()),
        "below-threshold module should be unmatched; unmatched={:?}",
        plan.unmatched_modules
    );
    assert!(
        plan.unmatched_modules.contains(&"no-match-module".to_owned()),
        "no-match module should be unmatched"
    );
}

// ─────────────────────────── test 4: wip phase ─────────────────────────────

#[test]
fn modules_in_wip_range_produce_blocked_needs_review_phase() {
    // Confidence in [0.3, 0.5) → BlockedNeedsReview phase
    let entries = vec![
        make_entry("above-thresh", TraitKind::MemoryBackend, 0.85),
        make_entry("wip-module", TraitKind::ComputeProvider, 0.35),
    ];
    let plan = build_plan("proj", "path", &entries, 0.5);
    let wip_phases: Vec<&_> = plan
        .phases
        .iter()
        .filter(|p| p.initial_status == PhaseStatus::BlockedNeedsReview)
        .collect();
    assert!(
        !wip_phases.is_empty(),
        "expected at least one BlockedNeedsReview phase; phases={:?}",
        plan.phases.iter().map(|p| &p.id).collect::<Vec<_>>()
    );
    assert!(
        wip_phases[0].id.starts_with("Pwip."),
        "wip phase id should start with Pwip., got {}",
        wip_phases[0].id
    );
}

// ─────────────────────────── test 5: render sections ──────────────────────

#[test]
fn render_produces_all_required_sections() {
    let entries = vec![
        make_entry("mem-a", TraitKind::MemoryBackend, 0.80),
        make_entry("compute-a", TraitKind::ComputeProvider, 0.75),
        make_no_match("glue-module"),
    ];
    let plan = build_plan("test-project", "file:///tmp/repo", &entries, 0.5);
    let md = render_markdown(&plan);

    assert!(md.contains("# test-project — Migration Plan"), "missing title");
    assert!(md.contains("## STATUS BANNER"), "missing status banner");
    assert!(md.contains("| Phase |"), "missing phase table");
    assert!(md.contains("## Per-phase detail"), "missing per-phase detail");
    assert!(md.contains("## Unmatched modules"), "missing unmatched section");
    assert!(md.contains("## Follow-up"), "missing follow-up section");
    assert!(md.contains("kei-import-project execute"), "missing execute hint");
    assert!(md.contains("STATUS-TRUTH MARKER"), "missing RULE 0.16 reference");
}

// ─────────────────────────── test 6: empty input ───────────────────────────

#[test]
fn empty_input_produces_zero_phases_and_warning_in_markdown() {
    let plan = build_plan("empty-proj", "file:///tmp/empty", &[], 0.5);
    assert_eq!(plan.phases.len(), 0);
    assert_eq!(plan.unmatched_modules.len(), 0);
    assert!((plan.total_confidence_avg - 0.0).abs() < 1e-9);
    let md = render_markdown(&plan);
    assert!(
        md.contains("WARNING") || md.contains("no modules"),
        "empty plan should warn; got: {md}"
    );
}
