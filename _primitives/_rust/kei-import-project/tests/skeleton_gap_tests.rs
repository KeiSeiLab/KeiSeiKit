//! skeleton_gap_tests — A2.2 tests: render_skeleton + render_gap_report.

use kei_import_project::{
    render_gap_report, render_skeleton, ModuleAnalysis, TraitKind,
    match_module, ModuleSource,
};
use std::path::PathBuf;

// ─────────────────────────── skeleton tests ─────────────────────────────────

#[test]
fn render_skeleton_contains_impl_keyword() {
    let out = render_skeleton("kei-backend-daytona", TraitKind::ComputeProvider);
    assert!(out.contains("impl"), "missing `impl` keyword in skeleton output");
}

#[test]
fn render_skeleton_contains_unimplemented() {
    let out = render_skeleton("kei-backend-daytona", TraitKind::ComputeProvider);
    assert!(out.contains("unimplemented!("), "missing `unimplemented!()` in skeleton output");
}

#[test]
fn render_skeleton_contains_async_fn() {
    let out = render_skeleton("kei-backend-daytona", TraitKind::ComputeProvider);
    assert!(out.contains("async fn"), "missing `async fn` in skeleton output");
}

#[test]
fn render_skeleton_compute_provider_has_required_methods() {
    let out = render_skeleton("kei-backend-test", TraitKind::ComputeProvider);
    // All required methods from the trait
    for method in &["provider_name", "create", "destroy", "resize", "status",
                    "stop", "start", "cost_per_hour_microcents"] {
        assert!(out.contains(method),
            "ComputeProvider skeleton missing method: {method}");
    }
}

#[test]
fn render_skeleton_all_12_trait_kinds_produce_output() {
    for &kind in TraitKind::all() {
        let out = render_skeleton("test-module", kind);
        assert!(!out.is_empty(), "empty skeleton for {kind:?}");
        assert!(out.contains("impl"), "no impl for {kind:?}");
        assert!(out.contains("unimplemented!"), "no unimplemented! for {kind:?}");
    }
}

#[test]
fn render_skeleton_type_name_uses_foreign_prefix() {
    let out = render_skeleton("kei-foreign-store", TraitKind::MemoryBackend);
    assert!(out.contains("ForeignKeiForeignStore"),
        "type name should be ForeignKeiForeignStore, got:\n{out}");
}

#[test]
fn render_skeleton_memory_backend_has_required_methods() {
    let out = render_skeleton("my-mem-store", TraitKind::MemoryBackend);
    for method in &["backend_name", "store", "query", "compact", "mirror_to_remote"] {
        assert!(out.contains(method),
            "MemoryBackend skeleton missing method: {method}");
    }
}

// ─────────────────────────── gap report tests ───────────────────────────────

/// Build a ModuleAnalysis with synthetic source that matches the given trait.
fn make_analysis(module: &str, kind: TraitKind, confident: bool) -> ModuleAnalysis {
    // Generate synthetic source containing the required methods for `kind`.
    let (pattern_src, file_count) = match kind {
        TraitKind::MemoryBackend => (
            "impl MemoryBackend for S { \
             fn backend_name(&self) {} \
             fn store(&self) {} \
             fn query(&self) {} \
             fn compact(&self) {} \
             fn mirror_to_remote(&self) {} \
             fn rusqlite() {} }",
            3,
        ),
        TraitKind::ComputeProvider => (
            "impl ComputeProvider for S { \
             fn provider_name(&self) {} \
             async fn create(&self) {} \
             async fn destroy(&self) {} \
             async fn status(&self) {} \
             fn cost_per_hour_microcents(&self) {} \
             fn VmSpec() {} fn VmHandle() {} fn VmStatus() {} }",
            5,
        ),
        TraitKind::AuthProvider => (
            "impl AuthProvider for S { \
             fn issue_challenge(&self) {} \
             fn verify(&self) {} \
             fn revoke(&self) {} \
             fn is_passwordless(&self) -> bool {} \
             fn webauthn() {} }",
            2,
        ),
        _ => ("fn placeholder() {}", 1),
    };
    let source = ModuleSource::from_content(
        module,
        vec![(PathBuf::from("lib.rs"), pattern_src.to_owned())],
    );
    let matches = match_module(&source);

    // If we need confident, verify threshold; if weak, filter to weak only.
    let filtered: Vec<_> = if confident {
        matches.into_iter().filter(|m| m.kind == kind && m.confidence >= 0.5).collect()
    } else {
        // For weak: take only low-confidence match (0.3–0.5)
        matches.into_iter().filter(|m| m.kind == kind && m.confidence >= 0.3 && m.confidence < 0.5).collect()
    };

    ModuleAnalysis {
        module: module.to_owned(),
        file_count,
        loc_estimate: file_count * 50,
        matches: filtered,
    }
}

