//! Shared types and data structures for ai-init.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Web,
    Cli,
    Library,
    System,
    #[default]
    Mixed,
}

impl ProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectType::Web => "web",
            ProjectType::Cli => "cli",
            ProjectType::Library => "library",
            ProjectType::System => "system",
            ProjectType::Mixed => "mixed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "web" => Some(ProjectType::Web),
            "cli" => Some(ProjectType::Cli),
            "library" | "lib" => Some(ProjectType::Library),
            "system" | "sys" => Some(ProjectType::System),
            "mixed" => Some(ProjectType::Mixed),
            _ => None,
        }
    }

    pub fn variants() -> &'static [&'static str] {
        &["web", "cli", "library", "system", "mixed"]
    }
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Information about a detected tool.
#[derive(Debug, Clone, Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub binary_name: String,
    pub installed: bool,
    pub path: Option<PathBuf>,
    pub description: String,
    pub usage: String,
}

impl ToolInfo {
    pub fn new(name: &str, binary_name: &str, description: &str, usage: &str) -> Self {
        Self {
            name: name.to_string(),
            binary_name: binary_name.to_string(),
            installed: false,
            path: None,
            description: description.to_string(),
            usage: usage.to_string(),
        }
    }
}

/// Project configuration gathered from user input.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub languages: Vec<String>,
    pub project_type: ProjectType,
    pub create_readme: bool,
    pub init_git: bool,
    pub initial_commit: bool,
    pub target_path: PathBuf,
    pub update_mode: bool,
    pub backup_existing: bool,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            languages: Vec::new(),
            project_type: ProjectType::Mixed,
            create_readme: true,
            init_git: true,
            initial_commit: false,
            target_path: PathBuf::new(),
            update_mode: false,
            backup_existing: false,
        }
    }
}

/// Context passed to templates for rendering.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateContext {
    pub project_name: String,
    pub description: String,
    pub languages: Vec<String>,
    pub project_type: String,
    pub created_date: String,
    pub tools: Vec<ToolInfo>,
    pub tools_available: usize,
    pub tool_code_summarizer: bool,
    pub tool_context_query: bool,
    pub tool_code_index: bool,
    pub tool_context_packer: bool,
}

impl TemplateContext {
    pub fn from_config(config: &ProjectConfig, tools: Vec<ToolInfo>) -> Self {
        let tools_available = tools.iter().filter(|t| t.installed).count();

        let tool_code_summarizer = tools.iter().any(|t| t.binary_name == "code-summarizer" && t.installed);
        let tool_context_query = tools.iter().any(|t| t.binary_name == "context-query" && t.installed);
        let tool_code_index = tools.iter().any(|t| t.binary_name == "code-index" && t.installed);
        let tool_context_packer = tools.iter().any(|t| t.binary_name == "context-packer" && t.installed);

        Self {
            project_name: config.name.clone(),
            description: config.description.clone(),
            languages: config.languages.clone(),
            project_type: config.project_type.as_str().to_string(),
            created_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            tools,
            tools_available,
            tool_code_summarizer,
            tool_context_query,
            tool_code_index,
            tool_context_packer,
        }
    }
}

/// Result of a dry run operation.
#[derive(Debug, Clone)]
pub struct DryRunResult {
    pub files: Vec<DryRunFile>,
    pub directories: Vec<PathBuf>,
    pub git_init: bool,
}

#[derive(Debug, Clone)]
pub struct DryRunFile {
    pub path: PathBuf,
    pub size_bytes: usize,
}

impl DryRunResult {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            directories: Vec::new(),
            git_init: false,
        }
    }

    pub fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.size_bytes).sum()
    }

    pub fn format_size(bytes: usize) -> String {
        if bytes < 1024 {
            format!("{} bytes", bytes)
        } else {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        }
    }
}

impl Default for DryRunResult {
    fn default() -> Self {
        Self::new()
    }
}
