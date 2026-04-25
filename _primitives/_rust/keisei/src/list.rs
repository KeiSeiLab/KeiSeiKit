//! `keisei list-adapters` — read-only dump of every registered adapter
//! and its detection state on this host.
//!
//! Constructor Pattern: single responsibility — render a tabular view.
//! No state mutation, no config touches.
//!
//! v0.21: adapters can advertise multiple scopes — we show the user-scope
//! config path by default (the fan-out target for `keisei mount`), plus a
//! trailing `scopes=...` column so an operator can see which adapters can
//! also take a `--scope=project` attach.

use crate::adapter;
use crate::error::Result;
use crate::scope::Scope;

pub fn run() -> Result<()> {
    let rows: Vec<Row> = adapter::all()
        .iter()
        .map(|a| Row {
            name: a.name().to_string(),
            detected: a.detect(),
            config_path: a.config_path(Scope::User).to_string_lossy().into_owned(),
            scopes: a
                .supported_scopes()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("|"),
        })
        .collect();
    print_table(&rows);
    Ok(())
}

struct Row {
    name: String,
    detected: bool,
    config_path: String,
    scopes: String,
}

fn print_table(rows: &[Row]) {
    let name_w = rows.iter().map(|r| r.name.len()).max().unwrap_or(0).max(7);
    println!(
        "{:<w1$}  detected  scopes         config_path (user)",
        "adapter",
        w1 = name_w
    );
    println!(
        "{:<w1$}  --------  -------------  ------------------",
        "-------",
        w1 = name_w
    );
    for r in rows {
        let mark = if r.detected { "yes     " } else { "no      " };
        println!(
            "{:<w1$}  {}  {:<13}  {}",
            r.name,
            mark,
            r.scopes,
            r.config_path,
            w1 = name_w
        );
    }
}
