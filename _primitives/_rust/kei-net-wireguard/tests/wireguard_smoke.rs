// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Smoke tests for `kei-net-wireguard`. Drive the `WireguardMode` impl
//! through a recording [`MockRunner`] — never invokes a real `wg-quick`.

use anyhow::Result as AnyResult;
use kei_net_wireguard::{parse_wg_dump, RunOutput, Runner, WireguardMode};
use kei_runtime_core::traits::network::{NetworkConfig, NetworkMode};
use kei_runtime_core::HasDna;
use std::sync::{Arc, Mutex};

/// Records every `(cmd, args)` invocation and replays a programmable
/// stdout. Default behaviour: succeed with empty stdout.
#[derive(Default)]
struct MockRunner {
    calls: Mutex<Vec<(String, Vec<String>)>>,
    stdout: Mutex<String>,
}

impl MockRunner {
    fn new() -> Self {
        Self::default()
    }

    fn with_stdout(self, s: &str) -> Self {
        *self.stdout.lock().unwrap() = s.to_string();
        self
    }

    fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().unwrap().clone()
    }
}

impl Runner for MockRunner {
    fn run(&self, cmd: &str, args: &[&str]) -> AnyResult<RunOutput> {
        self.calls.lock().unwrap().push((
            cmd.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        ));
        let body = self.stdout.lock().unwrap().clone();
        Ok(RunOutput::ok(body))
    }
}

fn cfg() -> NetworkConfig {
    NetworkConfig {
        mode: "wireguard".into(),
        hostname: "host-test".into(),
        auth_secret_env: None,
        allowed_ports: vec![51820],
    }
}

#[tokio::test]
async fn configure_invokes_wg_quick_up_with_iface() {
    let mock = Arc::new(MockRunner::new());
    let mode = WireguardMode::new(mock.clone(), None, "wg0").expect("ctor");
    mode.configure(&cfg()).await.expect("configure ok");

    let calls = mock.calls();
    assert_eq!(calls.len(), 1, "expected exactly one shell-out");
    assert_eq!(calls[0].0, "wg-quick");
    assert_eq!(calls[0].1, vec!["up".to_string(), "wg0".to_string()]);
}

#[tokio::test]
async fn teardown_invokes_wg_quick_down() {
    let mock = Arc::new(MockRunner::new());
    let mode = WireguardMode::new(mock.clone(), None, "wg1").expect("ctor");
    mode.teardown().await.expect("teardown ok");

    let calls = mock.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "wg-quick");
    assert_eq!(calls[0].1, vec!["down".to_string(), "wg1".to_string()]);
}

#[tokio::test]
async fn peers_parses_dump_output() {
    let dump = "PRIVKEY=\tIPUB=\t51820\toff\n\
                PEER1=\t(none)\t10.20.30.40:51820\t10.0.0.2/32\t1700000099\t111\t222\t25\n";
    let mock = Arc::new(MockRunner::new().with_stdout(dump));
    let mode = WireguardMode::new(mock.clone(), None, "wg0").expect("ctor");

    let peers = mode.peers().await.expect("peers ok");
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].addr, "10.20.30.40:51820");
    assert_eq!(peers[0].last_seen_ms, 1_700_000_099_000);
    assert_eq!(peers[0].bytes_rx, 111);
    assert_eq!(peers[0].bytes_tx, 222);

    let calls = mock.calls();
    assert_eq!(calls[0].0, "wg");
    assert_eq!(
        calls[0].1,
        vec!["show".to_string(), "wg0".to_string(), "dump".to_string()]
    );
}

#[tokio::test]
async fn parse_handles_empty_dump() {
    // Direct unit-style assertion against the exported parser, plus the
    // round-trip through `peers()` with empty stdout.
    assert!(parse_wg_dump("").is_empty());

    let mock = Arc::new(MockRunner::new().with_stdout(""));
    let mode = WireguardMode::new(mock, None, "wg0").expect("ctor");
    let peers = mode.peers().await.expect("peers ok");
    assert!(peers.is_empty());
}

#[test]
fn dna_has_wg_cap() {
    let mock = Arc::new(MockRunner::new());
    let mode = WireguardMode::new(mock, None, "wg0").expect("ctor");
    let caps = mode.dna().caps();
    // DNA caps are `-`-joined; spec mandates PR-AP-WG.
    assert_eq!(caps, "PR-AP-WG", "dna caps must be PR-AP-WG, got {caps}");
}

#[test]
fn mode_is_private() {
    let mock = Arc::new(MockRunner::new());
    let mode = WireguardMode::new(mock, None, "wg0").expect("ctor");
    assert!(!mode.is_public(), "WireGuard is a private mesh");
    assert_eq!(mode.mode_name(), "wireguard");
    assert_eq!(mode.iface(), "wg0");
}
