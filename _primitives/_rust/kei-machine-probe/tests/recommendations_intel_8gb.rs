//! Recommendations on Intel Mac with 8 GB and on Linux (NotViable).

use kei_machine_probe::{
    recommend, ArchInfo, BackendTier, Capability, CpuFamily, GpuInfo, Machine, MemoryInfo,
    OsFamily, OsInfo, ToolingInfo,
};

fn machine_intel(gb: u64, family: OsFamily) -> Machine {
    Machine {
        os: OsInfo { family, version: "12.7".into(), build: "21H1015".into() },
        arch: ArchInfo {
            family: CpuFamily::IntelX86_64,
            brand: "Intel(R) Core(TM) i7".into(),
            model_id: "MacBookPro16,4".into(),
            cores: 16,
        },
        memory: MemoryInfo {
            total_bytes: gb * 1024 * 1024 * 1024,
            available_bytes: 1_000_000_000,
            pressure_pct: 60,
        },
        gpu: GpuInfo::IntelIntegrated { name: "Intel UHD".into() },
        tooling: ToolingInfo::default(),
        source_commands: vec![],
    }
}

#[test]
fn intel_8gb_runs_small_models_only_via_ollama() {
    let m = machine_intel(8, OsFamily::Macos);
    let r = recommend(&m);
    assert_eq!(r.capability, Capability::RunsSmallModelsOnly);
    assert_eq!(r.viable_backends, vec![BackendTier::Ollama]);
    assert!(r.max_model_b_params <= 3);
}

#[test]
fn intel_16gb_runs_mid_models_no_mlx() {
    let m = machine_intel(16, OsFamily::Macos);
    let r = recommend(&m);
    assert_eq!(r.capability, Capability::RunsMidModels);
    assert!(!r.viable_backends.contains(&BackendTier::MlxNative));
    assert!(r.viable_backends.contains(&BackendTier::LlamaCpp));
}

#[test]
fn linux_marks_not_viable() {
    let m = machine_intel(32, OsFamily::Linux);
    let r = recommend(&m);
    assert_eq!(r.capability, Capability::NoLocalInferenceViable);
    assert_eq!(r.viable_backends, vec![BackendTier::NotViable]);
    assert_eq!(r.max_model_b_params, 0);
}
