// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Smoke tests for [`kei_net_ipsec::IpsecMode`].
//!
//! All shell-out is routed through [`kei_net_ipsec::MockRunner`]; nothing
//! invokes a real `swanctl` binary, so these tests run without root and
//! without strongSwan installed.

use kei_net_ipsec::{parse_sas_output, IpsecMode, MockRunner, RunOutput};
use kei_runtime_core::traits::network::{NetworkConfig, NetworkMode};
use kei_runtime_core::HasDna;
use std::sync::Arc;

fn empty_cfg() -> NetworkConfig {
    NetworkConfig {
        mode: "ipsec".into(),
        hostname: "test.example".into(),
        auth_secret_env: None,
        allowed_ports: vec![500, 4500],
    }
}

#[tokio::test]
async fn configure_invokes_swanctl_initiate() {
    let mock = MockRunner::new()
        .with_ok("swanctl_--load-all", "")
        .with_ok("swanctl_--initiate_--child_home", "");
    let mock = Arc::new(mock);
    let mode = IpsecMode::new(mock.clone(), None, "home").expect("ctor");
    mode.configure(&empty_cfg()).await.expect("configure ok");

    let calls = mock.recorded();
    assert!(
        calls.iter().any(|(c, a)| c == "swanctl" && a == &["--load-all".to_string()]),
        "must invoke swanctl --load-all"
    );
    assert!(
        calls.iter().any(|(c, a)| c == "swanctl"
            && a == &["--initiate".to_string(), "--child".to_string(), "home".to_string()]),
        "must invoke swanctl --initiate --child <name>"
    );
}

#[tokio::test]
async fn teardown_invokes_swanctl_terminate() {
    let mock = MockRunner::new().with_ok("swanctl_--terminate_--child_home", "");
    let mock = Arc::new(mock);
    let mode = IpsecMode::new(mock.clone(), None, "home").expect("ctor");
    mode.teardown().await.expect("teardown ok");

    let calls = mock.recorded();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "swanctl");
    assert_eq!(
        calls[0].1,
        vec!["--terminate".to_string(), "--child".to_string(), "home".to_string()]
    );
}

#[tokio::test]
async fn peers_parses_canned_sas_output() {
    let canned = "\
home: #1, ESTABLISHED, IKEv2, abcd_i* def_r
  local  'gw' @ 192.0.2.1[500]
  remote 'peer' @ 198.51.100.7[4500]
  encr_alg=AES_GCM_16
  home: #2, reqid 1, INSTALLED, TUNNEL, ESP:AES_GCM_16
    bytes_i (1.04K, 1067 bytes), packets_i (12 packets)
    bytes_o (3.50K, 3584 bytes), packets_o (24 packets)
";
    let mock = MockRunner::new().with_ok("swanctl_--list-sas", canned);
    let mode = IpsecMode::new(Arc::new(mock), None, "home").expect("ctor");
    let peers = mode.peers().await.expect("peers ok");
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].addr, "198.51.100.7");
    assert_eq!(peers[0].bytes_rx, 1067);
    assert_eq!(peers[0].bytes_tx, 3584);
}

#[tokio::test]
async fn dna_has_ip_cap() {
    let mock = MockRunner::new();
    let mode = IpsecMode::new(Arc::new(mock), None, "home").expect("ctor");
    let caps = mode.dna().caps();
    assert!(caps.contains("IP"), "DNA caps must contain IP token, got {caps:?}");
    assert!(caps.contains("PR"), "DNA caps must contain PR token");
    assert!(caps.contains("AP"), "DNA caps must contain AP token");
    assert_eq!(mode.mode_name(), "ipsec");
    assert!(mode.is_public(), "ipsec is the public-IP NetworkMode");
}

#[tokio::test]
async fn parse_handles_empty_input() {
    // Direct parser invariant.
    assert!(parse_sas_output("").is_empty());

    // End-to-end: empty `swanctl --list-sas` stdout must yield `Ok(vec![])`.
    let mock = MockRunner::new().with_ok("swanctl_--list-sas", "");
    let mode = IpsecMode::new(Arc::new(mock), None, "home").expect("ctor");
    let peers = mode.peers().await.expect("peers ok");
    assert!(peers.is_empty());
}

#[tokio::test]
async fn nonzero_exit_surfaces_swanctl_failed() {
    // Defensive: `swanctl` returning non-zero exit must NOT be silently
    // swallowed — it must propagate as Network error.
    let mock = MockRunner::new().with_run(
        "swanctl_--terminate_--child_home",
        RunOutput::fail(1, "child SA 'home' not found"),
    );
    let mode = IpsecMode::new(Arc::new(mock), None, "home").expect("ctor");
    let err = mode.teardown().await.expect_err("must fail");
    let msg = format!("{err}");
    assert!(
        msg.contains("network") || msg.contains("swanctl") || msg.contains("not found"),
        "error must surface swanctl failure context, got: {msg}"
    );
}
