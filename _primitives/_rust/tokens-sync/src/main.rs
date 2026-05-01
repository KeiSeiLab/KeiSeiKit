//! tokens-sync — emit Tailwind config + CSS custom properties from a single
//! design-tokens JSON file. One SSoT; no drift between CSS/JS sides.
//!
//! USAGE
//!   tokens-sync <tokens.json> --out-tailwind <path> --out-css <path>
//!
//! Input JSON shape (minimum):
//!   {
//!     "colors":   { "primary": "oklch(0.6 0.2 250)", ... },
//!     "fonts":    { "display": "Fraunces Variable, serif", ... },
//!     "spacing":  { "sm": "0.5rem", ... },
//!     "radius":   { "card": "0.75rem", ... }
//!   }
//!
//! At least one of --out-tailwind or --out-css must be supplied.

mod emit;
mod parse;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return ExitCode::SUCCESS;
    }

    let Some(input) = args.iter().find(|a| !a.starts_with("--")).cloned() else {
        eprintln!("tokens-sync: <tokens.json> required");
        print_help();
        return ExitCode::from(1);
    };
    let tailwind = flag(&args, "--out-tailwind").map(PathBuf::from);
    let css = flag(&args, "--out-css").map(PathBuf::from);

    if tailwind.is_none() && css.is_none() {
        eprintln!("tokens-sync: need at least one of --out-tailwind or --out-css");
        return ExitCode::from(1);
    }

    let tokens = match parse::load(&PathBuf::from(&input)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("tokens-sync: {e}");
            return ExitCode::from(1);
        }
    };

    if let Some(path) = tailwind {
        if let Err(e) = emit::tailwind_config(&tokens, &path) {
            eprintln!("tokens-sync: emit tailwind: {e}");
            return ExitCode::from(1);
        }
        println!("tailwind -> {}", path.display());
    }
    if let Some(path) = css {
        if let Err(e) = emit::css_vars(&tokens, &path) {
            eprintln!("tokens-sync: emit css: {e}");
            return ExitCode::from(1);
        }
        println!("css -> {}", path.display());
    }

    ExitCode::SUCCESS
}

fn print_help() {
    println!(
        "tokens-sync — design tokens JSON → Tailwind config + CSS vars

USAGE
  tokens-sync <tokens.json> --out-tailwind <path> --out-css <path>

Write at least one of --out-tailwind or --out-css (both allowed).
JSON schema: see source for minimum shape."
    );
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
