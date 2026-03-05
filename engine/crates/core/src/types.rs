use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// A node file as stored on disk (YAML).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFile {
    pub id: String,
    pub prereqs: Vec<String>,
    pub bloom: BloomLevel,
    pub assess: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typical_grade: Option<u8>,
}

/// A node in the graph (includes derived fields from filesystem path).
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub prereqs: Vec<String>,
    pub bloom: BloomLevel,
    pub assess: Vec<String>,
    pub context: Option<String>,
    pub tags: Vec<String>,
    pub typical_grade: Option<u8>,
    // Derived from filesystem path:
    pub domain: String,
    pub unit: String,
    pub file_path: PathBuf,
}

impl Node {
    /// Human-readable name derived from the filename.
    pub fn display_name(&self) -> String {
        self.file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.id)
            .replace('_', " ")
    }
}

/// Partial update for a node. None fields are left unchanged.
pub struct NodeUpdate {
    pub prereqs: Option<Vec<String>>,
    pub bloom: Option<BloomLevel>,
    pub assess: Option<Vec<String>>,
    pub context: Option<Option<String>>,
    pub tags: Option<Vec<String>>,
    pub typical_grade: Option<Option<u8>>,
}

/// A _prompt.yaml entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptFile {
    pub system: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BloomLevel {
    Know,
    Understand,
    Apply,
    Analyze,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentState {
    pub student_id: String,
    pub mastery: HashMap<String, MasteryRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryRecord {
    pub level: BloomLevel,
    pub mastered_at: DateTime<Utc>,
}

impl StudentState {
    pub fn new(student_id: impl Into<String>) -> Self {
        Self {
            student_id: student_id.into(),
            mastery: HashMap::new(),
        }
    }
}
