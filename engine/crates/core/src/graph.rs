use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::types::{Node, NodeFile, NodeUpdate, PromptFile, StudentState};

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub topo_order: Vec<String>,
    pub children: HashMap<String, Vec<String>>,
    /// Reverse index: node_id → vec of (encompassing_node_id, weight).
    pub encompassed_by: HashMap<String, Vec<(String, f32)>>,
    /// Prompt files keyed by their directory path relative to graph root.
    pub prompts: HashMap<PathBuf, String>,
    /// The root directory the graph was loaded from.
    pub graph_dir: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml error in {file}: {source}")]
    Yaml {
        file: String,
        source: serde_yaml::Error,
    },
    #[error("json error in {file}: {source}")]
    Json {
        file: String,
        source: serde_json::Error,
    },
    #[error("duplicate node id: {0}")]
    DuplicateNode(String),
    #[error("unknown prerequisite: node {node} requires {prereq}")]
    UnknownPrerequisite { node: String, prereq: String },
    #[error("cycle detected in graph")]
    CycleDetected,
    #[error("node not found: {0}")]
    NodeNotFound(String),
    #[error("node already exists: {0}")]
    NodeAlreadyExists(String),
    #[error("node has dependents: {node} is required by {dependents:?}")]
    HasDependents {
        node: String,
        dependents: Vec<String>,
    },
    #[error("prerequisite not found: {0}")]
    PrerequisiteNotFound(String),
}

pub struct ValidationReport {
    pub node_count: usize,
    pub edge_count: usize,
    pub domains: Vec<(String, usize)>,
    pub root_nodes: Vec<String>,
    pub leaf_nodes: Vec<String>,
    pub orphan_nodes: Vec<String>,
    pub encompasses_count: usize,
}

impl Graph {
    /// Load all YAML node files from a directory tree.
    pub fn load(dir: &Path) -> Result<Self, GraphError> {
        let mut nodes = HashMap::new();
        let mut prompts = HashMap::new();

        load_dir_recursive(dir, dir, &mut nodes, &mut prompts)?;

        let mut graph = Graph {
            nodes,
            topo_order: Vec::new(),
            children: HashMap::new(),
            encompassed_by: HashMap::new(),
            prompts,
            graph_dir: dir.to_path_buf(),
        };
        graph.rebuild_indices()?;
        Ok(graph)
    }

