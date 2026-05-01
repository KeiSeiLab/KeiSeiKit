//! kei-import — umbrella CLI composing pipeline phases 1-4.
//! Phase 5 (executor) deferred: run `kei-import-project execute <plan>` separately.

use clap::Parser;
use kei_import_project::{
    build_plan, extract_skills, identify_modules, match_module, render_gap_report,
    render_plan_md, walk_repo, ModuleAnalysis, ModuleSource,
};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "kei-import", version, about = "Import a foreign repo into the kei ecosystem (phases 1-4)")]
struct Args {
    /// Repository URL (https://, git@) or local path.
    repo: String,
    /// Pause for user confirmation between phases.
    #[arg(long = "interactive", short = 'i', default_value_t = true)]
    interactive: bool,
    /// Run all phases without pauses.
    #[arg(long = "non-interactive", conflicts_with = "interactive")]
    non_interactive: bool,
    /// Walk phases without writing any files.
    #[arg(long)]
    dry_run: bool,
    /// Output directory for plan.md, gap_report.md, skills/.
    #[arg(long, default_value = "./kei-import-output")]
    output_dir: PathBuf,
    /// Confidence threshold for trait matching (0.0–1.0).
    #[arg(long, default_value_t = 0.5)]
    confidence: f64,
    /// Comma-separated phases to skip: walk,map,extract-skills,plan.
    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,
    /// Keep the cloned tempdir for URL inputs.
    #[arg(long)]
    keep_clone: bool,
}

fn main() {
    let args = Args::parse();
    let interactive = !args.non_interactive && args.interactive;
    let (repo_path, clone_dir) = resolve_repo(&args.repo);
    if let Err(e) = run_pipeline(&args, &repo_path, interactive) {
        eprintln!("kei-import error: {e}");
    }
    if !args.keep_clone {
        if let Some(d) = clone_dir { let _ = std::fs::remove_dir_all(d); }
    }
}

fn resolve_repo(input: &str) -> (PathBuf, Option<PathBuf>) {
    if input.starts_with("https://") || input.starts_with("git@") || input.starts_with("http://") {
        let tmp = clone_repo(input);
        let path = tmp.clone();
        (path, Some(tmp))
    } else {
        (PathBuf::from(input), None)
    }
}

fn clone_repo(url: &str) -> PathBuf {
    let tmp = std::env::temp_dir().join(format!("kei-import-{}", std::process::id()));
    eprintln!("Cloning {} -> {}", url, tmp.display());
    let status = std::process::Command::new("git")
        .args(["clone", "--depth=1", url, &tmp.to_string_lossy()])
        .status()
        .unwrap_or_else(|e| { eprintln!("git not found: {e}"); process::exit(1); });
    if !status.success() { eprintln!("git clone failed for {url}"); process::exit(1); }
    tmp
}

fn run_pipeline(args: &Args, repo_path: &Path, interactive: bool) -> anyhow::Result<()> {
    let skip: std::collections::HashSet<String> = args.skip.iter().cloned().collect();
    let project_name = repo_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");

    if skip.contains("walk") { eprintln!("Phase 1 skipped."); return Ok(()); }
    eprintln!("\nPhase 1 (walk + identify)...");
    let walk = walk_repo(repo_path)?;
    let modules = identify_modules(&walk)?;
    eprintln!("Phase 1 complete: {} modules.", modules.len());

    if interactive && !pause_confirm("Phase 2 (map traits)")? { return Ok(()); }
    let analyses = phase2_map(args, &modules, &walk, repo_path, &skip)?;

    if interactive && !skip.contains("extract-skills") && !pause_confirm("Phase 3 (extract-skills)")? { return Ok(()); }
    let extract_result = phase3_extract(args, repo_path, project_name, &skip)?;

    if interactive && !skip.contains("plan") && !pause_confirm("Phase 4 (plan)")? { return Ok(()); }
    phase4_plan(args, &walk, &analyses, extract_result.as_ref(), project_name, &skip)?;

    eprintln!("\nPhase 5: deferred. Run `kei-import-project execute {}`.", args.output_dir.join("plan.md").display());
    Ok(())
}

