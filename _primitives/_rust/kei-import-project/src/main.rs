//! kei-import-project CLI — decompose foreign codebases into module tables.
//!
//! Subcommands:
//!   decompose <PATH>              — walk + identify modules, print markdown table
//!   register <PATH> [--registry-db] — walk + identify + write to kei-registry
//!   map <PATH> [--threshold] [--format] — walk + match traits, print summary
//!   extract-skills                — stub (Phase 3)
//!   plan                          — stub (Phase 4)
//!   execute                       — stub (Phase 5)

use clap::{Parser, Subcommand};
use kei_import_project::{
    execute_cmd, extract_skills, identify_modules, map_cmd, module_source::ModuleSource, plan_cmd,
    register_modules, render_skeleton, walk_repo, ExtractedSkill, TraitKind,
};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "kei-import-project", version, about = "Foreign project ingestion runtime")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Walk a path and identify language modules, printing a markdown summary table.
    Decompose {
        /// Path to the repository root.
        path: PathBuf,
    },
    /// Walk a repo, identify modules, and register each in kei-registry.
    Register {
        /// Path to the repository root.
        path: PathBuf,
        /// Override registry SQLite path (default: $KEI_REGISTRY_DB or ~/.claude/registry.sqlite).
        #[arg(long)]
        registry_db: Option<PathBuf>,
    },
    /// Map modules to kei-runtime-core traits via confidence-scored matching.
    Map {
        /// Path to the repository root.
        path: PathBuf,
        /// Only show matches at or above this confidence (default 0.3).
        #[arg(long, default_value = "0.3")]
        threshold: f64,
        /// Output format: markdown (default) or json.
        #[arg(long, default_value = "markdown")]
        format: String,
    },
    /// Generate a Rust impl skeleton for a module implementing a specific trait.
    Skeleton {
        /// Path to the module directory (or crate root).
        #[arg(long)]
        module: PathBuf,
        /// Trait to implement (e.g. compute-provider, memory-backend).
        #[arg(long, name = "trait")]
        trait_name: String,
        /// Output file path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Extract skills from foreign repo's README + docs/*.md and register in kei-registry.
    ExtractSkills {
        /// Path to the foreign repo to analyse.
        repo_path: PathBuf,
        /// Project slug (default: basename of repo_path).
        #[arg(long)]
        project_slug: Option<String>,
        /// Directory to write canonical SKILL.md fragments into.
        #[arg(long)]
        fragments_dir: Option<PathBuf>,
        /// Path to kei-registry SQLite (default: $KEI_REGISTRY_DB or ~/.claude/registry.sqlite).
        #[arg(long)]
        registry_db: Option<PathBuf>,
        /// Dry run: print plan, write nothing, register nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Generate a migration plan from a repo's architecture map.
    Plan {
        /// Path to the repository root.
        path: PathBuf,
        /// Human-readable project name (default: basename of path).
        #[arg(long)]
        project_name: Option<String>,
        /// Confidence threshold; modules below → unmatched (default 0.5).
        #[arg(long, default_value = "0.5")]
        threshold: f64,
        /// Output file path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Execute the migration plan: parse plan.md, emit per-phase agent prompts.
    Execute {
        /// Path to the plan.md file produced by the `plan` subcommand.
        plan_path: PathBuf,
        /// Override kei-ledger SQLite path (default: $KEI_LEDGER_DB or ~/.claude/agents/ledger.sqlite).
        #[arg(long)]
        ledger_db: Option<PathBuf>,
        /// Pre-register each phase as a 'queued' row in kei-ledger.
        #[arg(long)]
        prereg: bool,
        /// Output format: markdown (default) or json.
        #[arg(long, default_value = "markdown")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Decompose { path } => run_decompose(&path),
        Cmd::Register { path, registry_db } => run_register(&path, registry_db.as_deref()),
        Cmd::Map { path, threshold, format } => run_map(&path, threshold, &format),
        Cmd::Skeleton { module, trait_name, output } => {
            run_skeleton(&module, &trait_name, output.as_deref());
        }
        Cmd::Plan { path, project_name, threshold, output } => {
            run_plan(&path, project_name.as_deref(), threshold, output.as_deref());
        }
        Cmd::ExtractSkills { repo_path, project_slug, fragments_dir, registry_db, dry_run } => {
            run_extract_skills(&repo_path, project_slug, fragments_dir, registry_db, dry_run);
        }
        Cmd::Execute { plan_path, ledger_db, prereg, format } => {
            if let Err(e) = execute_cmd::run_execute(&plan_path, ledger_db.as_deref(), prereg, &format) {
                eprintln!("execute failed: {e}");
                process::exit(1);
            }
        }
    }
}

fn run_decompose(path: &PathBuf) {
    let walk = match walk_repo(path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("walk failed: {e}");
            process::exit(1);
        }
    };
    let modules = match identify_modules(&walk) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("identify failed: {e}");
            process::exit(1);
        }
    };

    println!("| Module | Kind | Root | Source files |");
    println!("|---|---|---|---|");
    for m in &modules {
        println!(
            "| {} | {:?} | {} | {} |",
            m.name,
            m.kind,
            m.root_dir.display(),
            m.source_files.len()
        );
    }
    eprintln!("\n{} module(s) found in {}", modules.len(), path.display());
}

