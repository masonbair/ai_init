//! Tool detection and registry for ai-init.
//!
//! Detects available AI agent tools on the system and provides
//! information about their installation status.

use crate::types::ToolInfo;
use std::collections::HashMap;
use std::path::PathBuf;

/// Defines the known tools that ai-init looks for.
const KNOWN_TOOLS: &[(&str, &str, &str, &str)] = &[
    (
        "CodeSummarizer",
        "code-summarizer",
        "Generates hierarchical context maps of the codebase for AI agents",
        "code-summarizer --project-root . --output .ai/context/",
    ),
    (
        "ContextQuery",
        "context-query",
        "Structure-aware code search combining text, AST patterns, and graph traversal",
        "context-query --pattern \"async function.*database\" --type structural",
    ),
    (
        "CodeIndex",
        "code-index",
        "Persistent semantic cache for AI agents - indexes codebases with tree-sitter for fast symbol lookup, dependency analysis, and code intelligence",
        "code-index daemon start",
    ),
    (
        "ContextPacker",
        "context-packer",
        "Smart context window packing - assembles relevant code within token budget",
        "context-packer --query \"implement feature\" --budget 8000 --format claude",
    ),
];

/// Tool detector that finds AI tools on the system.
pub struct ToolDetector {
    custom_paths: HashMap<String, PathBuf>,
}

impl ToolDetector {
    /// Create a new tool detector.
    pub fn new() -> Self {
        Self {
            custom_paths: HashMap::new(),
        }
    }

    /// Create a detector with custom tool paths.
    #[allow(dead_code)]
    pub fn with_custom_paths(custom_paths: HashMap<String, PathBuf>) -> Self {
        Self { custom_paths }
    }

    /// Add a custom path for a specific tool.
    pub fn add_custom_path(&mut self, tool_name: &str, path: PathBuf) {
        self.custom_paths.insert(tool_name.to_string(), path);
    }

    /// Detect all known tools and return their info.
    pub fn detect_all(&self) -> Vec<ToolInfo> {
        KNOWN_TOOLS
            .iter()
            .map(|(name, binary, desc, usage)| self.detect_tool(name, binary, desc, usage))
            .collect()
    }

    /// Detect a single tool.
    fn detect_tool(&self, name: &str, binary_name: &str, description: &str, usage: &str) -> ToolInfo {
        let mut info = ToolInfo::new(name, binary_name, description, usage);

        // Check custom path first
        if let Some(custom_path) = self.custom_paths.get(binary_name) {
            if custom_path.exists() && is_executable(custom_path) {
                info.installed = true;
                info.path = Some(custom_path.clone());
                return info;
            }
        }

        // Check PATH
        match which::which(binary_name) {
            Ok(path) => {
                info.installed = true;
                info.path = Some(path);
            }
            Err(_) => {
                info.installed = false;
                info.path = None;
            }
        }

        info
    }

    /// Get count of installed tools.
    #[allow(dead_code)]
    pub fn installed_count(&self) -> usize {
        self.detect_all().iter().filter(|t| t.installed).count()
    }

    /// Get total count of known tools.
    #[allow(dead_code)]
    pub fn total_count(&self) -> usize {
        KNOWN_TOOLS.len()
    }
}

impl Default for ToolDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a path is executable (Unix).
#[cfg(unix)]
fn is_executable(path: &PathBuf) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

/// Check if a path is executable (non-Unix fallback).
#[cfg(not(unix))]
fn is_executable(path: &PathBuf) -> bool {
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_detector_creation() {
        let detector = ToolDetector::new();
        assert_eq!(detector.total_count(), 4);
    }

    #[test]
    fn test_detect_all_returns_all_tools() {
        let detector = ToolDetector::new();
        let tools = detector.detect_all();
        assert_eq!(tools.len(), 4);

        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"CodeSummarizer"));
        assert!(names.contains(&"ContextQuery"));
        assert!(names.contains(&"CodeIndex"));
        assert!(names.contains(&"ContextPacker"));
    }

    #[test]
    fn test_custom_path_detection() {
        let mut detector = ToolDetector::new();

        // Add a custom path that doesn't exist
        detector.add_custom_path("code-summarizer", PathBuf::from("/nonexistent/path"));

        let tools = detector.detect_all();
        let summarizer = tools.iter().find(|t| t.binary_name == "code-summarizer").unwrap();

        // Should not be installed since path doesn't exist
        assert!(!summarizer.installed);
    }
}
