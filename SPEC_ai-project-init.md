# Tool Specification: `ai-init` (AI-Ready Project Initializer)

## Overview

`ai-init` is a command-line tool for Arch Linux that creates project directories pre-configured for AI agent workflows. It scaffolds AI context files, tool registries, and project metadata so that AI agents immediately understand available tooling and project structure when working in the directory.

**Problem Solved:** Eliminates the need to manually create and explain custom tooling to AI agents on every new project. The agent "just knows" what's available.

---

## Command Name Options

Recommended: **`ai-init`** (semantic, clear action)

Alternatives considered:
- `ai-mkdir` - User's original idea, very clear
- `aiproj` - Shorter, punchy
- `ai-scaffold` - Descriptive but verbose
- `mkaiproj` - Unix-style but less readable

**Decision: Use `ai-init` in spec, but support aliases**

---

## Core Functionality

### Basic Usage

```bash
# Create new AI-ready project directory
ai-init myproject

# Specify project type (future: uses templates)
ai-init myproject --type web

# Initialize in existing directory
cd existing-project
ai-init .

# Dry run to preview what will be created
ai-init myproject --dry-run

# Non-interactive mode (use defaults)
ai-init myproject --no-interactive
```

### What It Creates

```
myproject/
├── CLAUDE.md                    # AI instructions & project overview (root level)
├── .ai/                         # Hidden directory for AI context files
│   ├── TOOLS.md                 # Registry of available custom tools
│   ├── ARCHITECTURE.md          # Project architecture (template/placeholder)
│   ├── CONVENTIONS.md           # Coding standards, patterns
│   └── context/                 # Auto-generated context (for future tools)
├── .gitignore                   # Pre-configured with AI context rules
├── .git/                        # Initialized git repository
└── README.md                    # Standard project README (optional)
```

---

## Detailed Feature Specifications

### Feature 1: Interactive Project Setup

When run interactively, `ai-init` asks questions to customize the generated files:

```
🤖 AI Project Initializer

Project name: myproject
Description: A web application for task management
Primary language(s): TypeScript, Python
Project type: [web/cli/library/system/mixed]: web
Include README.md? [Y/n]: y
Git initialization? [Y/n]: y

✓ Created myproject/
✓ Generated CLAUDE.md with project context
✓ Created .ai/ directory structure
✓ Registered 4 available tools in .ai/TOOLS.md
✓ Initialized git repository
✓ Created .gitignore

Next steps:
  cd myproject
  # Start coding - AI agents now know about your tooling!
```

**Captured Information:**
- Project name
- Description/purpose
- Primary languages
- Project type (for future template selection)
- README inclusion preference
- Git initialization preference

**Output:** Customized CLAUDE.md with project-specific context

---

### Feature 2: CLAUDE.md Generation

Generated CLAUDE.md structure:

```markdown
# {PROJECT_NAME}

## Project Overview

{USER_PROVIDED_DESCRIPTION}

**Primary Languages:** {LANGUAGES}
**Project Type:** {TYPE}
**Created:** {DATE}

---

## AI Agent Instructions

### Available Custom Tooling

This project is configured with custom AI-agent tooling. **Before starting work, read `.ai/TOOLS.md`** to understand available commands.

Quick reference:
- Context generation: See `.ai/TOOLS.md` for CodeSummarizer usage
- Code search: See `.ai/TOOLS.md` for ContextQuery usage
- Architecture: Read `.ai/ARCHITECTURE.md` for system design

### Project Conventions

See `.ai/CONVENTIONS.md` for:
- Code style guidelines
- Architecture patterns
- Testing requirements
- Documentation standards

### Context Files

The `.ai/` directory contains AI-optimized context:
- `TOOLS.md` - Available custom tooling
- `ARCHITECTURE.md` - System design and structure
- `CONVENTIONS.md` - Project-specific conventions
- `context/` - Auto-generated context files (created by tools)

**IMPORTANT:** Always check these files before starting implementation work.

---

## Development Workflow

1. Read `.ai/ARCHITECTURE.md` to understand system structure
2. Use custom tools (see `.ai/TOOLS.md`) for context gathering
3. Follow conventions in `.ai/CONVENTIONS.md`
4. Update architecture docs when making structural changes

---

## For AI Agents: Tool Discovery

Available custom tools are registered in `.ai/TOOLS.md`. These tools are designed to help you:
- **Understand code faster** (semantic search, AST analysis)
- **Use context efficiently** (summarization, smart packing)
- **Navigate large codebases** (dependency graphs, call hierarchies)

Read `.ai/TOOLS.md` FIRST before performing any code analysis or context gathering.
```

