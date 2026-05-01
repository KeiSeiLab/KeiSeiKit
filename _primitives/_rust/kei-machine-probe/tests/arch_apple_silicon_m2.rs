//! Arch detection on M2 Pro fixture.

use kei_machine_probe::{detect_arch, AppleVariant, CpuFamily, MockRunner};

#[test]
fn detects_m2_pro_apple_silicon() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.model", "Mac14,7\n")
        .with_ok("sysctl_-n_machdep.cpu.brand_string", "Apple M2 Pro\n")
        .with_ok("sysctl_-n_hw.optional.arm64", "1\n")
        .with_ok("sysctl_-n_hw.ncpu", "10\n");

    let arch = detect_arch(&runner);

    assert_eq!(arch.family, CpuFamily::AppleSilicon(AppleVariant::M2Pro));
    assert_eq!(arch.brand, "Apple M2 Pro");
    assert_eq!(arch.model_id, "Mac14,7");
    assert_eq!(arch.cores, 10);
}

#[test]
fn detects_plain_m2() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.model", "Mac14,2\n")
        .with_ok("sysctl_-n_machdep.cpu.brand_string", "Apple M2\n")
        .with_ok("sysctl_-n_hw.optional.arm64", "1\n")
        .with_ok("sysctl_-n_hw.ncpu", "8\n");

    let arch = detect_arch(&runner);
    assert_eq!(arch.family, CpuFamily::AppleSilicon(AppleVariant::M2));
}

#[test]
fn detects_m3_max() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sysctl_-n_hw.model", "Mac15,9\n")
        .with_ok("sysctl_-n_machdep.cpu.brand_string", "Apple M3 Max\n")
        .with_ok("sysctl_-n_hw.optional.arm64", "1\n")
        .with_ok("sysctl_-n_hw.ncpu", "16\n");

    let arch = detect_arch(&runner);
    assert_eq!(arch.family, CpuFamily::AppleSilicon(AppleVariant::M3Max));
}
