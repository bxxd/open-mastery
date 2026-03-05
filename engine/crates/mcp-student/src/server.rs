use anyhow::{Context, Result};
use open_mastery_core::graph::Graph;
use open_mastery_core::types::{BloomLevel, StudentState};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Server {
    pub graph: Mutex<Graph>,
    progress_dir: PathBuf,
    students: Mutex<std::collections::HashMap<String, StudentState>>,
}

impl Server {
    pub fn new(graph: Graph, progress_dir: PathBuf) -> Self {
        Self {
            graph: Mutex::new(graph),
            progress_dir,
            students: Mutex::new(std::collections::HashMap::new()),
        }
    }

    fn get_or_load_student(&self, student_id: &str) -> Result<StudentState> {
        let mut students = self.students.lock().unwrap();
        if let Some(state) = students.get(student_id) {
            return Ok(state.clone());
        }

        let path = self.progress_dir.join(format!("{}.json", student_id));
        let state = if path.exists() {
            StudentState::load(&path)?
        } else {
            StudentState::new(student_id)
        };
        students.insert(student_id.to_string(), state.clone());
        Ok(state)
    }

    fn save_student(&self, state: &StudentState) -> Result<()> {
        let path = self
            .progress_dir
            .join(format!("{}.json", state.student_id));
        state.save(&path)?;
        let mut students = self.students.lock().unwrap();
        students.insert(state.student_id.clone(), state.clone());
        Ok(())
    }

    pub fn tools(&self) -> Value {
        json!([
            {
                "name": "get_frontier",
                "description": "Get the list of topics a student can learn next. Returns nodes where all prerequisites are mastered but the node itself is not yet mastered.",
                "inputSchema": {
                    "type": "object",
                    "required": ["student_id"],
                    "properties": {
                        "student_id": {
                            "type": "string",
                            "description": "The student identifier"
                        }
                    }
                }
            },
            {
                "name": "get_node",
                "description": "Get details about a specific topic node including its prerequisites, assessment types, Bloom level, and teaching guidance (prompt cascade).",
                "inputSchema": {
                    "type": "object",
                    "required": ["node_id"],
                    "properties": {
                        "node_id": {
                            "type": "string",
                            "description": "The node ID (e.g., 'frac.con.basics')"
                        }
                    }
                }
            },
            {
                "name": "record_mastery",
                "description": "Record that a student has mastered a topic. Returns the list of newly unlocked topics. Use this after assessing the student and confirming they demonstrate mastery at the 'apply' level or higher.",
                "inputSchema": {
                    "type": "object",
                    "required": ["student_id", "node_id"],
                    "properties": {
                        "student_id": {
                            "type": "string",
                            "description": "The student identifier"
                        },
                        "node_id": {
                            "type": "string",
                            "description": "The node ID being mastered"
                        },
                        "level": {
                            "type": "string",
                            "enum": ["know", "understand", "apply", "analyze"],
                            "description": "Bloom level of mastery demonstrated. Defaults to 'apply'."
                        }
                    }
                }
            },
            {
                "name": "get_progress",
                "description": "Get a student's full mastery state — all topics they've mastered with levels and timestamps.",
                "inputSchema": {
                    "type": "object",
                    "required": ["student_id"],
                    "properties": {
                        "student_id": {
                            "type": "string",
                            "description": "The student identifier"
                        }
                    }
                }
            }
        ])
    }

    pub fn call_tool(&self, name: &str, args: &Value) -> Result<String> {
        let graph = self.graph.lock().unwrap();

        match name {
            "get_frontier" => {
                let student_id = args["student_id"]
                    .as_str()
                    .context("Missing student_id")?;
                let state = self.get_or_load_student(student_id)?;
                let frontier = graph.get_frontier(&state);

                let nodes: Vec<Value> = frontier
                    .iter()
                    .map(|n| {
                        json!({
                            "id": n.id,
                            "name": n.display_name(),
                            "domain": n.domain,
                            "unit": n.unit,
                            "assess": n.assess,
                            "bloom": n.bloom,
                        })
                    })
                    .collect();

                Ok(serde_json::to_string_pretty(&json!({
                    "student_id": student_id,
                    "frontier_count": nodes.len(),
                    "nodes": nodes,
                }))?)
            }

            "get_node" => {
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

            "record_mastery" => {
                let student_id = args["student_id"]
                    .as_str()
                    .context("Missing student_id")?;
                let node_id = args["node_id"].as_str().context("Missing node_id")?;
                let level: BloomLevel = args
                    .get("level")
                    .and_then(|v| v.as_str())
                    .map(|s| serde_json::from_value(json!(s)))
                    .transpose()
                    .context("Invalid bloom level")?
                    .unwrap_or(BloomLevel::Apply);

                let mut state = self.get_or_load_student(student_id)?;
                let unlocked = state.record_mastery(&graph, node_id, level)?;
                self.save_student(&state)?;

                let node_name = graph
                    .get_node(node_id)
                    .map(|n| n.display_name())
                    .unwrap_or_else(|| node_id.to_string());

                let unlocked_details: Vec<Value> = unlocked
                    .iter()
                    .filter_map(|id| {
                        graph.get_node(id).map(|n| {
                            json!({ "id": n.id, "name": n.display_name() })
                        })
                    })
                    .collect();

                Ok(serde_json::to_string_pretty(&json!({
                    "mastered": {
                        "id": node_id,
                        "name": node_name,
                        "level": level,
                    },
                    "newly_unlocked_count": unlocked_details.len(),
                    "newly_unlocked": unlocked_details,
                }))?)
            }

            "get_progress" => {
                let student_id = args["student_id"]
                    .as_str()
                    .context("Missing student_id")?;
                let state = self.get_or_load_student(student_id)?;

                let total_nodes = graph.nodes.len();
                let mastered_count = state.mastery.len();

                let mastered: Vec<Value> = state
                    .mastery
                    .iter()
                    .map(|(id, record)| {
                        let name = graph
                            .get_node(id)
                            .map(|n| n.display_name())
                            .unwrap_or_else(|| id.clone());
                        json!({
                            "id": id,
                            "name": name,
                            "level": record.level,
                            "mastered_at": record.mastered_at.to_rfc3339(),
                        })
                    })
                    .collect();

                Ok(serde_json::to_string_pretty(&json!({
                    "student_id": student_id,
                    "total_nodes": total_nodes,
                    "mastered_count": mastered_count,
                    "mastered": mastered,
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
                        "name": "open-mastery",
                        "version": env!("CARGO_PKG_VERSION")
                    },
                    "instructions": "You are a math tutor powered by the Open Mastery knowledge graph. Use get_frontier to find what to teach next. Use get_node to get the topic details and teaching guidance (the prompt_cascade tells you HOW to teach). Teach, assess with problems matching the assess types, and call record_mastery when the student demonstrates mastery. Be encouraging and adaptive."
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