**Dynamic Sections:**
- Project name, description, languages, type from interactive prompts
- Date stamp
- Tool availability (checks if custom tools are installed)

---

### Feature 3: TOOLS.md Registry

`.ai/TOOLS.md` is a **registry of available custom tooling** on the system. It's auto-generated based on what's actually installed.

**Detection Logic:**
```bash
# Check if custom tools are in PATH
which code-summarizer >/dev/null 2>&1 && echo "CodeSummarizer: AVAILABLE"
which context-query >/dev/null 2>&1 && echo "ContextQuery: AVAILABLE"
which code-index >/dev/null 2>&1 && echo "CodeIndex: AVAILABLE"
which context-packer >/dev/null 2>&1 && echo "ContextPacker: AVAILABLE"
```

**Generated TOOLS.md Structure:**

```markdown
# Available AI Agent Tools

**Auto-generated:** {DATE}
**System:** Arch Linux
**Tools Detected:** 4 of 4 installed

This file is a registry of custom tools available on this system for AI agent workflows.

---

## Tool Status

✅ **CodeSummarizer** - Installed at `/usr/local/bin/code-summarizer`
✅ **ContextQuery** - Installed at `/usr/local/bin/context-query`
✅ **CodeIndex** - Installed at `/usr/local/bin/code-index`
✅ **ContextPacker** - Installed at `/usr/local/bin/context-packer`

---

## Tool Descriptions & Usage

### 1. CodeSummarizer

**Purpose:** Generates hierarchical context maps of the codebase for AI agents.

**Usage:**
```bash
code-summarizer --project-root . --output .ai/context/
```

**Output:**
- `.ai/context/ARCHITECTURE.md` - High-level system design
- `.ai/context/MODULE_MAPS/` - Per-module breakdowns
- `.ai/context/DEPENDENCY_GRAPH.md` - Import/call relationships

**When to use:** At project start, after major refactors, or when context feels stale.

**AI Agent Note:** Run this BEFORE doing broad codebase analysis to get efficient, structured context.

---

### 2. ContextQuery

**Purpose:** Structure-aware code search combining text, AST patterns, and graph traversal.

**Usage:**
```bash
# Find all async functions calling database
context-query --pattern "async function.*database" --type structural

# Find all callers of a function
context-query --symbol "authenticateUser" --show-callers

# Combine text and structure
context-query --text "TODO" --file-type "*.ts" --scope src/
```

**Output:** JSON with code snippets, file:line locations, relevance scores, dependency info.

**AI Agent Note:** Use this instead of basic grep/ripgrep for code search - it understands structure.

---

### 3. CodeIndex

**Purpose:** Persistent semantic cache for AI agents - indexes codebases with tree-sitter for fast symbol lookup, dependency analysis, and code intelligence.

**Usage:**
```bash
# Daemon management (background indexer with file watching)
code-index daemon start           # Start the indexing daemon
code-index daemon stop            # Stop the daemon
code-index daemon status          # Check if daemon is running
code-index daemon restart         # Restart the daemon

# One-time indexing (without daemon)
code-index index /path/to/project

