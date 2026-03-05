use std::path::Path;

use crate::graph::Graph;
use crate::types::{BloomLevel, MasteryRecord, StudentState};

#[derive(Debug, thiserror::Error)]
pub enum StudentError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unknown node: {0}")]
    UnknownNode(String),
}

impl StudentState {
    pub fn load(path: &Path) -> Result<Self, StudentError> {
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self, path: &Path) -> Result<(), StudentError> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Record mastery for a node. Returns IDs of newly unlocked nodes
    /// (nodes whose prerequisites are now all mastered).
    pub fn record_mastery(
        &mut self,
        graph: &Graph,
        node_id: &str,
        level: BloomLevel,
    ) -> Result<Vec<String>, StudentError> {
        if !graph.nodes.contains_key(node_id) {
            return Err(StudentError::UnknownNode(node_id.to_string()));
        }

        self.mastery.insert(
            node_id.to_string(),
            MasteryRecord {
                level,
                mastered_at: chrono::Utc::now(),
            },
        );

        // Find newly unlocked children
        let newly_unlocked = graph
            .children
            .get(node_id)
            .map(|children| {
                children
                    .iter()
                    .filter(|child_id| {
                        !self.mastery.contains_key(*child_id)
                            && graph.nodes[*child_id]
                                .prereqs
                                .iter()
                                .all(|prereq| self.mastery.contains_key(prereq))
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        Ok(newly_unlocked)
    }
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
    fn record_mastery_returns_unlocked() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let mut state = StudentState::new("test");

        // Find a root node to master
        let root_id = graph
            .nodes
            .values()
            .find(|n| n.prereqs.is_empty())
            .map(|n| n.id.clone())
            .expect("Should have at least one root node");

        let unlocked = state
            .record_mastery(&graph, &root_id, BloomLevel::Apply)
            .unwrap();

        // If this root has children, some should be unlocked
        if let Some(children) = graph.children.get(&root_id) {
            // At least one child whose only prereq is this root should be unlocked
            let single_prereq_children: Vec<_> = children
                .iter()
                .filter(|c| graph.nodes[*c].prereqs.len() == 1)
                .collect();
            if !single_prereq_children.is_empty() {
                assert!(!unlocked.is_empty());
            }
        }
    }

    #[test]
    fn record_mastery_unknown_node() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let mut state = StudentState::new("test");

        let result = state.record_mastery(&graph, "nonexistent.node", BloomLevel::Apply);
        assert!(result.is_err());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let graph = Graph::load(&test_graph_dir()).unwrap();
        let mut state = StudentState::new("test_roundtrip");

        let root_id = graph
            .nodes
            .values()
            .find(|n| n.prereqs.is_empty())
            .map(|n| n.id.clone())
            .expect("Should have at least one root node");

        state
            .record_mastery(&graph, &root_id, BloomLevel::Apply)
            .unwrap();

        let tmp = std::env::temp_dir().join("open_mastery_test_state.json");
        state.save(&tmp).unwrap();
        let loaded = StudentState::load(&tmp).unwrap();

        assert_eq!(loaded.student_id, "test_roundtrip");
        assert!(loaded.mastery.contains_key(&root_id));
        std::fs::remove_file(&tmp).ok();
    }
}
