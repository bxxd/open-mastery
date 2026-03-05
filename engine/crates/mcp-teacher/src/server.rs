use anyhow::{Context, Result};
use open_mastery_core::graph::Graph;
use open_mastery_core::types::{BloomLevel, Node, NodeUpdate};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::{artifacts, git};

pub struct Server {
    pub graph: Mutex<Graph>,
    graph_dir: PathBuf,
    repo_dir: Option<PathBuf>,
}

impl Server {
    pub fn new(graph: Graph, graph_dir: PathBuf, repo_dir: Option<PathBuf>) -> Self {
        Self {
            graph: Mutex::new(graph),
            graph_dir,
            repo_dir,
        }
    }

    pub fn tools(&self) -> Value {
        json!([
            // --- Browse & Traverse ---
            {
                "name": "list_domains",
                "description": "List all mathematical domains in the graph with node counts.",
                "inputSchema": { "type": "object", "properties": {} }
            },
            {
                "name": "list_nodes",
                "description": "List nodes in a domain, optionally filtered by unit, tag, or typical grade.",
                "inputSchema": {
                    "type": "object",
                    "required": ["domain"],
                    "properties": {
                        "domain": { "type": "string", "description": "Domain name (e.g., 'fractions', 'operations')" },
                        "unit": { "type": "string", "description": "Unit name to filter by (e.g., 'concepts', 'equivalence')" },
                        "tag": { "type": "string", "description": "Tag to filter by" },
                        "grade": { "type": "integer", "description": "Typical grade to filter by" }
                    }
                }
            },
            {
                "name": "search_nodes",
                "description": "Search nodes by ID, name, tag, or context substring.",
                "inputSchema": {
                    "type": "object",
                    "required": ["query"],
                    "properties": {
                        "query": { "type": "string", "description": "Search string" }
                    }
                }
            },
            {
                "name": "get_node",
                "description": "Get full details about a node: prerequisites, what it unlocks, depth, prompt cascade, and context.",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id"],
                    "properties": {
                        "node_id": { "type": "string", "description": "The node ID (e.g., 'frac.con.basics')" }
                    }
                }
            },
            {
                "name": "validate_graph",
                "description": "Validate graph structure. Returns health report: node count, edge count, domains, root nodes, leaf nodes, orphans.",
                "inputSchema": { "type": "object", "properties": {} }
            },
            // --- CRUD ---
            {
                "name": "create_node",
                "description": "Create a new node. Writes a YAML file at the path derived from the ID (domain.unit.topic → domain/unit/topic.yaml).",
                "inputSchema": {
                    "type": "object",
                    "required": ["id", "prereqs", "bloom", "assess"],
                    "properties": {
                        "id": { "type": "string", "description": "Three-part ID: domain.unit.topic (e.g., 'frac.con.basics')" },
                        "prereqs": { "type": "array", "items": { "type": "string" }, "description": "Prerequisite node IDs" },
                        "bloom": { "type": "string", "enum": ["know", "understand", "apply", "analyze"] },
                        "assess": { "type": "array", "items": { "type": "string" }, "description": "Assessment types" },
                        "context": { "type": "string", "description": "Pedagogical guidance for the LLM tutor" },
                        "tags": { "type": "array", "items": { "type": "string" } },
                        "typical_grade": { "type": "integer", "description": "Grade level (4-12+)" }
                    }
                }
            },
            {
                "name": "update_node",
                "description": "Update an existing node. Only provided fields are changed. Rewrites the YAML file.",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id"],
                    "properties": {
                        "node_id": { "type": "string" },
                        "prereqs": { "type": "array", "items": { "type": "string" } },
                        "bloom": { "type": "string", "enum": ["know", "understand", "apply", "analyze"] },
                        "assess": { "type": "array", "items": { "type": "string" } },
                        "context": { "type": "string", "description": "Set to null to clear" },
                        "tags": { "type": "array", "items": { "type": "string" } },
                        "typical_grade": { "type": "integer" }
                    }
                }
            },
            {
                "name": "delete_node",
                "description": "Delete a node and its YAML file. Fails if other nodes depend on it.",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id"],
                    "properties": {
                        "node_id": { "type": "string" }
                    }
                }
            },
            {
                "name": "add_prerequisite",
                "description": "Add a prerequisite edge. Validates no cycle would be created.",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id", "prerequisite_id"],
                    "properties": {
                        "node_id": { "type": "string", "description": "The node to add a prerequisite to" },
                        "prerequisite_id": { "type": "string", "description": "The prerequisite node ID" }
                    }
                }
            },
            {
                "name": "remove_prerequisite",
                "description": "Remove a prerequisite edge.",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id", "prerequisite_id"],
                    "properties": {
                        "node_id": { "type": "string" },
                        "prerequisite_id": { "type": "string" }
                    }
                }
            },
            // --- Prompts ---
            {
                "name": "get_prompt_cascade",
                "description": "Get the full concatenated system prompt for a node's path (subject → domain → unit → node context).",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id"],
                    "properties": {
                        "node_id": { "type": "string" }
                    }
                }
            },
            {
                "name": "set_prompt",
                "description": "Write or update a _prompt.yaml file at a given path.",
                "inputSchema": {
                    "type": "object",
                    "required": ["path", "system"],
                    "properties": {
                        "path": { "type": "string", "description": "Relative path (e.g., 'fractions' or 'fractions/equivalence' or '' for root)" },
                        "system": { "type": "string", "description": "The system prompt content" }
                    }
                }
            },
            // --- Git ---
            {
                "name": "git_status",
                "description": "Show uncommitted changes to graph files.",
                "inputSchema": { "type": "object", "properties": {} }
            },
            {
                "name": "git_commit",
                "description": "Stage and commit graph changes.",
                "inputSchema": {
                    "type": "object",
                    "required": ["message"],
                    "properties": {
                        "message": { "type": "string", "description": "Commit message" }
                    }
                }
            },
            {
                "name": "git_log",
                "description": "Show recent commit history.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "count": { "type": "integer", "description": "Number of commits (default 10)" }
                    }
                }
            },
            // --- Artifacts ---
            {
                "name": "save_artifact",
                "description": "Store a file (image, diagram) associated with a node or domain.",
                "inputSchema": {
                    "type": "object",
                    "required": ["scope", "filename", "data"],
                    "properties": {
                        "scope": { "type": "string", "description": "Node ID or domain name" },
                        "filename": { "type": "string" },
                        "data": { "type": "string", "description": "Base64-encoded file content" }
                    }
                }
            },
            {
                "name": "get_artifact",
                "description": "Retrieve a stored artifact as base64.",
                "inputSchema": {
                    "type": "object",
                    "required": ["scope", "filename"],
                    "properties": {
                        "scope": { "type": "string" },
                        "filename": { "type": "string" }
                    }
                }
            },
            {
                "name": "list_artifacts",
                "description": "List stored artifacts for a scope.",
                "inputSchema": {
                    "type": "object",
                    "required": ["scope"],
                    "properties": {
                        "scope": { "type": "string" }
                    }
                }
            }
        ])
    }

    pub fn call_tool(&self, name: &str, args: &Value) -> Result<String> {
        match name {
            // --- Browse & Traverse ---
            "list_domains" => {
                let graph = self.graph.lock().unwrap();
                let domains = graph.list_domains();
                let items: Vec<Value> = domains
                    .iter()
                    .map(|(name, count)| json!({ "domain": name, "node_count": count }))
                    .collect();
                Ok(serde_json::to_string_pretty(&json!({
                    "total_nodes": graph.nodes.len(),
                    "domains": items,
                }))?)
            }

            "list_nodes" => {
                let graph = self.graph.lock().unwrap();
                let domain = args["domain"].as_str().context("Missing domain")?;
                let unit = args.get("unit").and_then(|v| v.as_str());
                let tag = args.get("tag").and_then(|v| v.as_str());
                let grade = args.get("grade").and_then(|v| v.as_u64()).map(|g| g as u8);

                let mut nodes = graph.list_nodes(domain, unit, tag);
                if let Some(g) = grade {
                    nodes.retain(|n| n.typical_grade == Some(g));
                }

                let items: Vec<Value> = nodes
                    .iter()
                    .map(|n| {
                        json!({
                            "id": n.id,
                            "name": n.display_name(),
                            "unit": n.unit,
                            "bloom": n.bloom,
                            "assess": n.assess,
                            "typical_grade": n.typical_grade,
                            "prereq_count": n.prereqs.len(),
                        })
                    })
                    .collect();

                Ok(serde_json::to_string_pretty(&json!({
                    "domain": domain,
                    "count": items.len(),
                    "nodes": items,
                }))?)
            }

            "search_nodes" => {
                let graph = self.graph.lock().unwrap();
                let query = args["query"].as_str().context("Missing query")?;
                let results = graph.search_nodes(query);

                let items: Vec<Value> = results
                    .iter()
                    .map(|n| {
                        json!({
                            "id": n.id,
                            "name": n.display_name(),
                            "domain": n.domain,
                            "unit": n.unit,
                            "bloom": n.bloom,
                        })
                    })
                    .collect();

                Ok(serde_json::to_string_pretty(&json!({
                    "query": query,
                    "count": items.len(),
                    "results": items,
                }))?)
            }

            "get_node" => {
                let graph = self.graph.lock().unwrap();
                let node_id = args["node_id"].as_str().context("Missing node_id")?;
                let node = graph
                    .get_node(node_id)
                    .context(format!("Unknown node: {}", node_id))?;

                let prereq_details: Vec<Value> = node
                    .prereqs
                    .iter()
                    .filter_map(|id| {
                        graph.get_node(id).map(|n| {
                            json!({ "id": n.id, "name": n.display_name() })
                        })
                    })
                    .collect();

                let children: Vec<Value> = graph
                    .children
                    .get(node_id)
                    .map(|ids| {
                        ids.iter()
                            .filter_map(|id| {
                                graph.get_node(id).map(|n| {
                                    json!({ "id": n.id, "name": n.display_name() })
                                })
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let prompt_cascade = graph.get_prompt_cascade(node_id);

                Ok(serde_json::to_string_pretty(&json!({
                    "id": node.id,
                    "name": node.display_name(),
                    "domain": node.domain,
                    "unit": node.unit,
                    "prerequisites": prereq_details,
                    "unlocks": children,
                    "assess": node.assess,
                    "bloom": node.bloom,
                    "tags": node.tags,
                    "typical_grade": node.typical_grade,
                    "context": node.context,
                    "prompt_cascade": prompt_cascade,
                    "prereq_depth": graph.prereq_depth(node_id),
                }))?)
            }

            "validate_graph" => {
                let graph = self.graph.lock().unwrap();
                let report = graph.validate();
                let domains: Vec<Value> = report
                    .domains
                    .iter()
                    .map(|(name, count)| json!({ "domain": name, "node_count": count }))
                    .collect();

                Ok(serde_json::to_string_pretty(&json!({
                    "node_count": report.node_count,
                    "edge_count": report.edge_count,
                    "domains": domains,
                    "root_nodes": report.root_nodes,
                    "leaf_nodes": report.leaf_nodes,
                    "orphan_nodes": report.orphan_nodes,
                }))?)
            }

            // --- CRUD ---
            "create_node" => {
                let id = args["id"].as_str().context("Missing id")?;
                let prereqs: Vec<String> = args["prereqs"]
                    .as_array()
                    .context("Missing prereqs")?
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                let bloom: BloomLevel =
                    serde_json::from_value(args["bloom"].clone()).context("Invalid bloom")?;
                let assess: Vec<String> = args["assess"]
                    .as_array()
                    .context("Missing assess")?
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                let context = args.get("context").and_then(|v| v.as_str().map(String::from));
                let tags: Vec<String> = args
                    .get("tags")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();
                let typical_grade = args
                    .get("typical_grade")
                    .and_then(|v| v.as_u64())
                    .map(|g| g as u8);

                // Derive path from ID: domain.unit.topic → domain/unit/topic.yaml
                let (domain, unit, file_path) = id_to_path(id, &self.graph_dir)?;

                let node = Node {
                    id: id.to_string(),
                    prereqs,
                    bloom,
                    assess,
                    context,
                    tags,
                    typical_grade,
                    domain,
                    unit,
                    file_path,
                };

                let mut graph = self.graph.lock().unwrap();
                graph.create_node(node)?;

                Ok(serde_json::to_string_pretty(&json!({
                    "created": id,
                    "message": format!("Node {} created", id),
                }))?)
            }

            "update_node" => {
                let node_id = args["node_id"].as_str().context("Missing node_id")?;

                let prereqs = args.get("prereqs").and_then(|v| {
                    v.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                });
                let bloom = args
                    .get("bloom")
                    .and_then(|v| serde_json::from_value(v.clone()).ok());
                let assess = args.get("assess").and_then(|v| {
                    v.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                });
                let context = if args.get("context").map_or(false, |v| v.is_null()) {
                    Some(None) // Explicitly clear
                } else {
                    args.get("context")
                        .and_then(|v| v.as_str())
                        .map(|s| Some(s.to_string()))
                };
                let tags = args.get("tags").and_then(|v| {
                    v.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                });
                let typical_grade = if args.get("typical_grade").map_or(false, |v| v.is_null()) {
                    Some(None)
                } else {
                    args.get("typical_grade")
                        .and_then(|v| v.as_u64())
                        .map(|g| Some(g as u8))
                };

                let update = NodeUpdate {
                    prereqs,
                    bloom,
                    assess,
                    context,
                    tags,
                    typical_grade,
                };

                let mut graph = self.graph.lock().unwrap();
                let node = graph.update_node(node_id, update)?;
                let name = node.display_name();

                Ok(serde_json::to_string_pretty(&json!({
                    "updated": node_id,
                    "name": name,
                    "message": format!("Node {} updated", node_id),
                }))?)
            }

            "delete_node" => {
                let node_id = args["node_id"].as_str().context("Missing node_id")?;
                let mut graph = self.graph.lock().unwrap();
                let node = graph.delete_node(node_id)?;

                Ok(serde_json::to_string_pretty(&json!({
                    "deleted": node_id,
                    "name": node.display_name(),
                    "message": format!("Node {} deleted", node_id),
                }))?)
            }

            "add_prerequisite" => {
                let node_id = args["node_id"].as_str().context("Missing node_id")?;
                let prereq_id = args["prerequisite_id"]
                    .as_str()
                    .context("Missing prerequisite_id")?;
                let mut graph = self.graph.lock().unwrap();
                graph.add_prerequisite(node_id, prereq_id)?;

                Ok(serde_json::to_string_pretty(&json!({
                    "node_id": node_id,
                    "added_prerequisite": prereq_id,
                    "message": format!("Added prerequisite {} → {}", prereq_id, node_id),
                }))?)
            }

            "remove_prerequisite" => {
                let node_id = args["node_id"].as_str().context("Missing node_id")?;
                let prereq_id = args["prerequisite_id"]
                    .as_str()
                    .context("Missing prerequisite_id")?;
                let mut graph = self.graph.lock().unwrap();
                graph.remove_prerequisite(node_id, prereq_id)?;

                Ok(serde_json::to_string_pretty(&json!({
                    "node_id": node_id,
                    "removed_prerequisite": prereq_id,
                    "message": format!("Removed prerequisite {} → {}", prereq_id, node_id),
                }))?)
            }

            // --- Prompts ---
            "get_prompt_cascade" => {
                let graph = self.graph.lock().unwrap();
                let node_id = args["node_id"].as_str().context("Missing node_id")?;
                let cascade = graph
                    .get_prompt_cascade(node_id)
                    .context(format!("No prompt cascade for: {}", node_id))?;

                Ok(serde_json::to_string_pretty(&json!({
                    "node_id": node_id,
                    "prompt_cascade": cascade,
                }))?)
            }

            "set_prompt" => {
                let path = args["path"].as_str().context("Missing path")?;
                let system = args["system"].as_str().context("Missing system")?;

                let dir = if path.is_empty() {
                    self.graph_dir.clone()
                } else {
                    self.graph_dir.join(path)
                };
                std::fs::create_dir_all(&dir)?;

                let prompt_path = dir.join("_prompt.yaml");
                let content = format!("system: >\n  {}\n", system.replace('\n', "\n  "));
                std::fs::write(&prompt_path, &content)?;

                // Update in-memory prompts
                let rel_path = dir
                    .strip_prefix(&self.graph_dir)
                    .unwrap_or(std::path::Path::new(""))
                    .to_path_buf();
                self.graph.lock().unwrap().prompts.insert(rel_path.clone(), system.to_string());

                Ok(serde_json::to_string_pretty(&json!({
                    "path": prompt_path.display().to_string(),
                    "message": format!("Prompt updated at {}", path),
                }))?)
            }

            // --- Git ---
            "git_status" => {
                let repo_dir = self.repo_dir.as_ref().context("REPO_DIR not configured")?;
                let output = git::status(repo_dir)?;
                Ok(serde_json::to_string_pretty(&json!({
                    "status": output,
                }))?)
            }

            "git_commit" => {
                let repo_dir = self.repo_dir.as_ref().context("REPO_DIR not configured")?;
                let message = args["message"].as_str().context("Missing message")?;
                let output = git::commit(repo_dir, message)?;
                Ok(serde_json::to_string_pretty(&json!({
                    "output": output,
                }))?)
            }

            "git_log" => {
                let repo_dir = self.repo_dir.as_ref().context("REPO_DIR not configured")?;
                let count = args
                    .get("count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(10) as usize;
                let output = git::log(repo_dir, count)?;
                Ok(serde_json::to_string_pretty(&json!({
                    "log": output,
                }))?)
            }

            // --- Artifacts ---
            "save_artifact" => {
                let scope = args["scope"].as_str().context("Missing scope")?;
                let filename = args["filename"].as_str().context("Missing filename")?;
                let data = args["data"].as_str().context("Missing data")?;
                let path = artifacts::save_artifact(&self.graph_dir, scope, filename, data)?;
                Ok(serde_json::to_string_pretty(&json!({
                    "saved": path.display().to_string(),
                    "message": format!("Artifact saved: {}/{}", scope, filename),
                }))?)
            }

            "get_artifact" => {
                let scope = args["scope"].as_str().context("Missing scope")?;
                let filename = args["filename"].as_str().context("Missing filename")?;
                let data = artifacts::get_artifact(&self.graph_dir, scope, filename)?;
                Ok(serde_json::to_string_pretty(&json!({
                    "scope": scope,
                    "filename": filename,
                    "data": data,
                }))?)
            }

            "list_artifacts" => {
                let scope = args["scope"].as_str().context("Missing scope")?;
                let files = artifacts::list_artifacts(&self.graph_dir, scope)?;
                Ok(serde_json::to_string_pretty(&json!({
                    "scope": scope,
                    "files": files,
                }))?)
            }

            _ => anyhow::bail!("Unknown tool: {}", name),
        }
    }

    /// Handle a JSON-RPC request, return a JSON-RPC response (or null for notifications).
    pub fn handle_request(&self, request: Value) -> Value {
        let method = request["method"].as_str().unwrap_or("");
        let id = request["id"].clone();

        match method {
            "initialize" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": {
                        "name": "open-mastery-teacher",
                        "version": env!("CARGO_PKG_VERSION")
                    },
                    "instructions": "You are a curriculum authoring assistant for the Open Mastery math knowledge graph. Help teachers browse, create, edit, and validate the graph. The graph is organized by mathematical domain (number-sense, operations, fractions, etc.), not grade level. Node IDs are three-part keys: domain.unit.topic. Use list_domains and list_nodes to explore. Use create_node, update_node, delete_node, add_prerequisite, remove_prerequisite to modify. Use validate_graph to check structure. Use git tools to version changes. Every change writes YAML files to disk."
                }
            }),

            "notifications/initialized" => json!(null),

            "tools/list" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "tools": self.tools() }
            }),

            "tools/call" => {
                let tool_name = request["params"]["name"].as_str().unwrap_or("");
                let arguments = request["params"]
                    .get("arguments")
                    .cloned()
                    .unwrap_or(json!({}));

                match self.call_tool(tool_name, &arguments) {
                    Ok(output) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{ "type": "text", "text": output }]
                        }
                    }),
                    Err(e) => json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{ "type": "text", "text": format!("Error: {}", e) }],
                            "isError": true
                        }
                    }),
                }
            }

            _ => {
                if id.is_null() {
                    json!(null)
                } else {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32601,
                            "message": format!("Method not found: {}", method)
                        }
                    })
                }
            }
        }
    }
}

