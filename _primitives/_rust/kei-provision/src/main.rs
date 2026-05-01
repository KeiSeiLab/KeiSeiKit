//! kei-provision — unified VPS provisioner CLI.
//!
//! USAGE
//!   kei-provision <backend> create  <name> [--type T] [--location L]
//!                                          [--image I] [--ssh-key K]
//!                                          [--firewall F] [--user-data PATH]
//!   kei-provision <backend> status  <name>
//!   kei-provision <backend> destroy <name> [--force]
//!   kei-provision <backend> list
//!
//!   <backend>: hetzner | vultr
//!
//! ENV (RULE 0.8 — secrets single source)
//!   HCLOUD_TOKEN   — Hetzner API token
//!   VULTR_API_KEY  — Vultr API key
//!
//! Source via: `source ~/.claude/secrets/.env` before invocation.

use clap::{Parser, Subcommand};
use kei_provision::{resolve, CreateOpts};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "kei-provision",
    about = "Unified VPS provisioner — Hetzner, Vultr, (future) AWS/DO/Linode."
)]
struct Cli {
    /// Backend to use.
    backend: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Create a server (idempotent — returns existing IP if name/label taken).
    Create {
        name: String,
        /// Server type / plan (hetzner: `cx22`; vultr: `vc2-1c-1gb`).
        #[arg(long)]
        r#type: Option<String>,
        /// Datacenter (hetzner: `fsn1`; vultr: `ams`).
        #[arg(long)]
        location: Option<String>,
        /// Image / OS (hetzner: `debian-12`; vultr: os-id string).
        #[arg(long)]
        image: Option<String>,
        /// SSH key id/name.
        #[arg(long = "ssh-key")]
        ssh_key: Option<String>,
        /// Firewall id/name.
        #[arg(long)]
        firewall: Option<String>,
        /// cloud-init user-data file.
        #[arg(long = "user-data")]
        user_data: Option<PathBuf>,
    },
    /// Print server info (absent ⇒ "absent" line, exit 0).
    Status { name: String },
    /// Destroy server (idempotent on absent).
    Destroy {
        name: String,
        #[arg(long)]
        force: bool,
    },
    /// List all servers on this account.
    List,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let backend = match resolve(&cli.backend) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("kei-provision: {e}");
            return ExitCode::from(1);
        }
    };
    match run(&*backend, cli.cmd) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-provision [{}]: {e}", backend.name());
            ExitCode::from(2)
        }
    }
}

fn run(b: &dyn kei_provision::Backend, cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::Create {
            name,
            r#type,
            location,
            image,
            ssh_key,
            firewall,
            user_data,
        } => {
            let opts = CreateOpts {
                server_type: r#type,
                location,
                image,
                ssh_key,
                firewall,
                user_data_path: user_data,
            };
            let info = b.create(&name, &opts)?;
            println!("{}", info.ipv4.unwrap_or_else(|| "-".into()));
        }
        Cmd::Status { name } => match b.status(&name)? {
            None => println!("absent"),
            Some(i) => print_status(&i),
        },
        Cmd::Destroy { name, force } => b.destroy(&name, force)?,
        Cmd::List => {
            for i in b.list()? {
                println!(
                    "{}\t{}\t{}\t{}",
                    i.name,
                    i.status,
                    i.ipv4.unwrap_or_else(|| "-".into()),
                    i.id
                );
            }
        }
    }
    Ok(())
}

fn print_status(i: &kei_provision::ServerInfo) {
    println!("name={}", i.name);
    println!("id={}", i.id);
    println!("status={}", i.status);
    println!(
        "ipv4={}",
        i.ipv4.clone().unwrap_or_else(|| "-".into())
    );
}
