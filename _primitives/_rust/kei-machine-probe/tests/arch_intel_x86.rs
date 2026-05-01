//! Arch detection on an Intel MacBook Pro fixture.

use kei_machine_probe::{detect_arch, CpuFamily, MockRunner};

#[test]
fn detects_intel_x86_64_when_arm64_flag_zero() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.model", "MacBookPro16,4\n")
        .with_ok(
            "sysctl_-n_machdep.cpu.brand_string",
            "Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz\n",
        )
        .with_ok("sysctl_-n_hw.optional.arm64", "0\n")
        .with_ok("sysctl_-n_hw.ncpu", "16\n");

    let arch = detect_arch(&runner);
    assert_eq!(arch.family, CpuFamily::IntelX86_64);
    assert_eq!(arch.model_id, "MacBookPro16,4");
    assert_eq!(arch.cores, 16);
    assert!(arch.brand.contains("Intel"));
}

#[test]
fn intel_brand_without_arm64_flag_still_classifies() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.model", "iMac20,1\n")
        .with_ok(
            "sysctl_-n_machdep.cpu.brand_string",
            "Intel(R) Core(TM) i7\n",
        )
        .with_err("sysctl_-n_hw.optional.arm64", "key not found")
        .with_ok("sysctl_-n_hw.ncpu", "8\n");

    let arch = detect_arch(&runner);
    assert_eq!(arch.family, CpuFamily::IntelX86_64);
}
