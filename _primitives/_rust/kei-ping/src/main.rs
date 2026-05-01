// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `kei-ping` CLI — wraps PingStore (auto-selected backend).
//!
//! send <agent-id> <phase> [--dna X] [--branch B] [--note ...]
//! list [--max-age-s N] [--phase-prefix P] [--branch B]
//! clear <agent-id>
//! status   — prints backend kind + ping counts

use kei_ping::{auto_select, Heartbeat, PingFilter};
use std::collections::HashSet;
use std::env;
use std::time::Duration;

fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn pop_flag<'a>(args: &mut Vec<String>, name: &str) -> Option<String> {
    if let Some(idx) = args.iter().position(|a| a == name) {
        if idx + 1 < args.len() {
            let v = args.remove(idx + 1);
            args.remove(idx);
            return Some(v);
        }
    }
    None
}

fn usage() -> i32 {
    eprintln!(
        "kei-ping — cross-window agent heartbeat (auto: redis if alive, else sqlite)\n\n\
         Usage:\n  \
         kei-ping send <agent-id> <phase> [--session S] [--dna D] [--branch B] [--cwd C] [--note ...]\n  \
         kei-ping list [--max-age-s N=90] [--phase-prefix P] [--branch B] [--json]\n  \
         kei-ping clear <agent-id>\n  \
         kei-ping watch [--interval-s 3] [--max-age-s 90]   (poll-based; works on both backends)\n  \
         kei-ping status\n\n\
         Env:\n  \
         KEI_PING_REDIS_URL    (default redis://127.0.0.1:6379)\n  \
         KEI_PING_SQLITE_PATH  (default ~/.claude/agents/ping.sqlite)"
    );
    64
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::process::ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let sub = match args.first() {
        Some(s) => s.clone(),
        None => return std::process::ExitCode::from(usage() as u8),
    };
    args.remove(0);

    let store = match auto_select().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("kei-ping: backend init failed: {e}");
            return std::process::ExitCode::from(1);
        }
    };

    let code = match sub.as_str() {
        "send" => cmd_send(args, store.as_ref()).await,
        "list" => cmd_list(args, store.as_ref()).await,
        "clear" => cmd_clear(args, store.as_ref()).await,
        "watch" => cmd_watch(args, store.as_ref()).await,
        "status" => cmd_status(store.as_ref()).await,
        _ => usage(),
    };
    std::process::ExitCode::from(if code < 0 { 1 } else { code as u8 })
}

async fn cmd_send(mut args: Vec<String>, store: &dyn kei_ping::PingStore) -> i32 {
    let session = pop_flag(&mut args, "--session");
    let dna = pop_flag(&mut args, "--dna");
    let branch = pop_flag(&mut args, "--branch");
    let cwd = pop_flag(&mut args, "--cwd");
    let note = pop_flag(&mut args, "--note");
    if args.len() < 2 {
        return usage();
    }
    let h = Heartbeat {
        agent_id: args[0].clone(),
        session_id: session,
        phase: args[1].clone(),
        dna,
        branch,
        cwd,
        last_seen_epoch: now_epoch(),
        note,
    };
    match store.send(&h).await {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("send failed: {e}");
            1
        }
    }
}

async fn cmd_list(mut args: Vec<String>, store: &dyn kei_ping::PingStore) -> i32 {
    let max_age = pop_flag(&mut args, "--max-age-s")
        .and_then(|s| s.parse::<u64>().ok());
    let phase_prefix = pop_flag(&mut args, "--phase-prefix");
    let branch = pop_flag(&mut args, "--branch");
    let json = args.iter().any(|a| a == "--json");
    let f = PingFilter {
        max_age_s: max_age,
        phase_prefix,
        branch,
    };
    let list = match store.list(&f).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("list failed: {e}");
            return 1;
        }
    };
    if json {
        let s = serde_json::to_string_pretty(&list).unwrap_or_else(|_| "[]".into());
        println!("{s}");
    } else if list.is_empty() {
        println!("(no live heartbeats)");
    } else {
        let now = now_epoch();
        for h in &list {
            let age = now.saturating_sub(h.last_seen_epoch);
            println!(
                "{:>3}s  {:<24}  {:<28}  branch={:<32}  dna={}",
                age,
                h.agent_id,
                h.phase,
                h.branch.as_deref().unwrap_or("-"),
                h.dna.as_deref().unwrap_or("-")
            );
        }
    }
    0
}

async fn cmd_clear(args: Vec<String>, store: &dyn kei_ping::PingStore) -> i32 {
    if args.is_empty() {
        return usage();
    }
    match store.clear(&args[0]).await {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("clear failed: {e}");
            1
        }
    }
}

async fn cmd_watch(mut args: Vec<String>, store: &dyn kei_ping::PingStore) -> i32 {
    let interval = pop_flag(&mut args, "--interval-s")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3);
    let max_age = pop_flag(&mut args, "--max-age-s")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(90);
    let mut known: HashSet<String> = HashSet::new();
    println!("[kei-ping watch] backend={} interval={}s ttl={}s — Ctrl-C to stop", store.kind().as_str(), interval, max_age);
    let f = PingFilter { max_age_s: Some(max_age), ..Default::default() };
    loop {
        match store.list(&f).await {
            Ok(list) => {
                let now = now_epoch();
                let mut current: HashSet<String> = HashSet::new();
                for h in &list {
                    let key = format!("{}|{}", h.agent_id, h.phase);
                    current.insert(key.clone());
                    if !known.contains(&key) {
                        let age = now.saturating_sub(h.last_seen_epoch);
                        println!(
                            "[+] {:>3}s ago  {:<24}  phase={:<28}  branch={}  dna={}",
                            age, h.agent_id, h.phase,
                            h.branch.as_deref().unwrap_or("-"),
                            h.dna.as_deref().unwrap_or("-")
                        );
                    }
                }
                for gone in known.difference(&current) {
                    println!("[-] {} disappeared (timed out / cleared)", gone);
                }
                known = current;
            }
            Err(e) => eprintln!("watch list error: {e}"),
        }
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

async fn cmd_status(store: &dyn kei_ping::PingStore) -> i32 {
    let f = PingFilter {
        max_age_s: Some(86400),
        phase_prefix: None,
        branch: None,
    };
    let total = store.list(&f).await.map(|v| v.len()).unwrap_or(0);
    let f_live = PingFilter {
        max_age_s: Some(90),
        ..Default::default()
    };
    let live = store.list(&f_live).await.map(|v| v.len()).unwrap_or(0);
    println!(
        "backend: {}\nlive (≤90s):    {}\ntotal (≤24h):  {}",
        store.kind().as_str(),
        live,
        total
    );
    0
}
