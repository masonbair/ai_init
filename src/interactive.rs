//! Interactive prompts for ai-init.
//!
//! Handles user interaction for gathering project configuration.

use crate::config::Config;
use crate::types::{GenerationMode, ProjectConfig, ProjectType};
use colored::*;
use dialoguer::{Confirm, Input, Select};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum InteractiveError {
    #[error("User cancelled")]
    Cancelled,
    #[error("Input error: {0}")]
    InputError(#[from] dialoguer::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Interactive prompt handler.
pub struct InteractivePrompt {
    config: Config,
}

impl InteractivePrompt {
    /// Create a new interactive prompt handler.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Display the welcome banner.
    pub fn show_banner(&self) {
        println!();
        println!("{}", "AI Project Initializer".bold().cyan());
        println!();
    }

    /// Run the full interactive setup.
    pub fn run(
        &self,
        project_path: PathBuf,
        preset_name: Option<String>,
        preset_description: Option<String>,
        preset_languages: Vec<String>,
        preset_type: Option<ProjectType>,
        generation_mode: GenerationMode,
    ) -> Result<ProjectConfig, InteractiveError> {
        self.show_banner();

        // Project name
        let default_name = project_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());

        let name = if let Some(name) = preset_name {
            println!("{}: {}", "Project name".bold(), name);
            name
        } else {
            Input::new()
                .with_prompt("Project name")
                .default(default_name)
                .interact_text()?
        };

        // Description
        let description = if let Some(desc) = preset_description {
            println!("{}: {}", "Description".bold(), desc);
            desc
        } else {
            Input::new()
                .with_prompt("Description")
                .default(format!("A {} project", name))
                .interact_text()?
        };

        // Languages
        let languages = if !preset_languages.is_empty() {
            println!("{}: {}", "Primary language(s)".bold(), preset_languages.join(", "));
            preset_languages
        } else {
            let input: String = Input::new()
                .with_prompt("Primary language(s) (comma-separated)")
                .default("Rust".to_string())
                .interact_text()?;

            input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        // Project type
        let project_type = if let Some(pt) = preset_type {
            println!("{}: {}", "Project type".bold(), pt);
            pt
        } else {
            let types = ProjectType::variants();
            let selection = Select::new()
                .with_prompt("Project type")
                .items(types)
                .default(4) // "mixed"
                .interact()?;

            ProjectType::from_str(types[selection]).unwrap_or(ProjectType::Mixed)
        };

        // README
        let create_readme = Confirm::new()
            .with_prompt("Include README.md?")
            .default(self.config.defaults.create_readme)
            .interact()?;

        // Git
        let init_git = Confirm::new()
            .with_prompt("Initialize git repository?")
            .default(self.config.defaults.git_init)
            .interact()?;

        // Initial commit (only ask if git is enabled)
        let initial_commit = if init_git {
            Confirm::new()
                .with_prompt("Create initial commit?")
                .default(self.config.defaults.initial_commit)
                .interact()?
        } else {
            false
        };

        println!();

        Ok(ProjectConfig {
            name,
            description,
            languages,
            project_type,
            create_readme,
            init_git,
            initial_commit,
            target_path: project_path,
            update_mode: false,
            backup_existing: false,
            generation_mode,
        })
    }

    /// Simple non-interactive configuration builder.
    pub fn build_non_interactive(
        &self,
        project_path: PathBuf,
        name: Option<String>,
        description: Option<String>,
        languages: Vec<String>,
        project_type: Option<ProjectType>,
        no_readme: bool,
        no_git: bool,
        initial_commit: bool,
        update_mode: bool,
        backup_existing: bool,
        generation_mode: GenerationMode,
    ) -> ProjectConfig {
        let default_name = project_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());

        let name = name.unwrap_or(default_name);

        ProjectConfig {
            name: name.clone(),
            description: description.unwrap_or_else(|| format!("A {} project", name)),
            languages: if languages.is_empty() {
                vec!["Rust".to_string()]
            } else {
                languages
            },
            project_type: project_type.unwrap_or(self.config.defaults.project_type),
            create_readme: !no_readme && self.config.defaults.create_readme,
            init_git: !no_git && self.config.defaults.git_init,
            initial_commit,
            target_path: project_path,
            update_mode,
            backup_existing,
            generation_mode,
        }
    }
}

/// Ask user what to do when directory already exists.
pub fn ask_existing_directory(path: &PathBuf) -> Result<ExistingDirAction, InteractiveError> {
    println!(
        "{} Directory '{}' already exists.",
        "⚠".yellow(),
        path.display()
    );
    println!();

    let options = &[
        "Initialize in-place (add AI files to existing directory)",
        "Cancel and choose a different name",
    ];

    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(options)
        .default(0)
        .interact()?;

    match selection {
        0 => Ok(ExistingDirAction::InitializeInPlace),
        _ => Ok(ExistingDirAction::Cancel),
    }
}

/// Action to take when directory exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExistingDirAction {
    InitializeInPlace,
    Cancel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_interactive_config() {
        let config = Config::default();
        let prompt = InteractivePrompt::new(config);

        let project_config = prompt.build_non_interactive(
            PathBuf::from("/tmp/test-project"),
            Some("test-project".to_string()),
            Some("A test project".to_string()),
            vec!["Rust".to_string(), "Python".to_string()],
            Some(ProjectType::Cli),
            false,
            false,
            false,
            false,
            false,
            GenerationMode::Minimal,
        );

        assert_eq!(project_config.name, "test-project");
        assert_eq!(project_config.description, "A test project");
        assert_eq!(project_config.languages, vec!["Rust", "Python"]);
        assert_eq!(project_config.project_type, ProjectType::Cli);
        assert!(project_config.create_readme);
        assert!(project_config.init_git);
        assert!(!project_config.initial_commit);
        assert!(!project_config.update_mode);
        assert!(!project_config.backup_existing);
        assert_eq!(project_config.generation_mode, GenerationMode::Minimal);
    }
}