fn run_map(path: &PathBuf, threshold: f64, format: &str) {
    let repo_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("repo");
    let entries = match map_cmd::build_map(path, threshold) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("map failed: {e}");
            process::exit(1);
        }
    };
    match format {
        "json" => match map_cmd::render_json(&entries) {
            Ok(s) => println!("{s}"),
            Err(e) => {
                eprintln!("json render failed: {e}");
                process::exit(1);
            }
        },
        _ => print!("{}", map_cmd::render_markdown(&entries, threshold, repo_name)),
    }
    let confident = entries.iter().filter(|e| e.best_match.is_some()).count();
    eprintln!("\n{} module(s), {} with confident match (threshold ≥ {threshold:.2})",
        entries.len(), confident);
}

fn run_register(path: &PathBuf, registry_db: Option<&std::path::Path>) {
    let walk = match walk_repo(path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("walk failed: {e}");
            process::exit(1);
        }
    };
    let modules = match identify_modules(&walk) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("identify failed: {e}");
            process::exit(1);
        }
    };
    let result = match register_modules(&modules, path, registry_db) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("register failed: {e}");
            process::exit(1);
        }
    };
    println!(
        "{} registered, {} superseded, {} unchanged",
        result.registered, result.superseded, result.unchanged
    );
    eprintln!(
        "\n{} module(s) processed in {}",
        modules.len(),
        path.display()
    );
}


fn run_plan(path: &std::path::Path, project_name: Option<&str>, threshold: f64, output: Option<&std::path::Path>) {
    let name = project_name
        .map(|s| s.to_owned())
        .unwrap_or_else(|| {
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("project")
                .to_owned()
        });
    if let Err(e) = plan_cmd::run_plan(path, &name, threshold, output) {
        eprintln!("plan failed: {e}");
        process::exit(1);
    }
}

fn run_skeleton(module: &std::path::Path, trait_name: &str, output: Option<&std::path::Path>) {
    let kind = match TraitKind::from_str_ci(trait_name) {
        Some(k) => k,
        None => {
            eprintln!("unknown trait: {trait_name}");
            eprintln!("valid: compute-provider, auth-provider, notify-channel, git-backend,");
            eprintln!("       llm-backend, service-manager, memory-backend, scheduler,");
            eprintln!("       network-mode, backup, cost-guard, observability");
            process::exit(1);
        }
    };
    let module_name = module
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown-module");

    let _source = ModuleSource::from_dir(module_name, module).ok();
    let skeleton_src = render_skeleton(module_name, kind);

    match output {
        Some(path) => {
            if let Err(e) = std::fs::write(path, &skeleton_src) {
                eprintln!("write failed: {e}");
                process::exit(1);
            }
            eprintln!("wrote skeleton for {module_name} to {}", path.display());
        }
        None => print!("{skeleton_src}"),
    }
}

fn run_extract_skills(
    repo_path: &std::path::Path,
    project_slug: Option<String>,
    fragments_dir: Option<PathBuf>,
    registry_db: Option<PathBuf>,
    dry_run: bool,
) {
    let slug = project_slug.unwrap_or_else(|| {
        repo_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project")
            .to_string()
    });
    let frags = fragments_dir
        .unwrap_or_else(|| dirs_claude_home().join("registry-fragments"));
    let db = if dry_run {
        None
    } else {
        Some(
            registry_db
                .or_else(|| std::env::var("KEI_REGISTRY_DB").ok().map(PathBuf::from))
                .unwrap_or_else(|| dirs_claude_home().join("registry.sqlite")),
        )
    };

    if dry_run {
        let paths = kei_import_project::doc_walker::collect_doc_paths(repo_path);
        println!("dry-run: {} doc files found", paths.len());
        for p in &paths {
            println!("  {}", p.display());
        }
        println!("fragments-dir: {}", frags.display());
        println!("project-slug:  {slug}");
        println!("registry:      (dry-run — no writes)");
        return;
    }

    let result = match extract_skills(repo_path, &slug, &frags, db.as_deref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("extract-skills failed: {e}");
            process::exit(1);
        }
    };
    let unique_sources = {
        let mut seen = std::collections::HashSet::new();
        for s in &result.extracted {
            seen.insert(s.source_doc.clone());
        }
        seen.len()
    };
    println!(
        "extracted {} skills from {} markdown file(s)",
        result.extracted.len(),
        unique_sources,
    );
    println!(
        "{} registered, {} superseded, {} unchanged",
        result.registered, result.superseded, result.unchanged
    );
    let _ = ExtractedSkill {
        source_doc: PathBuf::new(),
        fragment_slug: String::new(),
        frontmatter_name: String::new(),
        frontmatter_description: String::new(),
        body: String::new(),
    };
}

fn dirs_claude_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".claude")
}
