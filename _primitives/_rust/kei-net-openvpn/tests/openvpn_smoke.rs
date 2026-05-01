// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Smoke tests for `OpenvpnMode`. We use a recording `MockRunner`
//! instead of `SystemRunner`, so these tests are hermetic — no
//! `systemctl`, no UNIX socket, no live OpenVPN.

use kei_net_openvpn::{parse_status_output, OpenvpnMode, RunOutput, Runner};
use kei_runtime_core::traits::network::{NetworkConfig, NetworkMode};
use kei_runtime_core::HasDna;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MockRunner {
    calls: Mutex<Vec<(String, Vec<String>)>>,
    /// Per-call result. If empty, default to status=0.
    next_result: Mutex<Option<RunOutput>>,
}

impl MockRunner {
    fn new() -> Arc<Self> {
        Arc::new(MockRunner::default())
    }

    fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().unwrap().clone()
    }

    fn set_next(&self, out: RunOutput) {
        *self.next_result.lock().unwrap() = Some(out);
    }
}

impl Runner for MockRunner {
    fn run(&self, program: &str, args: &[&str]) -> kei_net_openvpn::Result<RunOutput> {
        self.calls.lock().unwrap().push((
            program.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        ));
        let preset = self.next_result.lock().unwrap().take();
        Ok(preset.unwrap_or(RunOutput {
            status: 0,
            stdout: String::new(),
            stderr: String::new(),
        }))
    }
}

fn cfg() -> NetworkConfig {
    NetworkConfig {
        mode: "openvpn".into(),
        hostname: "vpn.example.test".into(),
        auth_secret_env: None,
        allowed_ports: vec![1194],
    }
}

#[tokio::test]
async fn configure_invokes_systemctl_start() {
    let runner = MockRunner::new();
    let mode = OpenvpnMode::with_runner(
        runner.clone(),
        "server",
        "/etc/openvpn/server/server.conf",
        None,
        None,
    )
    .expect("ctor");

    mode.configure(&cfg()).await.expect("configure ok");

    let calls = runner.calls();
    assert_eq!(calls.len(), 1, "exactly one systemctl invocation expected");
    assert_eq!(calls[0].0, "systemctl");
    assert_eq!(calls[0].1, vec!["start".to_string(), "openvpn-server@server".to_string()]);
}

#[tokio::test]
async fn teardown_invokes_systemctl_stop() {
    let runner = MockRunner::new();
    let mode = OpenvpnMode::with_runner(
        runner.clone(),
        "server",
        "/etc/openvpn/server/server.conf",
        None,
        None,
    )
    .expect("ctor");

    mode.teardown().await.expect("teardown ok");

    let calls = runner.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "systemctl");
    assert_eq!(calls[0].1, vec!["stop".to_string(), "openvpn-server@server".to_string()]);
}

#[tokio::test]
async fn peers_returns_empty_when_no_socket() {
    let runner = MockRunner::new();
    let mode = OpenvpnMode::with_runner(
        runner,
        "server",
        "/etc/openvpn/server/server.conf",
        None, // no mgmt socket
        None,
    )
    .expect("ctor");

    let peers = mode.peers().await.expect("peers ok");
    assert!(peers.is_empty(), "no socket → no peers");
}

#[tokio::test]
async fn dna_has_ov_cap() {
    let runner = MockRunner::new();
    let mode = OpenvpnMode::with_runner(
        runner,
        "server",
        "/etc/openvpn/server/server.conf",
        None,
        None,
    )
    .expect("ctor");

    let caps = mode.dna().caps();
    assert!(caps.contains("PR"), "DNA caps must include PR: {caps}");
    assert!(caps.contains("AP"), "DNA caps must include AP: {caps}");
    assert!(caps.contains("OV"), "DNA caps must include OV: {caps}");
    assert_eq!(mode.mode_name(), "openvpn");
    assert!(mode.is_public(), "OpenVPN exposes a public endpoint");
}

#[tokio::test]
async fn parse_status_handles_empty() {
    let v = parse_status_output("").expect("parse ok");
    assert!(v.is_empty(), "empty input → no peers");
}

#[tokio::test]
async fn configure_surfaces_systemctl_failure() {
    let runner = MockRunner::new();
    runner.set_next(RunOutput {
        status: 1,
        stdout: String::new(),
        stderr: "Unit openvpn-server@server.service not found.".into(),
    });
    let mode = OpenvpnMode::with_runner(
        runner.clone(),
        "server",
        "/etc/openvpn/server/server.conf",
        None,
        None,
    )
    .expect("ctor");

    let err = mode.configure(&cfg()).await.expect_err("must fail");
    let s = err.to_string();
    assert!(s.contains("provider"), "systemctl failure → provider err: {s}");
    assert!(s.contains("Unit openvpn-server@server.service not found."), "stderr surfaced: {s}");
}