# Query the index
code-index query symbol "UserController"      # Find symbols by name
code-index query file src/auth/login.ts       # Get all symbols in a file
code-index query dependencies src/auth/login.ts  # Get file dependencies
code-index query hot-files                    # Get frequently changed/complex files
code-index query kind function                # List all symbols of a specific kind

# Index management
code-index stats                  # Show index statistics
code-index reindex                # Re-index from scratch
code-index clear                  # Clear the entire index
code-index export                 # Export index to JSON

# Output formatting
code-index query symbol "Foo" --json          # JSON output
code-index query symbol "Foo" --format compact  # Compact format
```

**Output:** Human-readable or JSON format with symbols, dependencies, file metadata, and code locations.

**AI Agent Note:** This is the backend for ContextQuery and CodeSummarizer. Start the daemon once per system/workspace for continuous indexing, or use one-time `index` command for quick lookups.

---

### 4. ContextPacker

**Purpose:** Smart context window packing - assembles relevant code within token budget.

**Usage:**
```bash
# Pack context for a specific task
context-packer --query "implement user authentication" --budget 8000 --format claude

# For a specific file
context-packer --file src/auth/login.ts --budget 5000 --include-callers
```

**Output:** Pre-formatted context optimized for your token budget and target model.

**AI Agent Note:** Use this when you need to understand a feature but want to stay within token limits.

---

## Best Practices for AI Agents

1. **Start with CodeSummarizer:** Run it first to get high-level context (~200 tokens)
2. **Use ContextQuery for specifics:** Drill down to specific code with structure-aware search
3. **Let ContextPacker manage tokens:** When context budget is tight, use it to prioritize
4. **Trust the index:** CodeIndex is faster than re-parsing - use it for symbol/dependency lookups

---

## Tool Installation Status

If any tools show as "NOT INSTALLED", they can be built from specs in:
- `~/ai-agent-tooling-specs/` (if you have the spec documents)
- Or request installation instructions from the user

**Current Status:** All core tools installed ✅
```

**Key Features:**
- **Auto-detection:** Checks PATH for tools, marks as installed/missing
- **Usage examples:** Shows AI agents HOW to use each tool
- **Best practices:** Guides agents on WHEN to use each tool
- **Extensible:** Easy to add new tools to registry

---

### Feature 4: Git Integration

When git initialization is enabled:

1. **Run `git init`** in the project directory
2. **Create `.gitignore`** with AI-context rules:

```gitignore
# AI-generated context (regenerate on demand)
.ai/context/

# OS files
.DS_Store
Thumbs.db

# Editor files
.vscode/
.idea/
*.swp
*.swo
*~

# Environment
.env
.env.local

# Dependencies (language-specific, added based on project type)
node_modules/
__pycache__/
*.pyc
target/
dist/
build/
```

3. **Initial commit** (optional flag `--initial-commit`):
```bash
git add .
git commit -m "Initial commit: AI-ready project structure"
```

**Configuration:**
- User can disable git init with `--no-git`
- `.gitignore` rules are extensible based on project type

---

### Feature 5: Placeholder Architecture Files

Generated `.ai/ARCHITECTURE.md` (template for later filling):

```markdown
# {PROJECT_NAME} - Architecture

**Status:** Template - Update as architecture evolves
**Last Updated:** {DATE}

---

## System Overview

{PLACEHOLDER: High-level description of system architecture}

**Key Components:**
- Component 1: {PLACEHOLDER}
- Component 2: {PLACEHOLDER}

---

## Directory Structure

```
{PROJECT_NAME}/
├── {PLACEHOLDER: Describe main directories}
└── ...
```

---

## Data Flow

{PLACEHOLDER: Describe how data moves through the system}

---

## Key Decisions

### {Decision 1}
- **Rationale:** {PLACEHOLDER}
- **Alternatives Considered:** {PLACEHOLDER}
- **Trade-offs:** {PLACEHOLDER}

---

## Dependencies

### External Services
- {PLACEHOLDER: APIs, databases, third-party services}

### Internal Dependencies
- {PLACEHOLDER: Module relationships}

---

## For AI Agents

**Context Generation:** Run `code-summarizer` to auto-populate sections of this file.

**When to update:** After major architectural changes, new service integrations, or refactors.
```

