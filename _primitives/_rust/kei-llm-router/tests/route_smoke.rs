//! Test 1 — M3 Pro 18 GB → MLX is preferred, route picks Mlx.

use kei_llm_router::{decide, Backend, BackendKind, ModelMatch, RouteOpts};
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
fn m3_pro_picks_mlx_for_local_70b() {
    let machine = fixture_m3_pro_18gb();
    let candidates = vec![
        (BackendKind::Mlx, ModelMatch::exact()),
        (BackendKind::LlamaCpp, ModelMatch::exact()),
        (BackendKind::Ollama, ModelMatch::exact()),
    ];
    let opts = RouteOpts { require_local: true, ..RouteOpts::default() };
    let decision =
        decide(&machine, "llama-3-70b-local", &candidates, &opts, None).expect("route ok");
    assert_eq!(decision.backend.kind(), BackendKind::Mlx);
    match decision.backend {
        Backend::Mlx { model_id } => assert_eq!(model_id, "llama-3-70b-local"),
        other => panic!("expected Mlx variant, got {other:?}"),
    }
    assert!(!decision.rationale.is_empty());
}