    /// Recompute topo_order and children from current nodes.
    fn rebuild_indices(&mut self) -> Result<(), GraphError> {
        // Validate prerequisites exist
        for node in self.nodes.values() {
            for prereq in &node.prereqs {
                if !self.nodes.contains_key(prereq) {
                    return Err(GraphError::UnknownPrerequisite {
                        node: node.id.clone(),
                        prereq: prereq.clone(),
                    });
                }
            }
        }

        // Build children index
        self.children.clear();
        for node in self.nodes.values() {
            for prereq in &node.prereqs {
                self.children
                    .entry(prereq.clone())
                    .or_default()
                    .push(node.id.clone());
            }
        }

        // Build encompassed_by reverse index and validate encompasses references
        self.encompassed_by.clear();
        for node in self.nodes.values() {
            for (target_id, weight) in &node.encompasses {
                if !self.nodes.contains_key(target_id) {
                    return Err(GraphError::UnknownPrerequisite {
                        node: node.id.clone(),
                        prereq: target_id.clone(),
                    });
                }
                self.encompassed_by
                    .entry(target_id.clone())
                    .or_default()
                    .push((node.id.clone(), *weight));
            }
        }

        // Topological sort
        self.topo_order = topo_sort(&self.nodes)?;
        Ok(())
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Returns nodes where all prerequisites are mastered and the node itself is not yet mastered.
    pub fn get_frontier(&self, state: &StudentState) -> Vec<&Node> {
        self.topo_order
            .iter()
            .filter_map(|id| {
                let node = &self.nodes[id];
                if state.mastery.contains_key(id) {
                    return None;
                }
                let prereqs_met = node
                    .prereqs
                    .iter()
                    .all(|prereq| state.mastery.contains_key(prereq));
                if prereqs_met {
                    Some(node)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get the assembled prompt cascade for a node (subject → domain → unit → node context).
    pub fn get_prompt_cascade(&self, node_id: &str) -> Option<String> {
        let node = self.nodes.get(node_id)?;
        let rel_path = node
            .file_path
            .strip_prefix(&self.graph_dir)
            .unwrap_or(&node.file_path);

        let mut parts = Vec::new();

        // Walk from root to node's directory, collecting _prompt.yaml content
        let mut current = PathBuf::new();
        for component in rel_path.parent()?.components() {
            current = current.join(component);
            if let Some(prompt) = self.prompts.get(&current) {
                parts.push(prompt.as_str());
            }
        }

        // Add node context if present
        if let Some(context) = &node.context {
            parts.push(context.as_str());
        }

        if parts.is_empty() {
            None
        } else {
            Some(parts.join("\n"))
        }
    }

    // --- Query helpers ---

    /// List all unique domains with node counts.
    pub fn list_domains(&self) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for node in self.nodes.values() {
            *counts.entry(node.domain.clone()).or_default() += 1;
        }
        let mut result: Vec<_> = counts.into_iter().collect();
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }

    /// List nodes in a domain, optionally filtered by unit and/or tag.
    pub fn list_nodes(
        &self,
        domain: &str,
        unit: Option<&str>,
        tag: Option<&str>,
    ) -> Vec<&Node> {
        self.topo_order
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .filter(|n| n.domain == domain)
            .filter(|n| unit.map_or(true, |u| n.unit == u))
            .filter(|n| tag.map_or(true, |t| n.tags.iter().any(|nt| nt == t)))
            .collect()
    }

    /// Search nodes by id/tag/context substring.
    pub fn search_nodes(&self, query: &str) -> Vec<&Node> {
        let q = query.to_lowercase();
        self.nodes
            .values()
            .filter(|n| {
                n.id.to_lowercase().contains(&q)
                    || n.display_name().to_lowercase().contains(&q)
                    || n.tags.iter().any(|t| t.to_lowercase().contains(&q))
                    || n.context
                        .as_ref()
                        .map_or(false, |c| c.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// Longest path from a node to a root (no prereqs).
    pub fn prereq_depth(&self, node_id: &str) -> usize {
        let mut max_depth = 0;
        let mut stack = vec![(node_id, 0usize)];
        while let Some((id, depth)) = stack.pop() {
            if let Some(node) = self.nodes.get(id) {
                if node.prereqs.is_empty() {
                    max_depth = max_depth.max(depth);
                } else {
                    for prereq in &node.prereqs {
                        stack.push((prereq, depth + 1));
                    }
                }
            }
        }
        max_depth
    }

    /// Validate graph structure, returning a report.
    pub fn validate(&self) -> ValidationReport {
        let node_count = self.nodes.len();
        let edge_count: usize = self.nodes.values().map(|n| n.prereqs.len()).sum();

        let domains = self.list_domains();

        let root_nodes: Vec<String> = self
            .nodes
            .values()
            .filter(|n| n.prereqs.is_empty())
            .map(|n| n.id.clone())
            .collect();

        let leaf_nodes: Vec<String> = self
            .nodes
            .values()
            .filter(|n| !self.children.contains_key(&n.id))
            .map(|n| n.id.clone())
            .collect();

        let orphan_nodes: Vec<String> = self
            .nodes
            .values()
            .filter(|n| n.prereqs.is_empty() && !self.children.contains_key(&n.id))
            .map(|n| n.id.clone())
            .collect();

        let encompasses_count: usize = self.nodes.values().map(|n| n.encompasses.len()).sum();

        ValidationReport {
            node_count,
            edge_count,
            domains,
            root_nodes,
            leaf_nodes,
            orphan_nodes,
            encompasses_count,
        }
    }

    // --- Mutation methods ---

    pub fn create_node(&mut self, node: Node) -> Result<(), GraphError> {
        if self.nodes.contains_key(&node.id) {
            return Err(GraphError::NodeAlreadyExists(node.id));
        }
        for prereq in &node.prereqs {
            if !self.nodes.contains_key(prereq) {
                return Err(GraphError::PrerequisiteNotFound(prereq.clone()));
            }
        }
        self.save_node_yaml(&node)?;
        self.nodes.insert(node.id.clone(), node);
        self.rebuild_indices()
    }

    pub fn update_node(&mut self, id: &str, update: NodeUpdate) -> Result<&Node, GraphError> {
        let node = self
            .nodes
            .get_mut(id)
            .ok_or_else(|| GraphError::NodeNotFound(id.to_string()))?;

        if let Some(bloom) = update.bloom {
            node.bloom = bloom;
        }
        if let Some(assess) = update.assess {
            node.assess = assess;
        }
        if let Some(context) = update.context {
            node.context = context;
        }
        if let Some(tags) = update.tags {
            node.tags = tags;
        }
        if let Some(typical_grade) = update.typical_grade {
            node.typical_grade = typical_grade;
        }

        if let Some(encompasses) = update.encompasses {
            node.encompasses = encompasses;
        }

        let needs_rebuild = update.prereqs.is_some();
        if let Some(prereqs) = update.prereqs {
            for prereq in &prereqs {
                if !self.nodes.contains_key(prereq) {
                    return Err(GraphError::PrerequisiteNotFound(prereq.clone()));
                }
            }
            let node = self.nodes.get_mut(id).unwrap();
            node.prereqs = prereqs;
        }

        let node = &self.nodes[id];
        self.save_node_yaml(node)?;

        if needs_rebuild {
            self.rebuild_indices()?;
        }

        Ok(&self.nodes[id])
    }

    pub fn delete_node(&mut self, id: &str) -> Result<Node, GraphError> {
        let dependents: Vec<String> = self
            .children
            .get(id)
            .cloned()
            .unwrap_or_default();
        if !dependents.is_empty() {
            return Err(GraphError::HasDependents {
                node: id.to_string(),
                dependents,
            });
        }
        let node = self
            .nodes
            .remove(id)
            .ok_or_else(|| GraphError::NodeNotFound(id.to_string()))?;
        self.delete_node_yaml(&node)?;
        self.rebuild_indices()?;
        Ok(node)
    }

    pub fn add_prerequisite(
        &mut self,
        node_id: &str,
        prereq_id: &str,
    ) -> Result<(), GraphError> {
        if !self.nodes.contains_key(prereq_id) {
            return Err(GraphError::PrerequisiteNotFound(prereq_id.to_string()));
        }
        let node = self
            .nodes
            .get_mut(node_id)
            .ok_or_else(|| GraphError::NodeNotFound(node_id.to_string()))?;
        if !node.prereqs.contains(&prereq_id.to_string()) {
            node.prereqs.push(prereq_id.to_string());
        }
        let node = &self.nodes[node_id];
        self.save_node_yaml(node)?;
        self.rebuild_indices()
    }

    pub fn remove_prerequisite(
        &mut self,
        node_id: &str,
        prereq_id: &str,
    ) -> Result<(), GraphError> {
        let node = self
            .nodes
            .get_mut(node_id)
            .ok_or_else(|| GraphError::NodeNotFound(node_id.to_string()))?;
        node.prereqs.retain(|p| p != prereq_id);
        let node = &self.nodes[node_id];
        self.save_node_yaml(node)?;
        self.rebuild_indices()
    }

    // --- Persistence ---

    fn save_node_yaml(&self, node: &Node) -> Result<(), GraphError> {
        let node_file = NodeFile {
            id: node.id.clone(),
            prereqs: node.prereqs.clone(),
            bloom: node.bloom,
            assess: node.assess.clone(),
            context: node.context.clone(),
            tags: node.tags.clone(),
            typical_grade: node.typical_grade,
            encompasses: node.encompasses.clone(),
        };
        let yaml = serde_yaml::to_string(&node_file).map_err(|e| GraphError::Yaml {
            file: node.file_path.display().to_string(),
            source: e,
        })?;
        if let Some(parent) = node.file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&node.file_path, yaml)?;
        Ok(())
    }

    fn delete_node_yaml(&self, node: &Node) -> Result<(), GraphError> {
        if node.file_path.exists() {
            std::fs::remove_file(&node.file_path)?;
        }
        Ok(())
    }
}

/// Recursively load YAML files from a directory.
fn load_dir_recursive(
    root: &Path,
    dir: &Path,
    nodes: &mut HashMap<String, Node>,
    prompts: &mut HashMap<PathBuf, String>,
) -> Result<(), GraphError> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            load_dir_recursive(root, &path, nodes, prompts)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            if file_name == "_prompt.yaml" {
                // Load prompt file
                let data = std::fs::read_to_string(&path)?;
                let prompt_file: PromptFile =
                    serde_yaml::from_str(&data).map_err(|e| GraphError::Yaml {
                        file: path.display().to_string(),
                        source: e,
                    })?;
                let rel_dir = path
                    .parent()
                    .unwrap()
                    .strip_prefix(root)
                    .unwrap_or(Path::new(""))
                    .to_path_buf();
                prompts.insert(rel_dir, prompt_file.system);
            } else {
                // Load node file
                let data = std::fs::read_to_string(&path)?;
                let node_file: NodeFile =
                    serde_yaml::from_str(&data).map_err(|e| GraphError::Yaml {
                        file: path.display().to_string(),
                        source: e,
                    })?;

                if nodes.contains_key(&node_file.id) {
                    return Err(GraphError::DuplicateNode(node_file.id));
                }

                // Derive domain and unit from path:
                // graph/math/fractions/concepts/basics.yaml → domain="fractions", unit="concepts"
                let rel_path = path.strip_prefix(root).unwrap_or(&path);
                let components: Vec<&str> = rel_path
                    .components()
                    .filter_map(|c| c.as_os_str().to_str())
                    .collect();

                let (domain, unit) = match components.len() {
                    // domain/unit/file.yaml
                    3.. => (
                        components[0].to_string(),
                        components[1].to_string(),
                    ),
                    // domain/file.yaml (no unit)
                    2 => (components[0].to_string(), String::new()),
                    // file.yaml at root
                    _ => (String::new(), String::new()),
                };

                let node = Node {
                    id: node_file.id.clone(),
                    prereqs: node_file.prereqs,
                    bloom: node_file.bloom,
                    assess: node_file.assess,
                    context: node_file.context,
                    tags: node_file.tags,
                    typical_grade: node_file.typical_grade,
                    encompasses: node_file.encompasses,
                    domain,
                    unit,
                    file_path: path.to_path_buf(),
                };

                nodes.insert(node.id.clone(), node);
            }
        }
    }
    Ok(())
}

/// Kahn's algorithm for topological sort.
fn topo_sort(nodes: &HashMap<String, Node>) -> Result<Vec<String>, GraphError> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for node in nodes.values() {
        in_degree.entry(&node.id).or_insert(0);
    }
    for node in nodes.values() {
        for _prereq in &node.prereqs {
            *in_degree.get_mut(node.id.as_str()).unwrap() += 1;
        }
    }

    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(id, _)| *id)
        .collect();
    queue.sort();

    let mut result = Vec::with_capacity(nodes.len());
    while let Some(id) = queue.pop() {
        result.push(id.to_string());
        for other in nodes.values() {
            if other.prereqs.iter().any(|p| p == id) {
                let deg = in_degree.get_mut(other.id.as_str()).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(&other.id);
                    queue.sort();
                }
            }
        }
    }

    if result.len() != nodes.len() {
        return Err(GraphError::CycleDetected);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_graph_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join("graph")
            .join("math")
    }

    #[test]
    fn load_graph() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        assert!(graph.nodes.len() > 0, "Should load at least one node");
        eprintln!("Loaded {} nodes", graph.nodes.len());
    }

