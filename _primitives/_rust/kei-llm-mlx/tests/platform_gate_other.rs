//! Platform gate — non-target side of the truth-table.
//!
//! Linux ARM, macOS Intel, Windows on anything → `supported: false`
//! with the stable reason string. The reason is exposed to JSON
//! consumers (probe / version) so callers can do diagnostic routing.

use kei_llm_mlx::{is_supported, platform::REASON_NOT_MACOS};

#[test]
fn non_target_unsupported_with_stable_reason() {
    let s = is_supported();
    let is_target = cfg!(target_os = "macos") && cfg!(target_arch = "aarch64");
    if is_target {
        // On a target build, this test is vacuous. We still assert the
        // structural shape so an accidental swap of branches in
        // `is_supported()` would be caught.
        assert!(s.supported);
    } else {
        assert!(!s.supported, "non-target build must report supported=false");
        assert_eq!(
            s.reason.as_deref(),
            Some(REASON_NOT_MACOS),
            "reason string is part of the public contract",
        );
    }
}
