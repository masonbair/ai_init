//! Command-line argument parsing for ai-init.

use crate::types::ProjectType;
use clap::Parser;
use std::path::PathBuf;

/// AI-ready project initializer for agent workflows.
///
/// Creates project directories pre-configured with AI context files,
/// tool registries, and project metadata so that AI agents immediately
/// understand available tooling and project structure.
#[derive(Parser, Debug)]
#[command(name = "ai-init")]
#[command(author, version, about, long_about = None)]
#[command(after_help = "Examples:
  ai-init myproject                            Create new AI-ready project
  ai-init myproject --type web                 Create with specific project type
  ai-init .                                    Initialize in existing directory
  ai-init . --update --backup                  Update AI files in current directory
  ai-init myproject --repo https://github.com/user/repo  Clone and initialize repo
  ai-init myproject --dry-run                  Preview what will be created
  ai-init myproject --no-interactive           Use defaults, no prompts")]
pub struct Cli {
    /// Project directory name or path.
    /// Use '.' to initialize in the current directory.
    #[arg(value_name = "PROJECT")]
    pub project: PathBuf,

    /// Project type (web, cli, library, system, mixed)
    #[arg(short = 't', long = "type", value_name = "TYPE")]
    pub project_type: Option<String>,

    /// Primary programming language(s), comma-separated
    #[arg(short = 'l', long = "language", value_name = "LANGS")]
    pub languages: Option<String>,

    /// Project description
    #[arg(short = 'd', long = "description", value_name = "DESC")]
    pub description: Option<String>,

    /// Skip interactive prompts, use defaults
    #[arg(long = "no-interactive")]
    pub no_interactive: bool,

    /// Preview what will be created without making changes
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Skip git repository initialization
    #[arg(long = "no-git")]
    pub no_git: bool,

    /// Skip README.md creation
    #[arg(long = "no-readme")]
    pub no_readme: bool,

    /// Create initial git commit after setup
    #[arg(long = "initial-commit")]
    pub initial_commit: bool,

    /// Initialize in existing directory (don't fail if directory exists)
    #[arg(long = "in-place")]
    pub in_place: bool,

    /// Update/refresh AI files in existing repository
    #[arg(long = "update")]
    pub update: bool,

    /// Backup existing AI files before updating (with .bak extension)
    #[arg(long = "backup")]
    pub backup: bool,

    /// Clone repository from URL before initializing (e.g., https://github.com/user/repo)
    #[arg(long = "repo", value_name = "URL")]
    pub repo_url: Option<String>,

    /// Custom tool path override (format: tool-name=/path/to/binary)
    #[arg(long = "tool-path", value_name = "TOOL=PATH")]
    pub tool_paths: Vec<String>,
}

impl Cli {
    /// Parse the project type from CLI argument.
    pub fn parsed_project_type(&self) -> Option<ProjectType> {
        self.project_type.as_ref().and_then(|t| ProjectType::from_str(t))
    }

    /// Parse languages from comma-separated string.
    pub fn parsed_languages(&self) -> Vec<String> {
        self.languages
            .as_ref()
            .map(|l| {
                l.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Parse tool path overrides into a map.
    pub fn parsed_tool_paths(&self) -> Vec<(String, PathBuf)> {
        self.tool_paths
            .iter()
            .filter_map(|s| {
                let parts: Vec<&str> = s.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), PathBuf::from(parts[1])))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if the project path is current directory.
    pub fn is_current_dir(&self) -> bool {
        self.project == PathBuf::from(".")
    }

    /// Get the absolute project path.
    pub fn absolute_project_path(&self) -> std::io::Result<PathBuf> {
        if self.is_current_dir() {
            std::env::current_dir()
        } else if self.project.is_absolute() {
            Ok(self.project.clone())
        } else {
            let cwd = std::env::current_dir()?;
            Ok(cwd.join(&self.project))
        }
    }

    /// Derive project name from the path.
    pub fn project_name(&self) -> String {
        if self.is_current_dir() {
            std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| "project".to_string())
        } else {
            self.project
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "project".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_languages() {
        let cli = Cli {
            project: PathBuf::from("test"),
            project_type: None,
            languages: Some("Rust, Python, TypeScript".to_string()),
            description: None,
            no_interactive: false,
            dry_run: false,
            no_git: false,
            no_readme: false,
            initial_commit: false,
            in_place: false,
            update: false,
            backup: false,
            repo_url: None,
            tool_paths: vec![],
        };

        let langs = cli.parsed_languages();
        assert_eq!(langs, vec!["Rust", "Python", "TypeScript"]);
    }

    #[test]
    fn test_parse_tool_paths() {
        let cli = Cli {
            project: PathBuf::from("test"),
            project_type: None,
            languages: None,
            description: None,
            no_interactive: false,
            dry_run: false,
            no_git: false,
            no_readme: false,
            initial_commit: false,
            in_place: false,
            update: false,
            backup: false,
            repo_url: None,
            tool_paths: vec![
                "code-summarizer=/usr/local/bin/summarizer".to_string(),
                "context-query=/home/user/bin/cquery".to_string(),
            ],
        };

        let paths = cli.parsed_tool_paths();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0].0, "code-summarizer");
        assert_eq!(paths[0].1, PathBuf::from("/usr/local/bin/summarizer"));
    }

    #[test]
    fn test_project_name_from_path() {
        let cli = Cli {
            project: PathBuf::from("my-awesome-project"),
            project_type: None,
            languages: None,
            description: None,
            no_interactive: false,
            dry_run: false,
            no_git: false,
            no_readme: false,
            initial_commit: false,
            in_place: false,
            update: false,
            backup: false,
            repo_url: None,
            tool_paths: vec![],
        };

        assert_eq!(cli.project_name(), "my-awesome-project");
    }
}
