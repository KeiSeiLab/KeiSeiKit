// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `kei-compute-linode` CLI — thin operator surface over `LinodeCompute`.
//!
//! Subcommands:
//! - `dna`         — print the provider DNA serial.
//! - `cloud-init`  — render a cloud-init blob (raw or base64).
//! - `provision`   — create a Linode instance from flags.
//! - `status`      — read instance status by id.

use clap::{Parser, Subcommand};
use kei_compute_linode::{
    cloud_init::{render, render_base64, CloudInitTemplate},
    LinodeClient, LinodeCompute,
};
use kei_runtime_core::dna::HasDna;
use kei_runtime_core::traits::compute::{ComputeProvider, VmSpec};

#[derive(Parser, Debug)]
#[command(name = "kei-compute-linode", version, about = "Linode ComputeProvider CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print the provider DNA serial.
    Dna,
    /// Render a cloud-init blob.
    CloudInit {
        #[arg(long)]
        hostname: String,
        #[arg(long)]
        ssh_pubkey: String,
        #[arg(long, default_value = "false")]
        base64: bool,
    },
    /// Create a Linode instance.
    Provision {
        #[arg(long)]
        region: String,
        #[arg(long)]
        tier: String,
        #[arg(long)]
        ssh_pubkey: String,
        #[arg(long)]
        hostname: String,
        #[arg(long, default_value = "linode/debian12")]
        image: String,
    },
    /// Read instance status by id.
    Status {
        #[arg(long)]
        id: i64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Dna => cmd_dna(),
        Cmd::CloudInit {
            hostname,
            ssh_pubkey,
            base64,
        } => cmd_cloud_init(&hostname, &ssh_pubkey, base64),
        Cmd::Provision {
            region,
            tier,
            ssh_pubkey,
            hostname,
            image,
        } => cmd_provision(&region, &tier, &ssh_pubkey, &hostname, &image).await,
        Cmd::Status { id } => cmd_status(id).await,
    }
}

fn cmd_dna() -> anyhow::Result<()> {
    let cli = LinodeClient::new("placeholder");
    let p = LinodeCompute::new(cli, "linode/debian12")?;
    println!("{}", p.dna().as_str());
    Ok(())
}

fn cmd_cloud_init(hostname: &str, ssh: &str, b64: bool) -> anyhow::Result<()> {
    let t = CloudInitTemplate::new(hostname, ssh).run("apt-get update");
    let out = if b64 { render_base64(&t) } else { render(&t) };
    println!("{out}");
    Ok(())
}

async fn cmd_provision(
    region: &str,
    tier: &str,
    ssh: &str,
    hostname: &str,
    image: &str,
) -> anyhow::Result<()> {
    let client = LinodeClient::from_env()?;
    let provider = LinodeCompute::new(client, image)?;
    let user = kei_runtime_core::DnaBuilder::new("user")
        .cap("EM")
        .scope("kei-compute-linode-cli")
        .body(hostname.as_bytes())
        .build()?;
    let template = CloudInitTemplate::new(hostname, ssh).run("apt-get update");
    let spec = VmSpec {
        user_dna: user,
        region: region.to_string(),
        tier: tier.to_string(),
        ssh_pubkey: ssh.to_string(),
        cloud_init: render(&template),
        labels: vec![],
    };
    let h = provider.create(&spec).await?;
    println!("{}", serde_json::to_string_pretty(&h)?);
    Ok(())
}

async fn cmd_status(id: i64) -> anyhow::Result<()> {
    let client = LinodeClient::from_env()?;
    let resp = client.get_instance(id).await?;
    println!("{}", serde_json::to_string_pretty(&resp)?);
    Ok(())
}
