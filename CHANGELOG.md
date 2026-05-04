# Changelog

## Version 0.2.0 - Existing Repository Support (2026-05-04)

### New Features

#### 1. Update Existing Repositories (`--update`)
- Added `--update` flag to refresh AI files in existing repositories
- Automatically detects existing git repositories and preserves them
- Skips git re-initialization when repository already exists
- Preserves existing README.md files in update mode
- Intelligently handles existing project structures

**Usage:**
```bash
cd existing-project
ai-init . --update
```

#### 2. Backup Existing Files (`--backup`)
- Added `--backup` flag to create `.bak` files before overwriting
- Backs up CLAUDE.md, TOOLS.md, ARCHITECTURE.md, and CONVENTIONS.md
- Prevents accidental loss of customized AI files
- Works in conjunction with `--update`

**Usage:**
```bash
ai-init . --update --backup
```

#### 3. Clone and Initialize (`--repo`)
- Added `--repo <URL>` flag to clone repositories before initializing
- Supports any Git-compatible URL (GitHub, GitLab, etc.)
- Automatically enables update mode for cloned repos
- Always creates backups when working with cloned repositories
- Adds AI file entries to .gitignore automatically

**Usage:**
```bash
# Clone and add AI files
ai-init my-project --repo https://github.com/user/repository

# Clone with custom settings
ai-init local-copy --repo https://github.com/user/repo --type web --language "TypeScript"
```

#### 4. Smart .gitignore Management
- Automatically adds AI file entries to existing .gitignore
- Entries are commented out by default to give users control
- Prevents duplicate entries when running multiple times
- Only modifies .gitignore when working with existing repos

**Added entries:**
```gitignore
# AI-generated context files
.ai/context/
# Uncomment the lines below to exclude all AI files from git:
# CLAUDE.md
# .ai/
```

### Enhanced Installation

#### Cargo Install Support
- Recommended installation method using `cargo install --path .`
- Binary installs to `~/.cargo/bin/`
- Easy updates with `cargo install --path . --force`

**Installation:**
```bash
git clone https://github.com/yourusername/ai-init.git
cd ai-init
cargo install --path .
```

**Update:**
```bash
cd ai-init
git pull
cargo install --path . --force
```

### Technical Changes

#### Modified Files
- `src/cli.rs`: Added `--update`, `--backup`, and `--repo` flags
- `src/types.rs`: Added `update_mode` and `backup_existing` fields to `ProjectConfig`
- `src/generator.rs`:
  - Added `write_with_backup()` helper function
  - Added `ensure_ai_files_in_gitignore()` for smart .gitignore management
  - Enhanced existing repo detection
  - Improved file creation logic for update mode
- `src/git.rs`: Added `clone()` method for repository cloning
- `src/main.rs`:
  - Added repo cloning logic
  - Enhanced directory existence handling
  - Improved update mode detection
- `src/interactive.rs`: Updated to pass new configuration fields
- `README.md`: Comprehensive documentation updates

#### New Behavior
1. **Update Mode**: When enabled (via `--update`, `--in-place`, or `--repo`):
   - Detects existing git repositories
   - Preserves existing README.md
   - Updates .ai/ directory files
   - Regenerates CLAUDE.md and related files

2. **Backup Mode**: When enabled (via `--backup` or automatically with `--repo`):
   - Creates .bak files before overwriting
   - Provides clear warnings about backed up files
   - Preserves original content for comparison

3. **Repository Cloning**: When using `--repo`:
   - Clones repository first
   - Fails if target directory already exists
   - Automatically enables update and backup modes
   - Adds AI entries to .gitignore

### Breaking Changes
None - All changes are backward compatible. Existing usage patterns continue to work as before.

### Bug Fixes
- Fixed issue where existing directories couldn't be initialized
- Improved error messages for directory conflicts
- Better handling of git repository edge cases

### Documentation Updates
- Added "Working with Existing Repositories" section
- Added "Using Cargo Install" section with update instructions
- Added multiple examples for new features
- Updated command-line help with new examples
- Added .gitignore behavior documentation

### Testing
- Updated all test cases to include new configuration fields
- Tests pass successfully (17/18 passing, 1 pre-existing failure in tools.rs)
- Comprehensive manual testing of new features

## Version 0.1.0 - Initial Release

Initial release with basic project scaffolding functionality.
