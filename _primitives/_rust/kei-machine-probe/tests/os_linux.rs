//! OS detection on Linux via uname fallback when sw_vers is missing.

use kei_machine_probe::{detect_os, MockRunner, OsFamily};

#[test]
fn falls_back_to_uname_on_linux() {
    let runner = MockRunner::from_dir(".")
        .with_err("sw_vers_-productVersion", "command not found")
        .with_err("sw_vers_-buildVersion", "command not found")
        .with_ok("uname_-sr", "Linux 5.15.0-118-generic\n");

    let os = detect_os(&runner);
    assert_eq!(os.family, OsFamily::Linux);
    assert_eq!(os.version, "5.15.0-118-generic");
    assert!(os.build.is_empty());
}

#[test]
fn unknown_kernel_reports_other() {
    let runner = MockRunner::from_dir(".")
        .with_err("sw_vers_-productVersion", "no")
        .with_err("sw_vers_-buildVersion", "no")
        .with_ok("uname_-sr", "FreeBSD 14.0-RELEASE\n");

    let os = detect_os(&runner);
    assert_eq!(os.family, OsFamily::Other);
    assert_eq!(os.version, "14.0-RELEASE");
}

#[test]
fn no_uname_either_reports_other_with_empty_version() {
    let runner = MockRunner::from_dir(".")
        .with_err("sw_vers_-productVersion", "no")
        .with_err("sw_vers_-buildVersion", "no")
        .with_err("uname_-sr", "no");

    let os = detect_os(&runner);
    assert_eq!(os.family, OsFamily::Other);
    assert!(os.version.is_empty());
}
