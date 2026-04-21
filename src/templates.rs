//! Template loading and rendering for ai-init.
//!
//! Uses Tera for Jinja2-like template rendering with embedded templates.

use crate::types::TemplateContext;
use tera::{Context, Tera};
use thiserror::Error;

// Embed templates at compile time
const TEMPLATE_CLAUDE_MD: &str = include_str!("../templates/CLAUDE.md.tera");
const TEMPLATE_TOOLS_MD: &str = include_str!("../templates/TOOLS.md.tera");
const TEMPLATE_ARCHITECTURE_MD: &str = include_str!("../templates/ARCHITECTURE.md.tera");
const TEMPLATE_CONVENTIONS_MD: &str = include_str!("../templates/CONVENTIONS.md.tera");
const TEMPLATE_README_MD: &str = include_str!("../templates/README.md.tera");
const TEMPLATE_GITIGNORE: &str = include_str!("../templates/gitignore.tera");

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Failed to initialize template engine: {0}")]
    InitError(String),
    #[error("Failed to render template '{name}': {source}")]
    RenderError { name: String, source: tera::Error },
}

/// Template renderer using embedded Tera templates.
pub struct TemplateRenderer {
    tera: Tera,
}

impl TemplateRenderer {
    /// Create a new template renderer with embedded templates.
    pub fn new() -> Result<Self, TemplateError> {
        let mut tera = Tera::default();

        // Add all embedded templates
        tera.add_raw_template("CLAUDE.md", TEMPLATE_CLAUDE_MD)
            .map_err(|e| TemplateError::InitError(e.to_string()))?;
        tera.add_raw_template("TOOLS.md", TEMPLATE_TOOLS_MD)
            .map_err(|e| TemplateError::InitError(e.to_string()))?;
        tera.add_raw_template("ARCHITECTURE.md", TEMPLATE_ARCHITECTURE_MD)
            .map_err(|e| TemplateError::InitError(e.to_string()))?;
        tera.add_raw_template("CONVENTIONS.md", TEMPLATE_CONVENTIONS_MD)
            .map_err(|e| TemplateError::InitError(e.to_string()))?;
        tera.add_raw_template("README.md", TEMPLATE_README_MD)
            .map_err(|e| TemplateError::InitError(e.to_string()))?;
        tera.add_raw_template("gitignore", TEMPLATE_GITIGNORE)
            .map_err(|e| TemplateError::InitError(e.to_string()))?;

        Ok(Self { tera })
    }

    /// Render a template with the given context.
    pub fn render(&self, template_name: &str, ctx: &TemplateContext) -> Result<String, TemplateError> {
        let context = Context::from_serialize(ctx)
            .map_err(|e| TemplateError::RenderError {
                name: template_name.to_string(),
                source: e,
            })?;

        self.tera.render(template_name, &context).map_err(|e| {
            TemplateError::RenderError {
                name: template_name.to_string(),
                source: e,
            }
        })
    }

    /// Render CLAUDE.md template.
    pub fn render_claude_md(&self, ctx: &TemplateContext) -> Result<String, TemplateError> {
        self.render("CLAUDE.md", ctx)
    }

    /// Render TOOLS.md template.
    pub fn render_tools_md(&self, ctx: &TemplateContext) -> Result<String, TemplateError> {
        self.render("TOOLS.md", ctx)
    }

    /// Render ARCHITECTURE.md template.
    pub fn render_architecture_md(&self, ctx: &TemplateContext) -> Result<String, TemplateError> {
        self.render("ARCHITECTURE.md", ctx)
    }

    /// Render CONVENTIONS.md template.
    pub fn render_conventions_md(&self, ctx: &TemplateContext) -> Result<String, TemplateError> {
        self.render("CONVENTIONS.md", ctx)
    }

    /// Render README.md template.
    pub fn render_readme_md(&self, ctx: &TemplateContext) -> Result<String, TemplateError> {
        self.render("README.md", ctx)
    }

    /// Render .gitignore template.
    pub fn render_gitignore(&self, ctx: &TemplateContext) -> Result<String, TemplateError> {
        self.render("gitignore", ctx)
    }
}

impl Default for TemplateRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to initialize default template renderer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ProjectConfig, ProjectType, ToolInfo};

    fn create_test_context() -> TemplateContext {
        let config = ProjectConfig {
            name: "test-project".to_string(),
            description: "A test project for unit testing".to_string(),
            languages: vec!["Rust".to_string(), "Python".to_string()],
            project_type: ProjectType::Cli,
            create_readme: true,
            init_git: true,
            initial_commit: false,
            target_path: std::path::PathBuf::from("/tmp/test"),
        };

        let tools = vec![
            ToolInfo {
                name: "CodeSummarizer".to_string(),
                binary_name: "code-summarizer".to_string(),
                installed: true,
                path: Some(std::path::PathBuf::from("/usr/local/bin/code-summarizer")),
                description: "Test tool".to_string(),
                usage: "code-summarizer --help".to_string(),
            },
        ];

        TemplateContext::from_config(&config, tools)
    }

    #[test]
    fn test_renderer_creation() {
        let renderer = TemplateRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_render_claude_md() {
        let renderer = TemplateRenderer::new().unwrap();
        let ctx = create_test_context();
        let result = renderer.render_claude_md(&ctx);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("test-project"));
        assert!(content.contains("A test project for unit testing"));
    }

    #[test]
    fn test_render_tools_md() {
        let renderer = TemplateRenderer::new().unwrap();
        let ctx = create_test_context();
        let result = renderer.render_tools_md(&ctx);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("CodeSummarizer"));
    }
}
