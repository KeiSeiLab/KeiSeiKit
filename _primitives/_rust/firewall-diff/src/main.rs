//! firewall-diff — compare an intended ufw rule set (YAML) against the
//! running firewall (parsed from `ufw status numbered` output).
//!
//! USAGE
//!   firewall-diff --intent firewall-intent.yaml --status-file live.txt
//!   ufw status numbered | firewall-diff --intent firewall-intent.yaml --stdin
//!   firewall-diff --intent firewall-intent.yaml --json
//!
//! The tool does NOT execute `ufw` itself (defensive-only). Feed it the
//! output of `ufw status numbered` or have the skill pipe it in.
//!
//! EXIT
//!   0  intent ≡ live (no diff)
//!   1  usage / parse error
//!   2  differences found (live deviates from intent)

mod diff;
mod intent;
mod ufw;

use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "firewall-diff", about = "Diff intended ufw rules (YAML) vs live status.")]
struct Cli {
    /// Path to the intent YAML file.
    #[arg(long)]
    intent: PathBuf,

    /// Path to a file holding captured `ufw status numbered` output.
    #[arg(long, conflicts_with = "stdin")]
    status_file: Option<PathBuf>,

    /// Read the ufw status text from stdin (use when piping from the host).
    #[arg(long)]
    stdin: bool,

    /// Emit JSON instead of human text.
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let intent = match intent::load(&cli.intent) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("firewall-diff: intent: {e}");
            return ExitCode::from(1);
        }
    };

    let status_txt = match (&cli.status_file, cli.stdin) {
        (Some(p), false) => match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("firewall-diff: read {}: {e}", p.display());
                return ExitCode::from(1);
            }
        },
        (None, true) => {
            let mut s = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut s) {
                eprintln!("firewall-diff: stdin: {e}");
                return ExitCode::from(1);
            }
            s
        }
        _ => {
            eprintln!("firewall-diff: need --status-file <path> or --stdin");
            return ExitCode::from(1);
        }
    };

    let live = match ufw::parse(&status_txt) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("firewall-diff: parse ufw status: {e}");
            return ExitCode::from(1);
        }
    };

    let report = diff::compare(&intent, &live);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    } else {
        diff::render_human(&report);
    }

    if report.is_clean() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}
