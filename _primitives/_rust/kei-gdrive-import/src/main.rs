use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use kei_gdrive_import::cli::{Cli, Cmd};
use kei_gdrive_import::{classify, classify_remote, scan, scan_tree};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Cmd::Classify { path, remote } => run_classify(&path, remote),
        Cmd::ScanTree { root, remote } => run_scan(&root, remote),
    };
    match result {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("kei-gdrive-import: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run_classify(path: &str, remote: bool) -> anyhow::Result<String> {
    let c = if remote {
        classify_remote(path)?
    } else {
        classify(&PathBuf::from(path))
    };
    Ok(serde_json::to_string_pretty(&c)?)
}

fn run_scan(root: &str, remote: bool) -> anyhow::Result<String> {
    let entries = if remote {
        scan::scan_remote(root)?
    } else {
        scan_tree(&PathBuf::from(root))?
    };
    Ok(serde_json::to_string_pretty(&entries)?)
}
