//! ai-init - AI-Ready Project Initializer
//!
//! A command-line tool for creating project directories pre-configured
//! for AI agent workflows. Scaffolds AI context files, tool registries,
//! and project metadata so that AI agents immediately understand available
//! tooling and project structure.

mod cli;
mod config;
mod generator;
mod git;
mod interactive;
mod templates;
mod tools;
mod types;

use clap::Parser;
use cli::Cli;
use colored::*;
use config::Config;
use generator::ProjectGenerator;
use interactive::{ask_existing_directory, ExistingDirAction, InteractivePrompt};
use std::process::ExitCode;
use tools::ToolDetector;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("{} Failed to load config: {}", "⚠".yellow(), e);
        eprintln!("Using default configuration.");
        Config::default()
    });

    // Get absolute project path
    let project_path = cli.absolute_project_path()?;

    // Check if directory exists and handle accordingly
    if project_path.exists() && !cli.in_place && !cli.is_current_dir() {
        if cli.no_interactive {
            return Err(format!(
                "Directory '{}' already exists. Use --in-place to initialize in existing directory.",
                project_path.display()
            ).into());
        }

        match ask_existing_directory(&project_path)? {
            ExistingDirAction::InitializeInPlace => {
                // Continue with initialization
            }
            ExistingDirAction::Cancel => {
                println!("Cancelled.");
                return Ok(());
            }
        }
    }

    // Set up tool detector with custom paths
    let mut tool_detector = ToolDetector::new();
    for (name, path) in cli.parsed_tool_paths() {
        tool_detector.add_custom_path(&name, path);
    }

    // Also add paths from config
    for (name, path) in &config.tools.custom_paths {
        tool_detector.add_custom_path(name, path.clone());
    }

    // Build project configuration
    let project_config = if cli.no_interactive {
        let prompt = InteractivePrompt::new(config.clone());
        prompt.build_non_interactive(
            project_path,
            Some(cli.project_name()),
            cli.description.clone(),
            cli.parsed_languages(),
            cli.parsed_project_type(),
            cli.no_readme,
            cli.no_git,
            cli.initial_commit,
        )
    } else {
        let prompt = InteractivePrompt::new(config.clone());
        prompt.run(
            project_path,
            Some(cli.project_name()),
            cli.description.clone(),
            cli.parsed_languages(),
            cli.parsed_project_type(),
        )?
    };

    // Create generator
    let generator = ProjectGenerator::with_tool_detector(tool_detector)?;

    // Dry run or actual generation
    if cli.dry_run {
        let result = generator.dry_run(&project_config)?;
        generator.print_dry_run(&result);
    } else {
        let result = generator.generate(&project_config)?;
        generator.print_result(&result, &project_config);
    }

    Ok(())
}
