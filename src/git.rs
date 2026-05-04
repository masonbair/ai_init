//! Git operations for ai-init.
//!
//! Handles git repository initialization and initial commits.

use git2::{Repository, Signature};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum GitError {
    #[error("Git operation failed: {0}")]
    Git2Error(#[from] git2::Error),
    #[error("Repository already exists at {0}")]
    AlreadyExists(String),
    #[error("No files to commit")]
    NothingToCommit,
}

/// Git operations handler.
pub struct GitOperations;

impl GitOperations {
    /// Check if a directory is already a git repository.
    pub fn is_git_repo(path: &Path) -> bool {
        path.join(".git").exists() || Repository::discover(path).is_ok()
    }

    /// Clone a repository from a URL.
    pub fn clone(url: &str, path: &Path) -> Result<Repository, GitError> {
        let repo = Repository::clone(url, path)?;
        Ok(repo)
    }

    /// Initialize a new git repository.
    pub fn init(path: &Path) -> Result<Repository, GitError> {
        if Self::is_git_repo(path) {
            return Err(GitError::AlreadyExists(path.display().to_string()));
        }

        let repo = Repository::init(path)?;
        Ok(repo)
    }

    /// Initialize or open existing repository.
    #[allow(dead_code)]
    pub fn init_or_open(path: &Path) -> Result<Repository, GitError> {
        if Self::is_git_repo(path) {
            Ok(Repository::open(path)?)
        } else {
            Ok(Repository::init(path)?)
        }
    }

    /// Create an initial commit with all staged files.
    pub fn initial_commit(repo: &Repository, message: &str) -> Result<git2::Oid, GitError> {
        let mut index = repo.index()?;

        // Add all files
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        // Create signature
        let sig = Self::get_signature(repo)?;

        // Create the initial commit (no parents)
        let commit_id = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])?;

        Ok(commit_id)
    }

    /// Get the git signature from config or use defaults.
    fn get_signature(repo: &Repository) -> Result<Signature<'static>, GitError> {
        // Try to get from config
        if let Ok(config) = repo.config() {
            if let (Ok(name), Ok(email)) = (
                config.get_string("user.name"),
                config.get_string("user.email"),
            ) {
                return Ok(Signature::now(&name, &email)?);
            }
        }

        // Fall back to environment variables
        let name = std::env::var("GIT_AUTHOR_NAME")
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_else(|_| "ai-init".to_string());

        let email = std::env::var("GIT_AUTHOR_EMAIL")
            .unwrap_or_else(|_| format!("{}@localhost", name));

        Ok(Signature::now(&name, &email)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_is_git_repo_false() {
        let temp = TempDir::new().unwrap();
        assert!(!GitOperations::is_git_repo(temp.path()));
    }

    #[test]
    fn test_init_creates_repo() {
        let temp = TempDir::new().unwrap();
        let result = GitOperations::init(temp.path());

        assert!(result.is_ok());
        assert!(GitOperations::is_git_repo(temp.path()));
    }

    #[test]
    fn test_init_fails_if_exists() {
        let temp = TempDir::new().unwrap();

        // First init should succeed
        GitOperations::init(temp.path()).unwrap();

        // Second init should fail
        let result = GitOperations::init(temp.path());
        assert!(matches!(result, Err(GitError::AlreadyExists(_))));
    }

    #[test]
    fn test_init_or_open() {
        let temp = TempDir::new().unwrap();

        // First call creates
        let repo1 = GitOperations::init_or_open(temp.path()).unwrap();
        assert!(GitOperations::is_git_repo(temp.path()));

        // Second call opens
        let repo2 = GitOperations::init_or_open(temp.path()).unwrap();
        assert_eq!(repo1.path(), repo2.path());
    }
}
