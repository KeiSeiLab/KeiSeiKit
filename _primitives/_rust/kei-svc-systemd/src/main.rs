// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use clap::{Parser, Subcommand};
use kei_runtime_core::traits::service::{RestartPolicy, ServiceManager, ServiceUnit, TimerSpec};
use kei_runtime_core::HasDna;
use kei_svc_systemd::SystemdManager;

#[derive(Parser)]
#[command(name = "kei-svc-systemd", version)]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    Dna,
    /// Render a service+timer pair to /tmp (no systemctl invocation).
    Render {
        #[arg(long)] name: String,
        #[arg(long)] exec: String,
        #[arg(long, default_value = "*-*-* 03:07:00")] cron: String,
        #[arg(long, default_value = "/tmp")] out: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Dna => {
            let m = SystemdManager::with("/tmp", "systemctl", false, None)?;
            println!("{}", m.dna());
        }
        Cmd::Render { name, exec, cron, out } => {
            let m = SystemdManager::with(&out, "systemctl", false, None)?;
            let unit = ServiceUnit {
                name: name.clone(),
                exec_path: exec,
                args: vec!["run".into()],
                env: vec![],
                working_dir: "/".into(),
                user: None,
                restart_policy: RestartPolicy::OnFailure,
                timer_spec: Some(TimerSpec { on_calendar: cron, randomized_delay_sec: 60 }),
            };
            m.install(&unit).await?;
            println!("{out}/{name}.service + {out}/{name}.timer written");
        }
    }
    Ok(())
}