Similar template for `.ai/CONVENTIONS.md`:

```markdown
# {PROJECT_NAME} - Conventions

**Purpose:** Project-specific coding standards and patterns
**Last Updated:** {DATE}

---

## Code Style

{PLACEHOLDER: Language-specific style guide}

---

## Architecture Patterns

{PLACEHOLDER: Design patterns used in this project}

---

## Testing Requirements

{PLACEHOLDER: Testing standards, coverage requirements}

---

## Documentation Standards

{PLACEHOLDER: How to document code in this project}

---

## For AI Agents

Follow these conventions when writing code. If unclear, ask the user.
```

---

### Feature 6: Dry Run Mode

```bash
ai-init myproject --dry-run
```

**Output:**
```
🤖 AI Project Initializer (DRY RUN)

Would create:
  myproject/
  myproject/CLAUDE.md (723 bytes)
  myproject/.ai/
  myproject/.ai/TOOLS.md (2.1 KB)
  myproject/.ai/ARCHITECTURE.md (891 bytes)
  myproject/.ai/CONVENTIONS.md (654 bytes)
  myproject/.ai/context/ (directory)
  myproject/.gitignore (234 bytes)

Would initialize:
  Git repository in myproject/

Total: 7 files, 2 directories, 4.7 KB

Run without --dry-run to create project.
```

---

## Configuration

### System-Wide Config

Location: `~/.config/ai-init/config.toml`

```toml
[defaults]
# Default behavior for interactive prompts
git_init = true
create_readme = true
initial_commit = false

[paths]
# Where to look for templates (future feature)
templates_dir = "~/.config/ai-init/templates"

[tools]
# Custom tool paths (if not in PATH)
code_summarizer = "/usr/local/bin/code-summarizer"
context_query = "/usr/local/bin/context-query"
code_index = "/usr/local/bin/code-index"
context_packer = "/usr/local/bin/context-packer"

[generation]
# CLAUDE.md generation options
include_tool_registry = true
include_architecture_template = true
include_conventions_template = true
```

**User can override with flags:**
```bash
ai-init myproject --no-readme --no-git
```

---

## Technical Implementation

### Language

**Recommendation: Rust**

**Rationale:**
- Fast execution (important for CLI tools)
- Excellent string handling for template rendering
- Cross-compilation support (future: beyond Arch)
- Native Arch support (`cargo` is in official repos)
- Great CLI libraries (`clap`, `dialoguer`, `tera`)

**Alternative: Python** (if Rust feels like overkill)
- Faster to develop
- Still good performance for this use case
- Excellent templating (`jinja2`)
- User might already have Python workflow

### Key Libraries (Rust)

```toml
[dependencies]
clap = "4.5"              # CLI argument parsing
dialoguer = "0.11"         # Interactive prompts
tera = "1.19"              # Template rendering (like Jinja2)
serde = "1.0"              # Serialization for config
toml = "0.8"               # Config file parsing
which = "6.0"              # Tool detection in PATH
git2 = "0.18"              # Git operations
chrono = "0.4"             # Date/time for timestamps
colored = "2.1"            # Colored terminal output
```

### Project Structure

```
ai-init/
├── Cargo.toml
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── cli.rs                  # Argument parsing (clap)
│   ├── interactive.rs          # Interactive prompts (dialoguer)
│   ├── generator.rs            # File generation logic
│   ├── templates.rs            # Template rendering (tera)
│   ├── git.rs                  # Git operations (git2)
│   ├── tools.rs                # Tool detection & registry
│   ├── config.rs               # Config file handling
│   └── types.rs                # Shared types/structs
├── templates/                  # Embedded templates
│   ├── CLAUDE.md.tera
│   ├── TOOLS.md.tera
│   ├── ARCHITECTURE.md.tera
│   ├── CONVENTIONS.md.tera
│   └── gitignore.tera
├── README.md
└── tests/
    └── integration_tests.rs
```