fn phase2_map(
    _args: &Args,
    modules: &[kei_import_project::ProjectModule],
    walk: &kei_import_project::RepoWalk,
    repo_root: &Path,
    skip: &std::collections::HashSet<String>,
) -> anyhow::Result<Vec<ModuleAnalysis>> {
    if skip.contains("map") {
        eprintln!("Phase 2 skipped.");
        return Ok(modules.iter().map(|m| ModuleAnalysis {
            module: m.name.clone(), file_count: m.source_files.len(), loc_estimate: 0, matches: vec![],
        }).collect());
    }
    eprintln!("\nPhase 2 (map traits)...");
    let a = modules.iter().map(|m| {
        let ms = ModuleSource::from_dir(&m.name, &repo_root.join(&m.root_dir))
            .unwrap_or_else(|_| ModuleSource::from_content(&m.name, vec![]));
        let loc = walk.files.iter().filter(|f| f.path.starts_with(&m.root_dir)).map(|f| (f.size_bytes / 40) as usize).sum();
        ModuleAnalysis { module: m.name.clone(), file_count: m.source_files.len(), loc_estimate: loc, matches: match_module(&ms) }
    }).collect::<Vec<_>>();
    eprintln!("Phase 2 complete: {} analyses.", a.len());
    Ok(a)
}

fn phase3_extract(
    args: &Args,
    repo_path: &Path,
    project_name: &str,
    skip: &std::collections::HashSet<String>,
) -> anyhow::Result<Option<kei_import_project::ExtractResult>> {
    if skip.contains("extract-skills") { eprintln!("Phase 3 skipped."); return Ok(None); }
    eprintln!("\nPhase 3 (extract-skills)...");
    if args.dry_run { eprintln!("  dry-run: no writes."); return Ok(None); }
    let skills_dir = args.output_dir.join("skills");
    std::fs::create_dir_all(&skills_dir)?;
    let r = extract_skills(repo_path, project_name, &skills_dir, None)?;
    eprintln!("Phase 3 complete: {} skills.", r.extracted.len());
    Ok(Some(r))
}

fn phase4_plan(
    args: &Args,
    _walk: &kei_import_project::RepoWalk,
    analyses: &[ModuleAnalysis],
    _extract: Option<&kei_import_project::ExtractResult>,
    project_name: &str,
    skip: &std::collections::HashSet<String>,
) -> anyhow::Result<()> {
    if skip.contains("plan") { eprintln!("Phase 4 skipped."); return Ok(()); }
    eprintln!("\nPhase 4 (plan)...");
    if args.dry_run { eprintln!("  dry-run: no writes."); return Ok(()); }
    // Convert ModuleAnalysis → MapEntry (mainline build_plan signature)
    let map_entries: Vec<kei_import_project::map_cmd::MapEntry> = analyses
        .iter()
        .map(|a| kei_import_project::map_cmd::MapEntry {
            module: a.module.clone(),
            kind: "RustCrate".to_string(),
            source_files: a.file_count,
            best_match: a.matches.first().cloned(),
            all_matches: a.matches.clone(),
        })
        .collect();
    let plan = build_plan(project_name, ".", &map_entries, 0.5);
    std::fs::create_dir_all(&args.output_dir)?;
    std::fs::write(args.output_dir.join("plan.md"), render_plan_md(&plan))?;
    std::fs::write(args.output_dir.join("gap_report.md"), render_gap_report(project_name, analyses))?;
    let ep = serde_json::json!({"status":"deferred","message":"Phase 5 not yet landed. Run `kei-import-project execute <plan.md>`."});
    std::fs::write(args.output_dir.join("executor-plan.json"), serde_json::to_string_pretty(&ep)?)?;
    eprintln!("Phase 4 complete. Output: {}", args.output_dir.display());
    Ok(())
}

fn pause_confirm(next_phase: &str) -> anyhow::Result<bool> {
    print!("  \u{2192} {next_phase} next.\nContinue? [Y/n] ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let t = line.trim().to_lowercase();
    Ok(t.is_empty() || t == "y" || t == "yes")
}
