#![deny(warnings)]

//! Open Mastery Teacher MCP Server
//!
//! Full curriculum authoring toolkit: browse, create, edit, validate, git, artifacts.
//!
//! Usage:
//!   open-mastery-teacher                      # HTTP/SSE on port 3002
//!   open-mastery-teacher --port 8080          # HTTP/SSE on custom port
//!   open-mastery-teacher --stdio              # stdio transport
//!
//! Environment:
//!   GRAPH_DIR=../../graph/math                # path to graph YAML files
//!   REPO_DIR=../../                           # path to git repo root (for git tools)

mod artifacts;
mod git;
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
        .unwrap_or(3002);

    let graph_dir = std::env::var("GRAPH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../../graph/math"));

    let repo_dir = std::env::var("REPO_DIR").map(PathBuf::from).ok();

    eprintln!("Open Mastery Teacher MCP");
    eprintln!("  Graph dir: {}", graph_dir.display());
    if let Some(ref rd) = repo_dir {
        eprintln!("  Repo dir: {}", rd.display());
    }

    let graph = open_mastery_core::graph::Graph::load(&graph_dir)
        .context("Failed to load graph")?;

    eprintln!("  Loaded {} nodes", graph.nodes.len());

    let server = Arc::new(server::Server::new(graph, graph_dir, repo_dir));

    if use_stdio {
        stdio::serve(server)
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "open_mastery_teacher=info".into()),
            )
            .with_writer(std::io::stderr)
            .init();

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(sse::serve(server, port))
    }
}
