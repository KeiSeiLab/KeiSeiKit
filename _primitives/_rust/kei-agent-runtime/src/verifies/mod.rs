//! On-return verify capabilities.
//!
//! After v0.18 convergence wave: 3 command-driven verifies
//! (`quality::cargo-check-green`, `quality::tests-green`,
//! `safety::no-dep-bump`) are `CommandVerify` const wrappers. The LOC
//! walker (`quality::constructor-pattern`), the two report-parser
//! verifies (`output::*`), and the two git-diff scope verifies stay in
//! their own modules — shape too divergent to fold into `CommandVerify`.

pub mod command_verify;
pub mod output_report_format;
pub mod output_severity_grade;
pub mod quality_cargo_check_green;
pub mod quality_constructor_pattern;
pub mod quality_tests_green;
pub mod safety_no_dep_bump;
pub mod scope_files_denylist;
pub mod scope_files_whitelist;
