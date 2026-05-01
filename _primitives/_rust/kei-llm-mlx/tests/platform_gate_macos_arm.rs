//! Platform gate — macOS Apple Silicon side of the truth-table.
//!
//! On a macos+aarch64 build, `is_supported()` MUST return `supported: true`
//! and `reason: None`. The reverse (any other target) is asserted in
//! `platform_gate_other.rs`. Together the two tests pin the two-way
//! cfg! gate so refactors cannot silently flip it.

use kei_llm_mlx::is_supported;

#[test]
fn macos_aarch64_supported() {
    let s = is_supported();
    if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        assert!(s.supported, "macos+aarch64 build must report supported=true");
        assert!(s.reason.is_none(), "supported builds must have reason=None");
        assert_eq!(s.host_arch, "aarch64");
        assert_eq!(s.host_os, "macos");
    } else {
        // Non-target build: just confirm the inverse so this test is a
        // structural witness on every host. The thorough non-target
        // assertions live in `platform_gate_other.rs`.
        assert!(!s.supported);
    }
}
