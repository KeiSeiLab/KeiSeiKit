//! kei-forge binary entry point.
//!
//! Binds axum to `127.0.0.1:8747` and serves the atom-scaffolding wizard.
//! Port 8747 chosen for mnemonic `"TK4S"` (KT + 4 streams) and low conflict
//! probability — not registered with IANA, outside common dev-tool ranges.

use kei_forge::server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr: SocketAddr = "127.0.0.1:8747".parse()?;
    let app = server::app();
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("keisei forge ready — open http://localhost:8747/");
    tracing::info!(%addr, "kei-forge listening");

    axum::serve(listener, app).await?;
    Ok(())
}
