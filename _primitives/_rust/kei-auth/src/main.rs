//! kei-auth CLI — issue/verify/revoke.
//!
//! v0.14.1 security fix: the `--key` CLI flag was removed because it
//! leaked the HMAC signing secret through `/proc/<pid>/cmdline` and
//! shell history. The only supported key source is the `KEI_AUTH_KEY`
//! env var (sourced from `~/.claude/secrets/.env` per RULE 0.8).
//!
//! Token argument: pass `-` or set `KEI_AUTH_TOKEN` env var to avoid
//! leaking tokens via shell history or `/proc/<pid>/cmdline`.

use clap::{Parser, Subcommand};
use kei_auth::schema::open;
use kei_auth::scopes::Scope;
use kei_auth::tokens::{issue, revoke, verify};
use std::io::Read;
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

/// Read token from env `KEI_AUTH_TOKEN`, or from stdin when arg is `-`,
/// or return the arg as-is. Avoids token leakage via shell history.
fn resolve_token(arg: &str) -> anyhow::Result<String> {
    if let Ok(t) = std::env::var("KEI_AUTH_TOKEN") {
        return Ok(t.trim().to_owned());
    }
    if arg == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        return Ok(buf.trim().to_owned());
    }
    Ok(arg.to_owned())
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
    if k.len() < 32 {
        anyhow::bail!(
            "KEI_AUTH_KEY must be ≥32 bytes (got {}). \
             Generate a strong key: export KEI_AUTH_KEY=\"$(openssl rand -hex 32)\"",
            k.len()
        );
    }
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
            let t = resolve_token(&token)?;
            let out = verify(&conn, &t, &k)?;
            println!("user={} project={} scope={}", out.user_id, out.project, out.scope);
        }
        Cmd::Revoke { token } => {
            let t = resolve_token(&token)?;
            let n = revoke(&conn, &t)?;
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