### Template System

Using **Tera** (Rust's Jinja2-like templating):

**Example: CLAUDE.md.tera**
```markdown
# {{ project_name }}

## Project Overview

{{ description }}

**Primary Languages:** {{ languages | join(sep=", ") }}
**Project Type:** {{ project_type }}
**Created:** {{ created_date }}

---

## AI Agent Instructions

### Available Custom Tooling

This project is configured with custom AI-agent tooling. **Before starting work, read `.ai/TOOLS.md`** to understand available commands.

{% if tools_available > 0 %}
Quick reference:
{% if tool_code_summarizer %}- Context generation: See `.ai/TOOLS.md` for CodeSummarizer usage{% endif %}
{% if tool_context_query %}- Code search: See `.ai/TOOLS.md` for ContextQuery usage{% endif %}
{% if tool_code_index %}- Symbol lookup: See `.ai/TOOLS.md` for CodeIndex usage{% endif %}
{% if tool_context_packer %}- Context packing: See `.ai/TOOLS.md` for ContextPacker usage{% endif %}
{% else %}
No custom tools detected. See user for installation instructions.
{% endif %}

...
```

**Variables passed to templates:**
```rust
struct ProjectContext {
    project_name: String,
    description: String,
    languages: Vec<String>,
    project_type: String,
    created_date: String,
    tools_available: usize,
    tool_code_summarizer: bool,
    tool_context_query: bool,
    tool_code_index: bool,
    tool_context_packer: bool,
}
```

---

## Installation

### Via Cargo (Rust)

```bash
# Clone repo (or download release)
git clone https://github.com/yourusername/ai-init.git
cd ai-init

# Build and install
cargo build --release
sudo cp target/release/ai-init /usr/local/bin/

# Or use cargo install
cargo install --path .
```

### Via AUR (Arch Linux Package)

Create AUR package `ai-init`:

```bash
# PKGBUILD
pkgname=ai-init
pkgver=0.1.0
pkgrel=1
pkgdesc="AI-ready project initializer for agent workflows"
arch=('x86_64')
url="https://github.com/yourusername/ai-init"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
```

Users install with:
```bash
yay -S ai-init
# or
paru -S ai-init
```

---

## Usage Examples

### Example 1: Quick New Project

```bash
ai-init my-web-app
# Interactive prompts appear
# Fills in details, creates structure
cd my-web-app
# AI agent now knows about tooling!
```

### Example 2: Initialize Existing Project

```bash
cd existing-project
ai-init .
# Adds AI context files without changing existing code
```

### Example 3: Non-Interactive (Automation/Scripts)

```bash
ai-init my-lib \
  --no-interactive \
  --type library \
  --language "Rust" \
  --description "A utility library" \
  --no-readme
```

### Example 4: Custom Tool Setup

```bash
# User has custom tools in non-standard location
ai-init my-project \
  --tool-path code-summarizer=/home/user/bin/summarizer \
  --tool-path context-query=/home/user/bin/cquery
```

---

## Future Enhancements (Extensibility Design)

### 1. Template System

Support multiple project templates:

```bash
ai-init my-web-app --template react-typescript
ai-init my-cli --template rust-cli
ai-init my-lib --template python-library
```

**Template Structure:**
```
~/.config/ai-init/templates/
├── react-typescript/
│   ├── template.toml          # Template metadata
│   ├── CLAUDE.md.tera
│   ├── ARCHITECTURE.md.tera
│   └── scaffold/              # Additional files to create
│       ├── src/
│       ├── package.json
│       └── tsconfig.json
├── rust-cli/
│   └── ...
└── python-library/
    └── ...
```

**template.toml:**
```toml
[metadata]
name = "React TypeScript"
description = "Modern React app with TypeScript"
languages = ["TypeScript", "JavaScript"]

[files]
create = ["src/", "public/", "package.json", "tsconfig.json"]

[prompts]
# Custom prompts for this template
state_management = ["Redux", "Zustand", "Context API"]
```

### 2. Plugin System

Allow users to add custom tools to registry:

```bash
ai-init --register-tool my-custom-tool \
  --bin /path/to/tool \
  --description "My custom AI tool"
```

Adds to `~/.config/ai-init/tools.toml`:
```toml
[[custom_tools]]
name = "my-custom-tool"
bin = "/path/to/tool"
description = "My custom AI tool"
usage = "my-custom-tool --help"
```

Next `ai-init` run includes it in TOOLS.md.

### 3. Cloud Sync

Sync templates and configs across machines:

```bash
ai-init --sync-config
# Uploads ~/.config/ai-init/ to user's cloud (git repo, Dropbox, etc.)
```

### 4. Project Type Detection

Auto-detect project type from existing files:

```bash
cd existing-project-with-package-json
ai-init .
# Detects: "Found package.json - looks like a Node.js project. Use 'node' template? [Y/n]"
```

---

## Testing Strategy

### Unit Tests

Test individual components:
- Template rendering with various inputs
- Tool detection logic
- Config file parsing
- Git operations

### Integration Tests

Test full workflows:
```rust
#[test]
fn test_full_project_creation() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");

    // Run ai-init with test inputs
    let result = run_ai_init(
        &project_path,
        ProjectConfig {
            name: "test-project",
            description: "Test",
            languages: vec!["Rust"],
            interactive: false,
            // ...
        }
    );

    assert!(result.is_ok());
    assert!(project_path.join("CLAUDE.md").exists());
    assert!(project_path.join(".ai/TOOLS.md").exists());
    assert!(project_path.join(".git").exists());
}
```

### Manual Testing Checklist

- [ ] Create new project interactively
- [ ] Create new project non-interactively
- [ ] Initialize in existing directory
- [ ] Dry run mode
- [ ] Git initialization on/off
- [ ] Tool detection (with/without tools installed)
- [ ] Custom config file
- [ ] Template rendering with various languages
- [ ] Error handling (permissions, existing files, invalid paths)

---

## Error Handling

### Graceful Failures

- **Directory exists:** Ask to overwrite or initialize in-place
- **Git already initialized:** Skip git init, warn user
- **No write permissions:** Clear error message with suggested fix
- **Tool detection fails:** Continue anyway, mark tools as "NOT INSTALLED"
- **Config file malformed:** Use defaults, warn user

### Error Messages

```bash
# Example: Directory exists
Error: Directory 'myproject' already exists.

Options:
  1. Use a different name
  2. Initialize in existing directory: ai-init myproject --in-place
  3. Remove and recreate: rm -rf myproject && ai-init myproject
```

---

## Documentation

### README.md

Include in project repo:
- Installation instructions
- Quick start examples
- Configuration guide
- Template development guide (for extensibility)
- Contributing guidelines

### Man Page

Install man page for `man ai-init`:

```bash
sudo cp docs/ai-init.1 /usr/share/man/man1/
sudo mandb
```

---

## Success Metrics

### Measurable Goals

1. **Time Savings:** Reduce setup time from ~15 minutes (manual) to <30 seconds
2. **Consistency:** 100% of projects have proper AI context files
3. **Adoption:** Tool used in >80% of new projects
4. **Token Efficiency:** AI agents use 50% fewer tokens on context gathering in ai-init projects

### Validation

- Track number of projects created with `ai-init`
- Survey: "Did the AI agent find the TOOLS.md useful?"
- Compare token usage in projects with/without ai-init setup

---

## Security Considerations

1. **No secrets in generated files:** Ensure templates never include API keys, passwords
2. **Respect .gitignore:** Auto-generated context should be in .gitignore by default
3. **Safe defaults:** Don't auto-commit without explicit flag
4. **Input validation:** Sanitize project names, descriptions (no shell injection)
5. **Permission checks:** Fail gracefully if can't write to directory

---

## Dependencies on Other Tools

- **Independent:** `ai-init` works standalone, even if custom tools (CodeSummarizer, etc.) aren't installed yet
- **Discovery:** Detects and registers tools when they become available
- **Graceful degradation:** TOOLS.md marks tools as "NOT INSTALLED" if missing

**Build Order:**
1. Build `ai-init` first (no dependencies)
2. Build other tools (CodeSummarizer, etc.) later
3. Re-run `ai-init --update` in existing projects to refresh TOOLS.md

---

## Summary

`ai-init` is a foundational tool that:
- ✅ Creates AI-ready project directories with proper context scaffolding
- ✅ Auto-registers available custom tools
- ✅ Provides templates for architecture and conventions docs
- ✅ Integrates with git
- ✅ Works standalone, enhances other tools
- ✅ Extensible for future templates and plugins
- ✅ Fast, native Arch Linux support

**Next Steps:**
1. Build `ai-init` (Rust implementation)
2. Test with real projects
3. Use it to create projects for building other tools (CodeSummarizer, etc.)
4. Iterate based on real-world usage

---

## Appendix: Example Generated Files

### Full CLAUDE.md Example

```markdown
# TaskMaster Pro

## Project Overview

A web application for task management with real-time collaboration

**Primary Languages:** TypeScript, Python
**Project Type:** web
**Created:** 2026-04-21

---

## AI Agent Instructions

### Available Custom Tooling

This project is configured with custom AI-agent tooling. **Before starting work, read `.ai/TOOLS.md`** to understand available commands.

Quick reference:
- Context generation: See `.ai/TOOLS.md` for CodeSummarizer usage
- Code search: See `.ai/TOOLS.md` for ContextQuery usage
- Architecture: Read `.ai/ARCHITECTURE.md` for system design

### Project Conventions

See `.ai/CONVENTIONS.md` for:
- Code style guidelines
- Architecture patterns
- Testing requirements
- Documentation standards

### Context Files

The `.ai/` directory contains AI-optimized context:
- `TOOLS.md` - Available custom tooling
- `ARCHITECTURE.md` - System design and structure
- `CONVENTIONS.md` - Project-specific conventions
- `context/` - Auto-generated context files (created by tools)

**IMPORTANT:** Always check these files before starting implementation work.

---

## Development Workflow

1. Read `.ai/ARCHITECTURE.md` to understand system structure
2. Use custom tools (see `.ai/TOOLS.md`) for context gathering
3. Follow conventions in `.ai/CONVENTIONS.md`
4. Update architecture docs when making structural changes

---

## For AI Agents: Tool Discovery

Available custom tools are registered in `.ai/TOOLS.md`. These tools are designed to help you:
- **Understand code faster** (semantic search, AST analysis)
- **Use context efficiently** (summarization, smart packing)
- **Navigate large codebases** (dependency graphs, call hierarchies)

Read `.ai/TOOLS.md` FIRST before performing any code analysis or context gathering.
```

### Full .gitignore Example

```gitignore
# AI-generated context (regenerate on demand)
.ai/context/

# OS files
.DS_Store
Thumbs.db

# Editor files
.vscode/
.idea/
*.swp
*.swo
*~

# Environment
.env
.env.local

# Node.js (detected from project type)
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Python (detected from languages)
__pycache__/
*.py[cod]
*$py.class
.Python
venv/
ENV/

# Build outputs
dist/
build/
*.egg-info/

# Testing
.coverage
htmlcov/
.pytest_cache/
```

---

**End of Specification**
