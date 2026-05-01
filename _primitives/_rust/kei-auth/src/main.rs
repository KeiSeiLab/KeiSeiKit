//! kei-auth CLI — issue/verify/revoke.
//!
//! v0.14.1 security fix: the `--key` CLI flag was removed because it
//! leaked the HMAC signing secret through `/proc/<pid>/cmdline` and
//! shell history. The only supported key source is the `KEI_AUTH_KEY`
//! env var (sourced from `~/.claude/secrets/.env` per RULE 0.8).

use clap::{Parser, Subcommand};
use kei_auth::schema::open;
use kei_auth::scopes::Scope;
use kei_auth::tokens::{issue, revoke, verify};
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "kei-auth", version)]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Issue { #[arg(long)] user: String,
            #[arg(long)] project: String,
            #[arg(long, default_value = "read")] scope: String,
            #[arg(long, default_value_t = 86400)] ttl: i64 },
    Verify { token: String },
    Revoke { token: String },
}

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o { return p; }
    if let Ok(e) = std::env::var("KEI_AUTH_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/auth/auth.sqlite")
}

fn key() -> anyhow::Result<Vec<u8>> {
    let k = std::env::var("KEI_AUTH_KEY").map_err(|_| {
        anyhow::anyhow!(
            "KEI_AUTH_KEY env var not set.\n  \
             Set it before running kei-auth:\n    \
             export KEI_AUTH_KEY=\"$(openssl rand -hex 32)\"\n  \
             Or read from ~/.claude/secrets/.env (RULE 0.8 SSoT).\n  \
             The previous --key CLI flag was removed in v0.14.1 because \
             it leaked the secret via /proc/<pid>/cmdline."
        )
    })?;
    Ok(k.into_bytes())
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let conn = open(&db_path(cli.db))?;
    let k = key()?;
    match cli.cmd {
        Cmd::Issue { user, project, scope, ttl } => {
            let sc = Scope::from_str(&scope).map_err(|e| anyhow::anyhow!(e))?;
            println!("{}", issue(&conn, &user, &project, sc, ttl, &k)?);
        }
        Cmd::Verify { token } => {
            let out = verify(&conn, &token, &k)?;
            println!("user={} project={} scope={}", out.user_id, out.project, out.scope);
        }
        Cmd::Revoke { token } => {
            let n = revoke(&conn, &token)?;
            println!("revoked {} row(s)", n);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-auth: {e:#}"); ExitCode::from(1) }
    }
}