/// Convert a three-part node ID to a filesystem path.
/// "frac.con.basics" → ("fractions", "concepts", graph_dir/fractions/concepts/basics.yaml)
///
/// Uses the domain prefix mapping from GRAPH_FORMAT.md.
fn id_to_path(id: &str, graph_dir: &std::path::Path) -> Result<(String, String, PathBuf)> {
    let parts: Vec<&str> = id.splitn(3, '.').collect();
    anyhow::ensure!(parts.len() == 3, "Node ID must have 3 parts: domain.unit.topic (got: {})", id);

    let domain_prefix = parts[0];
    let unit_abbr = parts[1];
    let topic = parts[2];

    // Map domain prefix to directory name
    let domain_dir = match domain_prefix {
        "ns" => "number-sense",
        "ops" => "operations",
        "frac" => "fractions",
        "dec" => "decimals",
        "geo" => "geometry",
        "rat" => "ratios-proportions",
        "alg" => "algebra",
        "trig" => "trigonometry",
        "calc" => "calculus",
        "la" => "linear-algebra",
        "stat" => "statistics",
        "dm" => "discrete-math",
        other => anyhow::bail!("Unknown domain prefix: {}", other),
    };

    // Unit abbreviation is used as-is for the directory name.
    // The actual directory might use different names, but for new nodes
    // we create directories matching the abbreviation.
    let file_path = graph_dir
        .join(domain_dir)
        .join(unit_abbr)
        .join(format!("{}.yaml", topic));

    Ok((domain_dir.to_string(), unit_abbr.to_string(), file_path))
}
