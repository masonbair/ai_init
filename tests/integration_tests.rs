//! Integration tests for ai-init.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Get a command for the ai-init binary.
fn ai_init() -> Command {
    Command::cargo_bin("ai-init").unwrap()
}

#[test]
fn test_help_flag() {
    ai_init()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("AI-ready project initializer"));
}

#[test]
fn test_version_flag() {
    ai_init()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ai-init"));
}

#[test]
fn test_dry_run_mode() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("test-project");

    ai_init()
        .arg(&project_path)
        .arg("--dry-run")
        .arg("--no-interactive")
        .assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Would create"));

    // Verify nothing was actually created
    assert!(!project_path.exists());
}

#[test]
fn test_non_interactive_creation() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("test-project");

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .arg("--description")
        .arg("Test project description")
        .arg("--language")
        .arg("Rust,Python")
        .arg("--type")
        .arg("cli")
        .assert()
        .success();

    // Verify files were created
    assert!(project_path.exists());
    assert!(project_path.join("CLAUDE.md").exists());
    assert!(project_path.join(".ai").exists());
    assert!(project_path.join(".ai/TOOLS.md").exists());
    assert!(project_path.join(".ai/ARCHITECTURE.md").exists());
    assert!(project_path.join(".ai/CONVENTIONS.md").exists());
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join(".gitignore").exists());
    assert!(project_path.join(".git").exists());

    // Verify CLAUDE.md content
    let claude_content = fs::read_to_string(project_path.join("CLAUDE.md")).unwrap();
    assert!(claude_content.contains("test-project"));
    assert!(claude_content.contains("Test project description"));
    assert!(claude_content.contains("Rust, Python"));
    assert!(claude_content.contains("cli"));
}

#[test]
fn test_no_git_flag() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("no-git-project");

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .arg("--no-git")
        .assert()
        .success();

    // Verify .git was not created
    assert!(!project_path.join(".git").exists());
    assert!(!project_path.join(".gitignore").exists());
}

#[test]
fn test_no_readme_flag() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("no-readme-project");

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .arg("--no-readme")
        .assert()
        .success();

    // Verify README was not created
    assert!(!project_path.join("README.md").exists());
    // But other files should exist
    assert!(project_path.join("CLAUDE.md").exists());
}

#[test]
fn test_existing_directory_fails_without_in_place() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("existing-project");

    // Create the directory first
    fs::create_dir(&project_path).unwrap();

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_existing_directory_with_in_place() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("existing-project");

    // Create the directory first
    fs::create_dir(&project_path).unwrap();

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .arg("--in-place")
        .assert()
        .success();

    // Verify files were created
    assert!(project_path.join("CLAUDE.md").exists());
    assert!(project_path.join(".ai/TOOLS.md").exists());
}

#[test]
fn test_current_directory_initialization() {
    let temp = TempDir::new().unwrap();

    ai_init()
        .current_dir(temp.path())
        .arg(".")
        .arg("--no-interactive")
        .assert()
        .success();

    // Verify files were created in temp directory
    assert!(temp.path().join("CLAUDE.md").exists());
    assert!(temp.path().join(".ai/TOOLS.md").exists());
}

#[test]
fn test_initial_commit_flag() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("committed-project");

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .arg("--initial-commit")
        .assert()
        .success()
        .stdout(predicate::str::contains("initial commit"));

    // Verify commit was made
    let output = std::process::Command::new("git")
        .args(["log", "--oneline", "-1"])
        .current_dir(&project_path)
        .output()
        .unwrap();

    let log = String::from_utf8_lossy(&output.stdout);
    assert!(log.contains("Initial commit"));
}

#[test]
fn test_gitignore_contains_language_specific_rules() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("python-project");

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .arg("--language")
        .arg("Python")
        .assert()
        .success();

    let gitignore = fs::read_to_string(project_path.join(".gitignore")).unwrap();
    assert!(gitignore.contains("__pycache__"));
    assert!(gitignore.contains(".pytest_cache"));
}

#[test]
fn test_tools_md_contains_tool_info() {
    let temp = TempDir::new().unwrap();
    let project_path = temp.path().join("tools-test");

    ai_init()
        .arg(&project_path)
        .arg("--no-interactive")
        .assert()
        .success();

    let tools_content = fs::read_to_string(project_path.join(".ai/TOOLS.md")).unwrap();
    assert!(tools_content.contains("CodeSummarizer"));
    assert!(tools_content.contains("ContextQuery"));
    assert!(tools_content.contains("CodeIndex"));
    assert!(tools_content.contains("ContextPacker"));
}
