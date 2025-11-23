# Rust CLI Task Manager

A powerful terminal-based task management application built with Rust, featuring persistent storage, due dates, search functionality, and beautiful colored output.

## Features

### ✨ Core Functionality
- **Create tasks** with title, description, priority levels, tags, and due dates
- **List tasks** with flexible filtering and sorting options
- **Mark tasks as complete** with timestamp tracking
- **Delete tasks** permanently
- **Edit existing tasks** - update any field including due dates
- **Search tasks** by keyword in title or description
- **View statistics** - completion rates, priority breakdown, and overdue count

### 🎯 Smart Organization
- **Filter by status**: complete, incomplete, or all tasks
- **Filter by priority**: high, medium, or low
- **Filter by tags**: organize with custom tags
- **Filter overdue tasks**: quickly find tasks past their due date
- **Sort options**: by date, priority, or due date

### 💾 Data Management
- **Persistent storage** in JSON format (`~/.task-cli-data.json`)
- **Auto-save** after every modification
- **Instant loading** on startup

### 🎨 Beautiful Display
- **Color-coded priorities**: High (red), Medium (yellow), Low (cyan)
- **Status indicators**: ✓ for completed, ○ for pending
- **Overdue warnings**: Red ⚠ OVERDUE alerts for past-due tasks
- **Clean formatting** with separators and organized layout

## Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/emmanuelist/task-cli.git
cd task-cli

# Build the project
cargo build --release

# Run the application
cargo run -- --help

# Optional: Install globally
cargo install --path .
```

## Usage

### Quick Start

```bash
# Add your first task
task-cli add "Complete project documentation" --priority high

# List all tasks
task-cli list

# View statistics
task-cli stats
```

### Command Reference

#### Add a Task
```bash
task-cli add "Task title" [OPTIONS]

Options:
  -d, --description <TEXT>    Task description
  -p, --priority <LEVEL>      Priority: low, medium, high (default: medium)
  -t, --tags <TAGS>           Comma-separated tags
  -u, --due-date <DATE>       Due date in YYYY-MM-DD format

Examples:
  task-cli add "Fix critical bug" --priority high --due-date 2025-12-31
  task-cli add "Write tests" --priority medium --tags testing,rust
  task-cli add "Research" --description "Look into new framework" --tags learning
```

#### List Tasks
```bash
task-cli list [OPTIONS]

Options:
  -a, --all                   Show all tasks (including completed)
  -f, --filter <FILTER>       Filter by: priority:high, tag:rust, overdue, completed, incomplete
  -s, --sort-by <OPTION>      Sort by: date, priority, due-date (default: priority)

Examples:
  task-cli list                           # Show incomplete tasks
  task-cli list --all                     # Show all tasks
  task-cli list --filter priority:high    # High priority tasks only
  task-cli list --filter overdue          # Show overdue tasks
  task-cli list --sort-by due-date        # Sort by due date
```

#### Complete a Task
```bash
task-cli complete <ID>

Example:
  task-cli complete 3
```

#### Delete a Task
```bash
task-cli delete <ID>

Example:
  task-cli delete 3
```

#### Edit a Task
```bash
task-cli edit <ID> [OPTIONS]

Options:
  -t, --title <TEXT>          New title
  -d, --description <TEXT>    New description
  -p, --priority <LEVEL>      New priority
  --tags <TAGS>               New tags (comma-separated)
  -u, --due-date <DATE>       New due date (YYYY-MM-DD)

Example:
  task-cli edit 3 --title "Updated title" --priority low --due-date 2025-12-15
```

#### Search Tasks
```bash
task-cli search <QUERY>

Example:
  task-cli search "bug"          # Find tasks containing "bug"
  task-cli search "documentation" # Search in titles and descriptions
```

#### View Statistics
```bash
task-cli stats

Shows:
  - Total tasks
  - Completion rate
  - Tasks by priority
  - Overdue count
```

## Examples

### Daily Workflow

```bash
# Morning: Check what's overdue
task-cli list --filter overdue

# Add today's tasks
task-cli add "Team meeting" --priority high --due-date 2025-11-23 --tags work
task-cli add "Review PR" --priority medium --tags code-review,work
task-cli add "Grocery shopping" --priority low --tags personal

# Check all tasks sorted by due date
task-cli list --sort-by due-date

# Complete tasks as you go
task-cli complete 5

# Search for specific tasks
task-cli search "meeting"

# End of day: Check progress
task-cli stats
```

## Technical Details

### Dependencies
- `clap` (v4.5) - Command-line argument parsing with derive macros
- `serde` + `serde_json` (v1.0) - JSON serialization/deserialization
- `chrono` (v0.4) - Date and time handling
- `colored` (v2.1) - Terminal color output
- `anyhow` (v1.0) - Error handling
- `dirs` (v5.0) - Home directory detection

### Project Structure
```
src/
├── main.rs          # CLI setup with clap, command routing
├── task.rs          # Task struct, Priority enum, business logic
├── storage.rs       # JSON file I/O operations
├── commands.rs      # Command implementations (add, list, edit, etc.)
└── display.rs       # Pretty-printing with colors
```

### Data Storage
Tasks are stored as JSON in `~/.task-cli-data.json`:

```json
[
  {
    "id": 1,
    "title": "Example task",
    "description": "Task details",
    "priority": "High",
    "tags": ["rust", "cli"],
    "completed": false,
    "created_at": "2025-11-23T10:30:00Z",
    "completed_at": null,
    "due_date": "2025-12-31"
  }
]
```

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Submit pull requests

## License

MIT License - feel free to use this project however you'd like!

## Acknowledgments

Built with ❤️ using Rust and inspired by modern CLI tools like `exa`, `bat`, and `ripgrep`.
