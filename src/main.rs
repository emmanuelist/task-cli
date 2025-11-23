mod commands;
mod display;
mod storage;
mod task;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task-cli")]
#[command(about = "A terminal-based task management application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,

        /// Task description
        #[arg(short, long)]
        description: Option<String>,

        /// Priority level (low, medium, high)
        #[arg(short, long)]
        priority: Option<String>,

        /// Tags (comma-separated or multiple --tags)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Due date (YYYY-MM-DD format)
        #[arg(short = 'u', long)]
        due_date: Option<String>,
    },

    /// List tasks
    List {
        /// Show all tasks including completed ones
        #[arg(short, long)]
        all: bool,

        /// Filter tasks (e.g., priority:high, tag:rust, completed, incomplete, overdue)
        #[arg(short, long)]
        filter: Option<String>,

        /// Sort tasks by (date, priority, due-date)
        #[arg(short = 's', long)]
        sort_by: Option<String>,
    },

    /// Mark a task as complete
    Complete {
        /// Task ID to complete
        id: u32,
    },

    /// Delete a task
    Delete {
        /// Task ID to delete
        id: u32,
    },

    /// Edit an existing task
    Edit {
        /// Task ID to edit
        id: u32,

        /// New title
        #[arg(short = 't', long)]
        title: Option<String>,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// New priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// New due date (YYYY-MM-DD format)
        #[arg(short = 'u', long)]
        due_date: Option<String>,
    },

    /// Search tasks by keyword
    Search {
        /// Search query
        query: String,
    },

    /// Show task statistics
    Stats,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add {
            title,
            description,
            priority,
            tags,
            due_date,
        } => commands::add_task(title, description, priority, tags, due_date),

        Commands::List { all, filter, sort_by } => commands::list_tasks(all, filter, sort_by),

        Commands::Complete { id } => commands::complete_task(id),

        Commands::Delete { id } => commands::delete_task(id),

        Commands::Edit {
            id,
            title,
            description,
            priority,
            tags,
            due_date,
        } => commands::edit_task(id, title, description, priority, tags, due_date),

        Commands::Search { query } => commands::search_tasks(query),

        Commands::Stats => commands::show_stats(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
