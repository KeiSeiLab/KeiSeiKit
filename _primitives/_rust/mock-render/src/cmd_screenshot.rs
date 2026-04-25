//! `mock-render screenshot <url> --out <png> [--viewport WxH]`
//!
//! Extracted from `main.rs` in v0.14.1 per Constructor Pattern.

use crate::cli_args::{flag, parse_viewport};
use crate::render;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn run(args: &[String]) -> ExitCode {
    let Some(url) = args.first().cloned() else {
        eprintln!("screenshot: <url> required");
        return ExitCode::from(1);
    };
    let out = match flag(args, "--out") {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("screenshot: --out <png> required");
            return ExitCode::from(1);
        }
    };
    let viewport = flag(args, "--viewport").and_then(parse_viewport);

    match render::screenshot(&url, &out, viewport) {
        Ok(()) => {
            println!("{}", out.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("mock-render: {e}");
            ExitCode::from(1)
        }
    }
}
