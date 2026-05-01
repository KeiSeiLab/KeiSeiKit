//! frustration-matrix — longitudinal user-pushback scanner + firmware trainer
//! + likelihood-ratio classifier.
//!
//! Constructor Pattern: main.rs only dispatches. Work is in cubes:
//! categories / markdown / jsonl / since / row / scan / report / firmware /
//! firmware_corpus / firmware_ngram / classifier. CLI shape stable; extend
//! categories in categories.rs only, firmware behaviour in firmware*.rs,
//! classifier behaviour in classifier.rs.

mod categories;
mod classifier;
mod classifier_cli;
mod eval;
mod eval_gold;
mod eval_metrics;
mod eval_predict;
mod eval_report;
mod firmware;
mod firmware_corpus;
mod firmware_ngram;
mod hit;
mod jsonl;
mod markdown;
mod report;
mod row;
mod scan;
mod scan_classifier;
mod since;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "frustration-matrix",
    version,
    about = "Scan chatlogs for recurring user-pushback categories (regex-only, no ML)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Walk chatlogs, apply category regexes, write CSV or JSONL output.
    Scan {
        #[arg(long, default_value = "30d")]
        since: String,
        #[arg(long, default_value = "~/.claude/memory/chatlogs")]
        root: String,
        #[arg(long, value_enum, default_value_t = Fmt::Csv)]
        format: Fmt,
        #[arg(long, default_value = "sleep-reports/frustration-matrix.csv")]
        output: PathBuf,
        /// Skip raw `.jsonl` session transcripts; scan only curated `.md`.
        #[arg(long, default_value_t = false)]
        skip_jsonl: bool,
        /// Drop user messages shorter than N chars before regex match.
        /// Defaults to 8 — filters "да" / "ок" noise; raise for stricter scans.
        #[arg(long, default_value_t = 8)]
        min_len: usize,
        /// If set, load firmware bundle from this directory and classify
        /// each user line via likelihood-ratio instead of regex categories.
        /// Directory must contain `neutral.fw` + one `.fw` per category.
        #[arg(long)]
        model_dir: Option<PathBuf>,
    },
    /// Classify a single message via the loaded firmware bundle. Prints
    /// a one-line-per-category ranking (descending by normalized ratio).
    Classify {
        #[arg(long)]
        model_dir: PathBuf,
        /// Message to classify. Positional — quote it in shell.
        message: String,
        /// Drop messages shorter than N chars (see classifier::MIN_LEN).
        #[arg(long, default_value_t = classifier::MIN_LEN)]
        min_len: usize,
        /// Normalized log-ratio threshold (see classifier::THRESHOLD).
        #[arg(long, default_value_t = classifier::THRESHOLD)]
        threshold: f64,
    },
    /// Read scan output, aggregate, print top-N table.
    Report {
        #[arg(long, default_value = "sleep-reports/frustration-matrix.csv")]
        input: PathBuf,
        #[arg(long, default_value_t = 5)]
        top: usize,
        #[arg(long, value_enum, default_value_t = By::Category)]
        by: By,
    },
    /// Train a byte-level n-gram firmware from a corpus directory.
    /// Ports internal predecessor. Output
    /// is gzipped JSON, typically 10-50 KB per language class.
    Train {
        #[arg(long)]
        root: PathBuf,
        /// Context depth. internal calibration knee is 4 on 10-25 MB.
        #[arg(long, default_value_t = firmware::DEFAULT_MAX_DEPTH)]
        depth: usize,
        #[arg(long)]
        output: PathBuf,
        /// Fraction of the corpus held out for perplexity. Pass `0.1`
        /// to hold out the last 10% of chars.
        #[arg(long, default_value_t = 0.0)]
        holdout: f64,
    },
    /// Compare regex-based (v1) vs firmware-based (v2) classification on a
    /// hand-labelled gold set. Writes per-category CSV + prints summary.
    Eval {
        /// Path to `labeled-training-set.jsonl`. Only rows with
        /// `quality == "gold"` are used.
        #[arg(long)]
        gold: PathBuf,
        /// Directory with firmware bundle (`neutral.fw` + per-category `.fw`).
        #[arg(long)]
        model_dir: PathBuf,
        /// Output CSV path. One row per `(model, category)`.
        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Fmt {
    Csv,
    Jsonl,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum By {
    Category,
    Session,
}

fn main() -> ExitCode {
    match dispatch() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("frustration-matrix: {e:#}");
            ExitCode::from(1)
        }
    }
}

