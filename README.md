# Rust CLI Task Manager

A terminal-based task management application built with Rust, featuring persistent storage, filtering, and a clean CLI interface.

## Project Overview

Build a command-line task manager that allows users to create, update, delete, and organize tasks with tags and priorities. Tasks should persist between sessions using JSON file storage.

## Core Features

### Task Management
- Create tasks with title, description, priority (low/medium/high), and optional tags
- List all tasks with filtering options
- Mark tasks as complete/incomplete
- Delete tasks
- Edit existing tasks

### Filtering & Organization
- Filter tasks by status (complete/incomplete/all)
- Filter by priority level
- Filter by tags
- Sort tasks by creation date, priority, or due date

### Data Persistence
- Store tasks in a JSON file in the user's home directory
- Load tasks on startup
- Auto-save after each modification

## Technical Requirements

### Dependencies (suggested)
- `clap` (v4.x) - Command-line argument parsing
- `serde` + `serde_json` - JSON serialization
- `chrono` - Date/time handling
- `colored` - Terminal output coloring
- `anyhow` - Error handling

### Project Structure
```
src/
├── main.rs          # Entry point, CLI setup
├── task.rs          # Task struct and methods
├── storage.rs       # File I/O operations
├── commands.rs      # Command handlers
└── display.rs       # Pretty-printing utilities
```

## Command Interface

```bash
# Add a task
task-cli add "Implement storage module" --priority high --tags rust,backend

# List tasks
task-cli list                        # All incomplete tasks
task-cli list --all                  # All tasks
task-cli list --filter priority:high # High priority tasks
task-cli list --filter tag:rust      # Tasks with 'rust' tag

# Complete a task
task-cli complete 3

# Delete a task
task-cli delete 3

# Edit a task
task-cli edit 3 --title "New title" --priority low
```

## Data Model

```rust
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    priority: Priority,
    tags: Vec<String>,
    completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

enum Priority {
    Low,
    Medium,
    High,
}
```

## Implementation Milestones

1. **Basic Structure** - Set up project, define Task struct, implement basic CRUD operations in memory
2. **CLI Interface** - Integrate clap for command parsing
3. **Persistence** - Implement JSON storage with proper error handling
4. **Filtering** - Add filtering and sorting capabilities
5. **Polish** - Add colored output, better error messages, input validation

## Stretch Goals

- Add due dates and overdue task highlighting
- Interactive mode with arrow key navigation
- Export tasks to different formats (CSV, Markdown)
- Task statistics (completion rate, time tracking)
- Recurring tasks

## Success Criteria

- All commands work as specified
- Tasks persist correctly between program runs
- Clean error messages for invalid inputs
- Code is well-organized and follows Rust best practices
- Basic tests for core functionality

## Getting Started

```bash
cargo new task-cli
cd task-cli
# Add dependencies to Cargo.toml
cargo build
cargo run -- add "First task"
```
