use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::sync::Arc;

use crate::server::Server;

pub fn serve(server: Arc<Server>) -> Result<()> {
    eprintln!("Serving over stdio");

    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    for line in reader.lines() {
        let line = line.context("Failed to read from stdin")?;
        if line.trim().is_empty() {
            continue;
        }

        let request: serde_json::Value =
            serde_json::from_str(&line).context("Invalid JSON-RPC request")?;
        let response = server.handle_request(request);
        if !response.is_null() {
            println!("{}", serde_json::to_string(&response)?);
        }
    }

    Ok(())
}
