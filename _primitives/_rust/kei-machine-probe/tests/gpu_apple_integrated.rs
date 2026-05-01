//! GPU detection on Apple Silicon (M2 Pro / 19-core integrated).

use kei_machine_probe::{detect_gpu, GpuInfo, MockRunner};

const SP_M2_PRO: &str = r#"{
  "SPDisplaysDataType": [
    {
      "_name": "Apple M2 Pro",
      "spdisplays_mtlgpufamilysupport": "Apple9",
      "sppci_bus": "spdisplays_builtin",
      "sppci_cores": "19",
      "sppci_device_type": "spdisplays_gpu",
      "sppci_model": "Apple M2 Pro",
      "sppci_vendor": "sppci_vendor_Apple"
    }
  ]
}"#;

#[test]
fn parses_apple_m2_pro_19_cores() {
    let runner = MockRunner::from_dir(".").with_ok(
        "system_profiler_SPDisplaysDataType_-json",
        SP_M2_PRO,
    );

    let gpu = detect_gpu(&runner);

    match gpu {
        GpuInfo::AppleIntegrated { cores, name } => {
            assert_eq!(cores, 19);
            assert_eq!(name, "Apple M2 Pro");
        }
        other => panic!("expected AppleIntegrated, got {:?}", other),
    }
}
