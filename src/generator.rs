//! File generation logic for ai-init.
//!
//! Handles creating all project files and directories.

use crate::git::{GitError, GitOperations};
use crate::templates::{TemplateError, TemplateRenderer};
use crate::tools::ToolDetector;
use crate::types::{DryRunFile, DryRunResult, ProjectConfig, TemplateContext};
use colored::*;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Template error: {0}")]
    TemplateError(#[from] TemplateError),
    #[error("Git error: {0}")]
    GitError(#[from] GitError),
    #[error("Directory already exists: {0}")]
    DirectoryExists(PathBuf),
}

/// Project generator that creates all files and directories.
pub struct ProjectGenerator {
    renderer: TemplateRenderer,
    tool_detector: ToolDetector,
}

impl ProjectGenerator {
    /// Create a new project generator.
    pub fn new() -> Result<Self, GeneratorError> {
        Ok(Self {
            renderer: TemplateRenderer::new()?,
            tool_detector: ToolDetector::new(),
        })
    }

    /// Create a generator with a custom tool detector.
    pub fn with_tool_detector(tool_detector: ToolDetector) -> Result<Self, GeneratorError> {
        Ok(Self {
            renderer: TemplateRenderer::new()?,
            tool_detector,
        })
    }

    /// Write a file with optional backup of existing file.
    fn write_with_backup(
        path: &PathBuf,
        content: &str,
        backup: bool,
        result: &mut GenerationResult,
    ) -> Result<(), GeneratorError> {
        // If file exists and backup is enabled, create backup
        if path.exists() && backup {
            let backup_path = path.with_extension(
                format!("{}.bak", path.extension().and_then(|e| e.to_str()).unwrap_or(""))
            );
            fs::copy(path, &backup_path)?;
            result.add_warning(format!("Backed up existing {} to {}", path.display(), backup_path.display()));
        }

        fs::write(path, content)?;
        Ok(())
    }

    /// Ensure AI files are listed in .gitignore.
    fn ensure_ai_files_in_gitignore(
        gitignore_path: &PathBuf,
        backup: bool,
        result: &mut GenerationResult,
    ) -> Result<(), GeneratorError> {
        let ai_entries = vec![
            "# AI-generated context files",
            ".ai/context/",
            "CLAUDE.md",
            ".ai/",
        ];

        let mut content = if gitignore_path.exists() {
            fs::read_to_string(gitignore_path)?
        } else {
            String::new()
        };

        // Check if AI entries already exist
        let has_ai_section = content.contains("# AI-generated context files");

        if !has_ai_section {
            // Backup if needed
            if gitignore_path.exists() && backup {
                let backup_path = gitignore_path.with_extension("gitignore.bak");
                fs::copy(gitignore_path, &backup_path)?;
                result.add_warning(format!("Backed up existing .gitignore to {}", backup_path.display()));
            }

            // Append AI entries
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push('\n');
            content.push_str(&ai_entries.join("\n"));
            content.push('\n');

            fs::write(gitignore_path, content)?;
            result.add_warning("Added AI file entries to .gitignore".to_string());
        }

        Ok(())
    }

    /// Perform a dry run to preview what will be created.
    pub fn dry_run(&self, config: &ProjectConfig) -> Result<DryRunResult, GeneratorError> {
        let mut result = DryRunResult::new();
        let base_path = &config.target_path;

        // Detect tools for context
        let tools = self.tool_detector.detect_all();
        let ctx = TemplateContext::from_config(config, tools);

        // Main directories
        result.directories.push(base_path.clone());
        result.directories.push(base_path.join(".ai"));
        result.directories.push(base_path.join(".ai/context"));

        // CLAUDE.md
        let claude_content = self.renderer.render_claude_md(&ctx)?;
        result.files.push(DryRunFile {
            path: base_path.join("CLAUDE.md"),
            size_bytes: claude_content.len(),
        });

        // .ai/TOOLS.md
        let tools_content = self.renderer.render_tools_md(&ctx)?;
        result.files.push(DryRunFile {
            path: base_path.join(".ai/TOOLS.md"),
            size_bytes: tools_content.len(),
        });

        // .ai/ARCHITECTURE.md
        let arch_content = self.renderer.render_architecture_md(&ctx)?;
        result.files.push(DryRunFile {
            path: base_path.join(".ai/ARCHITECTURE.md"),
            size_bytes: arch_content.len(),
        });

        // .ai/CONVENTIONS.md
        let conv_content = self.renderer.render_conventions_md(&ctx)?;
        result.files.push(DryRunFile {
            path: base_path.join(".ai/CONVENTIONS.md"),
            size_bytes: conv_content.len(),
        });

        // README.md (if enabled)
        if config.create_readme {
            let readme_content = self.renderer.render_readme_md(&ctx)?;
            result.files.push(DryRunFile {
                path: base_path.join("README.md"),
                size_bytes: readme_content.len(),
            });
        }

        // .gitignore (if git is enabled)
        if config.init_git {
            let gitignore_content = self.renderer.render_gitignore(&ctx)?;
            result.files.push(DryRunFile {
                path: base_path.join(".gitignore"),
                size_bytes: gitignore_content.len(),
            });
            result.git_init = true;
        }

        Ok(result)
    }

    /// Generate the project.
    pub fn generate(&self, config: &ProjectConfig) -> Result<GenerationResult, GeneratorError> {
        let base_path = &config.target_path;
        let mut result = GenerationResult::new();

        // Check if we're in existing repo mode
        let is_existing_repo = base_path.exists() && GitOperations::is_git_repo(base_path);
        if is_existing_repo && config.update_mode {
            result.add_warning("Updating existing repository with AI files".to_string());
        }

        // Create base directory if it doesn't exist
        if !base_path.exists() {
            fs::create_dir_all(base_path)?;
            result.add_created_dir(base_path.clone());
        }

        // Create .ai directory structure
        let ai_dir = base_path.join(".ai");
        let context_dir = ai_dir.join("context");

        if !ai_dir.exists() {
            fs::create_dir_all(&context_dir)?;
            result.add_created_dir(ai_dir.clone());
            result.add_created_dir(context_dir);
        } else if !context_dir.exists() {
            fs::create_dir(&context_dir)?;
            result.add_created_dir(context_dir);
        }

        // Detect tools and create context
        let tools = self.tool_detector.detect_all();
        let ctx = TemplateContext::from_config(config, tools);

        // Generate CLAUDE.md
        let claude_content = self.renderer.render_claude_md(&ctx)?;
        let claude_path = base_path.join("CLAUDE.md");
        Self::write_with_backup(&claude_path, &claude_content, config.backup_existing, &mut result)?;
        result.add_created_file(claude_path, claude_content.len());

        // Generate .ai/TOOLS.md
        let tools_content = self.renderer.render_tools_md(&ctx)?;
        let tools_path = ai_dir.join("TOOLS.md");
        Self::write_with_backup(&tools_path, &tools_content, config.backup_existing, &mut result)?;
        result.add_created_file(tools_path, tools_content.len());

        // Generate .ai/ARCHITECTURE.md
        let arch_content = self.renderer.render_architecture_md(&ctx)?;
        let arch_path = ai_dir.join("ARCHITECTURE.md");
        Self::write_with_backup(&arch_path, &arch_content, config.backup_existing, &mut result)?;
        result.add_created_file(arch_path, arch_content.len());

        // Generate .ai/CONVENTIONS.md
        let conv_content = self.renderer.render_conventions_md(&ctx)?;
        let conv_path = ai_dir.join("CONVENTIONS.md");
        Self::write_with_backup(&conv_path, &conv_content, config.backup_existing, &mut result)?;
        result.add_created_file(conv_path, conv_content.len());

        // Generate README.md if enabled
        if config.create_readme {
            let readme_content = self.renderer.render_readme_md(&ctx)?;
            let readme_path = base_path.join("README.md");
            // Only create README if it doesn't exist in update mode
            if !config.update_mode || !readme_path.exists() {
                Self::write_with_backup(&readme_path, &readme_content, config.backup_existing, &mut result)?;
                result.add_created_file(readme_path, readme_content.len());
            }
        }

        // Git initialization
        if config.init_git && !is_existing_repo {
            // Generate .gitignore
            let gitignore_content = self.renderer.render_gitignore(&ctx)?;
            let gitignore_path = base_path.join(".gitignore");
            Self::write_with_backup(&gitignore_path, &gitignore_content, config.backup_existing, &mut result)?;
            result.add_created_file(gitignore_path, gitignore_content.len());

            // Initialize git repo
            match GitOperations::init(base_path) {
                Ok(repo) => {
                    result.git_initialized = true;

                    // Initial commit if requested
                    if config.initial_commit {
                        match GitOperations::initial_commit(
                            &repo,
                            "Initial commit: AI-ready project structure\n\nGenerated by ai-init",
                        ) {
                            Ok(_) => result.initial_commit_created = true,
                            Err(e) => result.add_warning(format!("Failed to create initial commit: {}", e)),
                        }
                    }
                }
                Err(GitError::AlreadyExists(_)) => {
                    result.add_warning("Git repository already exists, skipping initialization".to_string());
                }
                Err(e) => {
                    result.add_warning(format!("Failed to initialize git: {}", e));
                }
            }
        } else if is_existing_repo {
            result.add_warning("Working with existing git repository".to_string());
            // Update .gitignore to include AI files
            let gitignore_path = base_path.join(".gitignore");
            Self::ensure_ai_files_in_gitignore(&gitignore_path, config.backup_existing, &mut result)?;
        }

        result.tools_detected = ctx.tools_available;

        Ok(result)
    }

    /// Print dry run results.
    pub fn print_dry_run(&self, result: &DryRunResult) {
        println!();
        println!("{}", "🤖 AI Project Initializer (DRY RUN)".bold().cyan());
        println!();
        println!("Would create:");

        for dir in &result.directories {
            println!("  {}/", dir.display());
        }

        for file in &result.files {
            println!(
                "  {} ({})",
                file.path.display(),
                DryRunResult::format_size(file.size_bytes)
            );
        }

        if result.git_init {
            println!();
            println!("Would initialize:");
            println!("  Git repository");
        }

        println!();
        println!(
            "Total: {} files, {} directories, {}",
            result.files.len(),
            result.directories.len(),
            DryRunResult::format_size(result.total_size())
        );
        println!();
        println!("Run without {} to create project.", "--dry-run".yellow());
    }

    /// Print generation results.
    pub fn print_result(&self, result: &GenerationResult, config: &ProjectConfig) {
        println!();

        for dir in &result.created_dirs {
            println!("{} Created {}/", "✓".green(), dir.display());
        }

        for (file, _) in &result.created_files {
            println!("{} Generated {}", "✓".green(), file.display());
        }

        if result.git_initialized {
            println!("{} Initialized git repository", "✓".green());
        }

        if result.initial_commit_created {
            println!("{} Created initial commit", "✓".green());
        }

        println!(
            "{} Registered {} available tools in .ai/TOOLS.md",
            "✓".green(),
            result.tools_detected
        );

        for warning in &result.warnings {
            println!("{} {}", "⚠".yellow(), warning);
        }

        println!();
        println!("Next steps:");
        println!("  cd {}", config.name);
        println!("  # Start coding - AI agents now know about your tooling!");
    }
}

impl Default for ProjectGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default generator")
    }
}

