//! Machine struct JSON roundtrip — bytes equal after re-serialise.

use kei_machine_probe::{
    AppleVariant, ArchInfo, CpuFamily, GpuInfo, Machine, MemoryInfo, OsFamily, OsInfo,
    ToolingInfo,
};

fn sample() -> Machine {
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
            available_bytes: 5_000_000_000,
            pressure_pct: 50,
        },
        gpu: GpuInfo::AppleIntegrated { cores: 19, name: "Apple M2 Pro".into() },
        tooling: ToolingInfo {
            ollama: Some("0.3.12".into()),
            homebrew: Some("4.3.20".into()),
            llama_cpp: None,
        },
        source_commands: vec!["sysctl -n hw.model".into(), "vm_stat".into()],
    }
}

#[test]
fn roundtrip_preserves_struct() {
    let m = sample();
    let json = serde_json::to_string_pretty(&m).expect("serialize");
    let m2: Machine = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(m, m2);
}

#[test]
fn roundtrip_bytes_are_stable() {
    let m = sample();
    let j1 = serde_json::to_string_pretty(&m).expect("serialize 1");
    let m2: Machine = serde_json::from_str(&j1).expect("deserialize");
    let j2 = serde_json::to_string_pretty(&m2).expect("serialize 2");
    assert_eq!(j1, j2);
}