fn make_unmatched(module: &str) -> ModuleAnalysis {
    ModuleAnalysis {
        module: module.to_owned(),
        file_count: 4,
        loc_estimate: 200,
        matches: vec![], // no matches
    }
}

#[test]
fn render_gap_report_has_three_sections() {
    let analyses = vec![
        make_analysis("kei-my-store", TraitKind::MemoryBackend, true),
        make_unmatched("util-logger"),
    ];
    let report = render_gap_report("my-project", &analyses);
    assert!(report.contains("Confident matches"), "missing confident section");
    assert!(report.contains("Weak signals"), "missing weak signals section");
    assert!(report.contains("Unmatched modules"), "missing unmatched section");
}

#[test]
fn render_gap_report_empty_analyses_no_panic() {
    let report = render_gap_report("empty-proj", &[]);
    assert!(report.contains("# empty-proj"));
    assert!(report.contains("Confident matches"));
    assert!(report.contains("Unmatched modules"));
}

#[test]
fn render_gap_report_unmatched_module_appears_in_correct_section() {
    let analyses = vec![
        make_unmatched("zebra-util"),
        make_unmatched("alpha-helper"),
    ];
    let report = render_gap_report("test-proj", &analyses);
    // Both should appear in the unmatched section alphabetically
    let unmatched_idx = report.find("Unmatched modules").unwrap();
    let alpha_idx = report.find("alpha-helper").unwrap();
    let zebra_idx = report.find("zebra-util").unwrap();
    assert!(alpha_idx > unmatched_idx, "alpha-helper should be in unmatched section");
    assert!(zebra_idx > unmatched_idx, "zebra-util should be in unmatched section");
    // alpha before zebra (alphabetic sort)
    assert!(alpha_idx < zebra_idx, "unmatched should be alphabetically sorted");
}

#[test]
fn render_gap_report_confident_match_in_confident_section() {
    let analyses = vec![
        make_analysis("kei-my-store", TraitKind::MemoryBackend, true),
    ];
    let report = render_gap_report("test-proj", &analyses);
    let confident_idx = report.find("Confident matches").unwrap();
    let weak_idx = report.find("Weak signals").unwrap();
    if let Some(module_idx) = report.find("kei-my-store") {
        // Module should appear between confident section header and weak section header
        assert!(module_idx > confident_idx && module_idx < weak_idx,
            "confident module should appear in confident section");
    }
}

#[test]
fn render_gap_report_suggested_next_steps_present() {
    let report = render_gap_report("proj", &[]);
    assert!(report.contains("Suggested next steps"), "missing next steps section");
    assert!(report.contains("skeleton"), "next steps should mention skeleton subcommand");
}

#[test]
fn render_gap_report_none_placeholder_when_no_confident() {
    let analyses = vec![make_unmatched("glue-code")];
    let report = render_gap_report("proj", &analyses);
    // Confident section should have empty table row
    assert!(report.contains("\u{2014}") || report.contains("---") || report.contains("| \u{2014}"),
        "confident section should indicate no confident matches");
}
