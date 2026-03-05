#![deny(warnings)]

//! Open Mastery Student MCP Server
//!
//! Lean tutoring server: frontier, node details + prompt cascade, mastery tracking.
//!
//! Usage:
//!   open-mastery-student                      # HTTP/SSE on port 3001
//!   open-mastery-student --port 8080          # HTTP/SSE on custom port
//!   open-mastery-student --stdio              # stdio transport
//!
//! Environment:
//!   GRAPH_DIR=../../graph/math                # path to graph YAML files
//!   PROGRESS_DIR=../../progress               # path to student state files

mod server;
mod sse;
mod stdio;

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let use_stdio = args.contains(&"--stdio".to_string());

    let port: u16 = args
        .windows(2)
        .find(|w| w[0] == "--port")
        .and_then(|w| w[1].parse().ok())
        .unwrap_or(3001);

    let graph_dir = std::env::var("GRAPH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../../graph/math"));

    let progress_dir = std::env::var("PROGRESS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../../progress"));

    std::fs::create_dir_all(&progress_dir)
        .context("Failed to create progress directory")?;

    eprintln!("Open Mastery Student MCP");
    eprintln!("  Graph dir: {}", graph_dir.display());
    eprintln!("  Progress dir: {}", progress_dir.display());

    let graph = open_mastery_core::graph::Graph::load(&graph_dir)
        .context("Failed to load graph")?;

    eprintln!("  Loaded {} nodes", graph.nodes.len());

    let server = Arc::new(server::Server::new(graph, progress_dir));

    if use_stdio {
        stdio::serve(server)
    } else {
        // Initialize tracing for HTTP mode
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "open_mastery_mcp=info".into()),
            )
            .with_writer(std::io::stderr)
            .init();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(sse::serve(server, port))
    }
}
