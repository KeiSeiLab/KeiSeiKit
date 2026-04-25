//! kei-pet — CLI wrapper over the `kei_pet` library.
//!
//! Subcommands:
//!   validate <path>     Parse + run R1–R19 on a pet.toml, print PASS/FAIL
//!   show <path>         Print the rendered system-prompt overlay
//!   identity <action>   new | show — generate or display Ed25519 keypair
//!   tune <path> <kv>    Surgical axis edit (kv: `voice.tone_primary=warm`)

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use kei_pet::{parse, system_prompt, generate_keypair};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kei-pet", version, about = "Pet persona manifest tool")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Parse and validate a pet.toml file.
    Validate { path: PathBuf },

    /// Render and print the system-prompt overlay for a pet.toml.
    Show { path: PathBuf },

    /// Ed25519 identity operations.
    Identity {
        #[arg(value_parser = ["new", "show"])]
        action: String,
        #[arg(long, default_value = "~/.keisei/identity.key")]
        path: String,
    },

    /// Surgical edit of one axis. `path` is the pet.toml, `kv` is key=value.
    /// Example: `kei-pet tune ~/.keisei/pet.toml voice.tone_primary=warm`
    Tune { path: PathBuf, kv: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Validate { path } => cmd_validate(&path),
        Cmd::Show { path } => cmd_show(&path),
        Cmd::Identity { action, path } => cmd_identity(&action, &path),
        Cmd::Tune { path, kv } => cmd_tune(&path, &kv),
    }
}

fn cmd_validate(path: &PathBuf) -> Result<()> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    match parse(&text) {
        Ok(m) => {
            println!(
                "PASS — {} ({}) | schema v{} | {} interest(s) | {} routine(s)",
                m.identity.pet_name,
                m.identity.user_name,
                m.schema,
                m.interests.len(),
                m.routines.len(),
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn cmd_show(path: &PathBuf) -> Result<()> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let m = parse(&text)?;
    print!("{}", system_prompt(&m));
    Ok(())
}

fn cmd_identity(action: &str, path_str: &str) -> Result<()> {
    let path = expand_tilde(path_str);
    match action {
        "new" => {
            if path.exists() {
                bail!("identity file already exists at {} — refusing to overwrite", path.display());
            }
            let kp = generate_keypair();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, kp.secret_hex())?;
            set_permissions_0600(&path).ok();
            println!("generated new identity");
            println!("  public:  {}", kp.public_hex());
            println!("  user_id: {}", kp.user_id());
            println!("  stored:  {}", path.display());
            Ok(())
        }
        "show" => {
            if !path.exists() {
                bail!("no identity file at {}", path.display());
            }
            let secret_hex = fs::read_to_string(&path)?;
            let kp = kei_pet::identity::Keypair::from_secret_hex(secret_hex.trim())?;
            println!("public:  {}", kp.public_hex());
            println!("user_id: {}", kp.user_id());
            Ok(())
        }
        _ => bail!("unknown identity action: {action}"),
    }
}

fn cmd_tune(_path: &PathBuf, _kv: &str) -> Result<()> {
    // Full tune implementation (axis lookup + mutate + revalidate + persist)
    // arrives with `/pet-tune` skill (Day 2). Today we ship the parse layer
    // and leave this as a typed stub so the CLI surface is stable from v0.1.
    eprintln!("tune: not yet implemented (Day 2 — /pet-tune skill)");
    std::process::exit(2);
}

fn expand_tilde(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(s)
}

#[cfg(unix)]
fn set_permissions_0600(p: &std::path::Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(p)?.permissions();
    perms.set_mode(0o600);
    fs::set_permissions(p, perms)
}

#[cfg(not(unix))]
fn set_permissions_0600(_p: &std::path::Path) -> std::io::Result<()> { Ok(()) }
