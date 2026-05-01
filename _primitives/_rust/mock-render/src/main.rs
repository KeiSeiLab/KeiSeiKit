//! mock-render — enforces the WYSIWYD invariant (What You See Is What's Deployed)
//! for block-based site-builder. Every section = one source file; screenshot is
//! a render of that file; lock freezes the hash; verify fails if source mutated.
//!
//! USAGE
//!   mock-render screenshot <url> --out <png> [--viewport WxH]
//!   mock-render lock    --project <dir> --section <src> [--screenshot <png>]
//!   mock-render verify  --project <dir> --section <src>
//!   mock-render status  --project <dir>

mod cli_args;
mod cmd_lock;
mod cmd_screenshot;
mod cmd_verify;
mod hash;
mod render;
mod state;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("screenshot") => cmd_screenshot::run(&args[1..]),
        Some("lock") => cmd_lock::run(&args[1..]),
        Some("verify") => cmd_verify::run_verify(&args[1..]),
        Some("status") => cmd_verify::run_status(&args[1..]),
        Some("--help") | Some("-h") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(cmd) => {
            eprintln!("mock-render: unknown command '{cmd}'. Run with --help.");
            ExitCode::from(1)
        }
    }
}

fn print_help() {
    println!(
        "mock-render — WYSIWYD invariant enforcer for site-builder

USAGE
  mock-render screenshot <url> --out <png> [--viewport WxH]
  mock-render lock       --project <dir> --section <src> [--screenshot <png>]
  mock-render verify     --project <dir> --section <src>
  mock-render status     --project <dir>

EXIT
  0  ok
  1  usage / missing args
  2  WYSIWYD invariant violated (file drift / hash mismatch)"
    );
}
