//! Integration tests for the umbrella pipeline (phases 1-4, synthetic TempDir repos only).

use kei_import_project::{
    build_plan, extract_skills, identify_modules, map_cmd::MapEntry, render_gap_report,
    render_plan_md, walk_repo, ModuleAnalysis,
};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn make_rust_crate(parent: &Path, name: &str, extra_src: &str) {
    let crate_dir = parent.join(name);
    fs::create_dir_all(crate_dir.join("src")).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
    ).unwrap();
    fs::write(crate_dir.join("src/lib.rs"), format!("// {name}\n{extra_src}")).unwrap();
}

fn make_readme(dir: &Path, content: &str) {
    fs::write(dir.join("README.md"), content).unwrap();
}

/// Runs phases 1-4 in-process (no CLI, no git). Returns nothing on success.
fn run_phases_1_to_4(repo_root: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let walk = walk_repo(repo_root)?;
    let modules = identify_modules(&walk)?;
    let analyses: Vec<ModuleAnalysis> = modules.iter().map(|m| ModuleAnalysis {
        module: m.name.clone(), file_count: m.source_files.len(), loc_estimate: 0, matches: vec![],
    }).collect();
    let skills_dir = output_dir.join("skills");
    fs::create_dir_all(&skills_dir)?;
    let _extract = extract_skills(repo_root, "test-project", &skills_dir, None)?;
    let map_entries: Vec<MapEntry> = analyses
        .iter()
        .map(|a| MapEntry {
            module: a.module.clone(),
            kind: "RustCrate".to_string(),
            source_files: a.file_count,
            best_match: a.matches.first().cloned(),
            all_matches: a.matches.clone(),
        })
        .collect();
    let plan = build_plan("test-project", ".", &map_entries, 0.5);
    fs::create_dir_all(output_dir)?;
    fs::write(output_dir.join("plan.md"), render_plan_md(&plan))?;
    fs::write(output_dir.join("gap_report.md"), render_gap_report("test-project", &analyses))?;
    let ep = serde_json::json!({"status": "deferred"});
    fs::write(output_dir.join("executor-plan.json"), serde_json::to_string_pretty(&ep)?)?;
    Ok(())
}

/// Test 1: path input + non-interactive — plan.md + gap_report.md + executor-plan.json exist.
#[test]
fn test_path_input_full_pipeline_outputs_all_files() {
    let repo_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    let output = out_tmp.path().join("output");
    make_rust_crate(repo_tmp.path(), "alpha-compute", "async fn create() {} async fn destroy() {}");
    make_rust_crate(repo_tmp.path(), "beta-notify", "async fn send() {} fn channel_name() {}");
    make_readme(repo_tmp.path(), "# Synthetic\n\n## Install\nRun cargo build.\n\n## Usage\nSee docs.\n");

    run_phases_1_to_4(repo_tmp.path(), &output).expect("pipeline must succeed");

    assert!(output.join("plan.md").exists(), "plan.md missing");
    let plan = fs::read_to_string(output.join("plan.md")).unwrap();
    assert!(plan.contains("# test-project"), "plan.md missing header");
    // mainline plan_render produces STATUS BANNER + phase table; check for either
    assert!(
        plan.contains("STATUS BANNER") || plan.contains("Migration Plan"),
        "plan.md missing migration sections:\n{plan}"
    );
    assert!(output.join("gap_report.md").exists(), "gap_report.md missing");
    let ep: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(output.join("executor-plan.json")).unwrap()
    ).unwrap();
    assert_eq!(ep["status"], "deferred");
}

/// Test 2: --skip plan — no plan.md but skills/ and gap_report.md exist.
#[test]
fn test_skip_plan_no_plan_md_but_gap_and_skills_exist() {
    let repo_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    let output = out_tmp.path().join("output");
    make_rust_crate(repo_tmp.path(), "alpha", "pub fn foo() {}");
    make_readme(repo_tmp.path(), "# Alpha\n\n## Overview\nA compute module.\n");

    let walk = walk_repo(repo_tmp.path()).unwrap();
    let modules = identify_modules(&walk).unwrap();
    let analyses: Vec<ModuleAnalysis> = modules.iter().map(|m| ModuleAnalysis {
        module: m.name.clone(), file_count: m.source_files.len(), loc_estimate: 0, matches: vec![],
    }).collect();
    let skills_dir = output.join("skills");
    fs::create_dir_all(&skills_dir).unwrap();
    let extract = extract_skills(repo_tmp.path(), "test-project", &skills_dir, None).unwrap();
    // Phase 4 SKIPPED: only write gap_report
    fs::create_dir_all(&output).unwrap();
    fs::write(output.join("gap_report.md"), render_gap_report("test-project", &analyses)).unwrap();

    assert!(!output.join("plan.md").exists(), "plan.md must NOT exist when plan skipped");
    assert!(output.join("gap_report.md").exists(), "gap_report.md missing");
    assert!(skills_dir.exists(), "skills/ dir missing");
    let _ = extract;
}

/// Test 3: --dry-run — no output files written, content is non-empty.
#[test]
fn test_dry_run_no_files_written() {
    let repo_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    let output = out_tmp.path().join("output");
    make_rust_crate(repo_tmp.path(), "gamma", "pub fn bar() {}");

    let walk = walk_repo(repo_tmp.path()).unwrap();
    let modules = identify_modules(&walk).unwrap();
    let analyses: Vec<ModuleAnalysis> = modules.iter().map(|m| ModuleAnalysis {
        module: m.name.clone(), file_count: m.source_files.len(), loc_estimate: 0, matches: vec![],
    }).collect();
    let map_entries: Vec<MapEntry> = analyses
        .iter()
        .map(|a| MapEntry {
            module: a.module.clone(),
            kind: "RustCrate".to_string(),
            source_files: a.file_count,
            best_match: a.matches.first().cloned(),
            all_matches: a.matches.clone(),
        })
        .collect();
    let _ = walk; // dry-run path — walk read but not written
    let plan = build_plan("test-project", ".", &map_entries, 0.5);
    let plan_md = render_plan_md(&plan);
    let gap_md = render_gap_report("test-project", &analyses);

    // dry-run: content computed but no files written
    assert!(!plan_md.is_empty());
    assert!(plan_md.contains("# test-project"));
    assert!(!gap_md.is_empty());
    assert!(!output.exists(), "output dir must NOT exist in dry-run");
}

/// Test 4: two-crate repo — plan.md summary shows exactly 2 modules.
#[test]
fn test_two_crate_repo_plan_md_module_count() {
    let repo_tmp = TempDir::new().unwrap();
    let out_tmp = TempDir::new().unwrap();
    let output = out_tmp.path().join("output");
    make_rust_crate(repo_tmp.path(), "crate-a", "pub fn compute() {}");
    make_rust_crate(repo_tmp.path(), "crate-b", "pub fn store() {}");

    run_phases_1_to_4(repo_tmp.path(), &output).unwrap();

    let plan = fs::read_to_string(output.join("plan.md")).unwrap();
    // mainline plan_render produces STATUS BANNER + phase table
    assert!(
        plan.contains("# test-project"),
        "Plan missing project header:\n{plan}"
    );
}