fn dispatch() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Scan {
            since,
            root,
            format,
            output,
            skip_jsonl,
            min_len,
            model_dir,
        } => run_scan(since, root, format, output, skip_jsonl, min_len, model_dir),
        Cmd::Report { input, top, by } => run_report(input, top, by),
        Cmd::Train {
            root,
            depth,
            output,
            holdout,
        } => run_train(root, depth, output, holdout),
        Cmd::Classify {
            model_dir,
            message,
            min_len,
            threshold,
        } => run_classify(&model_dir, &message, min_len, threshold),
        Cmd::Eval {
            gold,
            model_dir,
            output,
        } => run_eval(gold, model_dir, output),
    }
}

/// Wire CLI args through the thin `eval::evaluate` orchestrator.
fn run_eval(gold: PathBuf, model_dir: PathBuf, output: PathBuf) -> Result<()> {
    let input = eval::EvalInput {
        gold_jsonl: gold,
        model_dir,
        output_csv: output,
    };
    eval::evaluate(&input)?;
    Ok(())
}

fn run_scan(
    since: String,
    root: String,
    format: Fmt,
    output: PathBuf,
    skip_jsonl: bool,
    min_len: usize,
    model_dir: Option<PathBuf>,
) -> Result<()> {
    let root = expand_tilde(&root);
    let fmt = match format {
        Fmt::Csv => scan::Format::Csv,
        Fmt::Jsonl => scan::Format::Jsonl,
    };
    let classifier = match model_dir {
        Some(dir) => Some(classifier::Classifier::load_from_dir(&dir)?),
        None => None,
    };
    scan::run(scan::ScanArgs {
        root: &root,
        since_spec: &since,
        format: fmt,
        output: &output,
        skip_jsonl,
        min_len,
        classifier: classifier.as_ref(),
    })?;
    Ok(())
}

/// Classify a single message via the firmware bundle at `dir`. Delegates
/// all printing to `classifier_cli::run`.
fn run_classify(
    dir: &Path,
    message: &str,
    min_len: usize,
    threshold: f64,
) -> Result<()> {
    classifier_cli::run(dir, message, min_len, threshold)
}

fn run_report(input: PathBuf, top: usize, by: By) -> Result<()> {
    let mode = match by {
        By::Category => report::GroupBy::Category,
        By::Session => report::GroupBy::Session,
    };
    report::run(&input, top, mode)
}

fn run_train(root: PathBuf, depth: usize, output: PathBuf, holdout: f64) -> Result<()> {
    let text = firmware_corpus::load_corpus_text(&root)?;
    let total = text.chars().count();
    let (train_text, held) = split_holdout(&text, holdout);
    let fw = firmware::Firmware::train_from_text(train_text, depth);
    fw.save(&output)?;
    let size = std::fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    eprintln!("frustration-matrix train: {} chars, {} contexts, depth={}, file={} ({} B)",
        total, fw.ngrams.len(), depth, output.display(), size);
    if !held.is_empty() {
        let ll = fw.log_likelihood(held);
        let n = held.chars().count().max(1) as f64;
        eprintln!("  holdout: {} chars, avg log-lik={:.4}, ppl={:.2}",
            n as usize, ll / n, (-ll / n).exp());
    }
    Ok(())
}

/// Split `text` at char-boundary `holdout` fraction. Returns (train, test).
/// If `holdout <= 0` or > 0.5, the test slice is empty.
fn split_holdout(text: &str, holdout: f64) -> (&str, &str) {
    if holdout <= 0.0 || holdout > 0.5 {
        return (text, "");
    }
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let cut_idx = (chars.len() as f64 * (1.0 - holdout)) as usize;
    let boundary = chars.get(cut_idx).map(|(i, _)| *i).unwrap_or(text.len());
    text.split_at(boundary)
}

/// Expand a leading `~/` using $HOME. Absolute/relative paths pass through.
fn expand_tilde(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(s)
}
