//! Test 6 — empty candidate list (every backend down) →
//!   * `decide` returns `Error::NoBackendAvailable`,
//!   * the tried-list mirrors the discovery output (here: `<none>`).

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
fn empty_candidates_yield_no_backend_available() {
    let machine = fixture_m3_pro_18gb();
    let opts = RouteOpts { require_local: true, ..RouteOpts::default() };
    let err = decide(&machine, "qwen3:4b", &[], &opts, None).expect_err("must error");
    match err {
        Error::NoBackendAvailable { model_id, tried } => {
            assert_eq!(model_id, "qwen3:4b");
            assert_eq!(tried, vec!["<none>".to_string()]);
        }
        other => panic!("expected NoBackendAvailable, got {other:?}"),
    }
}

#[test]
fn no_local_inference_viable_when_required() {
    let mut linux = fixture_m3_pro_18gb();
    linux.os.family = OsFamily::Linux;
    let opts = RouteOpts { require_local: true, ..RouteOpts::default() };
    let err = decide(&linux, "any", &[], &opts, None).expect_err("must error");
    match err {
        Error::NoCompatibleBackend { .. } => {}
        other => panic!("expected NoCompatibleBackend, got {other:?}"),
    }
}
