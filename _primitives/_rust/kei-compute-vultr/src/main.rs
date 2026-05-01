// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! kei-compute-vultr — CLI front-end for the Vultr Cloud v2 ComputeProvider.
//!
//! Subcommands mirror kei-compute-hetzner:
//!   dna          — print the provider's DNA
//!   cloud-init   — render YAML cloud-init from CLI args
//!   provision    — call POST /instances (real API, requires VULTR_API_KEY)
//!   status       — call GET /instances/<id>

use clap::{Parser, Subcommand};
use kei_compute_vultr::{CloudInitSpec, VultrCompute};
use kei_runtime_core::{ComputeProvider, HasDna, VmHandle, VmSpec};

#[derive(Parser, Debug)]
#[command(name = "kei-compute-vultr", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print the constructor DNA.
    Dna,
    /// Render a cloud-init YAML to stdout.
    CloudInit {
        #[arg(long)]
        user_handle: String,
        #[arg(long)]
        tailscale_auth_key: String,
        #[arg(long, default_value = "ANTHROPIC_API_KEY")]
        anthropic_api_key_env: String,
        #[arg(long)]
        git_remote_url: String,
        #[arg(long, default_value = "0 3 * * *")]
        schedule_cron: String,
        #[arg(long)]
        install_forgejo_local: bool,
        #[arg(long, default_value = "https://cp.example")]
        control_plane_url: String,
        #[arg(long)]
        base64: bool,
    },
    /// Provision a Vultr instance (live API call).
    Provision {
        #[arg(long)]
        region: String,
        #[arg(long)]
        tier: String,
        #[arg(long)]
        ssh_pubkey: String,
        #[arg(long, default_value = "")]
        cloud_init: String,
    },
    /// Get current status of a previously-provisioned instance.
    Status {
        #[arg(long)]
        instance_id: String,
        #[arg(long)]
        region: String,
        #[arg(long)]
        tier: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Dna => {
            let c = VultrCompute::from_env()?;
            println!("{}", c.dna());
        }
        Cmd::CloudInit {
            user_handle,
            tailscale_auth_key,
            anthropic_api_key_env,
            git_remote_url,
            schedule_cron,
            install_forgejo_local,
            control_plane_url,
            base64,
        } => {
            let spec = CloudInitSpec {
                user_handle,
                tailscale_auth_key,
                anthropic_api_key_env,
                git_remote_url,
                schedule_cron,
                install_forgejo_local,
                control_plane_url,
            };
            if base64 {
                println!("{}", spec.render_base64());
            } else {
                print!("{}", spec.render());
            }
        }
        Cmd::Provision {
            region,
            tier,
            ssh_pubkey,
            cloud_init,
        } => {
            let c = VultrCompute::from_env()?;
            let user_dna = kei_runtime_core::DnaBuilder::new("user")
                .cap("EM")
                .scope("cli")
                .body(b"kei-compute-vultr-cli")
                .build()?;
            let spec = VmSpec {
                user_dna,
                region,
                tier,
                ssh_pubkey,
                cloud_init,
                labels: vec![("project".into(), "kei".into())],
            };
            let h = c.create(&spec).await?;
            println!("{}", serde_json::to_string_pretty(&h)?);
        }
        Cmd::Status {
            instance_id,
            region,
            tier,
        } => {
            let c = VultrCompute::from_env()?;
            let h = VmHandle {
                dna: c.dna().clone(),
                external_id: instance_id,
                provider: c.provider_name().to_string(),
                region,
                tier,
                ipv4: None,
                ipv6: None,
                tailscale_ip: None,
                created_at_ms: 0,
            };
            let s = c.status(&h).await?;
            println!("{s:?}");
        }
    }
    Ok(())
}
