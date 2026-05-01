//! visual-diff — pixel-level PNG comparator for WYSIWYD drift detection.
//!
//! USAGE
//!   visual-diff <a.png> <b.png> [--out diff.png] [--threshold 5]
//!
//! Exit codes:
//!   0  images equal (within threshold)
//!   1  usage error
//!   2  images differ beyond threshold
//!
//! Prints percentage of mismatched pixels to stdout. Writes a red-overlay
//! diff PNG to <out> (default: ./diff.png) when images differ.

mod diff;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return ExitCode::SUCCESS;
    }

    let positional: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if positional.len() < 2 {
        eprintln!("visual-diff: need <a.png> <b.png>");
        print_help();
        return ExitCode::from(1);
    }

    let a = PathBuf::from(positional[0]);
    let b = PathBuf::from(positional[1]);
    let out = flag(&args, "--out")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("diff.png"));
    let threshold: f64 = flag(&args, "--threshold")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);

    match diff::compare(&a, &b, &out) {
        Ok(report) => {
            println!("{:.4}% differ ({} px of {})", report.pct, report.diff_px, report.total_px);
            if report.diff_png_written {
                eprintln!("wrote diff: {}", out.display());
            }
            if report.pct > threshold {
                ExitCode::from(2)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("visual-diff: {e}");
            ExitCode::from(1)
        }
    }
}

fn print_help() {
    println!(
        "visual-diff — pixel-level PNG comparator

USAGE
  visual-diff <a.png> <b.png> [--out diff.png] [--threshold 5]

OPTIONS
  --out FILE        write red-overlay diff PNG (default: diff.png)
  --threshold PCT   fail (exit 2) if mismatch exceeds PCT%% (default: 1.0)

EXIT
  0  equal (within threshold)
  1  usage / IO error
  2  differ beyond threshold"
    );
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
