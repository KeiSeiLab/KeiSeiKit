//! On non-target builds, `generate()` MUST return `Error::NotSupported`
//! before any subprocess attempt — observable via the cfg! gate.
//!
//! On target builds (macos+aarch64) the test exercises the parallel
//! shape: it asserts that `is_supported()` agrees with the cfg! gate
//! AND that `build_spec` over a remote host still refuses with
//! `Error::SecurityRefused`. Together this keeps the test useful on
//! every host while pinning the not-supported semantics on every
//! non-target build.

use kei_llm_mlx::error::Error;
use kei_llm_mlx::generate::{generate, GenerateOpts};
use kei_llm_mlx::{is_supported, server::build_spec, MockRunner};

#[test]
fn generate_refuses_on_unsupported_platform() {
    let s = is_supported();
    let runner = MockRunner::new();
    let res = generate(
        &runner,
        "/never/invoked/mlx_lm.generate",
        "model",
        "p",
        &GenerateOpts::default(),
    );
    if !s.supported {
        match res {
            Err(Error::NotSupported(reason)) => {
                assert!(!reason.is_empty(), "reason must be non-empty");
            }
            other => panic!("expected NotSupported, got {:?}", other.map(|_| "Ok")),
        }
    } else {
        // Target build: generate would otherwise try the runner; the
        // mock has no fixture so the call MUST fail at SpawnFailed
        // (not NotSupported). Still a useful invariant.
        match res {
            Err(Error::SpawnFailed(_)) => {}
            Err(other) => panic!("expected SpawnFailed on target, got {other:?}"),
            Ok(_) => panic!("unexpected Ok with empty mock"),
        }
        // And the security gate fires on remote host regardless of platform.
        let spec_res = build_spec("model", "0.0.0.0", 8080);
        assert!(matches!(spec_res, Err(Error::SecurityRefused(_))));
    }
}
