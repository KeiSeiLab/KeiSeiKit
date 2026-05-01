// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Denis Parfionovich
//
//! kei-compute-baremetal CLI — `dna` / `register` / `status` / `unregister`.
//! No HTTP, no cloud — pure SSH dispatch over the system `ssh` binary.

use clap::{Parser, Subcommand};
use kei_compute_baremetal::BaremetalCompute;
use kei_runtime_core::traits::compute::{ComputeProvider, VmHandle, VmSpec};
use kei_runtime_core::{DnaBuilder, HasDna};

#[derive(Parser)]
#[command(
    name = "kei-compute-baremetal",
    version,
    about = "Bare-metal ComputeProvider — register existing SSH boxes, no cloud API"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print this primitive's own DNA.
    Dna,
    /// Register an existing SSH-reachable box. Runs cloud-init shell remotely.
    Register {
        #[arg(long)]
        user_handle: String,
        #[arg(long, help = "user@host[:port]")]
        endpoint: String,
        #[arg(long, default_value = "host-1c-1gb")]
        tier: String,
        #[arg(long)]
        ssh_key: Option<String>,
        #[arg(long, help = "path to cloud-init shell script")]
        cloud_init_file: Option<String>,
    },
    /// SSH-ping the registered box.
    Status {
        #[arg(long, help = "ssh://user@host[:port]")]
        external_id: String,
    },
    /// Deregister (no hardware action).
    Unregister {
        #[arg(long)]
        external_id: String,
    },
}

fn build_handle(external_id: &str) -> Result<VmHandle, Box<dyn std::error::Error>> {
    let dna = DnaBuilder::new("vm-managed")
        .cap("BM")
        .scope("keiseikit.dev/vms/baremetal/cli")
        .body(external_id.as_bytes())
        .build()?;
    Ok(VmHandle {
        dna,
        external_id: external_id.to_string(),
        provider: "baremetal".into(),
        region: String::new(),
        tier: String::new(),
        ipv4: None,
        ipv6: None,
        tailscale_ip: None,
        created_at_ms: 0,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Dna => {
            let c = BaremetalCompute::new(None, None)?;
            println!("{}", c.dna());
        }
        Cmd::Register {
            user_handle,
            endpoint,
            tier,
            ssh_key,
            cloud_init_file,
        } => {
            let cloud_init = match cloud_init_file {
                Some(p) => std::fs::read_to_string(&p)?,
                None => String::new(),
            };
            let user_dna = DnaBuilder::new("user")
                .cap("EM")
                .scope("keiseikit.dev/users")
                .body(user_handle.as_bytes())
                .build()?;
            let c = BaremetalCompute::new(None, ssh_key)?;
            let spec = VmSpec {
                user_dna,
                region: endpoint,
                tier,
                ssh_pubkey: String::new(),
                cloud_init,
                labels: vec![],
            };
            let h = c.create(&spec).await?;
            println!("{}", serde_json::to_string_pretty(&h)?);
        }
        Cmd::Status { external_id } => {
            let c = BaremetalCompute::new(None, None)?;
            let h = build_handle(&external_id)?;
            let s = c.status(&h).await?;
            println!("{:?}", s);
        }
        Cmd::Unregister { external_id } => {
            let c = BaremetalCompute::new(None, None)?;
            let h = build_handle(&external_id)?;
            c.destroy(&h).await?;
            println!("unregistered: {}", external_id);
        }
    }
    Ok(())
}
