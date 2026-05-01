//! GPU detection on a 16" Intel MacBook Pro (AMD Radeon Pro discrete).

use kei_machine_probe::{detect_gpu, GpuInfo, MockRunner};

const SP_AMD_DISCRETE: &str = r#"{
  "SPDisplaysDataType": [
    {
      "_name": "AMD Radeon Pro 5500M",
      "sppci_bus": "spdisplays_pcie_device",
      "sppci_device_type": "spdisplays_gpu",
      "sppci_model": "AMD Radeon Pro 5500M",
      "sppci_vendor": "sppci_vendor_amd"
    },
    {
      "_name": "Intel UHD Graphics 630",
      "sppci_bus": "spdisplays_builtin",
      "sppci_device_type": "spdisplays_gpu",
      "sppci_model": "Intel UHD Graphics 630",
      "sppci_vendor": "sppci_vendor_intel"
    }
  ]
}"#;

#[test]
fn prefers_discrete_amd_over_intel_integrated() {
    let runner = MockRunner::from_dir(".").with_ok(
        "system_profiler_SPDisplaysDataType_-json",
        SP_AMD_DISCRETE,
    );

    let gpu = detect_gpu(&runner);

    match gpu {
        GpuInfo::Discrete { name } => assert_eq!(name, "AMD Radeon Pro 5500M"),
        other => panic!("expected Discrete, got {:?}", other),
    }
}

#[test]
fn returns_none_when_command_missing() {
    let runner = MockRunner::from_dir(".")
        .with_err("system_profiler_SPDisplaysDataType_-json", "missing");
    assert!(matches!(detect_gpu(&runner), GpuInfo::None));
}
