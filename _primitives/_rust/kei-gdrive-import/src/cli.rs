//! clap derive structs for the two-verb CLI.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "kei-gdrive-import",
    about = "Classify Google-Drive folders as PROJECT / AMBIGUOUS / NOT-A-PROJECT / ALREADY-REPO",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Classify a single folder. Emits one JSON object on stdout.
    /// With --remote, queries rclone lsf for marker filenames (no download).
    Classify {
        /// Folder path. Local FS by default; rclone remote spec with --remote
        /// (e.g. `drive:Projects/MyApp`).
        path: String,
        /// Treat path as an rclone remote spec.
        #[arg(long)]
        remote: bool,
    },
    /// Walk one level under <root>; emit a JSON array of classifications.
    ScanTree {
        /// Root folder. With --remote, treated as an rclone remote spec
        /// (e.g. `drive:Projects/`).
        root: String,
        /// Use `rclone lsjson` instead of local FS.
        #[arg(long)]
        remote: bool,
    },
}
