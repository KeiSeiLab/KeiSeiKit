//! Test 2 — Intel 8 GB → only Ollama tier viable; route returns Ollama.

use kei_llm_router::{decide, Backend, BackendKind, ModelMatch, RouteOpts};
use kei_machine_probe::{
    ArchInfo, CpuFamily, GpuInfo, Machine, MemoryInfo, OsFamily, OsInfo, ToolingInfo,
};

fn fixture_intel_8gb() -> Machine {
    Machine {
        os: OsInfo {
            family: OsFamily::Macos,
            version: "13.4".into(),
            build: "22F66".into(),
        },
        arch: ArchInfo {
            family: CpuFamily::IntelX86_64,
            brand: "Intel Core i5".into(),
            model_id: "MacBookPro15,2".into(),
            cores: 4,
        },
        memory: MemoryInfo {
            total_bytes: 8u64 * 1024 * 1024 * 1024,
            available_bytes: 2u64 * 1024 * 1024 * 1024,
            pressure_pct: 70,
        },
        gpu: GpuInfo::IntelIntegrated { name: "Iris Plus".into() },
        tooling: ToolingInfo::default(),
        source_commands: Vec::new(),
    }
}

#[test]
fn intel_8gb_picks_ollama() {
    let machine = fixture_intel_8gb();
    // Intel/8GB tier exposes ONLY Ollama as viable per W56 recommendations.
    let candidates = vec![(BackendKind::Ollama, ModelMatch::exact())];
    let opts = RouteOpts { require_local: true, ..RouteOpts::default() };
    let decision =
        decide(&machine, "llama3:8b", &candidates, &opts, None).expect("route ok");
    assert_eq!(decision.backend.kind(), BackendKind::Ollama);
    match decision.backend {
        Backend::Ollama { model_tag } => assert_eq!(model_tag, "llama3:8b"),
        other => panic!("expected Ollama variant, got {other:?}"),
    }
}

#[test]
fn intel_8gb_recommendations_exclude_mlx_and_llamacpp() {
    let machine = fixture_intel_8gb();
    let recs = kei_machine_probe::recommend(&machine);
    let kinds: Vec<_> = recs
        .viable_backends
        .iter()
        .filter_map(kei_llm_router::from_tier)
        .collect();
    assert_eq!(kinds, vec![BackendKind::Ollama]);
}
