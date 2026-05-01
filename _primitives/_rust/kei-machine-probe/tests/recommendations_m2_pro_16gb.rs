//! Recommendations on Apple Silicon M2 Pro / 16 GB.

use kei_machine_probe::{
    recommend, AppleVariant, ArchInfo, BackendTier, Capability, CpuFamily, GpuInfo, Machine,
    MemoryInfo, OsFamily, OsInfo, ToolingInfo,
};

fn machine_m2_pro_16gb() -> Machine {
    Machine {
        os: OsInfo { family: OsFamily::Macos, version: "14.5".into(), build: "23F79".into() },
        arch: ArchInfo {
            family: CpuFamily::AppleSilicon(AppleVariant::M2Pro),
            brand: "Apple M2 Pro".into(),
            model_id: "Mac14,7".into(),
            cores: 10,
        },
        memory: MemoryInfo {
            total_bytes: 17_179_869_184,
            available_bytes: 8_000_000_000,
            pressure_pct: 50,
        },
        gpu: GpuInfo::AppleIntegrated { cores: 19, name: "Apple M2 Pro".into() },
        tooling: ToolingInfo::default(),
        source_commands: vec![],
    }
}

#[test]
fn m2_pro_16gb_runs_large_models() {
    let r = recommend(&machine_m2_pro_16gb());
    assert_eq!(r.capability, Capability::RunsLargeModels);
    assert_eq!(
        r.viable_backends,
        vec![BackendTier::MlxNative, BackendTier::LlamaCpp, BackendTier::Ollama]
    );
    assert!(r.max_model_b_params >= 13);
    assert!(!r.rationale.is_empty());
}

#[test]
fn apple_silicon_8gb_runs_mid_models() {
    let mut m = machine_m2_pro_16gb();
    m.memory.total_bytes = 8_589_934_592;
    let r = recommend(&m);
    assert_eq!(r.capability, Capability::RunsMidModels);
    assert!(r.viable_backends.contains(&BackendTier::MlxNative));
}
