//! ssh-check — pre-deploy sshd_config linter for KeiSeiKit.
//!
//! Reads /etc/ssh/sshd_config + every /etc/ssh/sshd_config.d/*.conf (or
//! user-supplied paths), merges directives via last-wins precedence, and
//! reports violations of the hardened-baseline rule matrix.
//!
//! USAGE
//!   ssh-check                                    # default system paths
//!   ssh-check --config /etc/ssh/sshd_config --drop-in /etc/ssh/sshd_config.d
//!   ssh-check --json                             # JSON output for CI
//!   ssh-check --allow-user admin                 # extra allowed user
//!
//! EXIT
//!   0  no violations
//!   1  usage / parse error
//!   2  violations found

mod check;
mod parse;
mod rules;

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "ssh-check",
    about = "Lint sshd_config + drop-ins against the KeiSeiKit hardened baseline."
)]
struct Cli {
    /// Main sshd_config file.
    #[arg(long, default_value = "/etc/ssh/sshd_config")]
    config: PathBuf,

    /// Drop-in directory (sshd_config.d). Pass empty string to skip.
    #[arg(long, default_value = "/etc/ssh/sshd_config.d")]
    drop_in: PathBuf,

    /// Usernames that are acceptable in AllowUsers (repeatable).
    #[arg(long = "allow-user")]
    allow_user: Vec<String>,

    /// Emit JSON instead of human text.
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let merged = match parse::load_merged(&cli.config, &cli.drop_in) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("ssh-check: {e}");
            return ExitCode::from(1);
        }
    };

    let allow_users: Vec<String> = if cli.allow_user.is_empty() {
        vec!["keiadmin".into()]
    } else {
        cli.allow_user
    };
    let matrix = rules::hardened_matrix(&allow_users);
    let findings = check::evaluate(&merged, &matrix);

    if cli.json {
        let out = serde_json::to_string_pretty(&findings).unwrap_or_default();
        println!("{out}");
    } else {
        render_human(&findings);
    }

    if findings.iter().any(|f| f.severity != check::Severity::Ok) {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}

fn render_human(findings: &[check::Finding]) {
    let mut bad = 0usize;
    for f in findings {
        if f.severity == check::Severity::Ok {
            continue;
        }
        bad += 1;
        println!(
            "[{sev:<5}] {directive:<28} {source}  ({note})",
            sev = f.severity.label(),
            directive = f.directive,
            source = f.source,
            note = f.note
        );
    }
    if bad == 0 {
        println!("ssh-check: OK — hardened baseline satisfied.");
    } else {
        println!("ssh-check: {bad} violation(s).");
    }
}
