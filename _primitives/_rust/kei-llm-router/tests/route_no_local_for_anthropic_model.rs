//! Test 3 — `claude-opus-4-7` + `--require-local` → NoBackendAvailable.
//!
//! Even though the machine could run local models, NO local backend has
//! a remote-only model id. With `require_local=true`, route MUST refuse
//! to walk the registry fallback chain (which would yield Anthropic).

use kei_llm_router::{decide, Error, RouteOpts};
use kei_machine_probe::{
    AppleVariant, ArchInfo, CpuFamily, GpuInfo, Machine, MemoryInfo, OsFamily, OsInfo,
    ToolingInfo,
};

fn fixture_m3_pro_18gb() -> Machine {
    Machine {
        os: OsInfo {
            family: OsFamily::Macos,
            version: "15.6".into(),
            build: "24G80".into(),
        },
        arch: ArchInfo {
            family: CpuFamily::AppleSilicon(AppleVariant::M3Pro),
            brand: "Apple M3 Pro".into(),
            model_id: "Mac15,6".into(),
            cores: 12,
        },
        memory: MemoryInfo {
            total_bytes: 18u64 * 1024 * 1024 * 1024,
            available_bytes: 8u64 * 1024 * 1024 * 1024,
            pressure_pct: 30,
        },
        gpu: GpuInfo::AppleIntegrated { cores: 18, name: "Apple M3 Pro GPU".into() },
        tooling: ToolingInfo::default(),
        source_commands: Vec::new(),
    }
}

#[test]
fn require_local_refuses_remote_only_model() {
    let machine = fixture_m3_pro_18gb();
    let candidates = Vec::new(); // no local backend has claude-opus-4-7
    let opts = RouteOpts { require_local: true, ..RouteOpts::default() };
    let err = decide(&machine, "claude-opus-4-7", &candidates, &opts, None)
        .expect_err("must error");
    match err {
        Error::NoBackendAvailable { model_id, .. } => {
            assert_eq!(model_id, "claude-opus-4-7");
        }
        other => panic!("expected NoBackendAvailable, got {other:?}"),
    }
}

#[test]
fn require_local_returns_exit_code_2() {
    let err = Error::NoBackendAvailable {
        model_id: "claude-opus-4-7".into(),
        tried: vec!["mlx".into(), "llamacpp".into(), "ollama".into()],
    };
    assert_eq!(err.exit_code(), 2);
}
