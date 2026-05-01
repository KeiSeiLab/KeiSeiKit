//! OS detection on macOS via sw_vers.

use kei_machine_probe::{detect_os, MockRunner, OsFamily};

#[test]
fn parses_sw_vers_to_macos_info() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sw_vers_-productVersion", "14.5\n")
        .with_ok("sw_vers_-buildVersion", "23F79\n");

    let os = detect_os(&runner);
    assert_eq!(os.family, OsFamily::Macos);
    assert_eq!(os.version, "14.5");
    assert_eq!(os.build, "23F79");
}

#[test]
fn missing_build_falls_through_but_keeps_version() {
    let runner = MockRunner::from_dir(".")
        .with_ok("sw_vers_-productVersion", "13.6.1\n")
        .with_err("sw_vers_-buildVersion", "no build key");

    let os = detect_os(&runner);
    assert_eq!(os.family, OsFamily::Macos);
    assert_eq!(os.version, "13.6.1");
    assert_eq!(os.build, "");
}
