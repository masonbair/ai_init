# ai-init

AI-ready project initializer for efficient agent workflows.

## Overview

`ai-init` is a command-line tool that scaffolds new projects with AI agent context files, tool registries, and project metadata. It solves the problem of large context windows by pre-configuring projects so AI agents immediately understand available tooling, architecture patterns, and coding conventions.

### The Problem

When working with AI agents on large projects, agents often struggle with:
- Understanding what custom tools are available in the development environment
- Finding relevant architecture documentation spread across multiple files
- Learning project-specific conventions and patterns
- Consuming excessive context tokens to understand basic project structure

### The Solution

`ai-init` creates a standardized `.ai/` directory structure that gives AI agents instant access to:
- **Tool Registry** (`TOOLS.md`) - Comprehensive list of available development tools
- **Architecture Documentation** (`ARCHITECTURE.md`) - System design and component relationships
- **Coding Conventions** (`CONVENTIONS.md`) - Project-specific patterns and standards
- **Agent Instructions** (`CLAUDE.md`) - Project-level guidance for AI agents

This reduces context consumption and improves AI agent effectiveness from the first interaction.

## Features

- **Interactive Project Setup** - Guided prompts for project configuration
- **Automatic Tool Detection** - Scans your system for AI-assistant tools (code-summarizer, context-query, etc.)
- **Template-Based Generation** - Customizable Tera templates for all generated files
- **Git Integration** - Optional repository initialization with initial commit
- **Dry Run Mode** - Preview what will be created without making changes
- **Non-Interactive Mode** - Scriptable project creation for automation
- **Custom Tool Paths** - Override tool locations for non-standard installations
- **Configuration File Support** - Save preferences in `~/.config/ai-init/config.toml`

## Installation

### Prerequisites

- Rust 1.70 or later

### From Source

```bash
git clone https://github.com/yourusername/ai-init.git
cd ai-init
cargo build --release
sudo cp target/release/ai-init /usr/local/bin/
```

### Verify Installation

```bash
ai-init --version
```

## Quick Start

### Create a New Project

```bash
ai-init my-awesome-project
```

This launches an interactive prompt to configure your project.

### Non-Interactive Mode

```bash
ai-init my-project \
  --type web \
  --language "Rust,TypeScript" \
  --description "A web application for task management" \
  --no-interactive
```

### Initialize Current Directory

```bash
ai-init . --in-place
```

### Preview Changes (Dry Run)

```bash
ai-init my-project --dry-run
```

## Usage

### Basic Command Structure

```bash
ai-init [OPTIONS] <PROJECT>
```

### Options

| Option | Description |
|--------|-------------|
| `<PROJECT>` | Project directory name or path (use `.` for current directory) |
| `-t, --type <TYPE>` | Project type: web, cli, library, system, mixed |
| `-l, --language <LANGS>` | Programming languages (comma-separated) |
| `-d, --description <DESC>` | Project description |
| `--no-interactive` | Skip prompts, use defaults or provided values |
| `--dry-run` | Preview without creating files |
| `--no-git` | Skip git repository initialization |
| `--no-readme` | Skip README.md creation |
| `--initial-commit` | Create initial git commit after setup |
| `--in-place` | Initialize in existing directory |
| `--tool-path <TOOL=PATH>` | Custom tool path (e.g., `code-summarizer=/usr/local/bin/summarizer`) |

### Examples

**Web application with TypeScript and Rust:**
```bash
ai-init web-app --type web --language "TypeScript,Rust" --description "Full-stack web application"
```

**CLI tool with initial commit:**
```bash
ai-init cli-tool --type cli --language "Rust" --initial-commit
```

**Library project without README:**
```bash
ai-init math-lib --type library --language "Python" --no-readme
```

**Custom tool path override:**
```bash
ai-init my-project --tool-path "code-summarizer=/home/user/bin/my-summarizer"
```

## Generated Project Structure

After running `ai-init`, your project will have the following structure:

```
my-project/
├── .ai/
│   ├── TOOLS.md              # Registry of available development tools
│   ├── ARCHITECTURE.md       # System architecture documentation
│   ├── CONVENTIONS.md        # Coding standards and patterns
│   └── tool-manifests/       # Tool-specific configuration files
├── CLAUDE.md                 # AI agent instructions for this project
├── README.md                 # Project README (if --no-readme not set)
├── .gitignore                # Git ignore patterns
└── .git/                     # Git repository (if --no-git not set)
```

### Key Files

**CLAUDE.md** - Top-level instructions for AI agents working on your project. References the `.ai/` directory for additional context.

**.ai/TOOLS.md** - Comprehensive registry of detected tools with:
- Tool name and binary path
- Description and purpose
- Usage examples
- Installation status

**.ai/ARCHITECTURE.md** - Placeholder for system architecture documentation:
- Component relationships
- Data flow diagrams
- Technology stack
- Deployment architecture

**.ai/CONVENTIONS.md** - Project-specific coding standards:
- Naming conventions
- File organization patterns
- Testing requirements
- Documentation standards

## Configuration

`ai-init` supports a configuration file at `~/.config/ai-init/config.toml`:

```toml
[project]
default_type = "mixed"
default_languages = ["Rust"]
init_git = true
create_readme = true

[tools]
# Custom tool paths that override system detection
[tools.custom_paths]
code-summarizer = "/usr/local/bin/my-summarizer"
context-query = "/home/user/bin/cquery"

[templates]
# Custom template directory (optional)
# template_dir = "/home/user/.config/ai-init/templates"
```

### Custom Templates

You can override default templates by creating custom Tera templates in your template directory. Supported templates:
- `README.md.tera`
- `CLAUDE.md.tera`
- `TOOLS.md.tera`
- `ARCHITECTURE.md.tera`
- `CONVENTIONS.md.tera`
- `gitignore.tera`

## Development

### Building from Source

```bash
git clone https://github.com/yourusername/ai-init.git
cd ai-init
cargo build
```

### Running Tests

```bash
cargo test
```

### Project Structure

- `src/main.rs` - Entry point and orchestration
- `src/cli.rs` - Command-line argument parsing
- `src/config.rs` - Configuration file handling
- `src/generator.rs` - Project generation logic
- `src/tools.rs` - Tool detection and registry
- `src/templates.rs` - Template rendering
- `src/interactive.rs` - Interactive prompts
- `src/git.rs` - Git operations
- `src/types.rs` - Shared data structures
- `templates/` - Tera template files
- `tests/` - Integration tests

### Code Quality

This project follows industry-standard Rust practices:
- Comprehensive error handling using `thiserror`
- Unit tests for critical logic
- Integration tests for end-to-end workflows
- Documentation comments on public APIs
- Clippy lints for code quality

### Contributing

Contributions are welcome. Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass (`cargo test`)
5. Run `cargo clippy` and fix any warnings
6. Submit a pull request

## Related Tools

`ai-init` is designed to work with other AI-assistant tools:
- **code-summarizer** - Generates concise summaries of codebases
- **context-query** - Retrieves relevant code context for AI agents
- **code-index** - Builds searchable indexes of code repositories
- **context-packer** - Optimizes context for token efficiency

These tools are detected automatically if installed in your `$PATH`.

## License

MIT

## Author

Mason

## Acknowledgments

Built as part of a research project to optimize AI agent workflows and reduce context window consumption in large software projects.