    #[test]
    fn topo_order_valid() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let mut seen = std::collections::HashSet::new();
        for id in &graph.topo_order {
            let node = &graph.nodes[id];
            for prereq in &node.prereqs {
                assert!(seen.contains(prereq), "{prereq} should come before {id}");
            }
            seen.insert(id.clone());
        }
    }

    #[test]
    fn empty_student_frontier_has_root_nodes() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let state = StudentState::new("test");
        let frontier = graph.get_frontier(&state);
        // All frontier nodes should have no prerequisites
        for node in &frontier {
            assert!(node.prereqs.is_empty(), "{} has prereqs", node.id);
        }
        assert!(!frontier.is_empty(), "Should have at least one root node");
    }

    #[test]
    fn prompts_loaded() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        // Should have at least the root _prompt.yaml
        assert!(
            !graph.prompts.is_empty(),
            "Should load at least one prompt file"
        );
    }

    #[test]
    fn prompt_cascade_works() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        // Find any node and check that its cascade is non-empty
        if let Some(id) = graph.nodes.keys().next() {
            let cascade = graph.get_prompt_cascade(id);
            assert!(cascade.is_some(), "Should have a prompt cascade for {}", id);
        }
    }

    #[test]
    fn list_domains_works() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let domains = graph.list_domains();
        assert!(!domains.is_empty(), "Should have at least one domain");
    }

    #[test]
    fn search_nodes_works() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        // Search for something that should exist
        let results = graph.search_nodes("fraction");
        // If fractions domain exists, should find something
        if graph.nodes.values().any(|n| n.domain == "fractions") {
            assert!(!results.is_empty());
        }
    }

    #[test]
    fn validate_works() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let report = graph.validate();
        assert_eq!(report.node_count, graph.nodes.len());
        assert!(!report.root_nodes.is_empty());
    }

    #[test]
    fn encompasses_loaded() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        // We annotated 10 nodes with encompasses
        let nodes_with_encompasses: Vec<&str> = graph
            .nodes
            .values()
            .filter(|n| !n.encompasses.is_empty())
            .map(|n| n.id.as_str())
            .collect();
        assert!(
            nodes_with_encompasses.len() >= 10,
            "Expected at least 10 nodes with encompasses, got {}",
            nodes_with_encompasses.len()
        );
    }

    #[test]
    fn encompasses_reverse_index() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        // ops.mul.facts is encompassed by multiple nodes
        let encompassed = graph.encompassed_by.get("ops.mul.facts");
        assert!(
            encompassed.is_some() && !encompassed.unwrap().is_empty(),
            "ops.mul.facts should be encompassed by at least one node"
        );
    }

    #[test]
    fn encompasses_weights_valid() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        for node in graph.nodes.values() {
            for (target, weight) in &node.encompasses {
                assert!(
                    *weight > 0.0 && *weight <= 1.0,
                    "{} encompasses {} with invalid weight {}",
                    node.id, target, weight
                );
                assert!(
                    graph.nodes.contains_key(target),
                    "{} encompasses unknown node {}",
                    node.id, target
                );
            }
        }
    }

    #[test]
    fn encompasses_in_validation_report() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let report = graph.validate();
        assert!(
            report.encompasses_count >= 20,
            "Expected at least 20 encompasses edges, got {}",
            report.encompasses_count
        );
    }

    #[test]
    fn create_and_delete_node() {
        let dir = tempfile::tempdir().unwrap();
        // Create a minimal graph with one root node
        let root_dir = dir.path().join("test-domain").join("test-unit");
        std::fs::create_dir_all(&root_dir).unwrap();
        std::fs::write(
            root_dir.join("root.yaml"),
            "id: td.tu.root\nprereqs: []\nbloom: know\nassess: [solve]\n",
        )
        .unwrap();

        let mut graph = Graph::load(dir.path()).unwrap();
        assert_eq!(graph.nodes.len(), 1);

        // Create a child node
        let child = Node {
            id: "td.tu.child".to_string(),
            prereqs: vec!["td.tu.root".to_string()],
            bloom: crate::types::BloomLevel::Apply,
            assess: vec!["solve".to_string()],
            context: None,
            tags: vec![],
            typical_grade: None,
            encompasses: HashMap::from([("td.tu.root".to_string(), 0.8)]),
            domain: "test-domain".to_string(),
            unit: "test-unit".to_string(),
            file_path: root_dir.join("child.yaml"),
        };
        graph.create_node(child).unwrap();
        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.encompassed_by.contains_key("td.tu.root"));

        // Delete the child
        graph.delete_node("td.tu.child").unwrap();
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn update_node_encompasses() {
        let dir = tempfile::tempdir().unwrap();
        let root_dir = dir.path().join("d").join("u");
        std::fs::create_dir_all(&root_dir).unwrap();
        std::fs::write(
            root_dir.join("a.yaml"),
            "id: d.u.a\nprereqs: []\nbloom: know\nassess: [solve]\n",
        )
        .unwrap();
        std::fs::write(
            root_dir.join("b.yaml"),
            "id: d.u.b\nprereqs: [d.u.a]\nbloom: apply\nassess: [solve]\n",
        )
        .unwrap();

        let mut graph = Graph::load(dir.path()).unwrap();
        assert!(graph.nodes["d.u.b"].encompasses.is_empty());

        let update = NodeUpdate {
            prereqs: None,
            bloom: None,
            assess: None,
            context: None,
            tags: None,
            typical_grade: None,
            encompasses: Some(HashMap::from([("d.u.a".to_string(), 0.7)])),
        };
        graph.update_node("d.u.b", update).unwrap();
        assert_eq!(graph.nodes["d.u.b"].encompasses.len(), 1);
        assert_eq!(graph.nodes["d.u.b"].encompasses["d.u.a"], 0.7);

        // Verify it persists on disk
        let reloaded = Graph::load(dir.path()).unwrap();
        assert_eq!(reloaded.nodes["d.u.b"].encompasses["d.u.a"], 0.7);
    }
}
