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
use git::GitOperations;
use interactive::{ask_existing_directory, ExistingDirAction, InteractivePrompt};
use std::process::ExitCode;
use tools::ToolDetector;
use types::GenerationMode;

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

    // Handle repository cloning if --repo is provided
    if let Some(repo_url) = &cli.repo_url {
        println!("{} Cloning repository from {}...", "→".cyan(), repo_url);

        let project_path = cli.absolute_project_path()?;

        // Check if directory already exists
        if project_path.exists() {
            return Err(format!(
                "Directory '{}' already exists. Please choose a different name or remove the existing directory.",
                project_path.display()
            ).into());
        }

        // Clone the repository
        match GitOperations::clone(repo_url, &project_path) {
            Ok(_) => {
                println!("{} Repository cloned successfully", "✓".green());
            }
            Err(e) => {
                return Err(format!("Failed to clone repository: {}", e).into());
            }
        }
    }

    // Get absolute project path
    let project_path = cli.absolute_project_path()?;

    // Determine update mode (automatically true for cloned repos)
    let update_mode = cli.update || cli.repo_url.is_some() || (project_path.exists() && cli.in_place);
    let backup_mode = cli.backup || cli.repo_url.is_some(); // Always backup when cloning

    // Check if directory exists and handle accordingly
    if project_path.exists() && !cli.in_place && !cli.update && !cli.is_current_dir() {
        if cli.no_interactive {
            return Err(format!(
                "Directory '{}' already exists. Use --in-place or --update to work with existing directory.",
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

    // Determine generation mode from CLI flags
    let generation_mode = if cli.mcp {
        GenerationMode::Mcp
    } else if cli.verbose {
        GenerationMode::Verbose
    } else {
        GenerationMode::Minimal
    };

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
            update_mode,
            backup_mode,
            generation_mode,
        )
    } else {
        let prompt = InteractivePrompt::new(config.clone());
        let mut cfg = prompt.run(
            project_path,
            Some(cli.project_name()),
            cli.description.clone(),
            cli.parsed_languages(),
            cli.parsed_project_type(),
            generation_mode,
        )?;
        cfg.update_mode = update_mode;
        cfg.backup_existing = backup_mode;
        cfg
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
