//! kei-hibernate CLI.
//!
//! Subcommands: `export`, `import`, `inspect`. Thin dispatcher over
//! the library surface; each arm is <30 LOC.

use clap::{Parser, Subcommand};
use kei_hibernate::{export, import, inspect};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-hibernate", version, about = "Whole-brain export/import of KeiSei state")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Bundle kit_root into a tar.zst archive.
    Export {
        /// Output bundle path (e.g. bundle.tar.zst).
        #[arg(long)]
        out: PathBuf,
        /// Kit root (defaults to current directory).
        #[arg(long, default_value = ".")]
        kit_root: PathBuf,
    },
    /// Extract a bundle into kit_root (pass --dry-run to preview).
    Import {
        bundle: PathBuf,
        #[arg(long, default_value = ".")]
        kit_root: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    /// Print manifest contents without extracting.
    Inspect { bundle: PathBuf },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Export { out, kit_root } => run_export(&kit_root, &out),
        Cmd::Import { bundle, kit_root, dry_run } => run_import(&bundle, &kit_root, dry_run),
        Cmd::Inspect { bundle } => run_inspect(&bundle),
    }
}

fn run_export(kit_root: &std::path::Path, out: &std::path::Path) -> ExitCode {
    match export(kit_root, out) {
        Ok(meta) => {
            println!(
                "exported {} files ({} bytes) -> {}",
                meta.file_count,
                meta.total_bytes,
                out.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => fail(&format!("export failed: {e}")),
    }
}

fn run_import(bundle: &std::path::Path, kit_root: &std::path::Path, dry_run: bool) -> ExitCode {
    match import(bundle, kit_root, dry_run) {
        Ok(r) => {
            println!(
                "{} {} files, {} conflicts, extracted={}",
                if r.dry_run { "DRY-RUN:" } else { "imported:" },
                r.file_count,
                r.conflicts.len(),
                r.extracted
            );
            for c in &r.conflicts {
                println!("  conflict: {c}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => fail(&format!("import failed: {e}")),
    }
}

fn run_inspect(bundle: &std::path::Path) -> ExitCode {
    match inspect(bundle) {
        Ok(r) => {
            println!(
                "manifest v{} ts={} host={} files={} bytes={}",
                r.version, r.timestamp, r.machine_id, r.file_count, r.total_bytes
            );
            for p in &r.paths {
                println!("  {p}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => fail(&format!("inspect failed: {e}")),
    }
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("{msg}");
    ExitCode::FAILURE
}