/// Result of project generation.
#[derive(Debug, Default)]
pub struct GenerationResult {
    pub created_dirs: Vec<PathBuf>,
    pub created_files: Vec<(PathBuf, usize)>,
    pub git_initialized: bool,
    pub initial_commit_created: bool,
    pub tools_detected: usize,
    pub warnings: Vec<String>,
}

impl GenerationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_created_dir(&mut self, path: PathBuf) {
        self.created_dirs.push(path);
    }

    pub fn add_created_file(&mut self, path: PathBuf, size: usize) {
        self.created_files.push((path, size));
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ProjectType;
    use tempfile::TempDir;

    fn create_test_config(temp_dir: &TempDir) -> ProjectConfig {
        ProjectConfig {
            name: "test-project".to_string(),
            description: "A test project".to_string(),
            languages: vec!["Rust".to_string()],
            project_type: ProjectType::Cli,
            create_readme: true,
            init_git: true,
            initial_commit: false,
            target_path: temp_dir.path().join("test-project"),
            update_mode: false,
            backup_existing: false,
        }
    }

    #[test]
    fn test_dry_run() {
        let temp = TempDir::new().unwrap();
        let config = create_test_config(&temp);
        let generator = ProjectGenerator::new().unwrap();

        let result = generator.dry_run(&config).unwrap();

        assert!(!result.directories.is_empty());
        assert!(!result.files.is_empty());
        assert!(result.git_init);

        // Verify no files were actually created
        assert!(!config.target_path.exists());
    }

    #[test]
    fn test_generate_creates_files() {
        let temp = TempDir::new().unwrap();
        let config = create_test_config(&temp);
        let generator = ProjectGenerator::new().unwrap();

        let result = generator.generate(&config).unwrap();

        // Check directories exist
        assert!(config.target_path.exists());
        assert!(config.target_path.join(".ai").exists());
        assert!(config.target_path.join(".ai/context").exists());

        // Check files exist
        assert!(config.target_path.join("CLAUDE.md").exists());
        assert!(config.target_path.join(".ai/TOOLS.md").exists());
        assert!(config.target_path.join(".ai/ARCHITECTURE.md").exists());
        assert!(config.target_path.join(".ai/CONVENTIONS.md").exists());
        assert!(config.target_path.join("README.md").exists());
        assert!(config.target_path.join(".gitignore").exists());

        // Check git was initialized
        assert!(result.git_initialized);
        assert!(config.target_path.join(".git").exists());
    }
}
