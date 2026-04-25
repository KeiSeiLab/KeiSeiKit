//! kei-skill-importer CLI — `parse` (JSON), `convert` (write file),
//! `batch` (walk + convert; JSONL summary). Info logs go to stderr;
//! stdout is reserved for machine-readable output.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use kei_skill_importer::{decide_emit_path, import, EmitPath, SourceFormat};
use serde::Serialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(
    name = "kei-skill-importer",
    about = "Parse external AI-coding-tool skill files and emit them in KeiSeiKit canonical shapes."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Parse a skill file and print canonical JSON to stdout.
    Parse {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Auto)]
        format: FormatArg,
    },
    /// Parse + decide emit path + write file(s) into <output_dir>.
    Convert {
        path: PathBuf,
        #[arg(long)]
        output_dir: PathBuf,
        #[arg(long, value_enum, default_value_t = FormatArg::Auto)]
        format: FormatArg,
    },
    /// Walk <input_dir> and convert every candidate file.
    Batch {
        input_dir: PathBuf,
        #[arg(long)]
        output_dir: PathBuf,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum FormatArg { Auto, Openclaw, Cline, Cursor, Claude, Kimi }

impl From<FormatArg> for SourceFormat {
    fn from(f: FormatArg) -> Self {
        match f {
            FormatArg::Auto => SourceFormat::Auto,
            FormatArg::Openclaw => SourceFormat::OpenClaw,
            FormatArg::Cline => SourceFormat::Cline,
            FormatArg::Cursor => SourceFormat::Cursor,
            FormatArg::Claude => SourceFormat::ClaudeCode,
            FormatArg::Kimi => SourceFormat::Kimi,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Parse { path, format } => cmd_parse(&path, format.into()),
        Cmd::Convert {
            path,
            output_dir,
            format,
        } => cmd_convert(&path, &output_dir, format.into()),
        Cmd::Batch {
            input_dir,
            output_dir,
        } => cmd_batch(&input_dir, &output_dir),
    }
}

fn cmd_parse(path: &Path, format: SourceFormat) -> Result<()> {
    let skill = import(path, format).context("parse")?;
    let json = serde_json::to_string_pretty(&skill).context("serialize JSON")?;
    println!("{json}");
    Ok(())
}

#[derive(Serialize)]
struct ConvertSummary {
    emitted: String,
    paths: Vec<String>,
    skill_name: String,
    source_format: String,
}

fn cmd_convert(path: &Path, output_dir: &Path, format: SourceFormat) -> Result<()> {
    let skill = import(path, format).context("parse")?;
    let kind = decide_emit_path(&skill);
    let written = emit_one(&skill, output_dir, kind)?;
    let summary = ConvertSummary {
        emitted: kind.as_str().into(),
        paths: written.iter().map(|p| p.display().to_string()).collect(),
        skill_name: skill.name.clone(),
        source_format: skill.source_format.as_str().into(),
    };
    println!("{}", serde_json::to_string(&summary)?);
    Ok(())
}

fn emit_one(
    skill: &kei_skill_importer::ImportedSkill,
    output_dir: &Path,
    kind: EmitPath,
) -> Result<Vec<PathBuf>> {
    use kei_skill_importer::emit;
    let p = match kind {
        EmitPath::Atom => emit::as_atom::write(skill, output_dir)?,
        EmitPath::Recipe => emit::as_recipe::write(skill, output_dir)?,
        EmitPath::Primitive => emit::as_primitive::write(skill, output_dir)?,
    };
    Ok(vec![p])
}

#[derive(Serialize)]
struct BatchLine {
    source: String,
    ok: bool,
    emitted: Option<String>,
    error: Option<String>,
}

fn cmd_batch(input_dir: &Path, output_dir: &Path) -> Result<()> {
    if !input_dir.is_dir() {
        bail!("input_dir not a directory: {}", input_dir.display());
    }
    let mut count = 0usize;
    for entry in WalkDir::new(input_dir)
        .max_depth(8)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        if !is_candidate(entry.path()) {
            continue;
        }
        count += 1;
        let line = process_one(entry.path(), output_dir);
        println!("{}", serde_json::to_string(&line)?);
    }
    eprintln!("processed {count} file(s)");
    Ok(())
}

fn is_candidate(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    matches!(ext.as_str(), "md" | "mdc" | "yaml" | "yml")
}

fn process_one(path: &Path, output_dir: &Path) -> BatchLine {
    let source = path.display().to_string();
    match import(path, SourceFormat::Auto) {
        Ok(skill) => {
            let kind = decide_emit_path(&skill);
            match emit_one(&skill, output_dir, kind) {
                Ok(_) => BatchLine {
                    source,
                    ok: true,
                    emitted: Some(kind.as_str().into()),
                    error: None,
                },
                Err(e) => BatchLine {
                    source,
                    ok: false,
                    emitted: None,
                    error: Some(format!("emit: {e:#}")),
                },
            }
        }
        Err(e) => BatchLine {
            source,
            ok: false,
            emitted: None,
            error: Some(format!("parse: {e:#}")),
        },
    }
}
